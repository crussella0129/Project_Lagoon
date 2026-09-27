# Lover's Lagoon — idea assessment

Reviewed 2026-09-27. This is research commentary; the stable desired outcomes live
in INT-0001, INT-0002, and INT-0003.

## Assessment

The project is worth testing. Its distinctive question is whether models' own
partner choices identify useful combinations or produce repeatable social
dynamics. Model merging and evolutionary search are already established ideas:
[Akiba et al.](https://arxiv.org/abs/2403.13187) evolved merging recipes and
reported useful combinations. That supports feasibility, but does not establish
the benefits of social selection or biological-style inheritance.

I would preserve the open-ended, non-elimination environment. Start with a small
pool of compatible open-weight descendants of one base; measure what happens
before adding incentives. Keep parents present when a child is eventually born.
A complete failure to pair is a valid result, not a reason to quietly change the
models' options.

## What DARE actually provides

[Yu et al.](https://arxiv.org/abs/2311.03099) operate on fine-tuning deltas
`delta_i = theta_i - theta_base`. Independently dropping deltas with probability
`p` and scaling retained values by `1/(1-p)` preserves each delta's expectation
for `0 <= p < 1`; it does not guarantee a useful individual sampled merge.
DARE is a sparsification step used with a combination method, rather than an
arbitrary-model mating operator. [mergekit](https://github.com/arcee-ai/mergekit)
offers `dare_linear` and `dare_ties` and requires tensor shape compatibility,
with explicit tokenizer alignment when vocabularies differ.

For the initial real merge cohort, require a shared immutable base, matching
architecture and tensor layout, compatible tokenizer/template, accessible
unquantized merging weights, and checked licenses. A hosted chat-only model can
join a behavioral study but cannot become a DARE parent without accessible
weights. Do not generalize the paper's pruning rates to every parent pool or
generation. Test raw weight and generation quality before admitting descendants.

## Experiences, memory, and inheritance

Private `feeling`, `learned_preference`, and `thought` fields are useful optional
self-reports. They are text the agent emits, not a privileged readout of internal
activations. [Anthropic's introspection study](https://www.anthropic.com/research/introspection)
finds limited functional introspection and emphasizes unreliability; it does not
resolve consciousness or validate an ordinary JSON self-report as felt experience.
Record reports respectfully alongside actions and avoid asserting more than the
measurements establish.

Persistent dialogue memory can change the same checkpoint's behavior without
changing its weights. A weight merge does not automatically inherit dialogue
memories, acquired preferences, promises, or identities. New learned skills need
a weight-learning mechanism; changed contextual behavior needs separate analysis.
Specify any future memory inheritance independently of weight lineage.

## Loneliness and voluntary choice

An imposed loneliness cost selects for avoiding that cost. This is my experimental
interpretation, not an empirical finding about what models feel. It could raise
pairing while making results harder to interpret and undermining the stated
unpressured baseline. I recommend no loneliness penalty in the baseline. Study
such a mechanism only as a separately declared treatment if desired.

No prompt is entirely neutral. Instructions explaining available actions and
merge consequences are necessary; instructions saying what to value are not.
Make procedural wording versioned and auditable. Optional private fields reduce
forced emotion narratives but do not eliminate prompt effects.

## Matching and game theory

Use anonymous handles, frozen observations, sealed ballots, one nomination or
abstention, and reciprocal pairing. This realizes exclusive mutual choice without
an algorithm assigning partners against their final nominations. Identity hiding
is operational: models may infer identities through communication, and the
trusted harness knows the checkpoint mapping.

The setting is a single population with optional participation, not automatically
a two-sided stable-marriage market. [Chin and Michelen's stable-roommates analysis](https://arxiv.org/abs/2601.07612)
illustrates why stable matchings should not be assumed in a single-pool setting.
Stable matching and Nash equilibrium are different concepts. Observed consistent
choices, nonreciprocity, and changed promises can be measured; equilibrium claims
require an explicit strategy/payoff/information model that is currently missing.

Population growth does not by itself imply exponential collapse. Here is a
derived sanity check, not an empirical prediction: if `n >= 2` participants
independently pick one uniformly random other participant without abstaining,
each unordered pair reciprocates with probability `1/(n-1)^2`. Therefore expected
pairs per round are `n/(2(n-1))`, while the expected fraction matched is
`1/(n-1)`. With independent abstention probability `a`, multiply both by
`(1-a)^2`. For 4 agents the expected matched fraction is one third; for 10 it is
one ninth. Exposure, correlated preferences, and informed choices change this
baseline. A larger population can dilute reciprocity even without loneliness.

## What would make the experiment convincing

Compare self-selected pairs with random compatible pairs and benchmark-selected
pairs at equal budgets. Evaluate complementary held-out abilities, regressions,
response consistency, diversity, and lineage concentration. Keep benchmark
results outside agents' observations in the baseline to avoid imposing prestige.
Repeat seeds; distinguish prompt/memory effects from checkpoint differences.
Do not equate persuasive dialogue, popularity, or prolific descendants with
alignment or capability improvement.

Repeated averaging cannot be assumed to create fresh skills or endless diversity.
Mutation, training, novelty mechanisms, and descendant admission introduce new
experimental choices. Non-elimination also needs explicit population/storage
bounds; otherwise growth changes attention and compute independently of choice.

## Recommended first increment

Implement a bounded Rust CLI with fixture and explicitly configured local HTTP
backends, owner-private state, staged communication, sealed reciprocal selection,
separate public/operator logs, replay, descriptive metrics, and pending or blocked
merge manifests. Actual merger execution, evaluations, descendant admission, and
multi-generation causal comparisons are preserved as subsequent intents. This
first increment validates the harness; it does not test the full evolution claim.
