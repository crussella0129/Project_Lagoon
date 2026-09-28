# Build and output review follow-up

2026-09-27: user selected **Fix those issues and revise the draft study criteria; defer training** and explicitly requested local testing without waiting for GitHub CI (minutes exhausted).

- [x] Reproduce main's locked build failure and preserve fingerprints while fixing sha2 0.11 compatibility. The restored build and all 38 pre-existing tests passed before the output changes.
- [x] Bound response strings, retain finish reasons, distinguish generation limits, and validate replay locally. Partial/complete/null-content length responses are rejected as generation limits without applying memory or ballots.
- [x] Revise pilot coverage/opportunity criteria and assess the proposed matched-exit protocol and registration services. Exact uniform references reproduce the review's ~8% and ~57% probabilities; preference-crowding estimates remain underspecified.
- [x] Run the locked local build, all-target tests, formatting and clippy; record completion and publish a reviewable follow-up. [PR #6](https://github.com/crussella0129/Project_Lagoon/pull/6) is attached to this task and awaits human merge approval.

Training, actual merging, changing the live matching protocol, and external registration remain outside this repair. The draft can identify a matched-exit treatment requiring implementation before freezing; the existing voluntary matching behavior remains the baseline.

## Local acceptance evidence

Windows, Rust/Cargo 1.98.1, 2026-09-27, v0.3.0/schema 3:

- `cargo build --locked`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --locked --all-targets -- -D warnings`: pass.
- `cargo test --locked --all-targets`: 34 library, 6 CLI, 2 pairing-reference and 3 variance-study tests; **45 passed**, no failures.
- `cargo run --locked --example pairing-baseline`: exact E[k]=0.5714285714 / P(k≥2)=0.0803783166 for one round; 1.7989088890 / 0.5698527987 for three-round matched exit under its stated uniform assumptions.
- `git diff --check` and Sprint Loops `check-book.sh`: pass. The sandbox's parent-path canonicalization warning did not invalidate the Book check.

Tests cover Unicode limits/escaped byte bounds, a known-answer hash, bounded reason metadata, normal stop recording, partial/complete/null content with length termination, saved offline replay and rejection of changed finish-reason receipts. Existing CLI tests exercise fixture/sibling configurations and disposable vLLM/llama.cpp HTTP servers. No actual model/server conformance or training is claimed.

Implementation and research assessment: [output review](../research/output-review.md). The earlier [v0.2 validation](../research/instrument-validation.md) remains historical evidence for its recorded revision. GitHub CI is outside the current acceptance gate by explicit user instruction; no hosted result was requested, polled or used.

Tested implementation: `28585c58c6dc02c7c733794ff650fbba90d22207`; [T-012 completion](completed-tasks.md#t-012-post-sprint-0-maintenance). The subsequent evidence-only update reconciles INT-0001 with the user's local acceptance override. Sprint 0's terminal failure history is preserved; no historical CI result is relabeled a pass.
