# Sprint 0 approval preview

Proposed outcome: a working Rust command-line research harness for voluntary
model partner choice, with fixtures and a local inference adapter.

The implementation has seven tasks: configuration and CI; private/public state;
bounded concurrent rounds; local HTTP inference; reciprocal pairs and screened
merge requests; records/replay/metrics; CLI/examples/documentation.

The baseline preserves optional private self-reports and consequence-free
abstention. Anonymous handles hide configured checkpoint identities, and sealed
ballots prevent peers from observing one another's decisions before resolution.
Only reciprocal single selections become pairs. Failed calls are recorded as
failures rather than disguised as abstention.

Fusion is method-independent. The proposed catalog covers linear, TIES,
DARE-TIES, and DELLA recipes. Fixed-recipe and mutual-choice modes both permit
declining fusion. A social pair creates a pending request only when both parents
consent to the same exact offered recipe; disagreements stay blocked without a
fallback. Sprint 0 validates/records those choices, not the tensor algorithms.

This sprint emits pending or blocked merge requests. Real weight fusion,
evaluated descendants, and controlled multi-generation runs are preserved in
INT-0002 and INT-0003 and are subsequent implementation work.

- [Build draft](build-plan.draft.md)
- [Test draft](test-plan.draft.md)
- [Idea review](../sprint-research/idea-review.md)
- [Research report](../sprint-research/research-report.md)
- [Fusion methods and model choice](../sprint-research/fusion-method-review.md)

## Approval boundary

The invoked sprint-loop skill's phases/03-plan-phase.md says: "The user controls
plan approval. After approval, write" the canonical build and test plans.
EnterPlanMode and ExitPlanMode are unavailable in this host. These are scratch
drafts, implementation remains untouched, and neither canonical plan is locked.

Approval of this proposal accepts the first increment's bounded scope and baseline
choices. Then INT-0001 becomes planned, the canonical plans receive a bounded
critic review, and the installed helper locks them before build begins. The remote
profile retains human-approve; no remote push or merge has been authorized here.
