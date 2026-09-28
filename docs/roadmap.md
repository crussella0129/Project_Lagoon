# Project_Lagoon implementation roadmap

This tracks the authorized improvement plan. It is an implementation sequence,
not a claim that its proposed science has been validated. The v0.4 harness
is described in the [README](../README.md). Sprint 1 addresses Phase 0; later work
stays gated. The user superseded minimum-version search with **current stable Rust**.

| Step | Deliverable | Status / owner intent |
|---|---|---|
| 0.1 | Rolling stable support, local/CI checks | Implemented; [INT-0007](intents/INT-0007-supported-builds.md) |
| 0.2 | Correct remote/name and close T-008 | Already completed in prior maintenance |
| 0.3 | Required main checks, weekly grouped updates | Ruleset active; configuration in this sprint |
| 0.4 | Verbatim origin with clinical README | [ORIGIN.md](../ORIGIN.md) |
| 0.5 | Citation metadata and Zenodo integration | Metadata prepared; integration needs authenticated access |
| 0.6 | Welfare/ethics policy | [ETHICS.md](../ETHICS.md); runtime controls still to implement |
| 0.7 | Separate draft preregistrations | [Study A](../prereg/study-a-social.md), [Study B](../prereg/study-b-screening.md) |
| 1.1 | Versioned per-call seeds and stochastic/greedy arms | Implemented; [INT-0005](intents/INT-0005-observability-and-analysis.md) |
| 1.2 | Configurable phase string bounds and schema hashes | Implemented; INT-0005 |
| 1.3 | Bounded private peer/trust ledger and privacy tests | Implemented; INT-0005 |
| 1.4 | Pinned vLLM/llama.cpp conformance, receipts, measured output reserve | Pending G0; required for G1 |
| 1.5 | Matched exit, eligibility/enums/singletons/replay | Implemented; INT-0005 |
| 1.6 | Choice/message/event/outcome/profile export and Python analysis | Pending G0; INT-0005 |
| 2.1 | Optional typed profile states, history and write-once appearance | Pending G0; [INT-0004](intents/INT-0004-social-layer.md) |
| 2.2 | Four deterministic event types, random teams and fixtures | Pending G0; INT-0004 |
| 2.3 | Structured promises and optional delayed reveal | Pending G0; INT-0004 |
| 2.4 | Optional persistent pair bonds and reaffirmation | Pending G0; INT-0004 |
| 2.5 | Bounded recipient/operator-only whispers | Pending G0; INT-0004 |
| 2.6 | Appearance-first versus delayed appearance | Pending G0; INT-0004 |
| 2.7 | Voluntary leave and archive without penalty | Pending G0; INT-0004 |
| 2.8 | Behavioral wording and versioned disclaimer measurement | Pending G0; INT-0004 |
| 2.9 | Factual post-reveal manipulation checks | Pending G0; INT-0004 |
| 3.1 | Study A draft covering four arms and joint hypotheses | Draft prepared, not frozen/registered |
| 3.2 | Exploratory pilot with coverage/failure/disclaimer receipts | Requires G1 and completed instrument/social layer |
| 3.3 | Planted-parameter recovery through runner/export/analysis | Implement before any real Study A data; INT-0005 |
| 3.4 | Frozen manifest, OSF registration, verified timestamp, confirmation | Requires G2; [INT-0006](intents/INT-0006-publication-and-ethics.md) |
| 3.5 | Paper 1 and approved data/export DOI | Requires completed study, rights/privacy review and release approval |
| 4.1 | Pinned base plus eight LoRA specialists and disjoint datasets | Future [INT-0002](intents/INT-0002-evaluated-dare-descendants.md); training deferred |
| 4.2 | External scaled-delta fusion, exact rank-bounded representation | T-009; never average adapter factors |
| 4.3 | Deterministic H/G/Q, cheap comparators, oracle headroom by k | Required for G3 |
| 4.4 | Frozen dialogue and independent trained-pool confirmation | Requires G3 and Study B registration |
| 5.1 | Islands of six to eight and recorded migration | Future [INT-0003](intents/INT-0003-controlled-evolutionary-study.md) |
| 5.2 | Fixed population, age-based retirement and consensual reproduction | T-010; no culling of unchosen participants |
| 5.3 | Crossover/DARE comparators, menu-based curriculum mutation, cultural note | T-010; exact consent and budget per child |
| 5.4 | Pedigree, kinship and realized weight similarity | T-010; pedigree visibility is a treatment |
| 5.5 | Permitted weights/notes archive and optional exit interview | T-010; licensing and continuity qualifications |
| 5.6 | Choice/chat, choice/events, random and evaluation-selected arms | T-010; consent applies to every arm |
| 5.7 | Price decomposition; capability and parameter variance separately | T-010; toy DARE retention is a hypothesis, not a result |
| 5.8 | Operator-only constructor, no egress, small models, human merge gate | Required for G4 |
| Optional | Post-hoc documentary using permitted public events | After Study A; no show feedback into the experiment |

## Gates and remaining dependencies

| Gate | Required evidence | Current state |
|---|---|---|
| G0 | Phase 0 code/docs, current-stable checks and protected main | Passed on 46fb66f; [hosted run](https://github.com/crussella0129/Project_Lagoon/actions/runs/36374666278) |
| G1 | Committed conformance receipts from pinned real servers | Open; no real inference study |
| G2 | Pilot criteria justified/frozen and parameter recovery verified | Open; no confirmation |
| G3 | Study B oracle headroom meets frozen margin | Open; no Study B dialogue |
| G4 | Ethics, verified containment and human fusion gate operational | Open; no generational runs |

Follow the critical path: Phase 0 → instrument → social layer → pilot → registration
→ Study A → paper. Preparation of Study B remains separate; no rented compute,
training run or external publication has been started. The existing remote profile
requires human approval to merge sprint PRs. No gate is satisfied by a document that
merely describes the intended control.

The user explicitly deferred Zenodo activation to continue implementation. It is
now a release prerequisite (T-017), not a G0 prerequisite. No integration or DOI
has been claimed. This amendment does not relax conformance, study-registration,
containment or human merge/release gates.

## Guardrails

No loneliness or disclaimer penalty; no narration/audience feedback; no participant
tools or weight access; no LLM scoring of skill; no culling based on popularity; no
pilot/confirmation pooling. See [ETHICS.md](../ETHICS.md) for operative treatment and
publication rules. Historical speculation is retained separately in ORIGIN.md.
