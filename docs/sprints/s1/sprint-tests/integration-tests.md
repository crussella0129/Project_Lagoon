# Sprint 1 integration results

Code under test: `bc5cec1472eda883c0278ebe1d7a4f58cb222d61` (T-025 code plus CI setup fix).
All results below are local; hosted receipt is recorded separately in E2E results.

## Rust and Python boundaries

The eight CLI tests execute the built Rust binary. Existing disposable loopback
HTTP servers exercise vLLM-style tokenization and llama.cpp template/tokenization,
generation projection, timeouts/HTTP failures and redaction. They are test doubles,
not pinned real inference servers. The new exporter test runs capture → persisted
record → export and rejects modified seed receipts and existing output directories.

Python's six analysis tests independently read Rust-generated CSV using pandas:
embedded quotes, commas, newlines and Unicode recover exactly; failures have no
chosen flag; voluntary outside choices remain present; matched-exit records have
only actual calls. Feature joins reject missing/duplicate keys, nonfinite values,
post-decision visibility and within-set constant designs. Zero-nomination ratios
stop with an explicit error instead of silently excluding a round.

## Planted-parameter recovery (T-025 / INT-0005 analysis criterion)

Command: `uv run --project analysis python analysis/recover.py --binary
target/debug/lovers-lagoon.exe --output target/recovery-final`.

The [specification](../../../../analysis/recovery-spec.json) was written before
execution. Its SHA-256 is `9f023659d42d3c64ef5014975b56a8bffdf9bf3f5702c12f40dfea5b699d378b`.
The [receipt](../../../../analysis/recovery-receipt.json) contains full estimates,
99% seed-cluster intervals, audit counts and per-coefficient checks.

| Scenario | Actual decisions | Outside choices | Technical failures | Result |
|---|---:|---:|---:|---|
| Conditional logit | 2,560 | 142 | 0 | All five planted coefficients inside intervals and fixed error limits |
| Relative-nomination OLS | 2,560 | 0 | 0 | All three planted slopes inside intervals and fixed error limits |

Each scenario uses eight seeds, 40 rounds and eight fixture participants. Fixtures
pass through the real runner, persisted operator record, replay validation, exporter,
CSV read, explicit external-feature join and model fit. The OLS generator has a
known linear expectation; its parameters are not borrowed from a nonlinear softmax.
No real model, external judge or private-note inference is involved.

The first attempt failed an independent convergence assertion because the installed
ConditionalLogit wrapper ignores optimizer keyword arguments. The supported Newton
solver fixed this; seeds, coefficients and tolerances were unchanged. Two subsequent
complete pipeline runs passed. This fixed scenario is not interval-coverage or power
calibration, and cannot alone satisfy G2.

## External configuration

GitHub's active-main-rules endpoint returns ruleset 24093103 with deletion and
non-fast-forward protection, strict required `check` bound to GitHub Actions app
15368. Earlier full ruleset inspection confirms no bypass actors and main targeting.
No destructive red merge was attempted. Canonical repository/links use Project_Lagoon;
old names remain only where preserving historical evidence or crate/CLI compatibility.
