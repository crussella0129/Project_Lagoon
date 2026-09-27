# Completed Tasks Log (Append-Only)

## T-001 (sprint 0)
- **Description:** Establish Rust crate, bounded configuration, exact catalog fingerprints, and Rust CI checks. Isolate workspace from unrelated parent manifest.
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** Cargo.toml, Cargo.lock, src/lib.rs, src/config.rs, src/protocol.rs, .gitignore, .github/workflows/sprint-loops-ci.yml, .github/dependabot.yml, INT-0001
- **Commit:** `40ee2c5bd272390d20c68400adac9a9d39ec6678`

## T-002 (sprint 0)
- **Description:** isolate private observations and sealed response fields
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/lib.rs, src/observation.rs, src/protocol.rs
- **Commit:** `b208dce5a4f128b3f38f1824c355cd1d29549248`

## T-003 (sprint 0)
- **Description:** run bounded phases with frozen snapshots and sealed nominations
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/lib.rs, src/backend/mod.rs, src/backend/fixture.rs, src/runner.rs
- **Commit:** `a82b126e6b47430d9a0677560bb63550b0cb53c0`

## T-004 (sprint 0)
- **Description:** add bounded loopback chat completions adapter
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/backend/mod.rs, src/backend/local_http.rs
- **Commit:** `debc7d99e553d3076224c1911dc6a0bc1803d78c`

## T-005 (sprint 0)
- **Description:** screen exact mutual plans and preserve sibling parentage
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/lib.rs, src/matching.rs, src/merge_request.rs, src/runner.rs
- **Commit:** `5b6ad562984fb8a19a10940ffb0206010b5e7c72`

## T-006 (sprint 0)
- **Description:** persist private records and replay descriptive results
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/lib.rs, src/record.rs, src/replay.rs, src/report.rs, src/merge_request.rs
- **Commit:** `a3e8559c89f1582137d99e663798323d026f2cd2`

## T-007 (sprint 0)
- **Description:** expose tested CLI workflows and complete manifest accounting
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27
- **Files modified:** src/main.rs, examples/fixture-experiment.json, examples/sibling-experiment.json, examples/local-experiment.json, tests/cli.rs, docs/usage.md, README.md, docs/SUMMARY.md, src/config.rs, src/merge_request.rs, src/report.rs, src/runner.rs
- **Commit:** `f3133713db236d10d02f6f4f895050caa2d6cd95`

## T-011 (post-Sprint 0 maintenance)
- **Description:** Repair format/consent, randomized presentation and token-aware memory; verify the bounded Rust variance study and draft the next merge-screening experiment. Publish a follow-up after the user merged PR #1.
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md); proposed follow-on research for [INT-0002](../intents/INT-0002-evaluated-dare-descendants.md) and [INT-0003](../intents/INT-0003-controlled-evolutionary-study.md)
- **Completed:** 2026-09-27
- **Files modified:** Cargo.toml, Cargo.lock, src/backend, src/config.rs, src/observation.rs, src/protocol.rs, src/runner.rs, src/record.rs, src/replay.rs, src/report.rs, src/main.rs, tests/cli.rs, schemas, examples, .github/workflows/sprint-loops-ci.yml, README.md, PREREG.md, docs/usage.md, docs/intents, docs/research, docs/work/instrument-review.md, docs/SUMMARY.md
- **Commit:** `c06cce7008297639d7f4f210714a83088dec7dbe`
- **Verification:** [38 local tests, format and clippy](../research/instrument-validation.md); [PR #5](https://github.com/crussella0129/Lovers_Lagoon/pull/5). Hosted CI failed before runner execution; this maintenance completion does not realize INT-0001 or close T-008.

## T-012 (post-Sprint 0 maintenance)
- **Description:** Restore main's sha2 0.11 build compatibility, bound response strings and diagnose output token limits with replayable finish-reason receipts; verify matching-reference probabilities and revise pilot coverage/opportunity criteria while deferring training.
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md); draft follow-on research for [INT-0002](../intents/INT-0002-evaluated-dare-descendants.md) and [INT-0003](../intents/INT-0003-controlled-evolutionary-study.md)
- **Completed:** 2026-09-27
- **Files modified:** Cargo.toml, Cargo.lock, src/backend, src/config.rs, src/observation.rs, src/protocol.rs, src/runner.rs, src/record.rs, src/replay.rs, src/report.rs, schemas, examples, README.md, PREREG.md, docs/usage.md, docs/research/output-review.md, docs/work/output-review.md, docs/intents/INT-0001-voluntary-observable-partner-choice.md, docs/work/tasks.md, docs/SUMMARY.md
- **Commit:** `28585c58c6dc02c7c733794ff650fbba90d22207` (tested implementation; subsequent evidence-only commit reconciles acceptance state).
- **Verification:** [locked build, format, clippy and 45 passing local tests](output-review.md#local-acceptance-evidence); [PR #6](https://github.com/crussella0129/Lovers_Lagoon/pull/6). Per the user's explicit local-testing override, hosted CI is optional and INT-0001 is realized for the harness increment. T-008 remains an optional backlog follow-up. No real server conformance, training, fusion, external registration or live matched-exit treatment occurred.
