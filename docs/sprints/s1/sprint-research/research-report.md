# Sprint 1 Research Report

## Intents Reviewed

- [INT-0007](../../../intents/INT-0007-supported-builds.md) — created; supported toolchain and main protection; planned.
- [INT-0006](../../../intents/INT-0006-publication-and-ethics.md) — created; publication, ethics and study separation; planned.
- [INT-0004](../../../intents/INT-0004-social-layer.md) — created; later social features; proposed.
- [INT-0005](../../../intents/INT-0005-observability-and-analysis.md) — created; later instrument/analysis; proposed.
- [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md) — selected; preserve realized harness boundaries.
- [INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md) — selected; future Study B.
- [INT-0003](../../../intents/INT-0003-controlled-evolutionary-study.md) — selected; future Study C.

## 1. Sprint Goal

Implement Phase 0 of the user-supplied improvement plan, respecting its G0 barrier
before new features. The user subsequently authorized current Rust, replacing
minimum-version bisection with an explicit current-stable support floor. Preserve
the rest of the roadmap as gated work, not as completed implementation.

## 2. Existing Code Survey

| File | Relevance | Notes |
|------|-----------|-------|
| Cargo.toml | high | Claims Rust 1.85; will support verified current 1.98.1 |
| Cargo.lock | high | Preserve locked dependency versions |
| .github/workflows/sprint-loops-ci.yml | high | Stable-only `check` job |
| .github/dependabot.yml | high | Weekly already; only RNG dependencies grouped |
| README.md | high | Clinical overview; original text recoverable at b84713b |
| PREREG.md | high | Draft merge-screening plan; moving requires relative-link repair |
| docs/work/remote-profile.md | high | Project_Lagoon already correct; human merge approval |
| docs/work/completed-tasks.md | high | T-008 and T-013 already closed with local/CI evidence |
| src/runner.rs | medium | Per-call seed, exits and social features remain future work |
| src/config.rs | medium | Existing memory/provenance and decoding metadata |
| src/record.rs | medium | No claimed server conformance or social profiles yet |
| src/protocol.rs | medium | Private notes and sealed choice; no leave/distress pause |

## 3. External Sources

- [cargo-msrv](https://github.com/foresterre/cargo-msrv) and [find command](https://gribnau.dev/cargo-msrv/commands/find.html) — evaluated for requested bisection; installed 0.19.3 locally but not needed after current-Rust instruction.
- [Cargo rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html) — declared support floor rather than historical compatibility assertion.
- [GitHub repository rules](https://docs.github.com/en/rest/repos/rules) — update existing ruleset and inspect active branch rules.
- [GitHub citation/archive instructions](https://docs.github.com/en/repositories/archiving-a-github-repository/referencing-and-citing-content) — Zenodo needs account authorization and a GitHub release, not a tag alone.

## 4. Risks, Unknowns, Dependencies

- Verified `rustup check`: stable 1.98.1 is current on this host; no older-toolchain
  minimum claim will be made. Run the locked suite locally and on Linux CI.
- Existing ruleset 24093103 is disabled with no target branches. Preserve its
  deletion/non-fast-forward rules; bind required check contexts to GitHub Actions.
- Zenodo's settings page requires login; the browser tool failed to initialize
  (`failed to write kernel assets`). No authenticated integration is available.
  Do not mark G0 or the archival criterion complete on documentation alone.
- Restoring the origin also restores obsolete speculative claims and incentives.
  Label it historical, preserving exact source text without making it protocol.
- Draft Study A coefficients need explicit feature scaling, pre-choice information,
  run-level uncertainty and abstention handling. Leave empirical thresholds/models
  unresolved until a separately identified pilot; do not invent results.
- The roadmap includes compute, registration and publication gates; a code change
  is not permission to fabricate those outcomes or bypass human merge approval.

## 5. Recommended Approach

First establish supported Rust, CI and main protection; then preserve provenance,
write ethics and separate draft studies. Track every remaining phase in the Book.
Keep citation setup independently verifiable. Complete local and hosted checks,
create the sprint PR and leave merging to the user under the existing remote profile.
Alternative: immediately implement social features. Rejected because the supplied
plan explicitly says G0 must be completed before new features.

## Artifacts

Research evidence is recorded here; implementation receipts and exact CI links
will be retained under `sprint-tests/` and the work ledger. The supplied plan is
represented by the linked stable intents and `docs/roadmap.md` without publishing
the user's local attachment path.

## Subsequent user clarification

The user explicitly requested a `current` support floor rather than a numbered
minimum. See the approved plan amendment: no compiler bisection or pinned MSRV job.
[Dependabot options](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference)
do not provide a Rust floor field; [the toolchain action](https://github.com/dtolnay/rust-toolchain)
supports the rolling `stable` channel. Both local toolchain and CI follow that policy.

## Budget Override

Two additional primary documentation sources were checked after the user's
toolchain-policy clarification to avoid inventing a Dependabot configuration key.
