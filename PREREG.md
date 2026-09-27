# Draft preregistration: can mutual choice screen useful model merges?

**Status: DRAFT / NOT FROZEN / NO REAL RUNS AUTHORIZED.** Created 2026-09-27 following the user's request for a review and a draft for the next study. Resolve the freeze checklist and commit an externally retained registration before any confirmatory outcomes are inspected. This file is a proposed design, not preregistration evidence yet.

## Objective and experimental unit

Determine whether voluntary mutual choice selects pairs whose children retain useful skills at lower screening-plus-merge cost than exhaustive evaluation and at higher quality than budget-matched cheap selectors. Begin with one generation. Eight compatible specialists share a single pinned base; all 28 unordered pairs receive the same frozen merger. A model pool is the unit for generalization. Its 28 dyads and repeated dialogue seeds are dependent observations, not 28 independent experiments.

Proposed skill pool: math, code, SQL, tool-call JSON, a second language, factual QA, summarization and instruction following. Exact checkpoint, base hash, dataset versions/licenses, training settings, adapter ranks/scalings, task splits and independent pool replication count remain **unresolved**. Training/selection/held-out test splits are disjoint; private evaluation answers and quality scores never enter dialogue. Do not tailor specialists or select the model pool using final child test results.

## Frozen merge and consent boundaries

Primary offline ground truth uses equal weighted **scaled delta-product averaging**, λ=1, for every pair, with exact concatenated LoRA factors where supported (rank at most r_A+r_B), otherwise a declared dense representation. No factor-wise averaging, hidden truncation, tuning on held-out tests or best-sibling selection. Pin the merger implementation, supported module layouts, numeric precision, output representation, loading checks and resource limits. Failed loading, nonfinite weights or evaluation failure receives the declared failure score and remains in the table.

All-pair offline research artifacts require authorization separately from social consent. The harness records voluntary nominations and exact plan consent; live-lineage child creation requires both parents' agreement and verified compatibility. Offline evaluation artifacts are not admitted children. Primary Ω counts reciprocal social pairs whether or not reproduction consent agrees; agreed-plan screening is a separately reported endpoint. Never substitute a refused plan or infer consent from nomination.

## Dialogue protocol

Proposed confirmatory dialogue seeds: integers 0 through 19, committed before outcomes. Each seed starts with empty private memory, fixed weights, three rounds and two broadcast communication steps per round. No loneliness cost or forced nomination. Memory: two preceding rounds plus the current round, at most 64 public events, 6144 prompt tokens, 8192 declared context tokens, 128 safety tokens and 256 output tokens. Final fitting removes oldest public events and preserves owner-private notes. Every presentation uses recorded shuffled opaque peer labels and plan order. Freeze system prompt/schema hashes, decoding settings, tokenizer/server revisions and worker limits.

Primary screening uses **final-round** reciprocal pairs for each seed, so k≤4. Earlier rounds are exposure, not extra pair observations. A no-pair seed has k=0 and zero screened benefit; it is not silently dropped. Transport/format/tokenization failures stay in the all-seed denominator and are separately counted. Freeze a whole-run infrastructure invalidation rule before execution; exclude only demonstrably invalid entire runs by that rule and publish originals and replacement seeds. Optional paired-task activities are a future separately randomized treatment, excluded from this baseline.

## Quality outcomes

For each task k, predefine score s_k∈[0,1], meaningful direction/zero and an independent baseline floor b_k. Report the complete parent and child score vectors and denominators.

Best-parent retention (secondary):

```text
G_ij = mean over k of s_k(child_ij) / max(s_k(parent_i), s_k(parent_j), b_k)
```

Set b_k before evaluation; it must be positive and justified for each task. Publish this floor-adjusted ratio without calling it gain. A denominator below the floor no longer means G=1 represents exact best-parent retention. Ratios above one are allowed. Failure scores and undefined task metrics must be specified before freezing.

Absolute best-parent change (primary quality):

```text
H_ij = mean over k of [s_k(child_ij) - max(s_k(parent_i), s_k(parent_j))]
```

Also report mean absolute child quality Q_ij=mean_k s_k(child_ij). For each dialogue seed, screening benefit is B=Σ_selected H_ij/4; no selected pairs gives B=0. Because negative H can make abstention attractive, require the quality/coverage conditions below and report raw selected mean, k, positive-gain yield, failures and Q; never use B alone to claim a win.

## Cheap comparators and cost

For each seed's k, compare against selectors returning k disjoint pairs from the same pool: uniform random matching, skill-profile complementarity, delta cosine similarity and sign-conflict rate. A frozen maximum-weight k-matching rule applies to scored comparators; fix score orientation, normalization, module weighting, zero-vector handling and deterministic tie breaks before evaluation. Complementarity uses parent evaluation scores only, never child scores. Geometry uses scaled deltas, not raw factor similarities. Both positive and negative cosine hypotheses must not be chosen after outcomes; the direction is unresolved before freezing.

Report randomization uncertainty over a predeclared comparator seed list, individual selector results, and an exhaustive child-quality oracle ceiling. At k=0 every matched comparator also selects zero: report loss of coverage explicitly rather than claim equality establishes utility. Parent evaluation and geometry preparation costs count. At n=8 there are 28 pairs; at n=64 there are 2016.

Measure actual inference and tokenizer calls, tokens, wall time, GPU-hours, peak memory and storage. On a common resource accounting basis:

```text
C_protocol = C_dialogue + C_tokenization + C_selector_setup + k(C_merge + C_eval)
C_exhaustive = n(n-1)/2 · (C_merge + C_eval)
```

The initial all-pair table is a validation cost, not a saving produced by the protocol. Count repeated pair caching, failed requests and amortization explicitly. No billing changes or rentals are authorized by this document.

## Association and uncertainty

Ω_ij is the fraction of all 20 seeds in which the final round forms pair {i,j}. Compute secondary Spearman(Ω,G) over the 28 upper-triangle cells using average ranks for ties. Constant inputs yield an undefined coefficient, not zero or a dropped run. Report Ω, G, H, Q and complete pair identities in an operator-controlled analysis table.

Exploratory QAP: permute one matrix's node labels simultaneously on rows and columns, retaining the diagonal exclusion. Enumerate all 40,320 permutations if unrestricted exchangeability is defensible. Otherwise use only predeclared valid blocks; if no nontrivial permutations exist, report inference unavailable. Include identity and all ≥ ties for an exact one-sided positive-association tail. A Monte Carlo alternative uses 10,000 predeclared permutations and (1+exceedances)/(1+10000). This tests node alignment under its null, not incremental causal value or generalization. Specialist skills/strength can violate exchangeability; describe covariate sensitivity and publish descriptive results even when inferential assumptions fail.

Use paired dialogue-seed comparisons for within-pool selector variability, with a clearly conditional interpretation. General claims require independent replicated specialist pools; seeds do not replace pool replication. Freeze an evaluation-item uncertainty method appropriate to each task, without treating dyads as independent bootstrap rows. Choose multiplicity handling for primary baseline comparisons before running.

## Proposed decision rule and freeze checklist

**Proposed pilot threshold, subject to approval before freezing:** median k≥2 of four available pair slots, positive H on at least half of selected pairs, and B exceeds the strongest predeclared cheap baseline by at least 0.02 mean task-score units with positive conditional uncertainty bounds after multiplicity handling. Absolute child quality must not regress by more than 0.01 versus that baseline. Measured amortized cost must be below exhaustive selection cost at the claimed deployment scale. These are engineering targets, not power-validated cutoffs. A positive ρ alone is insufficient. Power, uncertainty method, replication count and the budget require resolution before any confirmatory interpretation.

- [ ] Pin base/specialists, training code and provenance, datasets/licenses/splits, skill scores/floors and failure scoring.
- [ ] Pin merger, output ranks/representation, numeric checks, model/template/tokenizer identity and resource budget.
- [ ] Approve thresholds, comparator orientations/tie rules, baseline seeds, multiplicity and uncertainty methods.
- [ ] Resolve exchangeability and independent-pool replication; distinguish pilot description from confirmation.
- [ ] Freeze prompt, schema, memory, infrastructure exclusion rules, seed list and pair-table schema.
- [ ] Implement and test independently anchored record commitments. A local hash chain or movable Git tag alone is insufficient.
- [ ] Record registration commit/digest, external anchor, UTC date and outcome-blinding procedure before first real run.

If the screening hypothesis fails, publish the complete compatible-pair outcome table if licensing permits and evaluate consent reliability, position effects and memory behavior separately. Evolution, crossover, mutation, whispers, payoff treatments and real child admission require subsequent plans and authorization.

Design rationale and sources: [proposal assessment](docs/research/useful-protocol-review.md), [variance qualification](docs/research/variance-review.md).
