# Optional siblings from different merge methods

2026-09-27: proposed refinement following the user's sibling question. This is a
design recommendation; no sibling checkpoint has been generated or evaluated.

## Recommendation

Permit consent to a bounded reproduction plan containing one or several child
recipes. Parents do not need to commit to one method for their entire lineage.
Each individual child's operation remains specified and recorded. For example,
the same parent pair could consent to one TIES child and one DARE-TIES child.
Keep single-child plans as the baseline, and offer an optional two-child batch
first; a configured ceiling of three is a proposed resource bound, not an
empirically optimal family size.

The [evolutionary merging study](https://arxiv.org/abs/2403.13187) supports
exploring multiple recipes, and [MOBO-Merge](https://arxiv.org/abs/2608.14264)
finds different operators useful in different settings. Neither study establishes
that method-diverse social siblings improve this experiment. That is a hypothesis
to test rather than a guaranteed source of useful variance.

## What consent covers

An immutable plan references a finite list of child recipes, each with its method,
parameters, base policy, and recorded seed specification, plus child count and
resource limits. Both parents choose the same offered plan, decline, or defer.
Single-child and multi-child plans are equally explicit. A vague request for
"whatever offspring works best" does not authorize unlimited search or births.

The later execution layer validates all jobs and resource bounds before starting
the batch. It never silently adds methods, drops planned siblings, or exceeds
consented counts. If capacity is unavailable, defer and record that outcome.
Execution failures are recorded per job; they do not imply lack of parental
consent and do not erase successful siblings or the social pair.

For an initial auditable protocol, identical plan IDs/fingerprints must match.
Broader consent to a bounded sampling policy could be a later extension, provided
the policy specifies allowed methods, parameter ranges, sampling rule, counts,
and budgets. A stochastic recipe does not need advance knowledge of its sampled
mask to be meaningful consent; the rule and realized seed are recorded.

## Sources of variation

- Different methods can change conflict handling and which deltas survive.
- Different valid coefficients or pruning settings can change a child's weights.
- Different random seeds can change DARE/DELLA pruning masks. Repeating an
  otherwise deterministic merge with another seed need not change the result.
  [DARE failure analysis](https://arxiv.org/abs/2410.09344) also shows why more
  extreme randomness/pruning should not be equated with better offspring.
- Each sibling starts from the same pinned original parents/base, independently.
  Merging the first child into the second would change parentage, not produce a
  direct sibling comparison. Dialogue-memory inheritance remains separate.

Weight differences alone do not prove behavior differences, and siblings share
parents and much of their structure. Measure common-task capability profiles and
response disagreement under matched decoding conditions. Report uncertainty by
parent pair/run rather than count correlated siblings as independent replications.

## Experimental and population consequences

For method effects, compare same-count batches under equal total tuning/evaluation
budgets: repeated stochastic seeds within one method versus multiple methods.
Separately compare single-child and sibling plans to study family-size effects.
Record differences in consent, successful jobs, admissions, attention, and storage.
Method and seed variation are not automatically equivalent amounts of diversity.

Valid siblings should not be reduced to an undisclosed "best child survives"
contest. Define basic child viability checks, bounded fair admission/exposure,
and an explicit waitlist before actual births. Parents remain present. More
children change future matching opportunities and representation, so lineage
concentration and active-population growth are part of the outcomes.

Sprint 0 only validates consent to single/batch plans and emits grouped pending
or blocked child requests. Real batch jobs, viability, diversity measurements,
and admission belong to INT-0002/INT-0003. The sibling option does not bypass
the existing plan approval gate or authorize additional compute now.
