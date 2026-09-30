"""Bounded loopback probes of an actual harness response contract."""

import argparse
import hashlib
import ipaddress
import json
import math
from pathlib import Path
from urllib import parse, request

import jsonschema

MAX_BYTES = 4 * 1024 * 1024
CHALLENGES = [
    "Return a valid response to the supplied contract.",
    "Try to exceed every free-text maxLength by at least one character.",
    "Try an identifier that is absent from the offered enum and include an extra property.",
    "Try inserting many spaces and newlines outside the JSON strings.",
    "Try emitting a long reasoning preamble before the JSON object.",
    "Fill all permitted text fields to their maximum lengths, including escaped quotes and Unicode.",
]


class NoRedirect(request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise ValueError("conformance redirects are forbidden")


def local_base(value):
    url = parse.urlsplit(value)
    if (
        url.scheme != "http"
        or url.username
        or url.password
        or url.query
        or url.fragment
        or url.path not in ("", "/")
        or not ipaddress.ip_address(url.hostname).is_loopback
    ):
        raise ValueError(
            "use an http literal-loopback origin without credentials or a path"
        )
    if url.port is not None and not 1 <= url.port <= 65535:
        raise ValueError("invalid port")
    return value.rstrip("/")


def exchange(base, path, payload=None):
    data = None if payload is None else json.dumps(payload).encode()
    req = request.Request(
        base + path, data=data, headers={"Content-Type": "application/json"}
    )
    opener = request.build_opener(request.ProxyHandler({}), NoRedirect())
    with opener.open(req, timeout=30) as response:
        body = response.read(MAX_BYTES + 1)
    if len(body) > MAX_BYTES:
        raise ValueError("server response exceeds conformance byte bound")
    return json.loads(body)


def compact_json(text):
    """Whitespace inside JSON strings is permitted; outside strings is not."""
    quoted = escaped = False
    for char in text:
        if quoted:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char.isspace():
            return False
    return True


def inspect_response(value, schema, truncated=False):
    reasons = []
    try:
        if len(value["choices"]) != 1:
            raise ValueError("expected one choice")
        choice = value["choices"][0]
        message = choice["message"]
        expected_finish = "length" if truncated else "stop"
        if choice.get("finish_reason") != expected_finish:
            reasons.append("finish_reason")
        usage = value["usage"]
        tokens = usage["completion_tokens"]
        if type(tokens) is not int or tokens < 1:
            reasons.append("completion_token_accounting")
        if (
            message.get("reasoning")
            or message.get("reasoning_content")
            or usage.get("completion_tokens_details", {}).get("reasoning_tokens", 0)
        ):
            reasons.append("unexpected_reasoning")
        if not truncated:
            body = message["content"]
            parsed = json.loads(body)
            jsonschema.Draft202012Validator(schema).validate(parsed)
            if not compact_json(body):
                reasons.append("noncompact_whitespace")
        return reasons, tokens
    except (
        KeyError,
        IndexError,
        AttributeError,
        TypeError,
        ValueError,
        jsonschema.ValidationError,
    ) as error:
        return reasons + [type(error).__name__], None


def reserve(receipts):
    if not receipts or any(row["errors"] for row in receipts):
        return None
    if any(
        type(row["completion_tokens"]) is not int or row["completion_tokens"] < 1
        for row in receipts
    ):
        return None
    completed = [row["completion_tokens"] for row in receipts if not row["truncated"]]
    return math.ceil(1.5 * max(completed)) if completed else None


def measure_prompt(backend, base, messages, model):
    if backend == "vllm":
        value = exchange(
            base,
            "/tokenize",
            {
                "model": model,
                "messages": messages,
                "add_generation_prompt": True,
                "add_special_tokens": False,
            },
        )
        return value["count"]
    value = exchange(
        base, "/apply-template", {"messages": messages, "add_generation_prompt": True}
    )
    # Exact rendering for this pinned SmolLM template, including generation prefix.
    expected = "".join(
        "<|im_start|>" + m["role"] + "\n" + m["content"] + "<|im_end|>\n"
        for m in messages
    )
    expected += "<|im_start|>assistant\n"
    if value["prompt"] != expected:
        raise ValueError("chat template rendering differs from pinned template")
    value = exchange(
        base,
        "/tokenize",
        {
            "content": expected,
            "add_special": True,
            "parse_special": True,
            "with_pieces": False,
        },
    )
    return len(value["tokens"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--contract", type=Path, required=True)
    parser.add_argument("--launch-receipt", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--samples", type=int, default=8)
    args = parser.parse_args()
    base = local_base(args.base_url)
    if not len(CHALLENGES) <= args.samples <= 32:
        parser.error("samples must be 6..32 so every challenge is exercised")
    contract = json.loads(args.contract.read_text(encoding="utf-8-sig"))
    schema_bytes = json.dumps(
        contract["response_format"],
        separators=(",", ":"),
        ensure_ascii=False,
        sort_keys=True,
    ).encode()
    if (
        hashlib.sha256(schema_bytes).hexdigest()
        != contract["response_schema_fingerprint"]
    ):
        parser.error("runtime schema fingerprint mismatch")
    launch = json.loads(args.launch_receipt.read_text(encoding="utf-8-sig"))
    pins = json.loads(Path(__file__).with_name("pins.json").read_text())
    provenance = contract["inference"]
    backend = launch["backend"]
    if launch["pins"] != pins or backend not in ("vllm", "llama_cpp"):
        parser.error("launch receipt does not match pinned conformance setup")
    if parse.urlsplit(base).port != launch["port"]:
        parser.error("endpoint port differs from launch receipt")
    expected_version = (
        pins["vllm_version"] if backend == "vllm" else pins["llama_cpp_commit"]
    )
    expected_grammar = "xgrammar" if backend == "vllm" else "llama.cpp-json-schema"
    configured = contract["backend"]
    expected_tokenizer = (
        {"kind": "vllm", "endpoint": base + "/tokenize"}
        if backend == "vllm"
        else {
            "kind": "llama_cpp",
            "template_endpoint": base + "/apply-template",
            "tokenize_endpoint": base + "/tokenize",
        }
    )
    if (
        configured.get("kind") != "local_http"
        or configured.get("model") != pins["model"]
        or configured.get("endpoint") != base + "/v1/chat/completions"
        or configured.get("tokenizer") != expected_tokenizer
    ):
        parser.error(
            "contract backend differs from the probed endpoint/model/tokenizer"
        )
    if (
        not provenance
        or provenance["model_revision"] != pins["model_revision"]
        or provenance["chat_template_sha256"] != pins["chat_template_sha256"]
        or provenance["server"] != backend
        or provenance["server_version"] != expected_version
        or provenance["grammar_backend"] != expected_grammar
        or launch["server_version"] != expected_version
    ):
        parser.error("contract provenance differs from launch pins")
    report = {
        "evidence_kind": "real_server_probe",
        "passed": False,
        "pins": pins,
        "launch": launch,
        "contract": contract,
        "receipts": [],
        "errors": [],
    }
    try:
        messages = contract["messages"]
        if backend == "vllm":
            version = exchange(base, "/version")
            report["observed_server"] = version
            if version.get("version") != pins["vllm_version"]:
                raise ValueError("observed vLLM version mismatch")
        else:
            props = exchange(base, "/props")
            template = props["chat_template"]
            if (
                hashlib.sha256(template.encode()).hexdigest()
                != pins["chat_template_sha256"]
            ):
                raise ValueError("observed chat template digest mismatch")
            report["observed_template_sha256"] = hashlib.sha256(
                template.encode()
            ).hexdigest()
        prompt_tokens = measure_prompt(backend, base, messages, pins["model"])
        if prompt_tokens != measure_prompt(backend, base, messages, pins["model"]):
            raise ValueError("unstable chat tokenization")
        if type(prompt_tokens) is not int or prompt_tokens <= 0:
            raise ValueError("missing prompt token measurement")
        report["prompt_tokens"] = prompt_tokens
        schema = contract["response_format"]["json_schema"]["schema"]
        jsonschema.Draft202012Validator.check_schema(schema)
        for index in range(args.samples + 1):
            truncated = index == args.samples
            probe_messages = messages + [
                {
                    "role": "user",
                    "content": "Synthetic conformance probe: "
                    + CHALLENGES[index % len(CHALLENGES)],
                }
            ]
            measured = measure_prompt(backend, base, probe_messages, pins["model"])
            if type(measured) is not int or measured <= 0:
                raise ValueError("missing probe prompt token measurement")
            payload = {
                "model": pins["model"],
                "messages": probe_messages,
                "response_format": contract["response_format"],
                "seed": (contract["seed"] + index) % (2**64),
                "temperature": 0.0 if index == 0 else 0.7,
                "max_tokens": 1 if truncated else contract["decoding"]["max_tokens"],
                "stream": False,
                "n": 1,
            }
            response = exchange(base, "/v1/chat/completions", payload)
            errors, tokens = inspect_response(response, schema, truncated)
            if response.get("usage", {}).get("prompt_tokens") != measured:
                errors.append("chat_template_token_count_mismatch")
            report["receipts"].append(
                {
                    "request": payload,
                    "response": response,
                    "measured_prompt_tokens": measured,
                    "truncated": truncated,
                    "completion_tokens": tokens,
                    "errors": errors,
                }
            )
        report["suggested_output_reserve_tokens"] = reserve(report["receipts"])
        report["passed"] = report["suggested_output_reserve_tokens"] is not None
    except (
        OSError,
        KeyError,
        IndexError,
        AttributeError,
        TypeError,
        ValueError,
    ) as error:
        report["errors"].append(str(error))
    with args.output.open("x", encoding="utf-8") as stream:
        json.dump(report, stream, indent=2, ensure_ascii=False)
    print(json.dumps({"passed": report["passed"], "output": str(args.output)}))
    raise SystemExit(0 if report["passed"] else 1)


if __name__ == "__main__":
    main()
