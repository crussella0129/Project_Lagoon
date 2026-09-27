# Test Critique — Sprint 0

## Concerns

Read-only self-review using the installed Sprint Loops test-critic contract; no
independent reviewer or live-model verification is claimed. Reviewed the locked
plans, INT-0001 criteria, seven task commits, code and the executed result files.

### C-001: Hosted acceptance remains unproved
- **Where:** INT-0001 AC-7; T-001 E3; GitHub run 36299042116.
- **Quote:** "Formatting, clippy, affected tests, and CI pass."
- **Failure mode:** evidence-drift
- **Why it matters:** local checks pass, but the hosted job failed before any step
  started. A successful sprint or realized intent would overstate CI evidence.
- **Suggested response:** defer-with-rationale; preserve local confirmations,
  retain INT-0001 active, and rerun hosted CI after account runner availability is
  restored. This requires an external account action, not an implementation fix.

### C-002: Standalone manifests and counts needed complete assertions
- **Where:** T-005 E2/T-006 E3; manifest and report tests.
- **Quote:** "both plan-consent outcomes"; "nonreciprocal selections".
- **Failure mode:** weak-assertion
- **Why it matters:** the initial recording retained these facts only indirectly;
  standalone manifests and explicit metrics needed them for review.
- **Suggested response:** tighten-assertion — addressed in T-007. Manifests now
  carry both consents, mode, exact plan, parents/declared metadata and explicit
  false execution/verification flags. Tests assert those fields. Reports explicitly
  assert a positive nonreciprocal count and a metadata-blocked agreed batch/job.

### C-003: Scope of inference and replay evidence
- **Where:** e2e-tests.md; T-004/T-006; INT-0002 follow-on.
- **Quote:** "No live model was installed or contacted".
- **Failure mode:** stub-leak
- **Why it matters:** controlled HTTP responses establish transport and protocol,
  not actual model preferences, merge quality or trusted authentication of logs.
- **Suggested response:** defer-with-rationale — documented in usage and follow-on
  intents. Replay compares structures, preserves times and runs offline after the
  CLI test server exits; it does not authenticate an operator's consistent rewrite.

Final re-review: C-002 addressed and all 29 tests rerun at the cited head; C-003's
scope boundary is explicit. C-001 cannot be resolved inside this repository.

## Confidence
block
