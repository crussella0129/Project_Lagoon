# RNG dependency PR repairs

2026-09-27. Maintenance requested before the user supplies the next sprint.
No new sprint is active. Validation is local, without waiting for hosted CI.

- [x] Inspect open PRs and reproduce the development branch failure.
- [x] Combine compatible RNG upgrades, migrate the example API and resolve conflicts.
- [x] Validate the locked build, all-target tests, format, clippy and study reproducibility locally.
- [ ] Update both existing PRs with the tested revision and final scope.

## Findings

[PR #7](https://github.com/crussella0129/Project_Lagoon/pull/7) already upgraded
`rand` to 0.10 on `codex/dev` (`89427ae71443b15e4fd34d849ed42a31ae0d64d7`).
`cargo test --locked --all-targets` fails with seven trait/method errors in the
variance example: the older ChaCha RNG implements rand_core 0.9 traits, while
the imported traits come from 0.10. The extension trait is now `RngExt`.

[PR #8](https://github.com/crussella0129/Project_Lagoon/pull/8) upgrades
`rand_distr` to 0.6 but leaves ChaCha at 0.9; [PR #9](https://github.com/crussella0129/Project_Lagoon/pull/9)
upgrades ChaCha to 0.10 but leaves distributions at 0.5 and conflicts with its base.
These are coupled upgrades. Preserve both PR histories in a shared repair revision
using rand 0.10, rand_chacha 0.10 and rand_distr 0.6, with the API migration.
Both existing PRs can point to that revision; merging either includes the complete
repair. No force push, automatic merge or PR closure is required.

Future version updates group these three dependencies, using Dependabot's
documented [dependency grouping](https://docs.github.com/en/code-security/tutorials/secure-your-dependencies/optimizing-pr-creation-version-updates).
This reduces fragmented upgrades; API changes still require local validation.

GitHub resolves the former `Lovers_Lagoon` remote to the user's renamed
`crussella0129/Project_Lagoon` repository. Current PR links use that canonical name.

## Local evidence

Windows / Rust 1.98.1. The repair resolves one RNG stack: rand 0.10.3,
rand_chacha 0.10.0, rand_distr 0.6.0 and rand_core 0.10.1.

- `cargo build --locked`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --locked --all-targets -- -D warnings`: pass.
- `cargo test --locked --all-targets`: all **45 tests pass** (34 library, 6 CLI,
  2 pairing-reference and 3 variance-study).
- `cargo run --locked --release --example variance-study`: full default study
  (seed 9, N=64, 4096 coordinates, six generations, eight replicates). Parsed JSON
  matches the entire committed `docs/research/variance-results.json` exactly.
  Archived results are preserved unchanged.
- `cargo tree --locked -d`: no duplicate package versions in the resolved graph.
- `git diff --check` and Book validation: pass.

No scientific protocol, model weights, response format or recorded result changed.
Local validation is the acceptance evidence; no hosted result is claimed.
