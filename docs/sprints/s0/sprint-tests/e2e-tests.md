# Sprint 0 End-to-End Results

Tested head: `2683e908854d1635645e23e5970bca2a4f1e9023`.
All four tests in `tests/cli.rs` passed using the actual Cargo-built executable.
[Canonical output](cargo-test.log).

| Named executed test | Locked clause / acceptance | Result |
| --- | --- | --- |
| cli_fixture_run_replay_report | T-007 E1, T-005 E1/E2, T-006 E1/E2/E3; AC-4/5/6/7 | pass; two pairs, two abstentions, pending-only jobs, all owner memories, report/catalog and byte-identical replay; existing output refused |
| cli_sibling_plan_records_requests_only | T-007 E1, T-005 E2, T-006 E3; AC-5/6/7 | pass; one pair/batch, two methods, original parent/base shared, zero executed merges/admitted children, identical replay |
| cli_local_http_pipeline | T-007 E2, T-004 E1; AC-2/4/7 | pass; six real HTTP requests through all stages, canary checks, pair/pending batch; offline replay after server exits |
| cli_local_failures_preserve_status | T-007 E2, T-004 E2; AC-2/4/7 | pass; malformed, non-success and timeout remain failures, never abstention; CLI output redacted and replay identical |

Manual `documented_smoke_and_required_checks` (T-007 E3/T-001 E3):

- `cargo fmt --check`: exit 0.
- `cargo clippy --locked --all-targets -- -D warnings`: exit 0.
- `cargo test --locked`: 25 library + four CLI tests passed, zero failures.
- Fixture run to `runs/s0-fixture-smoke`: two round-pairs/two abstentions, two
  pending batches/requests; population three; no children.
- Replay to `runs/s0-fixture-replay`: same results and artifact bytes (also asserted
  in the CLI test).
- Sibling run to `runs/s0-sibling-smoke`: one batch, two requests, one abstainer,
  no executed fusion/admitted child. All commands exited zero.

These ignored `runs/` artifacts contain synthetic fixture data and are local
inspection aids; canonical assertions live in the committed tests. Hosted CI has
not executed, and live-model/tensor-fusion E2E remains explicitly not-yet-possible.
[INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md) unlocks actual
checked checkpoints, a pinned merger, evaluation and admission. No fixture result
supports a consciousness, equilibrium, diversity, or fusion-quality conclusion.
