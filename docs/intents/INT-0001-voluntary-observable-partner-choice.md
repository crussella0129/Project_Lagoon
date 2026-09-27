# INT-0001 — Voluntary, observable partner choice

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0001
- **State:** active
- **Work evidence:** [Sprint 0 build plan](../sprints/s0/sprint-plans/build-plan.md)
- **Completion evidence:** [T-001 through T-007 implementation](../work/completed-tasks.md)
- **Code evidence:** [Rust harness](../../src/lib.rs); tested implementation head `2683e908854d1635645e23e5970bca2a4f1e9023`
- **Test evidence:** [Local unit](../sprints/s0/sprint-tests/unit-tests.md), [integration](../sprints/s0/sprint-tests/integration-tests.md), [E2E](../sprints/s0/sprint-tests/e2e-tests.md); [external CI block](../sprints/s0/failure-report.md)
- **Documentation evidence:** [Usage](../usage.md); [Research assessment](../sprints/s0/sprint-research/idea-review.md)

## Intent

Build a research harness in which multiple model-backed agents can communicate,
retain private self-reports, and independently select a single partner or abstain.
Preserve the README's emphasis on agency, comfortable participation, open choices,
and avoiding elimination pressure. Models receive procedural instructions about
available actions and consequences, without prescribed desires, personalities,
romance narratives, popularity classes, or instructions to reproduce.

The first increment is a Rust command-line harness with deterministic fixtures and
an explicitly configured local inference backend. It records reciprocal selections
and pending merge requests. It does not claim to create children or measure
consciousness. Actual fusion and multi-generation studies belong to INT-0002 and
INT-0003. These boundaries were approved for Sprint 0.

## Acceptance criteria

- **AC-1 — Valid configuration:** reject duplicate agents, invalid bounds, missing
  backend settings, and malformed model provenance before a round starts.
- **AC-2 — Privacy and blindness:** peer observations contain public messages and
  anonymous experiment handles, never another agent's private fields, checkpoint
  identity, backend metadata, or unrevealed ballot. The owning agent can access its
  own optional `feeling`, `learned_preference`, and `thought` self-report fields.
  Privacy describes harness access; an agent can voluntarily disclose content.
- **AC-3 — Independent rounds:** agents at each communication step act on one
  frozen public snapshot. Responses are staged before publication. All selection
  observations derive from a shared final snapshot under a declared memory policy,
  with recorded owner-specific fitting projections, and ballots are sealed until all attempts
  finish. Worker completion order does not change public ordering or outcomes.
- **AC-4 — Honest participation outcomes:** explicit abstention, invalid response,
  timeout, transport failure, and nonreciprocal selection remain distinct. Only
  A selecting B and B selecting A creates a pair. Each agent occurs in at most
  one pair per round. Abstention and failed pairing preserve the participant and
  its memory; there is no imposed loneliness cost, replacement, or exclusion.
- **AC-5 — Honest merge boundary:** configuration declares immutable bounded
  recipe IDs and reproduction-plan IDs in fixed-plan or mutual-choice mode.
  Each plan specifies one child or an optional bounded sibling batch, with each
  child's recipe/seed specification and total count/resource bounds. Baseline
  max_siblings is one; opt-in batches permit at most three in this first increment.
  Each agent separately consents to one exact offered plan, declines, or defers
  fusion. The model selects only a plan ID; the harness binds its receipt to the
  fingerprint in the immutable recorded observation. Reciprocal
  partner selection does not imply recipe consent. A pair produces a pending
  grouped request only when both parents agree to the same allowed plan; absent,
  declined, different, or invalid consent creates a clearly blocked request.
  Never silently substitute methods, add siblings, or truncate a consented batch.
  Record an individual pending/blocked child request for each declared recipe.
  Missing or incompatible declared common
  base revision, architecture/tensor-layout signature, tokenizer signature, or
  license metadata blocks automatic execution eligibility. Metadata compatibility
  is a preliminary claim, not verified tensor compatibility. No request is
  reported as a completed fusion, and no child is added to the population.
- **AC-6 — Reproducibility and measurement:** bounded multi-round runs persist
  configuration, seed, procedural prompt, backend/decoding metadata, owner-private
  self-reports and ballots in an operator-only record, and a separate public event
  projection. Replay uses recorded responses without new inference. Reports count
  participation, explicit abstention, invalid/failure outcomes, nominations,
  reciprocal pairs, plan-consent outcomes, requested child counts, and
  blocked/eligible batches and child requests with clear denominators. Record
  mode, recipe and plan payloads, per-owner/per-round shuffled card and opaque-handle
  order, schema hashes, declared memory/token budgets, token measurements and trimming,
  and exact consent. Persist private preferences while bounding public context. A
  fixed recorded fixture run replays identically, regardless of completion order.
- **AC-7 — Usable vertical slice:** fixture and local HTTP workflows run through
  the CLI with tests covering privacy, ordering, failures, pairing, and replay.
  Formatting, clippy, affected tests, and CI pass. Document how to run and what
  the experiment's observations can and cannot establish.

## Rationale

Separate the validity of the social protocol from the feasibility and quality of
weight merging. Without reliable privacy, ballots, and outcome accounting,
interesting transcripts cannot establish which selection rule caused an outcome.
Treat no-pair runs as evidence. Fix model weights during this first increment;
persistent textual preferences are contextual memory, not weight updates.

## Alternatives

- A forced stable-matching algorithm would override independent final choices;
  reciprocal single nominations implement the initial procedural commitment.
- A loneliness penalty may be a future experimental treatment, but changes the
  incentives and conflicts with an unpressured baseline.
- Hardcoding DARE precludes useful comparators and method choice. Offer a bounded
  immutable catalog with fixed-plan controls; method names alone are not recipes.
- Requiring a single method per family needlessly restricts variation. Optional
  explicitly consented sibling batches preserve reproducibility while allowing
  different child recipes; whether they add behavioral diversity is unresolved.
- Full Python orchestration would ease direct mergekit integration; Rust suits
  the user's preference and the typed protocol, with an external merger later.
- A desktop interface is premature before the protocol is testable. If pursued,
  the user's Electron-first preference applies.

## Consequences

Anonymous handles hide configured identities, but dialogue may reveal or suggest
identity. The trusted operator retains private research records; the first
increment does not provide cryptographic secrecy from that operator. Model text
is inert data and cannot change harness policy or grant tools. Provider inference
can be stochastic even with recorded seeds; only replay promises exactness.
The neutral prompt still influences behavior and is itself recorded evidence.

The first recipe catalog uses symmetric parent coefficients with typed bounded
parameters. Asymmetric inheritance, arbitrary executable recipes, and automatic
recipe optimization require a subsequent extension of this intent and its tests.

## Transition history

- 2026-09-27: created as `proposed` from README.md and the requested idea review;
  first-sprint implementation boundaries await concrete plan approval.
- 2026-09-27: revised while `proposed` following the user's method-comparison
  request; added exact-recipe consent, fixed versus mutual-choice mode, and no
  automatic fallback. This recommendation awaits plan approval.
- 2026-09-27: refined while `proposed` after the sibling question: consent can
  cover an exact single-child or bounded sibling plan; each child's recipe stays
  explicit. Baseline remains one child, with optional batches and separate counts.
- 2026-09-27: `proposed` → `planned` after the user approved the revised plan
  with "ok now proceed"; work is scheduled in the Sprint 0 build plan.
- 2026-09-27: `planned` → `active` as T-001 implementation begins.
- 2026-09-27: implementation T-001 through T-007 and all local checks complete.
  State remains `active` because hosted CI could not start due to GitHub account
  runner availability; no duplicate lifecycle transition is recorded. Preserve
  implementation evidence and carry verification forward as T-008.
- 2026-09-27: user selected instrument repairs and variance verification, then
  review plus a draft next-study preregistration. Added constrained output,
  harness-bound fingerprint receipts, recorded randomized presentation and
  declared token-aware memory without changing voluntary choice or the fusion gate.
  [Follow-up work](../work/instrument-review.md) preserves Sprint 0 evidence.
