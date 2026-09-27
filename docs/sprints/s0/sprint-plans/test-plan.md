# Sprint 0 Test Plan

Approved by the user on 2026-09-27. Verification uses fixtures and local test servers; no real fusion is claimed.

## Intent Traceability

All rows concern [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md).
Each test name below includes its owning build clause; integration/E2E compose
the same clauses rather than introduce additional acceptance claims.

| Intent | Acceptance criterion | Build task / EARS clause | Verification |
|--------|----------------------|--------------------------|--------------|
| INT-0001 | AC-1 | T-001 E1 | valid_fixture_and_local_configs |
| INT-0001 | AC-1 | T-001 E2 | rejects_invalid_config_before_backend_call |
| INT-0001 | AC-7 | T-001 E3 | ci_config_targets_work_branch_and_rust_checks |
| INT-0001 | AC-5 | T-001 E4 | recipe_catalog_validates_ids_payloads_and_mode |
| INT-0001 | AC-2 | T-002 E1 | peer_projection_excludes_private_canaries |
| INT-0001 | AC-2 | T-002 E2 | owner_updates_preserve_omitted_fields |
| INT-0001 | AC-2 | T-002 E3 | forged_peer_instructions_cannot_mutate_harness |
| INT-0001 | AC-3 | T-003 E1 | reordered_workers_share_frozen_snapshot |
| INT-0001 | AC-3 | T-003 E1 | concurrency_cap_and_ballot_barrier |
| INT-0001 | AC-4 | T-003 E2 | invalid_timeout_failure_are_not_abstention |
| INT-0001 | AC-3, AC-4 | T-003 E3 | all_abstain_and_all_fail_runs_terminate |
| INT-0001 | AC-2, AC-7 | T-004 E1 | local_server_receives_only_owner_projection |
| INT-0001 | AC-4, AC-2 | T-004 E2 | local_transport_failures_are_bounded_and_redacted |
| INT-0001 | AC-4 | T-005 E1 | reciprocal_pairs_are_exclusive_and_order_independent |
| INT-0001 | AC-4 | T-005 E1 | abstainers_and_unmatched_agents_keep_memory |
| INT-0001 | AC-5 | T-005 E2 | incomplete_or_incompatible_metadata_blocks_requests |
| INT-0001 | AC-5 | T-005 E2 | compatible_metadata_creates_unverified_pending_request |
| INT-0001 | AC-5 | T-005 E2 | mutual_plan_consent_is_required_without_fallback |
| INT-0001 | AC-5, AC-6 | T-005 E2, T-006 E3 | sibling_requests_preserve_parents_count_and_recipes |
| INT-0001 | AC-6, AC-2 | T-006 E1 | operator_record_and_public_projection_are_separate |
| INT-0001 | AC-6 | T-006 E2 | replay_matches_recording_without_backend_calls |
| INT-0001 | AC-6 | T-006 E2 | unsupported_or_inconsistent_records_are_rejected |
| INT-0001 | AC-6 | T-006 E3 | reports_keep_denominators_and_failure_categories |
| INT-0001 | AC-6 | T-006 E3 | random_nomination_baseline_matches_enumerated_cases |
| INT-0001 | AC-7 | T-007 E1 | cli_fixture_run_replay_report |
| INT-0001 | AC-7 | T-007 E1 | cli_sibling_plan_records_requests_only |
| INT-0001 | AC-7 | T-007 E2 | cli_local_http_pipeline |
| INT-0001 | AC-7 | T-007 E2 | cli_local_failures_preserve_status |
| INT-0001 | AC-7 | T-007 E3 | documented_smoke_and_required_checks |

## Unit Tests

### T-001 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `valid_fixture_and_local_configs` (E1): valid fixtures and literal-loopback local
  settings validate; behavioral-only agents retain missing merge provenance.
- `rejects_invalid_config_before_backend_call` (E2): table of duplicate IDs,
  zero/overflow limits, malformed revisions/signatures, missing endpoint/model,
  and non-loopback endpoint -> typed errors and zero counted backend calls.
- `ci_config_targets_work_branch_and_rust_checks` (E3): inspect CI/updater config
  as a required manual tooling check, not an implementation-mirroring unit test.
- `recipe_catalog_validates_ids_payloads_and_mode` (E4): valid catalogs plus
  duplicate/altered IDs, unsupported methods, incomplete payloads, nonfinite or
  out-of-range values, asymmetric coefficients, invalid DELLA probabilities,
  fixed mode with zero/multiple plans, missing recipe references, empty/oversized
  sibling batches, duplicate child recipe/seed jobs, unbounded budgets, and
  executable text -> validate or fail
  before any backend call. Validity never implies measured quality.

### T-002 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `peer_projection_excludes_private_canaries` (E1): distinct canary strings in
  peer fields, checkpoint/provider metadata, and ballots -> absent from serialized
  observation; public content and owner's fields remain available.
  Neutral recipe cards and mode are visible without revealing checkpoint mapping,
  any peer's consent, or a prescriptive "best method" claim (T-002 E1).
- `owner_updates_preserve_omitted_fields` (E2): update one field, omit two -> only
  that owner's supplied field changes; no private update appears publicly.
- `forged_peer_instructions_cannot_mutate_harness` (E3): forged system/config/
  tool/private-state directives remain data; next observations still pass canaries.

### T-003 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `invalid_timeout_failure_are_not_abstention` (E2): explicit null nomination,
  invalid target, self nomination, malformed JSON, oversized text, timeout, and
  transport failure -> distinct terminal statuses, one per handle, no fallback.
  Missing/invalid recipe consent remains separate from valid partner nomination;
  method errors cannot erase a social pair (T-003 E2).
- `all_abstain_and_all_fail_runs_terminate` (E3): paused Tokio time and fixture
  failures verify configured termination, no lingering tasks, unchanged population.
- Fixture stubs record every projected observation and controlled completion order.

### T-005 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `reciprocal_pairs_are_exclusive_and_order_independent` (E1): enumerate small
  ballot sets, including A->B->C->A cycles, popular targets, odd pools, and
  permutations -> exactly reciprocal pairs and at most one pair per handle.
- `abstainers_and_unmatched_agents_keep_memory` (E1): completed round -> all agents
  remain with their owner state, absent any hidden loneliness/retirement effect.
- `incomplete_or_incompatible_metadata_blocks_requests` (E2): absent-on-both-sides
  and mismatched base/architecture/layout/tokenizer/license metadata -> explicit
  blocked reasons and no eligible automatic request.
- `compatible_metadata_creates_unverified_pending_request` (E2): complete matching
  metadata -> pending/unverified manifest with immutable parent/base references,
  no execution timestamp, output checkpoint, admitted child, or weight claim.
- `mutual_plan_consent_is_required_without_fallback` (E2): same exact single/
  batch plan fingerprint
  -> pending if metadata permits; different/missing/declined/deferred/invalid
  recipes -> distinct blocked reasons while retaining the reciprocal pair.
  Reject same display-name plans with different child recipes/count/fingerprints;
  fixed mode also permits refusal, and no alternate plan is substituted.
- `sibling_requests_preserve_parents_count_and_recipes` (T-005 E2; T-006 E3):
  consent to a two-method batch -> two distinct child requests sharing the exact
  original parent/base revisions, one batch count and two requested-child counts.
  No silent count expansion/truncation, claimed births, or child-as-parent chaining;
  any blocked metadata/consent remains explicit at batch/child level.

### T-006 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `unsupported_or_inconsistent_records_are_rejected` (E2): unsupported schema,
  duplicate/missing outcomes, unknown IDs, altered declared pair results, or broken
  phase sequence -> explicit error rather than a successful report.
- `reports_keep_denominators_and_failure_categories` (E3): hand-calculated mixed
  outcomes and all-failure runs -> exact counts, clear denominators, no NaN/Inf or
  false abstention; no inferred equilibrium or child-quality metric.
  Plan consent/decline/deferral/disagreement counts stay separate from partner
  abstention and pairing; batch/requested-child denominators differ and
  mode/catalog/card order remain recorded (T-006 E3).
- `random_nomination_baseline_matches_enumerated_cases` (E3): exhaustively enumerate
  all directed single-nomination outcomes for n=2,3,4 -> average paired fraction
  1/(n-1); handle zero/single-agent denominators explicitly.

## Integration Tests

All integration tests concern [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md).

- `reordered_workers_share_frozen_snapshot` (T-002 E1; T-003 E1): delayed fixture
  backend runs with reversed completion orders -> identical per-phase snapshots,
  ordered publication, final pair set, and peer-private canary exclusion.
- `concurrency_cap_and_ballot_barrier` (T-003 E1): counted backend + controlled
  delays -> peak concurrency never exceeds the bound; no selection observation
  includes earlier completed ballots, including timeout outcomes.
- `local_server_receives_only_owner_projection` (T-004 E1; T-002 E1): captured real
  HTTP request -> correct procedural prompt/decoding/owner projection; a valid
  server JSON response traverses the shared protocol parser.
- `local_transport_failures_are_bounded_and_redacted` (T-004 E2): redirects,
  status errors, malformed bodies, oversized streaming body, and stalled server
  -> bounded typed statuses; canary headers/body/private text absent from public
  events and CLI error strings.
- `operator_record_and_public_projection_are_separate` (T-006 E1; T-002 E1):
  complete run -> operator record retains provenance and private data; public
  file lacks canaries/ballots/identity and peers receive no record handles.
- `replay_matches_recording_without_backend_calls` (T-006 E2; T-003 E1; T-005 E1):
  valid fixture record -> identical public events/pairs/report, including failures
  and preserved recorded time fields; use a panic-on-call backend to verify zero
  inference. Existing output directories are not overwritten (T-006 E1).

## End-to-End Tests

- **Status:** possible for this sprint's complete protocol slice.
- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- `cli_fixture_run_replay_report` (T-007 E1; T-006 E1/E2/E3; T-005 E1/E2): binary
  runs the documented multi-round fixture in a temporary directory, replays it,
  and reports metrics. Expect one pair per declared fixture round, an explicit
  abstainer who persists, correct manifests, identical derived replay artifacts,
  and no network backend calls or claimed children.
- `cli_sibling_plan_records_requests_only` (T-007 E1; T-005 E2; T-006 E1/E2/E3):
  binary runs and replays the opt-in sibling fixture -> one agreed batch with two
  per-child method requests, exact count/parentage, no real children or network
  inference, correct metrics and identical replay outputs.
- `cli_local_http_pipeline` (T-007 E2; T-004 E1): binary uses a disposable loopback
  server through all phases and validates pair, records, and public projection.
- `cli_local_failures_preserve_status` (T-007 E2; T-004 E2): malformed/timeouting
  HTTP server -> recorded failure category and no false voluntary abstention.
- `documented_smoke_and_required_checks` (T-007 E3; T-001 E3): run cargo fmt
  --check, cargo clippy --all-targets -- -D warnings, cargo test --locked, and
  the fixture CLI smoke; inspect docs against the implemented flags and output.

Real model inference followed by actual tensor fusion and child admission is
**not-yet-possible** in this sprint. Unlocked by
[INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md), which supplies
checked checkpoints/licenses/resources, a pinned merger, and child evaluation.
Causal multi-generation comparisons are unlocked by
[INT-0003](../../../intents/INT-0003-controlled-evolutionary-study.md).

## Validation scope and reporting

Record each check's command/result in the Test Phase and run meaningful affected
tests after coherent task groups. Do not label fixture transcripts as model
behavior or a local mock as proof of a real provider/model. A later live smoke
requires a configured local model and records its actual provenance. Hosted CI
results require an actual remote checkpoint; local checks alone do not claim it.
