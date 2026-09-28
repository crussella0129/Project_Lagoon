import copy
import json
import os
import tempfile
import unittest
from pathlib import Path

import pandas as pd
from analyze import (
    ROW_KEYS,
    audit,
    csv,
    fit_choice,
    join_features,
    load_choices,
    nomination_outcomes,
)
from recover import ROOT, command


class PipelineTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory()
        cls.root = Path(cls.temp.name)
        cls.binary = Path(
            os.environ.get(
                "LAGOON_BINARY",
                ROOT
                / "target/debug"
                / ("lovers-lagoon.exe" if os.name == "nt" else "lovers-lagoon"),
            )
        ).resolve()
        config = json.loads((ROOT / "examples/fixture-experiment.json").read_text())
        cls.text = 'Comma, "quote"\nUnicode 🐟'
        config["agents"][0]["backend"]["responses"][0]["body"] = json.dumps(
            {"public_message": cls.text}
        )
        cls.paths = []
        for mode in ["valid", "failure", "matched"]:
            variant = copy.deepcopy(config)
            if mode == "failure":
                variant["agents"][1]["backend"]["responses"][1].update(
                    transport_failure=True, body=None
                )
            if mode == "matched":
                variant["pairing"] = "matched_exit"
            path = cls.root / f"{mode}.json"
            path.write_text(json.dumps(variant), encoding="utf-8")
            run = cls.root / f"{mode}-run"
            command(cls.binary, "run", "--config", path, "--output", run)
            tables = cls.root / mode
            command(
                cls.binary,
                "export",
                "--record",
                run / "operator-record.json",
                "--output",
                tables,
            )
            cls.paths.append(tables)

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def test_csv_roundtrip_and_private_canary(self):
        table = csv(self.paths[0] / "messages.csv")
        self.assertEqual(table.text.iloc[0], self.text)
        self.assertEqual(len(table), 6)
        for path in self.paths[0].iterdir():
            self.assertNotIn("fixture-private", path.read_text(encoding="utf-8"))
        self.assertTrue(csv(self.paths[0] / "profiles.csv").empty)

    def test_outside_options_and_failures_are_distinct(self):
        choices = load_choices([self.paths[1]])
        info = audit(choices)
        self.assertEqual(info["failed_sets"], 1)
        self.assertEqual(info["abstained_sets"], 2)
        self.assertTrue(
            choices.loc[choices.status.eq("transport_failure"), "chosen"].eq("").all()
        )
        with self.assertRaisesRegex(ValueError, "technical failures"):
            fit_choice(choices, pd.DataFrame(), ["x"])
        with self.assertRaisesRegex(ValueError, "incomplete decisions"):
            nomination_outcomes(choices)

    def test_matched_exit_has_only_actual_choice_sets(self):
        table = load_choices([self.paths[2]])
        self.assertEqual(audit(table)["choice_sets"], 3)
        self.assertEqual(set(table["round"]), {0})
        self.assertEqual(csv(self.paths[2] / "events.csv").retired.tolist(), ["1"])

    def test_duplicates_and_changed_csv_fail(self):
        with self.assertRaisesRegex(ValueError, "duplicate record"):
            load_choices([self.paths[0], self.paths[0]])
        path = self.root / "corrupt"
        path.mkdir(exist_ok=True)
        for file in self.paths[0].iterdir():
            (path / file.name).write_bytes(file.read_bytes())
        with (path / "choices.csv").open("a") as stream:
            stream.write("corrupt")
        with self.assertRaisesRegex(ValueError, "digest mismatch"):
            load_choices([path])

    def test_feature_joins_visibility_and_identification(self):
        choices = load_choices([self.paths[0]])
        features = choices[ROW_KEYS].copy()
        features["x"] = 1.0
        features["visible_from_round"] = 0
        features["source"] = "test"
        self.assertEqual(
            len(join_features(choices, features, ROW_KEYS, ["x"])), len(choices)
        )
        with self.assertRaisesRegex(ValueError, "not identified"):
            fit_choice(choices, features, ["x"])
        with self.assertRaisesRegex(ValueError, "exactly"):
            join_features(choices, features.iloc[1:], ROW_KEYS, ["x"])
        with self.assertRaisesRegex(ValueError, "duplicate feature"):
            join_features(
                choices, pd.concat([features, features.iloc[:1]]), ROW_KEYS, ["x"]
            )
        features["visible_from_round"] = 10
        with self.assertRaisesRegex(ValueError, "visibility"):
            join_features(choices, features, ROW_KEYS, ["x"])
        features["visible_from_round"] = 0
        features.loc[0, "x"] = float("nan")
        with self.assertRaisesRegex(ValueError, "nonfinite"):
            join_features(choices, features, ROW_KEYS, ["x"])

    def test_zero_nominations_not_silently_removed(self):
        choices = load_choices([self.paths[0]])
        choices["chosen"] = choices.candidate.eq("__outside__").astype(int).astype(str)
        choices["status"] = "abstained"
        with self.assertRaisesRegex(ValueError, "zero-nomination"):
            nomination_outcomes(choices)


if __name__ == "__main__":
    unittest.main()
