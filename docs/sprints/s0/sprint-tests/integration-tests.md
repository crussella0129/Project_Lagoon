# Sprint 0 Integration Results

Tested head: `2683e908854d1635645e23e5970bca2a4f1e9023`.
Eight integration tests passed under `cargo test --locked`; these reside next to
the Rust modules and use internal fixture helpers. Separate `tests/privacy.rs`,
`tests/rounds.rs`, etc. were unnecessary; the locked named contracts are retained.
[Canonical output](cargo-test.log).

| Named executed test | Locked clause / acceptance | Result |
| --- | --- | --- |
| reordered_workers_share_frozen_snapshot | T-003 E1, T-002 E1; AC-2/3 | pass; reversed completion delays preserve projected observations, public order and outcomes |
| concurrency_cap_and_ballot_barrier | T-003 E1; AC-3 | pass; peak exactly two workers, completed selection observations share the same public snapshot |
| local_server_receives_only_owner_projection | T-004 E1, T-002 E1; AC-2/7 | pass; real loopback HTTP, owner canary, peer exclusion, decoding/seed, shared parser/private update |
| local_transport_failures_are_bounded_and_redacted | T-004 E2; AC-2/4 | pass; redirect/non-success, malformed/null content, declared/streamed oversize, stalled and truncated body; no error-body/header canary retained |
| abstainers_and_unmatched_agents_keep_memory | T-005 E1, T-003 E3; AC-4 | pass; all three participants retain owner memory, pair visible next round, missing consent blocks fusion |
| operator_record_and_public_projection_are_separate | T-006 E1, T-002 E1; AC-2/6 | pass; operator canaries retained, public projection omits private/ballot/provenance/consent, existing output preserved |
| replay_matches_recording_without_backend_calls | T-006 E2; AC-6 | pass; replay API takes no backend and all four files match byte-for-byte, including original timestamps |
| unsupported_or_inconsistent_records_are_rejected | T-006 E2; AC-6 | pass; nine schema/count/handle/order/outcome/pair/config/report corruption cases |

The CLI local pipeline below adds an offline replay proof after its disposable
server has exited. No live model was installed or contacted; local server responses
are controlled test fixtures. HTTP success records are private operator evidence;
redacted failure statuses contain no raw provider bodies.
