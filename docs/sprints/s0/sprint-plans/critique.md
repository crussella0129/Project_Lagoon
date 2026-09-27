# Plan Critique — Sprint 0

## Concerns

### C-001: Fixture verification cannot establish provider or fusion quality
- **Where:** build-plan.md T-004/T-007; test-plan.md End-to-End Tests.
- **Quote:** "Real model/child E2E is unlocked by INT-0002."
- **Failure mode:** missing-risk
- **Why it matters:** Passing a mock HTTP service is evidence about the adapter,
  not a specific model's response compliance or a useful tensor merge.
- **Suggested response:** defer-with-rationale. Already bounded by INT-0002 and
  explicit test-plan reporting; preserve this limit in usage and final evidence.

### C-002: Replay integrity is structural rather than authenticated provenance
- **Where:** build-plan.md T-006 E2; test-plan.md unsupported_or_inconsistent_records_are_rejected.
- **Quote:** "structurally inconsistent/tampered records"
- **Failure mode:** missing-risk
- **Why it matters:** Recomputing derived outcomes detects inconsistent changes,
  but a trusted operator can rewrite a fully consistent record. Do not imply
  authentication or secrecy from that operator.
- **Suggested response:** defer-with-rationale. Reject inconsistent records and
  unknown schemas, record content fingerprints, and document that this is not
  signed evidence. Authenticated external provenance is outside INT-0001.

## Confidence
proceed-with-caveats
