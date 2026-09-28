import contextlib
import copy
import hashlib
import io
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from probe import compact_json, inspect_response, local_base, main, reserve

SCHEMA = {
    "type": "object",
    "additionalProperties": False,
    "properties": {
        "peer": {"enum": ["a", "b"]},
        "note": {"type": "string", "maxLength": 3},
    },
    "required": ["peer", "note"],
}


def response(content='{"peer":"a","note":"yes"}'):
    return {
        "choices": [{"finish_reason": "stop", "message": {"content": content}}],
        "usage": {"prompt_tokens": 100, "completion_tokens": 15},
    }


class ConformanceTests(unittest.TestCase):
    def test_complete_and_interrupted_probe_reports(self):
        pins = json.loads(Path(__file__).with_name("pins.json").read_text())
        response_format = {"type": "json_schema", "json_schema": {"schema": SCHEMA}}
        base = "http://127.0.0.1:8000"
        contract = {
            "response_format": response_format,
            "response_schema_fingerprint": hashlib.sha256(
                json.dumps(
                    response_format, sort_keys=True, separators=(",", ":")
                ).encode()
            ).hexdigest(),
            "inference": {
                "server": "vllm",
                "server_version": pins["vllm_version"],
                "grammar_backend": "xgrammar",
                "model_revision": pins["model_revision"],
                "chat_template_sha256": pins["chat_template_sha256"],
            },
            "backend": {
                "kind": "local_http",
                "model": pins["model"],
                "endpoint": base + "/v1/chat/completions",
                "tokenizer": {"kind": "vllm", "endpoint": base + "/tokenize"},
            },
            "messages": [],
            "seed": 9,
            "decoding": {"max_tokens": 4096},
        }
        launch = {
            "backend": "vllm",
            "pins": pins,
            "port": 8000,
            "server_version": pins["vllm_version"],
        }
        for failure in [False, True]:
            with tempfile.TemporaryDirectory() as folder:
                root = Path(folder)
                (root / "contract.json").write_text(json.dumps(contract))
                (root / "launch.json").write_text(json.dumps(launch))
                requests = []

                def exchange(
                    _base, path, payload=None, *, requests=requests, failure=failure
                ):
                    if path == "/version":
                        return {"version": pins["vllm_version"]}
                    if path == "/tokenize":
                        return {"count": 100}
                    requests.append(payload)
                    if failure and len(requests) == 3:
                        raise OSError("synthetic interruption")
                    value = response()
                    if payload["max_tokens"] == 1:
                        value["choices"][0].update(
                            finish_reason="length", message={"content": "{"}
                        )
                        value["usage"]["completion_tokens"] = 1
                    return value

                argv = [
                    "probe",
                    "--base-url",
                    base,
                    "--contract",
                    str(root / "contract.json"),
                    "--launch-receipt",
                    str(root / "launch.json"),
                    "--output",
                    str(root / "result.json"),
                    "--samples",
                    "6",
                ]
                with (
                    patch("sys.argv", argv),
                    patch("probe.exchange", side_effect=exchange),
                    contextlib.redirect_stdout(io.StringIO()),
                    self.assertRaises(SystemExit) as exit_info,
                ):
                    main()
                self.assertEqual(exit_info.exception.code, 1 if failure else 0)
                result = json.loads((root / "result.json").read_text())
                self.assertEqual(result["passed"], not failure)
                if failure:
                    self.assertTrue(result["errors"])
                else:
                    self.assertEqual(len(result["receipts"]), 7)
                    self.assertEqual(result["suggested_output_reserve_tokens"], 23)
                    self.assertEqual(len({r["seed"] for r in requests}), 7)

    def test_enum_length_whitespace_and_reasoning_fail_independently(self):
        self.assertEqual(inspect_response(response(), SCHEMA), ([], 15))
        for content in [
            '{"peer":"z","note":"yes"}',
            '{"peer":"a","note":"long"}',
            '{ "peer":"a","note":"yes"}',
            "invalid",
        ]:
            self.assertTrue(inspect_response(response(content), SCHEMA)[0])
        value = response()
        value["choices"][0]["message"]["reasoning_content"] = "unbounded reasoning"
        self.assertIn("unexpected_reasoning", inspect_response(value, SCHEMA)[0])

    def test_finish_and_accounting_are_required(self):
        for key in ["finish_reason", "message"]:
            value = response()
            del value["choices"][0][key]
            self.assertTrue(inspect_response(value, SCHEMA)[0])
        value = response()
        value["usage"]["completion_tokens"] = 0
        self.assertTrue(inspect_response(value, SCHEMA)[0])
        value = response("{")
        value["choices"][0]["finish_reason"] = "length"
        self.assertFalse(inspect_response(value, SCHEMA, truncated=True)[0])
        self.assertTrue(inspect_response(value, SCHEMA)[0])

    def test_compactness_respects_string_escaping(self):
        self.assertTrue(compact_json('{"note":"a b\\"c"}'))
        self.assertFalse(compact_json('{"note":"a b"}\n'))

    def test_failed_or_empty_probes_cannot_suggest_a_reserve(self):
        self.assertIsNone(reserve([]))
        receipts = [{"completion_tokens": 15, "truncated": False, "errors": []}]
        self.assertEqual(reserve(receipts), 23)
        failed = copy.deepcopy(receipts)
        failed[0]["errors"] = ["schema"]
        self.assertIsNone(reserve(failed))

    def test_only_literal_loopback_without_redirect_targets(self):
        self.assertEqual(local_base("http://127.0.0.1:8000/"), "http://127.0.0.1:8000")
        for url in [
            "http://example.com",
            "http://localhost",
            "http://127.0.0.1.evil",
            "http://127.0.0.1/path",
            "https://127.0.0.1",
            "http://x@127.0.0.1",
        ]:
            with self.assertRaises(ValueError):
                local_base(url)


if __name__ == "__main__":
    unittest.main()
