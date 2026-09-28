Finalized - DO NOT EDIT

# Sprint 1 Test Plan

## Intent Traceability

| Intent | Acceptance criterion | Build task / EARS clause | Verification |
|--------|----------------------|--------------------------|--------------|
| [INT-0007](../../../intents/INT-0007-supported-builds.md) | AC-1 | T-014 compiler support | supported_build_local; supported_build_ci |
| INT-0007 | AC-2 | T-014 enforced checks | active_main_rules; pr_required_checks |
| INT-0007 | AC-3 | T-014 housekeeping | grouped_updates_and_canonical_links |
| [INT-0006](../../../intents/INT-0006-publication-and-ethics.md) | AC-1 | T-015 verbatim source | origin_blob_equivalence |
| INT-0006 | AC-2 | T-015 eight ethics topics | ethics_scope_review |
| INT-0006 | AC-3 | T-015 split studies/roadmap | study_b_content_preserved; documentation_links; study_a_design_review |
| INT-0006 | AC-4 preparation | T-016 citation validity | cff_schema_validation |
| INT-0006 | AC-4 activation | T-016 verified readiness or blocker | zenodo_access_receipt (blocked activation does not pass AC-4) |

## Unit Tests

No new source behavior is introduced. Existing unit tests protect INT-0007 AC-1;
run the full existing locked suite with formatting and clippy. Documentation
checks are bounded validation commands, not a new test framework.

- `origin_blob_equivalence`: compare the excerpt bytes with the original Git blob.
- `study_b_content_preserved`: compare moved content after accounting only for
  repaired relative links and an explicit new location/status note if needed.
- `cff_schema_validation`: official CFF schema validation, no invented identifier.

## Integration Tests

- `supported_build_local`: cargo fmt --check, cargo clippy --locked --all-targets -- -D warnings,
  cargo test --locked --all-targets; retain suite counts.
- `active_main_rules`: inspect active main rules, required contexts and Actions app
  binding; no bypass and existing protection rules preserved.
- `grouped_updates_and_canonical_links`: inspect YAML and current metadata; historical
  origin names intentionally remain verbatim. T-008 already has completion evidence.
- `documentation_links`: local relative links resolve, including moved Study B links.
- `ethics_scope_review` / `study_a_design_review`: read-only review against linked
  intent and proposal, including no real-data claims and empirical decisions left open.

## End-to-End Tests

- **Status:** possible for build/PR checks; externally blocked for archive activation.
- `supported_build_ci`: observe successful `check` and `msrv` jobs for the submitted
  sprint head on Linux, then inspect required-check state on its main-targeted PR.
- `pr_required_checks`: GitHub reports both jobs required and uses the active rule;
  no artificial failing merge is attempted. Distinguish configuration evidence from
  an actually attempted rejected merge.
- `zenodo_access_receipt`: authenticated settings must show enabled integration;
  if access remains unavailable, document it and leave INT-0006 AC-4 / G0 open.
  A DOI-producing release is unlocked by a later approved release under INT-0006.
