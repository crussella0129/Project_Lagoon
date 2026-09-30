# Project_Lagoon

A Rust research harness for voluntary partner selection and explicit model-fusion
agreements among language-model participants.

The research program separates three questions: which observable traits predict
partner choice, whether mutual choice identifies useful model-fusion pairs, and
how selection and variation affect capability diversity across generations.

## Protocol

Participants receive public communication and their own persistent private notes.
Procedural instructions define available actions and consequences without assigning
personalities, social roles, preferred partners, or a requirement to reproduce.

Each participant can nominate one peer or abstain. Nominations remain sealed until
all attempts finish. Reciprocal nominations form an exclusive pair; abstention,
failed requests, and unsuccessful matching preserve the participant and its memory.
The baseline imposes no penalty for abstention.
Optional matched-exit mode retires reciprocal partners from later rounds while
preserving their state; this does not imply agreement to fusion.

Partner selection and reproduction agreement are separate decisions. Participants
may agree to an offered finite plan, decline, or defer. Each plan specifies child
recipes, seeds, and resource limits. Optional sibling batches can use different
methods, with every child derived from the same original parents and base. The
harness binds a selected plan ID to its immutable fingerprint; participants do not
need to reproduce the hash.

## Current implementation

The v0.4 CLI provides:

- Deterministic fixtures and explicitly configured local HTTP inference.
- Phase-specific JSON schemas and recorded, shuffled opaque peer and plan
  presentation.
- Bounded public-memory windows, persistent owner-private notes, and server token
  accounting before generation.
- Character-bounded response schemas and recorded finish reasons, with token-limit
  termination counted separately from invalid responses.
- Frozen phase snapshots, bounded concurrent calls, sealed nominations, and
  distinct abstention and technical-failure outcomes.
- Operator records, separate public transcripts, descriptive reports, and offline
  replay.
- Recorded per-call sampling seeds, configurable phase response bounds and an
  optional bounded owner-private peer ledger.
- Operator-only server provenance, runtime schema export and a pinned conformance
  kit; actual server verification remains required before study runs.
- Replay-checked research CSV tables and a Python planted-parameter recovery
  pipeline for conditional logit and relative-nomination OLS.
- Pending or blocked fusion manifests with exact plan receipts and declared
  compatibility checks.

The harness does not train models, execute weight fusion, evaluate descendants, or
admit children. Declared compatibility metadata is not verification of the actual
weights. Private fields such as `feeling`, `learned_preference`, and `thought` are
recorded textual reports; they do not establish subjective experience. Persistent
notes alter context, not model weights.

## Quick start

Requires current stable Rust. The repository and CI follow the `stable` channel;
no older compiler minimum is supported. Dependabot groups Cargo and GitHub Actions
updates weekly, with changes tested against current stable.
From the repository, update your toolchain and use new output directories:

```powershell
rustup update stable
cargo run --locked -- run --config examples/fixture-experiment.json --output runs/demo
cargo run --locked -- replay --record runs/demo/operator-record.json --output runs/demo-replay
cargo run --locked -- report --record runs/demo/operator-record.json
cargo run --locked -- export --record runs/demo/operator-record.json --output runs/demo-tables
```

The fixture uses synthetic participants and produces pending requests, not merged
models. See [usage and local inference](docs/usage.md) for configuration, privacy,
tokenizer requirements, and record-version compatibility.
The [conformance kit](conformance/README.md) and [analysis guide](analysis/README.md)
describe server evidence requirements and research-table use.

## Research direction

Study A is an inference-only comparison of profiles, communication, skill events
and revealed promise histories. Study B compares mutual-choice merge screening
against random pairing, skill-profile complementarity and weight-geometry selectors
under matched budgets. Study C examines generational change after those instruments
and evaluation gates are established.

DARE is a candidate transformation, not a prescribed optimum. The included toy
variance study examines averaging, DARE followed by averaging, and coordinate or
block crossover. Parameter variance is distinct from behavioral diversity; these
results do not establish the quality of real merged models.

The [Study A](prereg/study-a-social.md) and [Study B](prereg/study-b-screening.md)
preregistrations remain drafts. Study A's social features are not implemented yet.
No real social-selection, specialist-training, fusion or generational study has
been performed by this implementation. The [roadmap](docs/roadmap.md) tracks the
required implementation and evidence gates.

See [design values and origin](ORIGIN.md), the [ethics policy](ETHICS.md) and
[citation metadata](CITATION.cff). Archival integration status and release
requirements are tracked in the [publication guide](docs/publication.md).

- [Proposal assessment](docs/research/useful-protocol-review.md)
- [Variance analysis and reproducible study](docs/research/variance-review.md)
- [Fusion-method assessment](docs/sprints/s0/sprint-research/fusion-method-review.md)
- [Implementation and verification](docs/research/instrument-validation.md)
- [Build, output and study-criteria follow-up](docs/research/output-review.md)
- [Project Book](docs/README.md)
