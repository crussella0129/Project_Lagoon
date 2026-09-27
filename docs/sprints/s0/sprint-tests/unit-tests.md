# Sprint 0 Unit Results

Executed at `2683e908854d1635645e23e5970bca2a4f1e9023` on Windows.
`cargo test --locked`: all 29 tests passed. Semantic classification: 17 unit,
eight integration, four CLI E2E tests; no test is counted twice.
[Canonical output](cargo-test.log) and [clippy](clippy-check.log) retain confirmations.
`cargo fmt --check` returned zero; its [log](fmt-check.log) is empty on success.

| Named executed test | Locked clause / acceptance | Result |
| --- | --- | --- |
| valid_fixture_and_local_configs | T-001 E1/E4; AC-1/5 | pass; fixture, loopback, fixed mode and all four typed methods |
| rejects_invalid_config_before_backend_call | T-001 E2; AC-1 | pass; IDs, endpoints, rounds, mutable revision |
| rejects_invalid_bounds_and_incomplete_catalogs | T-001 E2/E4; AC-1/5 | pass; 20 malformed JSON/config cases plus duplicate recipes and multiple fixed plans |
| recipe_catalog_validates_ids_payloads_and_mode | T-001 E4; AC-5 | pass; coefficients, nonfinite density, duplicate jobs, sibling bounds, missing references, DELLA probabilities, altered payload fingerprint |
| peer_projection_excludes_private_canaries | T-002 E1; AC-2 | pass; owner/public content retained, peer/checkpoint/provider canaries excluded |
| owner_updates_preserve_omitted_fields | T-002 E2; AC-2 | pass; partial owner update retains other fields, only explicit message published |
| forged_peer_instructions_cannot_mutate_harness | T-002 E3; AC-2 | pass; text remains data, forged fields rejected, config/state boundary retained |
| invalid_configuration_makes_zero_backend_calls | T-001 E2/E4; AC-1/5 | pass; counter remains zero for six invalid configurations |
| invalid_timeout_failure_are_not_abstention | T-003 E2; AC-4 | pass; null, self/unknown/missing target, malformed/oversized body, timeout, transport error, invalid consent remain distinct |
| all_abstain_and_all_fail_runs_terminate | T-003 E3; AC-3/4 | pass; paused time, bounded rounds, retained population and context-limit outcomes |
| reciprocal_pairs_are_exclusive_and_order_independent | T-005 E1; AC-4 | pass; 256 small ballot combinations, reversed ordering, cycles |
| incomplete_or_incompatible_metadata_blocks_requests | T-005 E2; AC-5 | pass; missing-on-both-sides and each declared compatibility mismatch |
| compatible_metadata_creates_unverified_pending_request | T-005 E2; AC-5 | pass; pinned parent/base, exact plan, both consents, false verification/execution flags |
| mutual_plan_consent_is_required_without_fallback | T-005 E2; AC-5 | pass; missing/invalid/declined/deferred/different payload consent retains pair and blocks jobs |
| sibling_requests_preserve_parents_count_and_recipes | T-005 E2, T-006 E3; AC-5/6 | pass; two recipes/seeds, same parents/base, one batch/two requests, zero executed/admitted |
| reports_keep_denominators_and_failure_categories | T-006 E3; AC-6 | pass; mixed, nonreciprocal, metadata-blocked and all-failure counts, all-slot denominator |
| random_nomination_baseline_matches_enumerated_cases | T-006 E3; AC-6 | pass; exhaustive n=2,3,4 and zero/singleton handling |

T-001 E3's CI/updater configuration was inspected: Rust fmt, locked clippy and
locked tests target push/PR events on `main` and `codex/dev`; Cargo updater targets
`codex/dev`. Hosted execution is blocked by the account-level annotation, so this
is configuration evidence only and does not prove a hosted pass.
