# Blending, DARE and finite-population variance

Reviewed 2026-09-27. The pasted formula is correct for **DARE sparsification followed by equal averaging of two deltas**, with independent masks, independent zero-mean parents of equal variance, and no additional scaling. It is not a formula for every DARE-based merger or for neural-network skill scores.

Write q = 1-p and c = (m_A x_A + m_B x_B)/(2q). Since E[m]=q and E[m²]=q, independent zero-mean parents of variance V give Var(c)=V/(2q). Thus R=1/[2(1-p)]: 0.5 for averaging, 1 for p=0.5, and 5 for p=0.9. Six applications to fresh independent parent distributions give 0.015625, 1 and 15625 respectively. The original review's exact sample numbers cannot be reproduced without its seed, implementation and sampling design.

For equal parent means µ and correlation ρ, the one-generation ratio becomes

```text
R = 1/(2q) + ρ/2 + p µ²/(2qV)
```

A global multiplier λ adds a factor λ². Even p=0.5 does not universally preserve variance once means, covariance, weights or rescaling change. [DARE's paper](https://arxiv.org/abs/2311.03099) separates delta sparsification/rescaling from the downstream merger. [TIES](https://arxiv.org/abs/2306.01708) adds trimming, sign selection and disjoint merging; those nonlinear operations fall outside the calculation above. The runnable catalog's `dare_ties` must not be equated with this toy `dare_average` treatment.

## Closed populations and drift

The Rust study maintains N models, chooses two distinct parents uniformly for each child, and independently produces N children. It tracks mean coordinate variance across the population using divisor N, not variance within a single model's parameter vector. Let V be that variance and M be the mean squared population mean across coordinates. Exact conditional expectations are:

```text
A = 1/(2q) - 1/[2(N-1)]
B = p/(2q)
E[V_next | pool] = (N-1)/N · (A V + B M)
E[M_next | pool] = M + (A V + B M)/N
```

Whole-coordinate or whole-block crossover has conditional child variance V, giving E[V_next]=(N-1)V/N and E[M_next]=M+V/N. It preserves a parent's marginal value distribution but finite resampling still removes diversity. With N=64, linear averaging retains (31/64)^6 ≈ 0.012915 after six generations; neutral crossover retains (63/64)^6 ≈ 0.909837 in expectation. Crossover is not a guarantee of functional compatibility, complementary skills, stable lineages or diversity preservation.

## Reproducible study

```powershell
cargo run --locked --example variance-study -- --seed 9 --population 64 --coordinates 4096 --generations 6 --replicates 8
cargo test --locked --example variance-study
```

[Recorded results](variance-results.json) include all settings, five treatments, generation-by-generation empirical variance, mean squared population means, second moments, and the conditional finite-population expectations initialized from the measured starting moments. Replicates share their initial pool across treatments; each method uses a separate reproducible ChaCha8 stream. Settings reject work above 150 million coordinate operations. The study has no trained network, selection fitness, mutation, admission policy or TIES implementation.

Generation six for the command above:

| Treatment | Observed V/V₀ | Finite-population expectation |
| --- | ---: | ---: |
| Equal averaging | 0.012946 | 0.012915 |
| DARE averaging, p=0.5 | 1.018508 | 1.023632 |
| DARE averaging, p=0.9 | 15777.705930 | 15596.042466 |
| Coordinate crossover | 0.909002 | 0.909837 |
| Block-64 crossover | 0.911225 | 0.909837 |

Analytical tests enumerate every parent-sign and mask case at correlations -1, 0 and 1, check finite-population drift, and check bounded reproducibility. Empirical values are sample outcomes, not validation of real model performance. High-drop recursive DARE deserves evaluation, but this calculation cannot establish that actual recursive DARE-TIES fails or that specialist dynasties are impossible. Task training and structured variation can change the outcome.

Recommendation: keep exact singleton/sibling consent, compare fixed operators and controlled method choices, and investigate coordinate/block/layer crossover as a separate recipe family before generational claims. Measure held-out skills and interference independently of parameter variance. Do not declare p=0.5 the best merger or a substitute for a tested mutation/learning mechanism.
