# Instrument maintenance validation

2026-09-27, Windows, Rust/Cargo 1.98.1. This follow-up preserves Sprint 0's historical evidence and fixes its measurement instrument. Rust v0.2 uses record schema 2; old schema-1 records require the old revision to replay.

## Evidence

- `cargo fmt --check`: pass.
- `cargo clippy --locked --all-targets -- -D warnings`: pass.
- `cargo test --locked --all-targets`: 29 library tests, 6 CLI integration tests and 3 variance-example tests, all passing (38 total).
- The mocked HTTP reproduction of the pasted formatting failure yields one reciprocal pair and one pending batch; the fenced ballot is counted and all three valid plan IDs receive agree receipts despite an obsolete supplied hash.
- Seeded owner/round presentation and schema tests cover opaque aliases, card permutation, model-view redaction, exact receipt binding, unknown-plan rejection and procedural text sent once.
- Six-round token-budget tests preserve private preferences and the full public log while fitting per-call projections; exhausted context, tokenizer failure and timeout prevent generation and replay offline.
- CLI HTTP tests exercise vLLM chat token counts, llama.cpp template/tokenize projection equality, tokenizer failure redaction and replay with mock servers gone. Existing tests cover worker limits, snapshot barriers, privacy, sealed matching, sibling parentage, failure categories and persistence.
- Replay rejects altered token counts in fixtures, permutation/omission/schema receipts and existing structural corruptions. External token counts remain trusted measurements, not independently verified offline.
- The bounded [Rust variance study](../../examples/variance-study.rs) ran with seed 9, N=64, 4096 coordinates, six generations and eight replicates. [Full results](variance-results.json) and [analytical qualifications](variance-review.md) are retained.

No real inference server, trained model, tokenizer/model identity verification, specialist training or actual merge was exercised. The HTTP tests establish the client contract, not that a deployment honors constrained decoding or serves the claimed weights. Structured output removes a source of formatting failure only when supported/enforced by the configured backend.

## Ferric boundary

Read-only provenance: Animus_Ferric commit `508edfd38ae4ba1f1a9ece06d3b02002cadba5fd`, clean worktree when inspected. Its `ferric-iron` library authors tool-action schemas; it is not a decoder. The OpenAI provider submits the strict schema and the inference backend enforces it. `ferric-valve/src/transform.rs` passes requests with an existing `response_format` through. Lagoon uses this contract and can reuse the running valve directly; it does not add Ferric's tool/scratchpad protocol or claim a new shared-library decoder dependency. Ferric files were not modified.

## Hosted checks

Sprint 0 PR #1 was merged by the user before this follow-up was published. Its hosted checks failed before runner execution because of GitHub account payment/spending-limit availability. Local success above does not supersede that failure; T-008 remains open.

[Follow-up PR #5](https://github.com/crussella0129/Lovers_Lagoon/pull/5) contains tested implementation commit `c06cce7008297639d7f4f210714a83088dec7dbe`. Both its [pull-request run 36325178794](https://github.com/crussella0129/Lovers_Lagoon/actions/runs/36325178794) and [push run 36325176040](https://github.com/crussella0129/Lovers_Lagoon/actions/runs/36325176040) concluded failure. The pull-request job had **zero steps**. Its annotation states that the job was not started because recent account payments failed or the spending limit needs increasing. [Machine-readable evidence](ci-block.json) records the check/run IDs and exact message. No billing or visibility changes were made; INT-0001 remains active and the PR remains unmerged under the remote profile's human-approval policy.
