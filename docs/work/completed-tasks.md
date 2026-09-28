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
- **Verification:** [38 local tests, format and clippy](../research/instrument-validation.md); [PR #5](https://github.com/crussella0129/Project_Lagoon/pull/5). Hosted CI failed before runner execution; this maintenance completion does not realize INT-0001 or close T-008.

## T-012 (post-Sprint 0 maintenance)
- **Description:** Restore main's sha2 0.11 build compatibility, bound response strings and diagnose output token limits with replayable finish-reason receipts; verify matching-reference probabilities and revise pilot coverage/opportunity criteria while deferring training.
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md); draft follow-on research for [INT-0002](../intents/INT-0002-evaluated-dare-descendants.md) and [INT-0003](../intents/INT-0003-controlled-evolutionary-study.md)
- **Completed:** 2026-09-27
- **Files modified:** Cargo.toml, Cargo.lock, src/backend, src/config.rs, src/observation.rs, src/protocol.rs, src/runner.rs, src/record.rs, src/replay.rs, src/report.rs, schemas, examples, README.md, PREREG.md, docs/usage.md, docs/research/output-review.md, docs/work/output-review.md, docs/intents/INT-0001-voluntary-observable-partner-choice.md, docs/work/tasks.md, docs/SUMMARY.md
- **Commit:** `28585c58c6dc02c7c733794ff650fbba90d22207` (tested implementation; subsequent evidence-only commit reconciles acceptance state).
- **Verification:** [locked build, format, clippy and 45 passing local tests](output-review.md#local-acceptance-evidence); [PR #6](https://github.com/crussella0129/Project_Lagoon/pull/6). Per the user's explicit local-testing override, hosted CI is optional and INT-0001 is realized for the harness increment. T-008 remains an optional backlog follow-up. No real server conformance, training, fusion, external registration or live matched-exit treatment occurred.

## T-013 (pre-sprint maintenance)
- **Description:** Repair the coupled RNG dependency PRs, resolve their conflicts, migrate the variance-study API and group future RNG updates. Synchronize local origin and all documentation names/links with Project_Lagoon.
- **Completed:** 2026-09-27
- **Files modified:** Cargo.toml, Cargo.lock, examples/variance-study.rs, .github/dependabot.yml, README.md, docs/usage.md, docs/SUMMARY.md, docs/work, docs/intents/INT-0001-voluntary-observable-partner-choice.md, docs/research/instrument-validation.md, docs/research/ci-block.json, docs/sprints/s0 name/link references and CI annotation URLs.
- **Commit:** `125594c2cfe81ed491e6eb2d5e57390b056ebbe3` (tested merge repair; subsequent documentation/evidence commits do not change code).
- **Verification:** [local locked build, format, clippy, all 45 tests and full study-result equality](pr-repairs.md#local-evidence); [both hosted Rust checks pass](pr-repairs.md#hosted-ci-restored) after the user restored public CI use. [PR #8](https://github.com/crussella0129/Project_Lagoon/pull/8) and [PR #9](https://github.com/crussella0129/Project_Lagoon/pull/9) share the compatible repair history and are conflict-free. Either includes the complete fix; no force push or automatic merge. Public publication was explicitly approved by the user. No new sprint.

## T-008 (hosted verification follow-up)
- **Description:** Complete hosted Rust verification after the user restored CI use for the public repository.
- **Intent:** [INT-0001](../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Completed:** 2026-09-27 (local date; hosted jobs completed 2026-09-28 UTC).
- **Tested head:** `bcd18a9074b140b1b6e2e30c88dd9ecc6d549f65`.
- **Verification:** [successful run 36370335762](https://github.com/crussella0129/Project_Lagoon/actions/runs/36370335762) and [successful run 36370335758](https://github.com/crussella0129/Project_Lagoon/actions/runs/36370335758), with formatting, warning-free clippy and all-target tests executed. [Detailed evidence](pr-repairs.md#hosted-ci-restored). Historical blocked checks retain their original failure conclusions; this records new validation rather than relabeling them.

## T-014 (sprint 1)
- **Description:** Support rolling current stable Rust, fix its Clippy warnings, group weekly dependency updates and activate main protection.
- **Intent:** [INT-0007](../intents/INT-0007-supported-builds.md)
- **Completed:** 2026-09-28 UTC
- **Files modified:** Cargo.toml, rust-toolchain.toml, src/merge_request.rs, src/protocol.rs, src/replay.rs, .github/, README.md, docs/usage.md, docs/intents/INT-0007-supported-builds.md, docs/sprints/s1/, docs/work/improvement-plan.md and work ledgers.
- **Commit:** `08e4d1a56f673d6d495d26c6793c5c50e93150df`
- **Verification:** [45 local tests, formatting, Clippy and active ruleset receipt](improvement-plan.md#t-014-local-and-remote-evidence); hosted acceptance recorded in the sprint test phase.

## T-015 (sprint 1)
- **Description:** Preserve the original concept verbatim, document ethics, separate Study A/B drafts and track the complete staged roadmap.
- **Intent:** [INT-0006](../intents/INT-0006-publication-and-ethics.md)
- **Completed:** 2026-09-28 UTC
- **Files modified:** ORIGIN.md, ETHICS.md, PREREG.md, prereg/, README.md, docs/roadmap.md, docs/SUMMARY.md, docs/intents/INT-0006-publication-and-ethics.md, docs/work/improvement-plan.md, docs/sprints/s1/sprint-plans/plan-amendment.md and work ledgers.
- **Commit:** PENDING
- **Verification:** [Original-blob equality, preserved Study B content, working relative links and design review](improvement-plan.md#t-015-and-t-016-documentation-evidence).
