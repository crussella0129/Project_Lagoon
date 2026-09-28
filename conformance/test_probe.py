import copy
import unittest

from probe import compact_json, inspect_response, local_base, reserve

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
