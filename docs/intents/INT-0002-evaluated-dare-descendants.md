# INT-0002 — Evaluated DARE descendants

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0002
- **State:** proposed
- **Work evidence:** none
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** [Research assessment](../sprints/s0/sprint-research/idea-review.md)

## Intent

Turn voluntary, reciprocal partner selections into real DARE-derived checkpoint
artifacts, evaluate them, and admit usable descendants without replacing their
parents. This preserves the README's eventual family and lineage exploration.
This is follow-on work, not an acceptance claim for Sprint 0.

## Acceptance criteria

- **AC-1:** verify immutable model revisions, actual tensor names/shapes, shared
  base provenance, tokenizer mapping/chat template, accessible weights, licenses,
  and resource bounds before executing a merge.
- **AC-2:** record the merger version, exact method (DARE plus task arithmetic or
  DARE-TIES), recipe, seed, parent/base hashes, and child hash. A failed job creates
  no active child and never destroys either parent.
- **AC-3:** test loading, finite weights, coherent generation, held-out capability
  retention, and behavioral regressions before admitting a child. Criteria and
  non-admission reasons are explicit; failed artifacts remain research evidence.
- **AC-4:** record a lineage graph with parent and child checkpoint identity.
  Distinguish weight inheritance from a separately declared memory policy. Private
  parental memories are not silently copied into a child or shared with peers.
- **AC-5:** demonstrate at least one real compatible-checkpoint merge end to end
  using declared hardware/storage limits; report measured costs and limitations.

## Rationale

DARE sparsifies parameter deltas relative to a base and can retain complementary
abilities in studied settings. Neither conversation nor mutual selection proves
compatible weights or superior descendants. Use a maintained merger such as
mergekit behind a Rust job boundary instead of reimplementing tensor merging.

## Alternatives

Textual roleplay offspring cannot substitute for a weight artifact. Arbitrary
cross-architecture fusion, distillation, and training from social transcripts
would introduce different methods and require separate intents.

## Consequences

Checkpoint selection, licenses, merger pinning, hardware, storage retention, and
evaluation thresholds remain unresolved inputs for the later fusion plan.
Recursively merged descendants may drift away from the small-delta regime.

## Transition history

- 2026-09-27: created as `proposed`; preserves the fusion objective while the
  protocol is investigated first. No fusion run has been scheduled or performed.
