# INT-0002 — Evaluated model-fusion descendants

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0002
- **State:** proposed
- **Work evidence:** none
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** [Research assessment](../sprints/s0/sprint-research/idea-review.md)

## Intent

Turn voluntary, reciprocal partner selections into real method-declared checkpoint
artifacts, evaluate them, and admit usable descendants without replacing their
parents. This preserves the README's eventual family and lineage exploration.
This is follow-on work, not an acceptance claim for Sprint 0.

## Acceptance criteria

- **AC-1:** verify immutable model revisions, actual tensor names/shapes, shared
  base provenance, tokenizer mapping/chat template, accessible weights, licenses,
  and resource bounds before executing a merge.
- **AC-2:** use a method registry and record both parents' exact reproduction-plan
  consent for a single child or bounded siblings, and each child's
  merger version, method, parameters, seed, parent/base hashes, and child hash.
  Revalidate consent and compatibility before execution. A failed job creates
  no active child and never destroys either parent.
- **AC-3:** test loading, finite weights, coherent generation, held-out capability
  retention, and behavioral regressions before admitting a child. Criteria and
  non-admission reasons are explicit; failed artifacts remain research evidence.
- **AC-4:** record a lineage graph with parent and child checkpoint identity.
  Distinguish weight inheritance from a separately declared memory policy. Private
  parental memories are not silently copied into a child or shared with peers.
- **AC-5:** demonstrate at least one real compatible-checkpoint merge end to end
  using declared hardware/storage limits; report measured costs and limitations.
- **AC-6:** compare an initial shortlist (linear, TIES, DARE-TIES, DELLA) on
  compatible pairs at reported equal tuning budgets and held-out tests. Report
  pool-specific capability trade-offs and failures rather than a universal winner.
- **AC-7:** validate the consented batch/resource limits before execution; create
  each sibling independently from the same pinned parents/base. Record per-child
  method, recipe, seed, hashes, validation and failures. Defer unavailable capacity
  explicitly rather than silently change count/methods. Do not treat siblings as
  independent experimental replications or select only a hidden benchmark winner.
- **AC-8:** evaluate coordinate/block/layer crossover as a separate proposed
  recipe family before generational claims. For LoRA, distinguish scaled delta
  products from factor arithmetic and record ranks, dense/concatenated output,
  precision and any truncation error. A toy DARE-average variance calculation
  does not select the real merger or establish skill retention.

## Rationale

DARE sparsifies parameter deltas relative to a base and can retain complementary
abilities in studied settings. Neither conversation nor mutual selection proves
compatible weights or superior descendants. DELLA/TIES are credible alternatives;
recent multi-objective work also finds different best operators across settings.
Use a maintained merger such as mergekit behind a Rust job boundary instead of
reimplementing tensor merging.

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
- 2026-09-27: revised while `proposed` after the user's request: generalized
  beyond DARE and added mutual recipe consent and pool-specific comparison.
- 2026-09-27: refined while `proposed` following the sibling question: support
  finite consented batches with independent child jobs and measured diversity.
- 2026-09-27: reviewed blending variance and the specialist-pool proposal; added
  delta-representation and crossover evaluation requirements. Intent remains
  proposed; no actual crossover operator or weight fusion has been implemented.
