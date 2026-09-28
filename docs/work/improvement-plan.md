# Improvement-plan implementation

Sprint 1 implements the Phase 0 prerequisite. See the [roadmap](../roadmap.md)
for all later phases and [plan amendment](../sprints/s1/sprint-plans/plan-amendment.md)
for the user's current-stable-only Rust policy.

## T-014 local and remote evidence

- Current stable observed: rustc 1.98.1 (48a229cea 2026-09-01), Cargo 1.98.1.
  `rustup check` reports stable up to date. This is a receipt, not a pinned minimum.
- `cargo fmt --check`: pass.
- `cargo clippy --locked --all-targets -- -D warnings`: pass after three equivalent
  let-chain rewrites for current Clippy's collapsible-if lint.
- `cargo test --locked --all-targets -j 1`: 45 pass (34 library, 6 CLI,
  2 pairing-reference, 3 variance). An initial parallel compilation failed with
  Windows paging-file error 1455; serialized compilation and execution passed.
- Cargo.lock unchanged. No source behavior or record version changed.
- Ruleset [24093103](https://github.com/crussella0129/Project_Lagoon/rules/24093103)
  activated for refs/heads/main. Active branch-rule API confirms deletion,
  non-fast-forward and required-status protections. Required context `check` is
  bound to GitHub Actions app 15368, strict freshness enabled, bypass actors empty.
  Configuration retained in `.github/main-ruleset.json`. No failed merge was staged.
- Weekly Cargo and GitHub Actions update groups configured. T-008 was already
  closed and origin/remote-profile already point to Project_Lagoon.
- Hosted verification follows on the complete sprint PR; see sprint test evidence.
