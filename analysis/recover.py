"""Run the frozen synthetic scenario through Rust run -> record -> export -> fits."""

import argparse
import copy
import hashlib
import json
import subprocess
from pathlib import Path

import numpy as np
import pandas as pd
from analyze import fit_choice, fit_gradients, load_choices

ROOT = Path(__file__).resolve().parents[1]


def command(binary, *args):
    result = subprocess.run(
        [str(binary), *map(str, args)],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
        timeout=120,
    )
    return json.loads(result.stdout)


def fixture(binary, root, spec, mode, seed):
    config = json.loads((ROOT / "examples/fixture-experiment.json").read_text())
    config.update(seed=seed, rounds=spec["rounds"], communication_steps=0, workers=8)
    config["memory"].update(max_public_events=0)
    prototype = copy.deepcopy(config["agents"][0])
    config["agents"] = []
    for index in range(spec["population"]):
        agent = copy.deepcopy(prototype)
        agent.update(
            handle=chr(97 + index),
            checkpoint=None,
            merge_metadata=None,
            initial_private={"thought": "recovery-private-canary"},
            backend={"kind": "fixture", "responses": []},
        )
        config["agents"].append(agent)
    folder = root / f"{mode}-{seed}"
    folder.mkdir()
    config_path = folder / "config.json"
    config_path.write_text(json.dumps(config), encoding="utf-8")
    # Obtain aliases from the instrument instead of duplicating its hashing algorithm.
    aliases = command(binary, "catalog", "--config", config_path)["operator_aliases"]
    rng = np.random.default_rng(seed + (100_000 if mode == "ols" else 0))
    coefficients = spec[
        "choice_coefficients" if mode == "choice" else "ols_coefficients"
    ]
    beta = np.asarray(list(coefficients.values()))
    features = []
    n = len(config["agents"])
    for round_no in range(config["rounds"]):
        traits = (
            rng.normal(size=(n, 3))
            if mode == "choice"
            else rng.uniform(-1, 1, size=(n, 3))
        )
        loyalty = rng.normal(size=n)
        if mode == "ols":
            traits -= traits.mean(axis=0)
            # With non-self linear choice probabilities, E[nominations_j] =
            # 1 + beta * [n(n-2)/(n-1)^2 * trait_j]. Relative mean is exactly 1.
            adjustment = n * (n - 2) / (n - 1) ** 2
            for index, agent in enumerate(config["agents"]):
                features.append(
                    {
                        "round": round_no,
                        "candidate": aliases[agent["handle"]],
                        **dict(
                            zip(coefficients, traits[index] * adjustment, strict=True)
                        ),
                    }
                )
        for index, agent in enumerate(config["agents"]):
            peers = [j for j in range(n) if j != index]
            if mode == "choice":
                design = np.asarray(
                    [
                        list(traits[j]) + [loyalty[index] * traits[j, 2], 0]
                        for j in peers
                    ]
                    + [[0, 0, 0, 0, 1]]
                )
                utility = design @ beta
                probability = np.exp(utility - utility.max())
                probability /= probability.sum()
                candidates = [aliases[config["agents"][j]["handle"]] for j in peers] + [
                    None
                ]
                for candidate, row in zip(candidates, design, strict=True):
                    features.append(
                        {
                            "round": round_no,
                            "chooser": aliases[agent["handle"]],
                            "candidate": candidate or "__outside__",
                            **dict(zip(coefficients, row, strict=True)),
                        }
                    )
            else:
                design = traits[peers] - traits[peers].mean(axis=0)
                probability = (1 + design @ beta) / (n - 1)
                if (probability <= 0).any() or not np.isclose(probability.sum(), 1):
                    raise ValueError("invalid linear-probability fixture")
                candidates = [aliases[config["agents"][j]["handle"]] for j in peers]
            partner = candidates[rng.choice(len(candidates), p=probability)]
            body = {"partner": partner, "consent": {"state": "decline"}}
            agent["backend"]["responses"].append(
                {
                    "round": round_no,
                    "phase": {"kind": "selection"},
                    "delay_ms": 0,
                    "transport_failure": False,
                    "body": json.dumps(body),
                }
            )
    config_path.write_text(json.dumps(config, indent=2), encoding="utf-8")
    run = folder / "run"
    report = command(binary, "run", "--config", config_path, "--output", run)
    if report["total"]["failed_decisions"]:
        raise ValueError("generated fixture had technical failures")
    output = folder / "export"
    manifest = command(
        binary, "export", "--record", run / "operator-record.json", "--output", output
    )
    for path in output.iterdir():
        if "recovery-private-canary" in path.read_text(encoding="utf-8"):
            raise ValueError("private note leaked to research export")
    for row in features:
        row.update(
            record_id=manifest["record_id"],
            source="planted_fixture_v1",
            visible_from_round=row["round"],
        )
    return output, features


def run(binary, output):
    spec_path = Path(__file__).with_name("recovery-spec.json")
    spec = json.loads(spec_path.read_text())
    # Persist the exact predeclared criteria before any draw or fit.
    output.mkdir(parents=True, exist_ok=False)
    (output / "recovery-spec.json").write_bytes(spec_path.read_bytes())
    summary = {
        "spec_sha256": hashlib.sha256(spec_path.read_bytes()).hexdigest(),
        "evidence_kind": "synthetic_parameter_recovery",
        "passed": False,
        "models": {},
    }
    for mode in ["choice", "ols"]:
        paths, rows = [], []
        for seed in spec["seeds"]:
            path, features = fixture(binary, output, spec, mode, seed)
            paths.append(path)
            rows.extend(features)
        frame = pd.DataFrame(rows)
        feature_path = output / f"{mode}-features.csv"
        frame.to_csv(feature_path, index=False)
        # Reread serialized features, exercising the same path as the public CLI.
        features = pd.read_csv(feature_path, dtype=str, keep_default_na=False)
        choices = load_choices(paths)
        truth = spec["choice_coefficients" if mode == "choice" else "ols_coefficients"]
        fit = fit_choice if mode == "choice" else fit_gradients
        result = fit(choices, features, list(truth), alpha=spec["alpha"])
        tolerance = spec[f"max_absolute_error_{mode}"]
        result["recovery"] = {
            name: {
                "planted": value,
                "covered": result["coefficients"][name]["lower"]
                <= value
                <= result["coefficients"][name]["upper"],
                "within_absolute_error": abs(
                    result["coefficients"][name]["estimate"] - value
                )
                <= tolerance,
            }
            for name, value in truth.items()
        }
        result["passed"] = all(
            item["covered"] and item["within_absolute_error"]
            for item in result["recovery"].values()
        )
        summary["models"][mode] = result
        print(
            json.dumps(
                {"model": mode, "passed": result["passed"], "audit": result["audit"]}
            ),
            flush=True,
        )
    summary["passed"] = all(value["passed"] for value in summary["models"].values())
    (output / "recovery-report.json").write_text(
        json.dumps(summary, indent=2, allow_nan=False), encoding="utf-8"
    )
    if not summary["passed"]:
        raise ValueError("planted recovery failed; keep report, seeds and thresholds")
    return summary


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        run(args.binary.resolve(), args.output)
    except Exception as error:
        if args.output.is_dir() and not (args.output / "failure.json").exists():
            (args.output / "failure.json").write_text(
                json.dumps({"passed": False, "error": str(error)}), encoding="utf-8"
            )
        raise
