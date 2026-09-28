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

## T-015 and T-016 documentation evidence

- Original README excerpt is byte-identical to Git blob b84713b:README.md.
- Study B relocation preserves its text, changing only relative links to examples/docs.
  PREREG.md is a navigation stub for historical links.
- Changed Markdown relative links resolve. Study A explicitly distinguishes draft,
  exploratory pilot and confirmation, and handles trait scaling, information timing,
  outside options, non-identifiability and run-level dependence.
- Ethics review covers all eight requested areas and explicitly identifies absent
  runtime safeguards; it does not imply v0.3 implements leave or distress pauses.
- CITATION.cff validates against the official CFF 1.2.0 schema; author name was
  checked against the public GitHub profile. No invented affiliation, DOI or release date.
- Zenodo could not be authenticated: browser initialization failed. The user
  explicitly deferred activation and authorized continued implementation. T-017
  remains a release prerequisite; integration is not claimed.

## T-022 instrument evidence

Implemented versioned per-call seed receipts (including independently calculated
golden encoding), backend transmission and replay checks; per-phase Unicode
limits generated into schemas; and opt-in bounded owner-written peer memory.
Package/record versions are 0.4.0/4, with explicit old-record rejection.

Formatting and warning-free Clippy pass. All 48 local tests pass, including new
seed/tamper, configured-bound/retention and peer-ledger privacy/replay tests.
The existing local HTTP test now asserts the derived seed received by the server.
Cargo.lock changes only the package version at this boundary.

## T-023 matched-exit evidence

Opt-in matched exit retires social pairs independently of reproduction consent,
filters subsequent calls/peer enums and announces retirement while preserving
private state. Terminal/singleton populations produce no synthetic abstentions.
Reports use actual per-round decision slots and separate unique population coverage.

Formatting and warning-free Clippy passed; all 51 tests pass, including retirement,
declined-consent, singleton/empty, reduced-enum, privacy retention and replay-tamper
cases. A Windows linker failure was environmental: C: had zero free bytes. Removed
only the verified workspace `target/debug/incremental` build cache (about 3.7 GiB),
then tested with one build job, incremental compilation off and dev/test debug
symbols off. These local resource settings do not change the test suite or CI policy.

## T-024 conformance-kit evidence

53 Rust tests, formatting and warning-free Clippy passed; Python's five negative
fixture tests, ruff formatting and lint checks passed. Runtime contracts and declared
server/model/template/grammar provenance are recorded without peer disclosure.
The kit pins server versions, model revision and template digest, verifies received
schemas/usage/finish reasons and only proposes a reserve after all probes pass.

[Primary sources, exact commands and limitations](../../conformance/README.md)
are retained with the kit. No actual vLLM or llama.cpp probe was executed: vLLM is
absent and Docker's Linux engine is unavailable locally. G1 remains open; empirical
sample maxima do not establish worst-case schema token bounds. Both local templates
require replacement provenance values before use. No token reserve was reduced.

## T-025 export and recovery evidence

The replay-validated exporter writes bounded CSV choices/messages/events/outcomes/
profiles plus record/config fingerprints and table digests. Profiles are explicitly
unavailable. Tests cover Unicode/quote/newline round-trip, private-note exclusion,
outside options versus missing failed decisions, matched exit, changed data,
feature joins/visibility and unidentifiable or undefined analyses.

54 Rust tests, formatting and warning-free Clippy pass. Six analysis integration
tests pass. Conformance fixtures additionally exercise complete and interrupted
report assembly; endpoint/model/tokenizer must match the contract being probed.
Ruff formatting/lint and conformance tests pass at the task boundary.

[Fixed synthetic recovery receipt](../../analysis/recovery-receipt.json): eight
seeds × 40 rounds × eight participants per estimator, 2,560 decisions each.
Conditional logit retains 142 voluntary outside choices; both scenarios have zero
technical failures. All eight planted slopes fall within 99% seed-cluster sandwich
intervals and the predeclared absolute-error tolerances. OLS uses its own linear
probability generator; it does not equate nonlinear choice coefficients to linear
selection gradients. This is an implementation check, not coverage calibration.

The initial fit stopped at an independent gradient convergence check: statsmodels
0.15.0's ConditionalLogit wrapper ignores solver keyword arguments. Switched to
the supported Newton method and retained the independent gradient check. Seeds,
coefficients, sample sizes and acceptance limits were not changed. Two subsequent
full pipeline runs passed, including the final table-digest path. The local first
attempt remains in ignored `target/recovery-v1`; no failure was recast as a pass.

Required CI now covers Rust, the locked Python environment, ruff, both test suites
and full recovery. Weekly grouped Dependabot updates include the `uv` ecosystem
at `/analysis`, as documented by [uv](https://docs.astral.sh/uv/guides/integration/dependabot/).
Real conformance, social profiles, empirical coverage/pilot, freeze and registration
remain open; no real study, training or fusion was run.

The first extended hosted run [36378555673](https://github.com/crussella0129/Project_Lagoon/actions/runs/36378555673)
failed before executing tests because setup-uv has release `v10.2.0` but no floating
`v10` ref. Replaced the workflow reference with the verified exact release. This
is a CI setup correction, not a test exclusion; all required checks remain enabled.
