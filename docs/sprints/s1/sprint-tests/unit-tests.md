# Sprint 1 unit results

Implementation task commits: T-014 `08e4d1a`, T-015 `ff6dbce`, T-016 `ed44e22`,
T-022 `6828aad`, T-023 `71bfdf6`, T-024 `2841483`, T-025 `c2523f1`.
Final code tree is `bc5cec1472eda883c0278ebe1d7a4f58cb222d61`; its only change after
the T-025 local checks is the setup-uv action reference and a documentary receipt.

## Canonical local checks

- `cargo fmt --check`: pass.
- `cargo clippy --locked --all-targets -j 1 -- -D warnings`: pass.
- `cargo test --locked --all-targets -j 1`: **54 passed**, zero failed/ignored:
  34 library, eight CLI, seven instrument, two pairing-reference and three variance tests.
- `uv run --project analysis --locked ruff format --check analysis conformance`: pass.
- `uv run --project analysis --locked ruff check analysis conformance`: pass.
- Python unittest discovery: **six conformance + six analysis tests passed**.

Rust stable 1.98.1/Cargo 1.98.1 on Windows; Python 3.12.14, uv 0.12.17 and the
committed analysis lock. Local low-disk workaround: one Cargo job, incremental
off and dev/test debug info off. CI runs the same suites with its normal profile.

## EARS / intent assertions

| Task / intent criterion | Named executed test or check | Assertion and result |
|---|---|---|
| T-014 / INT-0007 AC-1,3 | `supported_build_local`; `current_stable_and_grouped_updates` | Stable selected; no numeric minimum; weekly grouped Cargo/Actions/uv updates; locked checks pass |
| T-015 / INT-0006 AC-1 | `origin_blob_equivalence` | Original excerpt equals b84713b99506b103a95427ac895baa461328d5f0 byte for byte |
| T-015 / INT-0006 AC-2 | `ethics_scope_review` | Eight required topics present; manual/future distress/exit/containment controls identified |
| T-015 / INT-0006 AC-3 | `study_b_preserved_with_documented_matched_exit_amendment`; `changed_documentation_links`; `study_a_design_review` | Original Study B retained except repaired links and explicitly implemented optional matched exit; Study A has separate arm/measurement/visibility/failure/freeze decisions; current links resolve |
| T-016 / INT-0006 AC-4 preparation | `cff_schema_validation` | Official CFF 1.2 schema passes; version 0.4.0 matches crate; no DOI/date/affiliation invented |
| T-022 / INT-0005 seeds | `call_seed_receipts_and_tampering` | Independent golden SHA tuple, distinct owner/phase calls, actual backend seed and replay rejection |
| T-022 / INT-0005 limits | `configured_unicode_limits_and_retention` | Configured Unicode lengths agree with schemas; carried notes survive smaller later input bounds |
| T-022 / INT-0005 memory | `peer_ledger_privacy_and_replay` | Owner-only ledger, bounded keys/trust/notes, omission retention and explicit clear; replay preserved |
| T-023 / INT-0005 matched exit | `matched_exit_replay_and_counts`; `matched_exit_singleton_and_empty`; `matched_exit_record_tampering` | Declined-consent pairs retire; later eligibility/enums/counts shrink; zero/singleton termination and forged/missing/extra record rejection |
| T-024 / INT-0005 declared provenance | `server_provenance_privacy`; `cli_runtime_schema_contract` | Local metadata required, operator-only receipts, valid schema hash and unknown-owner rejection |
| T-024 / INT-0005 conformance preparation | `ConformanceTests` (six named methods) | Enum/length/whitespace/reasoning/finish/usage negatives; literal-loopback policy; complete versus interrupted report assembly and reserve suppression on failure |
| T-025 / INT-0005 exports | `cli_export_privacy_and_failures`; `PipelineTests` (six named methods) | Replay before export, aliases/positions, abstention versus failure, hash/duplicate checks, Unicode CSV round-trip, no private canaries, no overwrite/tamper acceptance |

One-off Phase 0 validations were rerun on the final implementation with the two
documented Study B edits allowed; no scientific criterion was weakened. Runtime
behavior checks are not evidence that real servers obey their declared grammar.
