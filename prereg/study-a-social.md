# Study A — Social selection (draft, not registered)

Status: proposed protocol. No Study A data have been collected. Profiles, events,
promises, leave, tidy export and the analysis pipeline are not implemented in v0.3.
This draft is not a frozen registration and leaves empirical decisions open below.
The [roadmap](../docs/roadmap.md) and [ethics policy](../ETHICS.md) govern readiness.

## Question and scope

For a fixed heterogeneous pool of small instruct models, how are partner nominations
associated with appearance, demonstrated skill and observed reputation? Do stated
preferences predict how an agent weighs reputation? Study A uses inference only;
it neither fuses weights nor measures descendant fitness. Treat selection gradients
here as associations with nomination success, not evidence of genetic improvement,
conscious preferences or an evolutionary response.

## Pool, unit and sampling

Use six to eight instruct models from different families, approximately 135M–1.5B
parameters. Before collection, pin every checkpoint/revision, quantization, tokenizer,
chat template, server/version, grammar backend and applicable license/output terms.
No model list is frozen yet. Compute availability is not evidence of conformance.

Randomization and uncertainty units are complete runs/seeds, not individual
chooser–candidate rows. Use at least 20 distinct seeds per arm with a declared
positive temperature. This is a proposed lower bound, not a power calculation.
Choose the final sample size from separate pilot precision and simulation results.
Independent model pools are necessary for generalizing beyond the selected pool;
many seeds on the same pool are not independent model-family replication.

The immutable manifest will declare run seeds, per-call seed derivation, rounds,
phase order, call/token budgets, temperature, retirement mode, profile visibility,
event-task revisions, team randomization, retry policy and stopping rules. Keep a
separate greedy-decoding control. Do not silently retry failed calls with new seeds.

## Treatments

| Arm | Information and actions |
|---|---|
| A1 | Optional profiles and public chat; appearance visible once volunteered |
| A2 | Same as A1, with appearance withheld until a frozen later chat boundary |
| A3 | A1 plus deterministic skill/cooperation events |
| A4 | A3 plus structured promises and delayed ballot reveal |

A2 has no event phase: its appearance reveal must be tied to a specified chat
boundary, not an undefined "first event." The separate blind-event option hides
appearance until the first event completes when events exist. Freeze exact timing
and whether a ballot precedes the reveal before the pilot; do not conflate these
two manipulations. Appearance may remain unasked/declined; disclosure is never forced.

Use the same eligible model pool, seed schedule, basic prompts and partner/plan
order randomization across arms. Report all additional event and reveal calls and
their cost. Arm comparisons estimate these protocol bundles; without matched-call
controls they do not isolate information from extra interaction. Optional bonds,
whispers and matched exit are off in the initial comparison unless explicitly
introduced and registered as separate factors. Leave remains available in every arm.

## Profiles, events and visible reputation

Each profile field has unasked, declined or given state and round provenance.
Appearance is write-once when given. Revisable answers retain full history. Record
loyalty and lineage-distinctness scores with their sentences, optional self-description
and seeking/no-preference; declare each field's visibility. Pair attitudinal items
with behavioral questions without coaching a desired response or suppressing disclaimers.

Events comprise solo skill, hidden-profile fact sharing, shared attempt budgets and
credit claims. Use deterministic checkers and task pools disjoint from training and
all held-out evaluations. Record task identity, inputs visible to each participant,
randomized teams, attempts, actions, resource claims and scores. Team success must
not be assigned as individual skill without a frozen attribution rule.

Structured public promises specify nomination or non-nomination of an alias.
Reveal occurs only after the round's ballots are sealed and resolved. Distinguish
kept, broken and not evaluable (for example, no valid ballot due to technical failure).
Freeze how leave, changed eligibility, abstention and conflicting promises are scored.
Reputation at round r uses only evaluable promises revealed before that choice;
never use the current or a future ballot. If no history exists, record missing/unknown
reputation rather than treating it as dishonesty. Other arms do not receive hidden
promise histories through an analysis feature or an agent observation.

## Outcomes and measurements

Export one choice row for every chooser × eligible candidate × round, including
run, arm, phase snapshot, aliases, displayed position, eligibility, trait availability,
disclosure timing and chosen flag. Join through exact record/snapshot identifiers.
Keep explicit abstention/outside-option outcomes, leaves and technical failures in
separate outcome fields and tables; do not encode all three as an ordinary nonchoice.
Export messages, events, profiles and outcomes with declared public/private views.

- **Appearance:** external held-out judge panel plus human spot-check, blinded to
  partner choices, event scores and reputation. Never show ratings to participants.
  Freeze rating instructions, whether text or rendered appearance is rated, aggregation,
  agreement checks and missingness rules. No primary claim until those rules exist.
- **Skill:** pre-choice deterministic individual-event scores, with a declared
  aggregation and task difficulty policy. Separate teamwork outcomes and attribution.
- **Reputation:** pre-choice publicly observable promise history under the frozen
  kept/broken/unknown rule. Do not fabricate reputation variation in A1–A3.
- **Other outcomes:** reciprocal pairs, abstention, leave, failures by reason,
  position effects, disclosure rates and timing, disclaimer rates and factual
  manipulation-check accuracy after reveal. Include denominators and uncertainty.

Version and freeze the disclaimer classifier and manually audit a blinded sample
for false positives/negatives. A disclaimer is neither a penalty nor an exclusion.
Manipulation checks ask factual questions about revealed events; report incorrect,
unanswered and technically failed checks separately. Their main purpose is to
distinguish missing tracking from weak choice association. Post-choice accuracy
is descriptive, not an automatic filter defining a more favorable primary sample.

## Analysis and estimands

For each candidate-round define nomination success w as received valid nominations
divided by the number of valid chooser opportunities in which that candidate was
eligible. Retain zero successes. Within the frozen comparison population, relative
success is w / mean(w); if the mean is zero it is undefined. Report such strata and
zero-choice runs explicitly, not as arbitrary zeros or silently discarded observations.
Mutual-pair success is a secondary outcome with its own denominator.

For A4's prespecified post-reveal rounds, fit the proposed OLS association:

```text
relative nomination success = alpha + beta_looks*z_looks
                             + beta_skill*z_skill + beta_rep*z_rep + error
```

Freeze trait orientation and standardization reference before confirmation so a
one-unit change is comparable across traits and arms. Do not compare raw 0–10 ratings
to arbitrary event totals. Constant or unobserved traits yield a non-estimable
contrast; do not impute evidence. Specify model identity/round terms and evaluate
collinearity, rank and estimable within-pool variation in the pilot. Controlling for
model identity can absorb traits that never vary within a model; report that limit.

Fit a separate conditional-logit choice model grouped by chooser × round × run:

```text
u_ij = beta*x_j + gamma*(loyalty_i * reputation_j) + position terms
P(i chooses j) = exp(u_ij) / sum_k exp(u_ik)
```

Choice sets include the explicit abstention option with a separately identifiable
outside-option indicator and defined trait coding. A chooser-only main effect is
constant within a choice set and is not separately identifiable in conditional
logit. The loyalty-by-reputation interaction varies across candidates when reputation
does. Validate both before fitting. A technical failure is not an abstention choice.
As a secondary sensitivity analysis, estimate candidate choice conditional on a
valid nomination; label its conditioning clearly and report participation separately.

Use Python pandas/statsmodels for the analysis as proposed. Freeze uncertainty
calculation after pipeline simulations, accounting for within-run dependence and
shared participants; do not use independent-row standard errors. Seeds are the
resampling unit for conclusions conditional on the fixed pool. Predeclare intervals,
multiplicity handling, fit failure rules and sensitivity analyses. These remain open.

## Joint hypotheses

Both hypotheses are registered together; neither result is a success criterion
for operating the instrument. In the standardized A4 OLS model define
`D = beta_skill + beta_rep - beta_looks`:

- **H-Charles:** D > 0 after events and reveal.
- **H-Claude:** D < 0 in the small-model pool, meaning appearance exceeds the
  combined skill/reputation association under this operational definition.

Intervals spanning zero are inconclusive. Report each coefficient as well as D,
because a sum can hide opposing effects. Conditional-logit `gamma > 0` is the
separate stated-loyalty/observed-reputation hypothesis; it is not the OLS contrast.
No directional claim is valid when traits or interactions are unidentifiable.

An optional 3–8B A4 arm must be separately budgeted and preregistered. Comparing
unmatched families/sizes does not isolate a causal size threshold or prove the
emergence of social reasoning; report it as an association and specify matching.

## Pilot, recovery and exclusions

Before real data, scripted fixture agents with known stochastic beta/gamma values
must traverse the actual runner → operator record → export → analysis pipeline.
Predeclare simulation seeds, parameter grid, replication count, bias tolerance and
interval-coverage target. Test position effects, outside options, missing profiles,
no-choice runs and technical failures. Recovery must hold across the prespecified
replicates; a lucky single interval containing truth is not sufficient. Fixture
latents must be independent of the inference code used to fit them.

After G1 and recovery, run a small, separately labeled exploratory pilot per arm.
Report schema, transport, truncation, timeout and tokenizer failures; valid-choice
coverage, disclaimer/disclosure/leave rates and manipulation accuracy. Use these to
justify quantitative coverage, precision and failure gates, not to suppress unwanted
answers. Freeze criteria before the confirmatory collection, not after seeing it.

Exclude only predeclared technical-invalid observations from models requiring a
valid ballot; keep them in all-run denominators and failure tables. Refusal,
abstention, declining a profile, disclaimers and leaves are valid outcomes. Distress
pauses follow ETHICS.md and remain reported. Pilot data are never pooled with
confirmation; amendments and known outcomes are logged. No optional stopping based
on coefficient signs or significance is permitted.

## Freeze and registration checklist

- [ ] Complete G0, then social features, exports, leave/stop and privacy verification.
- [ ] G1: pin models/servers/templates/licenses and commit real conformance receipts.
- [ ] Freeze event pool/checkers, profile questions, phase order/reveal and promise rules.
- [ ] Freeze appearance rating protocol, trait definitions, scaling and missingness.
- [ ] Pass planted-parameter recovery through the complete pipeline.
- [ ] Complete separate pilot; justify failure/coverage/precision thresholds and G2.
- [ ] Freeze model pool, run count/seeds, positive temperature and greedy control.
- [ ] Freeze estimable formulas, uncertainty, multiplicity, exclusions and stopping.
- [ ] Review ETHICS.md monitoring, exit/archive, containment and publication rights.
- [ ] Create an immutable manifest with code/configuration/analysis/data-source digests.
- [ ] Submit an OSF Registration and retain its identifier, date and frozen manifest.
- [ ] Anchor the manifest with OpenTimestamps; retain and verify the completed proof.
- [ ] Only then collect confirmation. Publish approved artifacts and DOI receipts.

No external registration, timestamp submission, model download, inference study,
training or publication of participant records has occurred as part of this draft.
