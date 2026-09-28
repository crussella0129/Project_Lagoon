# Sprint 1 Build Plan

## Intents

- [INT-0007](../../../intents/INT-0007-supported-builds.md) — planned; AC-1–3.
- [INT-0006](../../../intents/INT-0006-publication-and-ethics.md) — planned; AC-1–4, with external integration tracked separately if access is unavailable.

## Schema Tree

- Improvement-plan G0
  - T-014: Supported compiler, CI and branch protection.
  - T-015: Origin, ethics and study separation.
  - T-016: Citation metadata and archival setup.

## Execution Sequence

### T-014: Support current Rust and require passing checks on main
- **Intent:** [INT-0007](../../../intents/INT-0007-supported-builds.md)
- **Touches:** Cargo.toml, rust-toolchain.toml, README.md, .github/, docs/work/, sprint evidence.
- **Depends on:** none.
- **Acceptance criterion:** AC-1–3.
- **Success criterion (EARS):**
  - **WHEN** building the locked project, **THEN** declared/pinned Rust 1.98.1 and rolling stable **SHALL** pass their prescribed local and CI checks.
  - **WHEN** main is updated, **THEN** its active ruleset **SHALL** require GitHub Actions `check` and `msrv` success with strict freshness and no bypass, preserving deletion/force-push protection.
  - **WHEN** scheduled dependency updates run, **THEN** both ecosystems **SHALL** use weekly grouped updates; current repository references and hosted-CI work status **SHALL** remain accurate.

### T-015: Preserve provenance and separate the study and ethics documents
- **Intent:** [INT-0006](../../../intents/INT-0006-publication-and-ethics.md)
- **Touches:** ORIGIN.md, ETHICS.md, PREREG.md, prereg/, README.md, docs/roadmap.md, docs/SUMMARY.md, docs/work/.
- **Depends on:** T-014 local checks; hosted CI can run with the complete sprint diff.
- **Acceptance criterion:** AC-1–3.
- **Success criterion (EARS):**
  - **WHEN** reading ORIGIN.md, **THEN** its historical excerpt **SHALL** equal the original public README at b84713b and be labeled as historical rather than current protocol.
  - **WHEN** reviewing the ethics policy, **THEN** all eight required topics **SHALL** be present with implemented/manual/future distinctions.
  - **WHEN** following study documentation, **THEN** Study B content **SHALL** be preserved with working links, Study A **SHALL** cover the proposed measurements and freeze decisions, and the roadmap **SHALL** retain all phases and gates without claiming unimplemented features.

### T-016: Add valid citation metadata and resolve archival setup as far as access permits
- **Intent:** [INT-0006](../../../intents/INT-0006-publication-and-ethics.md)
- **Touches:** CITATION.cff, docs/publication.md, README.md, docs/work/, sprint evidence.
- **Depends on:** T-015.
- **Acceptance criterion:** AC-4 preparation; external activation remains required for realization.
- **Success criterion (EARS):**
  - **WHEN** reading citation metadata, **THEN** it **SHALL** identify the verified author, license and canonical repository and validate against CFF without a fabricated DOI.
  - **WHEN** inspecting archival readiness, **THEN** the setup receipt **SHALL** either verify enabled Zenodo integration or record the actual access blocker and an open activation task, keeping G0 open.
- **Notes:** User requested implementation of the supplied staged plan and then current Rust; no repeated scope approval is needed. The runtime has no Claude Code Plan Mode tool. Local read-only plan critique uses the installed critic contract. No release, external registration or merge is authorized by this sprint.
