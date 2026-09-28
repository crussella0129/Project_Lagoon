# Sprint 1 instrument extension

Authorized by the original implementation request and the subsequent instruction
"Defer Zenodo; continue implementation". Phase 0's current-stable CI passed on
46fb66f2d00583592b62179fb16debc730f33886 in run 36374666278; main protection is active.
G0 is satisfied under that explicit Zenodo deferral. This supplemental plan extends
the open sprint before its one human merge checkpoint; original locked plans remain
unchanged provenance. No real study, training or fusion execution is added.

## Intents

[INT-0005](../../../intents/INT-0005-observability-and-analysis.md), now planned and
then active for instrument v0.4. Real-server conformance remains an evidence gate,
not something fixtures can certify. Social profiles/events remain INT-0004's next increment.

## Build tasks and test traceability

### T-022: Reproducible calls and configurable bounded memory
- **Touches:** config, protocol, observation, runner, replay, record, examples, usage.
- **Depends on:** G0.
- **EARS:** WHEN a call is made THEN its versioned tuple-derived seed SHALL be
  recorded, sent to the backend and checked on replay; distinct owners/phases/steps
  SHALL have distinct tested seeds (`call_seed_receipts_and_tampering`).
- **EARS:** WHEN phase limits change THEN schema/hash and parser SHALL agree, old
  defaults SHALL remain valid, and carried notes SHALL survive a smaller next-phase
  input bound (`configured_unicode_limits_and_retention`).
- **EARS:** WHEN opt-in peer memory is updated THEN only valid peer aliases with
  bounded trust/notes SHALL persist, omission SHALL retain and an empty map SHALL
  clear it; peer/public projections SHALL exclude it (`peer_ledger_privacy_and_replay`).
- **Compatibility:** bump package/record version; old records require the prior
  harness. Do not silently reinterpret their seeds or response schemas.

### T-023: Optional matched exit with honest denominators
- **Touches:** session, observation, replay, report, examples, usage.
- **Depends on:** T-022.
- **EARS:** WHEN reciprocal nominations resolve in matched-exit mode THEN both
  agents SHALL retire regardless of fusion consent, preserve their state, disappear
  from later calls/enums and be publicly announced (`matched_exit_replay_and_counts`).
- **EARS:** WHEN fewer than two agents remain THEN the run SHALL terminate without
  synthetic abstentions or division by zero (`matched_exit_singleton_and_empty`).
- **EARS:** WHEN replaying truncated, extra or forged retirement/call data THEN replay
  SHALL reject it (`matched_exit_record_tampering`). Repeated rounds remain default.

### T-024: Server provenance and a conformance kit
- **Touches:** provenance configuration/records, conformance scripts/docs/fixtures.
- **Depends on:** T-022.
- **EARS:** WHEN local inference is configured THEN operator records SHALL retain
  declared server/version, grammar/template and model revision receipts without
  exposing them to peers (`server_provenance_privacy`).
- **EARS:** WHEN the conformance kit evaluates responses THEN it SHALL separately
  check enum/string bounds, whitespace, reasoning, finish reasons, template and
  token accounting, derive a 1.5x reserve, and fail incomplete evidence
  (`conformance_negative_fixtures`). Actual vLLM/llama.cpp runs require installed
  servers/models; no synthetic report may pass G1.

### T-025: Tidy exports and planted-parameter recovery
- **Touches:** export module/CLI, analysis scripts, CLI tests, documentation.
- **Depends on:** T-022 and T-023.
- **EARS:** WHEN exporting a replay-validated record THEN the CLI SHALL produce
  correctly escaped CSV choices/messages/events/outcomes/profiles with positions,
  explicit outside options and technical failures distinguished, without private
  notes (`cli_export_privacy_and_failures`). Unimplemented social fields are missing,
  not fabricated. Preserve record fingerprints for joining future measurements.
- **EARS:** WHEN the planted fixture pipeline runs THEN generated ballots SHALL
  traverse the real runner/record/export path and conditional-logit/OLS analysis
  SHALL meet predeclared recovery assertions (`planted_parameter_recovery`). Scientific
  traits enter through explicit fixture metadata, never inferred from private notes.

## Verification and critic

Run formatting, warning-free Clippy and all-target Rust tests after coherent changes;
Python uses ruff formatting/checks and named analysis/conformance tests. Retain actual
CI head and counts. Review negative paths, information timing, finite fits and public
privacy before finalization. No claim of G1, real model behavior or confirmatory power
follows from these fixture tests. Read-only plan critique: proceed-with-caveats;
the real-server dependency remains explicit and does not block fixture implementation.
