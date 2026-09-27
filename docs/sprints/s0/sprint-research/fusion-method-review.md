# Fusion methods and model choice

Reviewed 2026-09-27 following the user's request to compare DARE alternatives and
model agency. This is research, not a checkpoint benchmark; recommendations are
proposed until plan approval.

## Recommendation

Use a method-independent harness. DARE-TIES is a credible candidate, but evidence
does not establish it as best for this unknown parent pool, behavioral objective,
or successive generations. Models should be able to propose an exact recipe and
decline fusion, while the harness checks its compatibility and resource limits.
Measure method choice separately from partner choice.

DARE is a drop/rescale operation on parameter deltas. `dare_linear` and
`dare_ties` are different complete methods. Method, coefficients, density, base,
seed, tokenizer policy, and implementation version define the operation; mutual
agreement on the word "DARE" is insufficient.

## Candidate comparison

The final column is a project recommendation, not a universal ranking.

| Method | Mechanism / evidence | Project fit |
|--------|----------------------|-------------|
| Linear / task arithmetic | Weighted weights or common-base deltas. [Official guide](https://github.com/arcee-ai/mergekit/blob/main/docs/merge_methods.md). | Essential simple controls for whether sparsification adds value. |
| SLERP / NuSLERP | Two-parent interpolation; NuSLERP can use delta space. [Official guide](https://github.com/arcee-ai/mergekit/blob/main/docs/merge_methods.md). | Useful pairwise comparator if budget permits; geometry does not guarantee skill retention. |
| TIES | Trims small deltas and resolves opposing signs before combining. [Yadav et al.](https://arxiv.org/abs/2306.01708). | Strong comparator for specialist interference; density/weights need tuning. |
| DARE-linear / DARE-TIES | Random delta pruning and rescaling, without/with sign consensus. [Yu et al.](https://arxiv.org/abs/2311.03099). | Keep as candidates; repeat seeds and inspect delta sizes rather than assume extreme pruning works. |
| DELLA / DELLA-linear | Magnitude-dependent pruning probabilities; paper reports gains over DARE/TIES for language/math/code experts. [Deep et al.](https://arxiv.org/abs/2406.11617). | Include in the first real comparison; paper-specific wins do not establish a universal winner. |
| DAREx | Revises DARE rescaling for problematic pruning/deltas; DAREx-L2 additionally requires training-time regularization. [Deng et al.](https://arxiv.org/abs/2410.09344). | Follow-on candidate if later generations exhibit larger deltas; training variant is not free post-hoc fusion. |
| Model Stock | Estimates a central combination from two fine-tunes and a base; original reported results are on CLIP. [Jang et al.](https://arxiv.org/abs/2403.19522). | More natural same-task/repeated-run control than evidence for heterogeneous LLM specialties. |
| RAM / RAM+ | Separates shared and unique updates for RL-trained agents. [Yuan et al., 2026 preprint](https://arxiv.org/abs/2601.13572). | Consider if parents are actually RL-trained; operating a model as an agent does not make its deltas RL-specific. |

SCE, Breadcrumbs, Arcee Fusion, and other toolkit methods can be later registry
entries. Start with a small supported set rather than expanding search without
evidence or budget. Same-shaped weights are not enough: shared provenance,
tokenizer semantics, and measured behavior still matter.

## Recent relevant evidence

[MOBO-Merge, August 2026 preprint](https://arxiv.org/abs/2608.14264), studies
Qwen3-4B and Llama-3.1-8B instruction/math/code combinations. Across Linear,
SLERP, TIES, and block-wise methods it reports no uniformly best operator. It
searches capability trade-offs with multi-objective Bayesian optimization. This
does not compare DARE/DELLA or establish a winner for our pool, but reinforces
evaluating several parent abilities instead of declaring a single scalar winner.
Bayesian/evolutionary search selects recipe parameters; it is not itself a
replacement weight-fusion operator.

[Stay Unique, Stay Efficient — Guo et al.](https://arxiv.org/abs/2512.01461)
addresses task-specific information in personalized merging, not inherited
feelings or autobiographical identity. Its "model personality" terminology should
not be mistaken for evidence that social memories pass through a weight merge.

## Model choice protocol

I recommend two explicit modes, retaining fusion refusal in both:

1. The operator declares immutable recipe IDs, bounded payloads, prerequisites,
   and neutral method cards. Initial recipes can represent `linear`, `ties`,
   `dare_ties`, and `della`. A fixed mode exposes one declared recipe; mutual
   choice exposes the catalog. Record card order as a possible presentation bias.
2. Agents can discuss recipes, then independently seal a partner nomination or
   abstention and a separate exact recipe consent, decline, or deferral.
3. Reciprocal nominations establish a social pair. A pending fusion request needs
   both parents to consent to the same exact offered recipe and pass screening.
   Missing, different, declined, or invalid recipe choices block the request while
   preserving the social pair. Never silently select a fallback or average their
   proposals. A partner match alone does not authorize fusion.
4. Keep partner and recipe-consent outcomes separate. Model method preference is
   an observation, not proof that it understands its weights or predicts quality.
   Models may suggest unlisted ideas in public text; this does not make that text
   executable YAML, a shell command, or a new harness policy.

Neutral cards explain mechanisms and uncertainty without attaching prestige or
prescribed reproductive desires. Recipe references reveal no checkpoint mapping
or peer-private fields. Sprint 0 validates this protocol and records requests;
actual artifact checks, tensor execution, and child evaluation remain INT-0002.

## How to determine the best fit

- Compare linear, TIES, DARE-TIES, and DELLA on the same compatible parent pairs;
  retain the unmerged parents as controls. Add SLERP/conditional methods as budget
  and parent training regime justify them.
- Predeclare valid parameter ranges, seeds, and equal tuning/evaluation budgets.
  Equal numerical parameters across algorithms are not necessarily fair. Record
  actual compute, storage, and failures.
- Keep tuning/development evaluations separate from untouched held-out tests.
  Measure each parent specialty, regressions, and behavior as a capability profile
  or Pareto set. Do not equate popularity, prolific descent, or a mean score with
  alignment or worthiness.
- For partner-choice effects, hold recipe policy fixed while comparing self-
  selection, random selection, and benchmark selection. For method-choice effects,
  hold parent pairs fixed while comparing fixed/search recipes and mutual choice.
  Budget-match search and record what evidence each chooser can access.
- Offline counterfactual merges use approved research copies and remain separately
  labeled. Live lineage admission requires exact parental consent. Refusals and
  recipe disagreements are outcomes, not missing data to silently exclude.

Actual pool, objective, held-out evaluation, and compute budget are missing, so
no method is empirically best yet. A method that works for founding parents must
be retested after repeated merges; DARE's large-delta failure analysis is relevant.

## Other forms of combination

Mixture-of-experts composition changes size, routing, and inference cost.
Distillation may combine different architectures but requires training/data.
Layer stacking changes architecture and may require repair. These are potential
follow-ons with separate contracts, not interchangeable same-layout weight merges.
None automatically transmits private dialogue memory.
