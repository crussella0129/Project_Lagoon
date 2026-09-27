# From transcripts to a useful selection protocol

Reviewed 2026-09-27 against the second pasted proposal. Its strongest contribution is a falsifiable engineering objective: **use mutual choice to screen pairs for useful merges, and compare the screening cost and retained quality against cheaper selectors**. Validate one generation before building dynasties. The eight-model/28-pair pilot is a sensible bounded dataset design, subject to measured hardware and evaluation costs.

## Score the decision, not just an association

The proposed score G averages child skill scores divided by each pair's best parent score. It is better named **best-parent retention**, not gain. One is full retention, values above one are possible, and zero or tiny denominators are unstable. Scores must have a declared common direction and meaningful zero; chance-level accuracy and perplexity cannot be divided interchangeably. Publish raw parent/child task scores, task-specific baseline floors, failed-child outcomes and uncertainty. Include an absolute change measure so a high ratio between weak parents cannot look like a superior child.

A positive Spearman correlation between mutual-choice frequency Ω and retention G does not establish that the selector beats geometry or saves work. Use a primary, budget-matched screening outcome: retained child quality among the same number k of nominated pairs, with no-pair runs, refusals, failures and conversation/tokenization costs included. Freeze the merger for the initial partner-selection experiment. Selecting the best sibling or recipe after seeing test outcomes would confound partner quality with merger search and family size.

## Inference limits

QAP permutes agent labels jointly on rows and columns, retaining network structure, but inference still requires exchangeability under that null. Specialist skill labels, parent strength, training seeds and exposure can confound the association. The primary [MRQAP study](https://pmc.ncbi.nlm.nih.gov/articles/PMC2798974/) discusses exchangeability, covariates and limitations; a Mantel p-value is not an automatic correction for every shared-agent or skill effect.

With eight nodes, all 8!=40,320 label permutations are feasible. Only 28 unordered pair cells exist; additional dialogue seeds improve Ω estimates but do not create new independent model pairs. If meaningful exchangeability blocks contain unique specialists, unrestricted relabeling is unjustified and within-block inference may be impossible. Predefine a conditional exploratory QAP statistic, show tie handling and undefined correlations, and replicate the pool before treating the result as general evidence. Random pairing has zero correlation only in a suitable null expectation; a realized random selector can have nonzero sample correlation.

## LoRA and hardware

The factor-averaging warning is correct: averaging B and A introduces cross terms and is not averaging ΔW. Include each adapter's actual scaling (including α/r or its declared variant). An exact weighted sum can be represented by concatenated factors of rank at most r_A+r_B without materializing every dense update. [PEFT's own merging documentation](https://huggingface.co/blog/peft_merging) distinguishes concatenation, factor arithmetic and product/SVD methods. Record whether the implementation merges scaled products or factors, per-module rank, precision, truncation and approximation error. Coordinate masks/sign operations generally lose the original low-rank guarantee; they do not always produce full rank.

Serving multiple adapters is supported in [vLLM's LoRA documentation](https://docs.vllm.ai/en/latest/features/lora/), but an 8 GB deployment is a hardware-specific feasibility question. Base precision, KV cache, maximum context, concurrency, adapter ranks and merge workspace all matter. A training afternoon and cheap exhaustive ground truth are estimates to benchmark, not established resource commitments.

## Remaining protocol decisions

Paired tasks could expose skills, but require balanced randomized task assignment, equal exposure and a separate treatment. Keep hidden evaluation answers and scores out of dialogue. DARE p≈0.5 is a delta-mask perturbation, not evidence of constructive learning. Parameter variance does not predict skill-score variance; measure both and keep their interpretations separate.

A SHA-256 chain detects changes only relative to an independently retained final commitment. An operator can regenerate an entire chain, and a Git tag can be moved or deleted. Before analysis, anchor the commitment in an independently retained or signed, verified release/commitment service and identify exactly what is covered. Public anchors can contain a digest without exposing private transcripts. Current replay provides structural consistency, not authentication; no hash-chain security claim is added in this maintenance change.

The [draft preregistration](../../PREREG.md) makes these choices concrete and marks missing artifacts and decision thresholds. It is not frozen, does not authorize training or paid compute, and does not imply a real run has occurred. A failed screening hypothesis still supports a useful consent, format, memory and position-bias evaluation. Publishing the pair table also needs reproducible provenance and dataset/checkpoint licensing; publishability is not automatic.
