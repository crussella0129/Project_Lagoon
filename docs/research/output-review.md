# Build, output and coverage review

2026-09-27. Scope selected by the user: repair the build/output issues and revise
the draft criteria; defer training. Validate locally without waiting for GitHub CI.

## Instrument findings

The locked build at main's `4bbb561ebc7436b795c015f868196d48e637e908` failed:
sha2 0.11's digest output no longer implemented the formatting trait used by the
old fingerprint helper. Encoding each digest byte with padded lowercase hex fixes
the build without changing the SHA-256 payload or hash representation. A known
answer for the JSON bytes `null` and the existing hard-coded alias tests verify
compatibility. The build and 38 original tests passed after this repair, before
adding output changes. Earlier v0.2 validation was evidence for its tested commit,
not evidence that later dependency updates on main compiled.

v0.3/schema 3 bounds public messages at 128 Unicode characters and each private
note at 64. Every response-schema string has a finite maximum; offered peer/plan
enums remain exact. The parser checks public/private limits and configuration
checks initial notes. Byte guards remain independent. Example budgets now reserve
4096 output tokens and 3968 prompt tokens plus 128 safety tokens within 8192.

The review correctly identified a confound: ignored `finish_reason=length` made
token exhaustion resemble model formatting failure. Length termination now yields
`generation_limit`, even with valid JSON, retaining partial content/reason only in
the operator record. No ballot, public message or memory update is applied. Null
content on length termination is an empty diagnostic receipt. Offline replay
derives the same status and rejects inconsistent termination/outcome records.
Missing reasons remain visible as unreported rather than presumed normal stops.

String bounds do not guarantee output completion. Unicode escapes change byte
length; flexible whitespace and reasoning output can consume additional tokens.
The [XGrammar compiler API](https://xgrammar.mlc.ai/docs/latest/api/python/grammar_compiler.html)
exposes whitespace controls, including flexible whitespace by default. The
[vLLM structured-output documentation](https://docs.vllm.ai/en/latest/features/structured_outputs/)
describes separate reasoning-parser considerations. These deployment settings
must be pinned and tested; the client cannot establish server conformance from
HTTP mocks. No real model/server was run for this repair.

## Coverage calculation

Under independent uniform non-self nominations, with no abstentions or failures,
let K be reciprocal pair count. For r disjoint pairs:

```text
a_r = E[binomial(K,r)] = n! / [(n-2r)! · 2^r · r! · (n-1)^(2r)]
P(K=k) = sum_{r=k..floor(n/2)} (-1)^(r-k) · binomial(r,k) · a_r
```

For retirement between rounds, propagate the remaining-population distribution
with transition n -> n-2K using that round's exact probabilities. Retirement is
per seed; each participant can belong to only one cumulative pair. The bounded
[Rust reference](../../examples/pairing-baseline.rs) checks the formula against
complete nomination enumeration for n=2..5 and checks normalization/retirement.

| Eight-agent reference | Expected pairs | Probability of at least two |
| --- | ---: | ---: |
| One final round | 0.571429 | 0.080378 |
| Three fresh rounds with matched exit | 1.798909 | 0.569853 |

Reproduce with `cargo run --locked --example pairing-baseline`. This verifies the
review's rounded uniform-reference numbers. Its popularity-model percentages
cannot be reproduced without defining the preference generator, concentration,
ties and round dependence. Neither reference predicts real dialogue behavior.

The median-two threshold is poorly calibrated to the current final-round
baseline. Matched exit is a plausible alternative for obtaining cumulative
disjoint pairs, but changes exposure, denominators, cost and Ω. It remains a draft
treatment, not an implemented live behavior. The [revised preregistration](../../PREREG.md)
requires a primary protocol choice and tested implementation before confirmation.

## Opportunity and registration

A selector cannot beat the true maximum-H matching oracle under identical
eligibility, k, quality constraints and benefit aggregation. A pilot whose
oracle-minus-cheap-baseline gap is smaller than the target margin offers no
observed opportunity for that target; evaluation uncertainty must still be shown.
Publish headroom by k, including zero-pair seeds in workload aggregation. Redesign
and calibrate on the exploratory pilot; confirm on separately trained independent
pool replicates with frozen decisions and disjoint evaluation data. Dialogue
seeds within one pool do not create pool replication.

OSF remains a suitable proposed registration service: its official guidance
describes frozen documents and an embargo up to four years. Create an actual
registration and retain identifiers/digests, rather than uploading only an editable
project file ([registration guide](https://help.osf.io/article/330-welcome-to-registrations)).
Its announced project transition preserves registrations; use the direct workflow
([transition guidance](https://help.osf.io/article/768-osf-projects-transition-registration-questions-and-use-cases)).
OSF states that accounts are free ([account guide](https://help.osf.io/article/390-profile-and-account)).

OpenTimestamps is an optional digest anchor. The official client documents pending
calendar receipts, proof upgrading and Bitcoin verification; calendars receive
opaque digests rather than file contents. Preserve the file and proof, and verify
the completed attestation. It proves an existence commitment, not truth,
authorship or that outcomes were unknown
([client workflow](https://github.com/opentimestamps/opentimestamps-client)).
No external registration or timestamp was submitted. Training, real fusion and
confirmation remain deferred.

## Local validation

Final locked build, all-target tests, format, clippy and Book checks are recorded
in [the maintenance work record](../work/output-review.md). Hosted CI is not an
acceptance gate for this repair, per the user's explicit request. Historical
Sprint 0 and v0.2 evidence remains at its original paths and revisions.
