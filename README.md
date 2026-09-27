# Lover's Lagoon

A Rust research harness for voluntary partner selection and explicit model-fusion
agreements among language-model participants.

The central research question is whether interaction-based mutual selection can
identify model pairs whose merged descendants retain useful capabilities. A later
objective is to examine how partner selection, fusion methods, and sources of
variation affect capability diversity and lineage structure across generations.

## Protocol

Participants receive public communication and their own persistent private notes.
Procedural instructions define available actions and consequences without assigning
personalities, social roles, preferred partners, or a requirement to reproduce.

Each participant can nominate one peer or abstain. Nominations remain sealed until
all attempts finish. Reciprocal nominations form an exclusive pair; abstention,
failed requests, and unsuccessful matching preserve the participant and its memory.
The baseline imposes no penalty for abstention.

Partner selection and reproduction agreement are separate decisions. Participants
may agree to an offered finite plan, decline, or defer. Each plan specifies child
recipes, seeds, and resource limits. Optional sibling batches can use different
methods, with every child derived from the same original parents and base. The
harness binds a selected plan ID to its immutable fingerprint; participants do not
need to reproduce the hash.

## Current implementation

The v0.2 CLI provides:

- Deterministic fixtures and explicitly configured local HTTP inference.
- Phase-specific JSON schemas and recorded, shuffled opaque peer and plan
  presentation.
- Bounded public-memory windows, persistent owner-private notes, and server token
  accounting before generation.
- Frozen phase snapshots, bounded concurrent calls, sealed nominations, and
  distinct abstention and technical-failure outcomes.
- Operator records, separate public transcripts, descriptive reports, and offline
  replay.
- Pending or blocked fusion manifests with exact plan receipts and declared
  compatibility checks.

The harness does not train models, execute weight fusion, evaluate descendants, or
admit children. Declared compatibility metadata is not verification of the actual
weights. Private fields such as `feeling`, `learned_preference`, and `thought` are
recorded textual reports; they do not establish subjective experience. Persistent
notes alter context, not model weights.

## Quick start

Requires Rust 1.85 or later. From the repository, use new output directories:

```powershell
cargo run --locked -- run --config examples/fixture-experiment.json --output runs/demo
cargo run --locked -- replay --record runs/demo/operator-record.json --output runs/demo-replay
cargo run --locked -- report --record runs/demo/operator-record.json
```

The fixture uses synthetic participants and produces pending requests, not merged
models. See [usage and local inference](docs/usage.md) for configuration, privacy,
tokenizer requirements, and record-version compatibility.

## Research direction

The proposed first study compares mutual-choice screening against random pairing,
skill-profile complementarity, and weight-geometry selectors under matched budgets.
Evaluation should include child capability retention, absolute performance,
selection coverage, failures, and measured cost.

DARE is a candidate transformation, not a prescribed optimum. The included toy
variance study examines averaging, DARE followed by averaging, and coordinate or
block crossover. Parameter variance is distinct from behavioral diversity; these
results do not establish the quality of real merged models.

The [draft preregistration](PREREG.md) remains unfrozen. No real specialist-training,
fusion, or generational study has been performed by this implementation.

- [Proposal assessment](docs/research/useful-protocol-review.md)
- [Variance analysis and reproducible study](docs/research/variance-review.md)
- [Fusion-method assessment](docs/sprints/s0/sprint-research/fusion-method-review.md)
- [Implementation and verification](docs/research/instrument-validation.md)
- [Project Book](docs/README.md)
