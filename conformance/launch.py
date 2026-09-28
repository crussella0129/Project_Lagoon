"""Launch a pinned conformance server; never install packages or download a GGUF."""

import argparse
import hashlib
import importlib.metadata
import json
import subprocess
import sys
from pathlib import Path


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("backend", choices=["vllm", "llama_cpp"])
    parser.add_argument("--template", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--port", type=int, default=8000)
    parser.add_argument("--llama-server", type=Path)
    parser.add_argument("--gguf", type=Path)
    parser.add_argument("--gguf-sha256")
    args = parser.parse_args()
    pins = json.loads(Path(__file__).with_name("pins.json").read_text())
    if not 1024 <= args.port <= 65535:
        parser.error("port must be 1024..65535")
    if digest(args.template) != pins["chat_template_sha256"]:
        parser.error("template digest does not match the pinned model")
    receipt = {"backend": args.backend, "pins": pins, "port": args.port}
    if args.backend == "vllm":
        version = importlib.metadata.version("vllm")
        if version != pins["vllm_version"]:
            parser.error(
                "install the pinned vLLM version in a separate serving environment"
            )
        command = [
            sys.executable,
            "-m",
            "vllm.entrypoints.openai.api_server",
            "--model",
            pins["model"],
            "--revision",
            pins["model_revision"],
            "--tokenizer-revision",
            pins["model_revision"],
            "--chat-template",
            str(args.template.resolve()),
            "--host",
            "127.0.0.1",
            "--port",
            str(args.port),
            "--max-model-len",
            "8192",
            "--dtype",
            "float16",
            "--structured-outputs-config",
            '{"backend":"xgrammar","disable_any_whitespace":true}',
        ]
        receipt["server_version"] = version
    else:
        if not all([args.llama_server, args.gguf, args.gguf_sha256]):
            parser.error(
                "llama.cpp requires an explicit binary, GGUF and expected artifact digest"
            )
        if digest(args.gguf) != args.gguf_sha256:
            parser.error("GGUF digest mismatch")
        version = subprocess.run(
            [str(args.llama_server.resolve()), "--version"],
            capture_output=True,
            text=True,
            check=True,
            timeout=15,
        )
        version_text = version.stdout + version.stderr
        if (
            pins["llama_cpp_commit"][:9] not in version_text
            or pins["llama_cpp_build"] not in version_text
        ):
            parser.error("llama.cpp build does not match pins.json")
        command = [
            str(args.llama_server.resolve()),
            "--model",
            str(args.gguf.resolve()),
            "--alias",
            pins["model"],
            "--chat-template-file",
            str(args.template.resolve()),
            "--jinja",
            "--reasoning",
            "off",
            "--host",
            "127.0.0.1",
            "--port",
            str(args.port),
            "--ctx-size",
            "8192",
            "--parallel",
            "1",
        ]
        receipt.update(
            {
                "server_version": pins["llama_cpp_commit"],
                "binary_sha256": digest(args.llama_server),
                "gguf_sha256": args.gguf_sha256,
            }
        )
    receipt["command"] = command
    # A launch receipt is provenance, not a passing conformance result.
    with args.receipt.open("x", encoding="utf-8") as stream:
        json.dump(receipt, stream, indent=2)
    subprocess.run(command, check=True)


if __name__ == "__main__":
    main()
