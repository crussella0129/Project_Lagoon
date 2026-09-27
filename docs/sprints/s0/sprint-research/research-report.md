# Sprint 0 Research Report

## Intents Reviewed

- [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md) — created and revised; relevance: protocol baseline plus single/batch reproduction-plan consent; current state: proposed.
- [INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md) — created and revised; relevance: method-independent fusion and pool-specific comparisons; current state: proposed.
- [INT-0003](../../../intents/INT-0003-controlled-evolutionary-study.md) — created and revised; relevance: separate partner-choice and method-choice experiments; current state: proposed.

## 1. Sprint Goal

Build the smallest observable Lover's Lagoon protocol: a Rust CLI that runs bounded
communication and sealed reciprocal partner selection, respects private state and
voluntary abstention, replays recorded outcomes, and emits honest pending/blocked
merge requests. Include deterministic fixtures and a configured local HTTP
inference adapter. This advances INT-0001; actual weight fusion and evolutionary
experiments await INT-0002 and INT-0003. The proposed sprint boundary requires
plan approval before implementation.

## 2. Existing Code Survey

| File | Relevance | Notes |
|------|-----------|-------|
| README.md | high | The only original product specification: social choice, DARE fusion, privacy, non-elimination, and lineages. Preserve it. |
| LICENSE | high | Apache-2.0 repository license; not a license grant for external model weights. |
| docs/.sprint-loop-book | high | New schema v2 / substrate contract 4 marker. |
| docs/work/remote-profile.md | high | GitHub, main base, codex/dev work, human-approve merge policy. |
| docs/work/tasks.md | high | Empty persistent backlog; executable tasks are not scheduled before plan approval. |
| docs/work/completed-tasks.md | medium | Empty append-only completion ledger. |
| docs/sprints/s0/sprint-meta.md | high | Initialized active Sprint 0; research provenance. |
| docs/intents/INT-0001-voluntary-observable-partner-choice.md | high | Proposed privacy/choice protocol and measurable boundaries. |
| docs/intents/INT-0002-evaluated-dare-descendants.md | high | Later real weight merge and child evaluation requirements. |
| docs/intents/INT-0003-controlled-evolutionary-study.md | high | Later controls, learning distinction, and population policy. |
| .github/dependabot.yml | medium | Generated GitHub Actions updater targets codex/dev; add Cargo intake with the crate. |
| .gitignore | medium | Generated transient-only ignores; target and run outputs belong in the build plan. |

No implementation, crate manifest, test suite, model pool, endpoint settings,
checkpoint artifacts, or preexisting CI workflow exists. Cargo and rustc are
available locally. TaskCreate is unavailable in this host; the research progress
artifact tracks this intake and the Book ledger will track approved build tasks.

## 3. External Sources

- [Language Models are Super Mario — Yu et al., ICML 2024](https://arxiv.org/abs/2311.03099) — DARE drops/rescales fine-tuning deltas of homologous models; successful settings are not arbitrary fusion guarantees.
- [mergekit official repository](https://github.com/arcee-ai/mergekit) — maintained merger CLI, DARE variants, tensor/tokenizer alignment, out-of-core operation; prefer an external merger boundary later.
- [Evolutionary Optimization of Model Merging Recipes — Akiba et al.](https://arxiv.org/abs/2403.13187) — evolutionary recipe search is established; social parent selection remains an unproven mechanism.
- [Signs of introspection in large language models — Anthropic](https://www.anthropic.com/research/introspection) — limited, unreliable functional introspection; self-report alone cannot establish experience or accurate internal state.
- [The random stable roommates problem typically has no solution — Chin and Michelen](https://arxiv.org/abs/2601.07612) — single-pool stability is not a universal matching guarantee; do not import two-sided assumptions.

- [mergekit official Merge Method Guide](https://github.com/arcee-ai/mergekit/blob/main/docs/merge_methods.md) — method registry, variants and prerequisites; no universal best method.
- [TIES-Merging — Yadav et al.](https://arxiv.org/abs/2306.01708) — trimming and sign-conflict resolution as a candidate alongside DARE.
- [DELLA-Merging — Deep et al.](https://arxiv.org/abs/2406.11617) — magnitude-based sampling; reported language/math/code gains over DARE/TIES are setting-specific.
- [DARE the Extreme — Deng et al.](https://arxiv.org/abs/2410.09344) — failure at large deltas/extreme pruning; modified rescaling and training-time variants.
- [Model Stock — Jang et al.](https://arxiv.org/abs/2403.19522) — geometry-based few-model combination; original CLIP evidence does not establish LLM-specialty transfer.
- [Behavior Knowledge Merge in Reinforced Agentic Models — Yuan et al.](https://arxiv.org/abs/2601.13572) — 2026 preprint; RAM targets RL-specific sparse heterogeneous updates.
- [Multi-Objective Bayesian Optimization for Model Merging — Agarwal et al.](https://arxiv.org/abs/2608.14264) — August 2026 preprint; different best operators across evaluated settings and a capability-trade-off search layer.
- [Stay Unique, Stay Efficient — Guo et al.](https://arxiv.org/abs/2512.01461) — task-specific information preservation; terminology about personality does not imply emotional continuity.

Sources accessed 2026-09-27. These thirteen primary sources inform the bounded
recommendation; exploratory search results were not treated as supporting evidence.

## 4. Risks, Unknowns, Dependencies

- **Risk R-1 — Privacy leakage:** serializing whole agent structs or provider
  errors can expose private state, checkpoint identity, or credentials. Explicit
  public/owner projections and transport error redaction need adversarial tests.
- **Risk R-2 — Ordering effects:** peers must not see same-step responses or
  partial ballots. Freeze observations, stage results, and order publication by
  configured anonymous handle rather than arrival time.
- **Risk R-3 — Invalid-response coercion:** timeout/invalid JSON must not become
  voluntary abstention or an automatic alternate partner. Separate statuses.
- **Risk R-4 — Incentive drift:** loneliness cost, popularity rank, elimination,
  and forced matchmaking would alter the baseline. None is enabled in this plan.
- **Risk R-5 — False inheritance:** private memory changes are not new weights,
  and a merge request is not a child. Use precise states; real fusion is INT-0002.
- **Risk R-6 — Stochastic inference:** live seed support may vary. Record actual
  requests/responses; promise exact replay, not identical live model reruns.
- **Risk R-7 — Population dilution:** independent uniform nominations predict
  expected matched fraction 1/(n-1), not an automatic exponential law. Measure
  exposure/reciprocity separately; generations and population policy are INT-0003.
- **Risk R-8 — Unvalidated provenance:** matching registry metadata is necessary
  screening, not tensor verification or legal approval. Automatic execution is
  absent until INT-0002 checks actual artifacts and licenses.
- **Risk R-9 — Harness manipulation:** model messages remain data, cannot become
  system prompts, invoke tools, change config, or cross the private-state boundary.
  Textual persuasion may still influence agents; this is part of the environment.
- **Risk R-10 — Method choice is not method quality:** models may choose familiar
  names or disagree. Separate pairing from exact reproduction-plan consent; no fallback.
  Record fixed/mutual mode and card order. Later fixed-pair comparisons, held-out
  tests, and budget-matched search controls belong to INT-0002/INT-0003.
- **Risk R-11 — Sibling count is not useful variance:** more children increase
  compute and attention opportunities; siblings share parentage. Keep one-child
  baseline plans, optional bounded batches, exact per-child recipes/counts, and
  distinct batch/request denominators. Real diversity and fair admission need
  equal-count/budget controls and shared-parentage analysis in INT-0002/INT-0003.
- **Unknown:** starting checkpoint family/revisions, hardware and storage budget,
  descendant memory policy, child thresholds, and scientific payoff model. None
  blocks fixtures/local-adapter construction; all block claims about real fusion.
- **Dependency:** Rust protocol/config/CLI ecosystem; established merger through
  a later external process. No greenfield Python harness is needed.
- **Dependency:** local loopback chat-completions service for optional live runs.
  Fixtures and local HTTP test servers avoid credentials and paid inference.
- **Dependency:** the skill's Plan Phase requires concrete user approval before
  plans become executable. EnterPlanMode/ExitPlanMode are not exposed by this host;
  preserve the same gate with scratch drafts and unchanged implementation source.

## 5. Recommended Approach

Primary: typed Rust core; bounded concurrent backend calls against frozen
observations; optional model-authored private self-reports; one sealed ballot per
agent; exclusive reciprocal pairing; pending/blocked manifests; separate public
and operator-only records; replay and descriptive report commands. Record exact
procedural prompts and decoding metadata. Match decisions do not use capability
rankings. No automatic model downloads, weight jobs, or fine-tuning occur.

Following the user's method-comparison request, expose declared immutable recipes
in fixed-plan or mutual-choice mode. Permit independent exact-plan consent, decline,
or deferral. Pairing alone never authorizes fusion; recipe disagreements create
blocked requests without changing the social pair. Compare real methods later.

The sibling refinement permits an exact plan with one or a bounded list of child
recipes. Baseline max_siblings=1; an optional two-child example and ceiling of
three are proposed, not empirical optima. Every request retains the same original
parents and per-child method/seed; no tensor execution or automatic population
growth is added to this first sprint.

Alternative considered: integrate Python/mergekit and real multi-generation runs
immediately. Rejected for this sprint because parent pool, compute, inheritance,
and evaluation criteria are unresolved and protocol correctness is prerequisite.
The later Rust merger adapter may invoke the mature Python merger ecosystem.

Rationale: preserves voluntary choice while making the eventual causal experiment
auditable. Start with an offline fixture vertical slice, then verify local HTTP
integration without presenting it as evidence of improved or conscious models.
Self-selection/random/benchmark comparisons and true tensor fusion are explicitly
retained in the follow-on intents.

## Artifacts

- [Idea assessment](idea-review.md) — review, research findings, and derived random-matching sanity check.
- [Fusion-method review](fusion-method-review.md) — alternatives, evidence limits, recipe consent, and comparative experiment design.
- [Sibling-batch review](sibling-batch-review.md) — optional method-diverse families, exact plan consent, bounds, and count/diversity controls.
- [Research progress](progress.md) — completed intake and remaining approval gate.
- [Sprint metadata](../sprint-meta.md) — initialized sprint provenance.
- [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md), [INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md), [INT-0003](../../../intents/INT-0003-controlled-evolutionary-study.md) — stable proposed outcomes.

## Budget Override

The user explicitly expanded the research during planning to ask whether DARE is
best, what alternatives exist, and whether models should choose. This spans merge
operators, newer RL-specific approaches, multi-objective search, and behavioral
inheritance. Thirteen selected primary sources exceed the default five-source
limit for that cross-cutting comparison; the code survey stays below twenty and
the added research remains within the phase's thirty-minute research allowance.
No tensor experiments or extra implementation scope have been silently approved.
