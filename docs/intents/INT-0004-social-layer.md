# INT-0004 — Social selection layer

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0004
- **State:** proposed
- **Work evidence:** none
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** [Roadmap](../roadmap.md)

## Intent

Implement the improvement plan's inference-only social environment after G0.
Agents choose whether and when to describe themselves, communicate, participate
and leave. Model text stays inert. Real runs additionally require conformance G1.

## Acceptance criteria

- Answer state distinguishes unasked, declined and given with round provenance;
  appearance is write-once when chosen and revisable fields retain history.
- Field visibility is explicit for appearance, loyalty, lineage distinctness,
  self-description and seeking; disclosure is optional and costless.
- Solo skill, hidden-profile sharing, shared-attempt-budget and credit-claim events
  have deterministic checkers, fixture tests, disjoint task pools and recorded teams.
- Public structured promises have optional delayed ballot reveal and kept/broken
  accounting. Optional bonds allow reaffirmation or another nomination.
- Bounded whispers reach only recipients/operator; appearance timing is a treatment.
- Leaving archives the participant without penalty. Behavioral questions, a frozen
  disclaimer classifier and factual manipulation checks add measurements without
  penalizing or suppressing disclaimers.

## Rationale

Observe behavior and available information before drawing conclusions about stated
preferences. Build Study A capabilities that later generational studies can reuse.

## Alternatives

Inferring promises from prose creates annotation ambiguity. LLM skill judges violate
the deterministic-scoring requirement. Forced profiles or participation change incentives.

## Consequences

Requires a new record version, privacy tests and larger configurable schemas. The
current harness provides none of these social features yet. See the roadmap for
the build order and gates; no training or real child admission is included here.

## Transition history

- 2026-09-28: created as `proposed`; scheduled after improvement-plan gate G0.
