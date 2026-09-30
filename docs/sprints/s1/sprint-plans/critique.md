# Plan Critique — Sprint 1

## Concerns

### C-001: Archival access cannot be treated as an implemented integration
- **Where:** build-plan.md T-016; INT-0006 AC-4.
- **Quote:** "record the actual access blocker and an open activation task"
- **Failure mode:** missing-risk
- **Why it matters:** Citation metadata alone does not enable Zenodo. G0 expressly
  requires Phase 0 completion, so later feature work remains gated if access fails.
- **Suggested response:** defer-with-rationale; retain the activation task and
  unresolved intent criterion. Both plans already distinguish preparation from activation.

### C-002: No destructive live merge test is necessary
- **Where:** test-plan.md pr_required_checks.
- **Quote:** "no artificial failing merge is attempted"
- **Failure mode:** e2e-drift
- **Why it matters:** Observing active protection and required checks is configuration
  evidence; it must not be described as a deliberately rejected merge attempt.
- **Suggested response:** reject; the plan explicitly preserves that distinction and
  tests the feasible GitHub behavior without modifying main.

## Confidence
proceed-with-caveats
