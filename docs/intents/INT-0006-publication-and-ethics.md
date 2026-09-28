# INT-0006 — Publication and ethics

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0006
- **State:** active
- **Work evidence:** [T-015 and T-016 build plan](../sprints/s1/sprint-plans/build-plan.md)
- **Completion evidence:** none
- **Code evidence:** [Citation metadata](../../CITATION.cff)
- **Test evidence:** [Sprint 1 partial acceptance](../sprints/s1/sprint-tests/test-report.md)
- **Documentation evidence:** [Origin](../../ORIGIN.md); [ethics](../../ETHICS.md); [publication prerequisites](../publication.md); [draft studies](../../PREREG.md)

## Intent

Preserve the project's design provenance, document precautionary treatment of
participants, distinguish the three proposed studies, and make released work
citable. Keep the README clinical. Documentation of a safeguard must not imply
that the current harness implements it. Research remains exploratory until a
frozen, independently registered protocol and its operational gates exist.

## Acceptance criteria

- **AC-1:** ORIGIN.md contains the original public concept verbatim with its source
  commit and separates historical proposals from current protocol authority.
- **AC-2:** ETHICS.md covers the eight welfare, consent, distress, exit, containment,
  media, vocabulary and licensing requirements and identifies unimplemented controls.
- **AC-3:** Study B's draft is preserved at `prereg/study-b-screening.md`; a separate
  Study A draft covers treatments, sampling, outcomes, hypotheses, exclusions,
  parameter recovery, registration and unresolved freeze decisions.
- **AC-4:** Valid citation metadata names the repository and verified author without
  inventing a DOI. Zenodo integration and a verified release DOI remain required
  before any claim of immutable archival publication. Per the user's explicit
  deferral, they no longer block G0 or further implementation.

## Rationale

Design values and procedural protections are part of the method, while self-reports
are not evidence of consciousness. Cheap inference-only Study A precedes training.
Publication records must separate implemented, proposed and externally verified facts.

## Alternatives

Leaving the origin only in Git history obscures it. Replacing the clinical README
with the original proposal obscures implementation limits. A movable tag or invented
DOI is not an independent archive. A single preregistration conflates distinct studies.

## Consequences

Zenodo authentication and GitHub release publication are external dependencies.
Unavailable access leaves archival activation open; setup instructions do not
satisfy integration. The user explicitly deferred it to allow implementation.
No paid compute, real experiments, external
registration, release, training or generational execution is part of Phase 0.
Private operator data require separate publication review even if the code is public.

## Transition history

- 2026-09-28: created as `proposed` from the authorized improvement plan.
- 2026-09-28: `proposed` → `planned`; Phase 0 documentation and integration scheduled
  in Sprint 1, with external access recorded as a dependency.
- 2026-09-28: `planned` → `active`; T-015 documentation implementation started.
- 2026-09-28: revised while `active` after "Defer Zenodo; continue implementation";
  citation preparation remains in Phase 0, archival activation moves to the release
  gate and no longer blocks G0. T-017 remains open; no claim of activation is made.

## Current evidence and remaining work

Sprint 1 proves provenance, policy, study separation and citation preparation.
The intent remains active: T-017 requires real archival activation and the later
approved release DOI; T-021 requires a frozen, independently registered Study A.
Documented instructions are not evidence those external actions occurred.
