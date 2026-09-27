# Running Lover's Lagoon

Sprint 0 implements the partner-choice protocol in Rust. Agents communicate,
retain their own optional self-reports, and independently nominate one peer or
explicitly abstain. Reciprocal nominations form a pair. Separate mutual consent
to an exact reproduction plan and matching declared metadata produce **pending,
unverified requests**. The harness does not load or merge weights, create children,
change the population, prescribe personalities, or impose a loneliness penalty.

## Fixture workflow

Install Rust with Cargo (edition 2024; minimum Rust 1.85). From the repository:

```powershell
cargo run --locked -- run --config examples/fixture-experiment.json --output runs/demo
cargo run --locked -- replay --record runs/demo/operator-record.json --output runs/demo-replay
cargo run --locked -- report --record runs/demo/operator-record.json
cargo run --locked -- catalog --config examples/fixture-experiment.json
```

Each output directory must be new. The fixture has three synthetic participants,
two rounds, one reciprocal pair and one voluntary abstainer each round. Expect
two pair instances, two pending batches and two child requests, with zero executed
merges or admitted children. Its messages, private fields, and checkpoint metadata
are fabricated test data. They are not observations of real models.

Run the opt-in sibling fixture:

```powershell
cargo run --locked -- run --config examples/sibling-experiment.json --output runs/siblings
cargo run --locked -- replay --record runs/siblings/operator-record.json --output runs/siblings-replay
```

Expect one social pair, one agreed pending batch, and two child requests: TIES and
DARE plus TIES, with different declared seeds. Both requests reference the same
original parents and common base. No first child becomes the second child's parent.
The additional single-child plan remains available; no extra siblings are silently
added. Whether different methods produce useful behavioral diversity remains an
empirical question for the later fusion/evaluation increment.

## Explicit local inference

Copy `examples/local-experiment.json` and replace each backend's `model` and
`endpoint` with your own running server. The adapter posts chat-completions JSON
to the **exact URL** supplied, for example `http://127.0.0.1:8000/v1/chat/completions`.
Only HTTP URLs with literal loopback IPs (`127.0.0.1` or `[::1]`) are accepted.
Hostnames, credentials in URLs, query strings, fragments, redirects, and environment
proxies are disabled. No remote or paid service is configured by default.

```powershell
cargo run --locked -- catalog --config examples/local-experiment.json
cargo run --locked -- run --config examples/local-experiment.json --output runs/local
```

You start the inference server separately. The template uses the same placeholder
model for all handles; configure distinct served checkpoints if that is your
experimental question. Seeds and decoding settings are submitted and recorded,
but a server may ignore them. Inference is not promised deterministic. Only replay
is exact. The actual served weights are not verified by the HTTP adapter.

The template omits merge provenance, so any agreed pair remains blocked at the
metadata boundary. To study eligibility, supply `checkpoint: {model, revision}`
with an immutable 40-hex commit and `merge_metadata: {base: {model, revision},
architecture, tensor_layout, tokenizer, license}`. Signatures are operator-supplied
claims. Both parents need matching base/revision, architecture, layout, tokenizer,
and license metadata. Matching labels do not verify tensors, licenses, or quality.
The fixture's synthetic metadata must not be reused as real provenance.

## Choices and privacy

Each phase uses a frozen public snapshot. Calls run with the configured worker cap;
the timeout begins when a worker starts a call. The harness waits for every attempt
before publishing communication in handle order. Final nominations and consent
stay sealed. Subsequent rounds reveal pair handles, not individual ballots or
consent. Communication and selection failures do not substitute random choices.
Private state and the whole population persist across rounds.
Persisted textual memory changes context; it does not update model weights.

Communication responses use JSON:

```json
{"public_message":"optional public text","private_update":{"learned_preference":"optional brief self-report"}}
```

Selection responses require an explicit `partner` key:

```json
{"partner":null,"consent":{"state":"defer"}}
```

For agreement, use another handle and the exact plan card values:

```json
{"partner":"b","consent":{"state":"agree","plan_id":"single","fingerprint":"copy the offered fingerprint"}}
```

`decline` and `defer` remain available in both `fixed_plan` and `mutual_choice` mode.
Fixed mode requires one offered plan. Mutual mode may offer several. Missing or
invalid consent does not erase an otherwise valid partner nomination, but it blocks
a merge request. Both parents must agree to the same offered fingerprint, covering
recipes, child count, seeds and the resource envelope. `catalog` displays these
cards without making inference calls. Changing a payload changes its fingerprint.

The owner receives optional `feeling`, `learned_preference`, and `thought` fields.
Omitting a field or supplying null retains its previous value; an empty string
clears its text. These are brief textual self-reports, not hidden reasoning or
evidence of consciousness. The default initial state is empty. Private fields,
checkpoint mapping, backend settings, and other agents' ballots are excluded from
peer observations. Public messages remain untrusted data; they cannot modify
configuration or grant filesystem/tool access. A model may follow misleading peer
text or voluntarily disclose its own information. This is a harness access boundary,
not a guarantee of resistance to social persuasion.

The trusted operator and configured inference server receive owner-private data.
Output files inherit normal filesystem permissions; they are not encrypted. Agents
receive no filesystem tools or record paths. Restrict file access when deploying
outside this trusted-operator setting. Keep `operator-record.json` and
`merge-requests.json` private. Public transcripts may contain voluntary disclosures.

## Catalog and bounds

Typed methods are `linear`, `ties`, `dare_ties`, and `della`. The first increment
uses positive symmetric parent coefficients and bounded lambda/density parameters;
DELLA additionally declares epsilon. These are recipe specifications for future
execution, not an implemented kernel or recommendation that one method is best.

`max_siblings` defaults by convention to one in the baseline configs; set it to two
or three to opt in. Every offered plan must fit that limit and contain unique
recipe/seed jobs. Plans have total compute-seconds and output-byte bounds. These
are consented future budgets, not measured resource consumption.

Configuration caps population at 128, rounds at 100, communication steps at ten,
workers at 128, and call timeout at five minutes. Response text is capped at 1 MiB,
serialized observation at 4 MiB, and private-state text at 64 KiB, with smaller
configurable limits. Oversized observations skip inference with `context_limit`;
there is no silent truncation. File reads and individual persisted artifacts have
a 64 MiB limit. Aggregate memory/runtime depend on population, history and configured
bounds; large experiments may exceed the artifact limit. HTTP envelopes have a
separate finite bound allowing JSON escaping. Error bodies and headers are not
copied into public records or CLI errors. Successful response text is kept in the
operator record for replay.

## Artifacts and metrics

Every successful CLI run or replay writes:

| File | Contents / audience |
| --- | --- |
| `operator-record.json` | Versioned config/provenance, prompt, seed/decoding, projected requests, timings, replies, private states, ballots and derived results; trusted operator |
| `public.json` | Explicit public messages and reciprocal pair handles |
| `merge-requests.json` | Grouped pending/blocked decisions and individual child recipe/parentage manifests; trusted operator |
| `report.json` | Descriptive counts, failure categories and explicit denominators |

Reports distinguish all decision slots from valid nominations, voluntary
abstentions and failed decisions. Paired fraction is twice the social-pair count
divided by **all agent-round decision slots**, including failures and abstentions.
Consent counts cover valid ballots; blocked-reason counts cover paired groups.
`nonreciprocal_nominations` counts valid nominations outside a reciprocal pair.
`blocked_pairs` includes every paired group whose request cannot proceed;
`blocked_batches` counts the subset with an agreed exact plan blocked by metadata.
Agreed batches include metadata-blocked plans. `consented_child_jobs` counts their
declared jobs; `pending_child_requests` counts metadata-eligible jobs, and
`blocked_child_jobs` counts the rest. A pair with missing/different consent has no
agreed child specification. Counts never imply execution or successful inheritance.

The independent uniform, non-self, no-abstention nomination baseline expects
`n / (2(n-1))` pairs and matched fraction `1 / (n-1)` for `n >= 2`. Zero/singleton
pools have zero expected pairs; the zero-population fraction is undefined. This is
a descriptive reference, not evidence of equilibrium or a causal control.

An agent failure still yields a persisted record and a successful CLI exit so it
can be analyzed. Invalid configuration, inconsistent records and persistence
failures return a nonzero exit. Replay validates phase order, handles, observations,
normalized outcomes, pairs, manifests and reports without initializing any backend.
It preserves original times and rejects inconsistent records. Fingerprints provide
structural consistency, not authentication against an operator who rewrites every
dependent field consistently.

## Verification and next increment

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests cover frozen snapshots, owner-only projections, bounded concurrent calls,
real HTTP to disposable loopback test servers, failures, exact consent, sibling
parentage, persistence, replay and CLI workflows. They establish protocol behavior,
not real-model preferences or fusion quality. CI uses the same checks on `main`
and `codex/dev`; a hosted pass requires an actual remote run.

[INT-0002](intents/INT-0002-evaluated-dare-descendants.md) supplies real checked
checkpoints, a pinned external merger and evaluation before child admission.
[INT-0003](intents/INT-0003-controlled-evolutionary-study.md) supplies causal
comparisons and generational studies. [Method research](sprints/s0/sprint-research/fusion-method-review.md)
and [sibling research](sprints/s0/sprint-research/sibling-batch-review.md) explain
why the current catalog permits exact single-child or opt-in sibling plans.
