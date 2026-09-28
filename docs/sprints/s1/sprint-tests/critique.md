# Test Critique — Sprint 1

Read-only local review using the installed test-critic contract. Reviewed locked
plans with the explicit user amendment and instrument extension, completed-task
commits, all three test artifacts and linked INT-0005/6/7 acceptance criteria.
Final pass reflects the successful CI result on bc5cec1472eda883c0278ebe1d7a4f58cb222d61.

## Concerns

### C-001: Real-server acceptance remains unproved
- **Where:** INT-0005 Acceptance criteria; e2e-tests.md Genuine limits.
- **Quote:** "commit real vLLM and llama.cpp conformance receipts"
- **Failure mode:** EARS-coverage
- **Why it matters:** Fixture replies cannot prove that grammar, reasoning or maximum output behavior holds on a real server.
- **Suggested response:** defer-with-rationale. The pre-build instrument extension scoped T-024 to provenance/kit preparation and explicitly assigned actual execution to T-018. Keep INT-0005 active, G1 open and the reserve unchanged. Do not label the whole improvement plan complete.
- **Disposition:** Accepted; no real conformance result or realized intent is claimed.

### C-002: Recovery is not coverage calibration
- **Where:** integration-tests.md Planted-parameter recovery.
- **Quote:** "eight seeds, 40 rounds and eight fixture participants"
- **Failure mode:** weak-assertion
- **Why it matters:** A fixed passing scenario establishes the estimator/export contract, not nominal confidence-interval coverage or real-study power. Few-cluster normal intervals need empirical validation.
- **Suggested response:** defer-with-rationale. Keep the unchanged planted specification and report; leave calibration, pilot, frozen zero-outcome/missingness and multiplicity rules to T-020 before G2.
- **Disposition:** Accepted and prominently documented in analysis/README.md and the roadmap. No G2 claim.

### C-003: Historical plan clauses were superseded explicitly
- **Where:** locked test-plan.md T-014/T-016 versus sprint-plans/plan-amendment.md.
- **Quote:** "both jobs required"; "keeping G0 open"
- **Failure mode:** evidence-drift
- **Why it matters:** The original clauses request a numbered minimum/second CI job and Zenodo as a G0 prerequisite; the user explicitly replaced those requirements.
- **Suggested response:** reject (the critique is wrong because the user-authorized amendment controls current execution). Retain locked history and show stable-only CI plus the open release prerequisite.
- **Disposition:** Verified. Active required `check` runs all implemented suites; archival work remains open. INT-0007 is eligible for realization; INT-0006 remains active.

### C-004: Final checkpoint needs its own required-check observation
- **Where:** e2e-tests.md Main protection.
- **Quote:** "created only after Book closure"
- **Failure mode:** evidence-drift
- **Why it matters:** The existing branch CI receipt cannot claim that an as-yet unopened PR has passed its merge check.
- **Suggested response:** defer-with-rationale. Create the main-targeted checkpoint after closing the Book, inspect its head/required checks, and leave human merge approval in place.
- **Disposition:** Retained as the final workflow action, not a claimed passing PR result in this report.

## Confidence
proceed-with-caveats
