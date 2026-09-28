# Sprint 1 authorized plan amendment

The user clarified after plan locking: "no just make the support floor 'current'
and set it to that in the dependabot". This supersedes numbered Rust-floor and
two-job requirements in T-014 and its tests. The locked files remain historical.

- Select `stable` in rust-toolchain.toml and CI; remove Cargo's numeric rust-version.
- README promises current stable only and tells local users to update that channel.
- Dependabot uses weekly Cargo/action groups; it has no valid compiler-floor key.
- Require only GitHub Actions `check` on main. No `msrv` job or requirement is added.
- `supported_build_local` / `supported_build_ci` validate rolling stable.
- `active_main_rules` / `pr_required_checks` require only `check` (Actions app 15368).

INT-0007 records the revised semantic criteria. All other tasks and gates remain.

Read-only critic recheck: every changed clause retains its named test; no hidden
dependency or reduced safety gate. A moving channel is an explicit user choice,
not a claim of reproducibility across compiler releases. Record actual versions in
test receipts. Verdict: clean for this amendment; prior Zenodo caveat remains.

Current stable validation exposed three `collapsible_if` Clippy warnings in
src/merge_request.rs, src/protocol.rs and src/replay.rs. Equivalent let-chain
rewrites are included in T-014, verified by its existing full-suite requirement.

## Zenodo deferral

The user answered "Defer Zenodo; continue implementation". T-016's citation and
honest-readiness requirements remain; T-017 archival activation moves to a release
prerequisite and no longer blocks G0. Preserve the access receipt and open task.
The initial critique's G0 caveat is superseded by this explicit user decision.
