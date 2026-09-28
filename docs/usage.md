# Running Project_Lagoon

Sprint 0 implements the partner-choice protocol in Rust. Agents communicate,
retain their own optional self-reports, and independently nominate one peer or
explicitly abstain. Reciprocal nominations form a pair. Separate mutual consent
to an exact reproduction plan and matching declared metadata produce **pending,
unverified requests**. The harness does not load or merge weights, create children,
change the population, prescribe personalities, or impose a loneliness penalty.

## Fixture workflow

Install Rust with Cargo (edition 2024; current stable supported). Run
`rustup update stable` to update the channel selected by `rust-toolchain.toml`.
From the repository:

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
`endpoint` and tokenizer endpoint with your own running server. The adapter posts chat-completions JSON
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

Each request supplies a strict `response_format` JSON schema. This follows
Animus Ferric's schema-to-server boundary; its valve passes an existing
`response_format` through. Point the chat endpoint at a running Ferric valve if
desired, and point the tokenizer at its actual upstream model server. Ferric's
tool-action/scratchpad schema is not imported into this experiment. The server
enforces decoding; this client cannot prove that it honored the schema. It never
retries without constraints. See [vLLM structured outputs](https://docs.vllm.ai/en/latest/features/structured_outputs/)
and [llama.cpp server API](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md).

`tokenizer: {kind: "vllm", endpoint: "http://127.0.0.1:8000/tokenize"}` counts
the same chat messages with the generation prompt. For llama.cpp use
`{kind: "llama_cpp", template_endpoint: "http://127.0.0.1:8000/apply-template",
tokenize_endpoint: "http://127.0.0.1:8000/tokenize"}`. It templates the chat, then
tokenizes with special-token parsing and insertion enabled, matching the server's
text-completion path. Template/tokenizer and inference must use the same model,
adapter and template settings. Configure `context_window_tokens` for the usable
per-request/per-slot capacity; llama.cpp counting does not discover that capacity.
vLLM's reported `max_model_len` further lowers the configured capacity if necessary.
Both adapters are covered by disposable HTTP mocks; a real server/model conformance
run remains required before scientific use.

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

Operator routing handles stay private. Peers receive stable, per-seed opaque
aliases, with candidate handles and plan cards independently shuffled per owner
and round. The same round's communication and selection share that presentation.
Full operator observations record the permutation. `catalog` prints an
operator-only alias map for authoring fixtures. Reports tabulate each candidate/
plan's zero-based position, intended call exposures and choices. Exposure counts
include failed attempts whose projection may not have reached a model; filter the
operator call statuses/token trials when estimating effects among delivered calls.
Shuffling mitigates order confounding; it does not prove absence of label bias.

Communication responses use JSON:

```json
{"public_message":"optional public text","private_update":{"feeling":null,"learned_preference":"optional brief self-report","thought":null}}
```

Selection responses require an explicit `partner` key:

```json
{"partner":null,"consent":{"state":"defer"},"private_update":null}
```

For agreement, select an offered opaque handle and plan ID:

```json
{"partner":"p-c1bc7d533fa54f3f","consent":{"state":"agree","plan_id":"single"},"private_update":null}
```

`decline` and `defer` remain available in both `fixed_plan` and `mutual_choice` mode.
Fixed mode requires one offered plan. Mutual mode may offer several. Missing or
invalid consent does not erase an otherwise valid partner nomination, but it blocks
a merge request. The harness binds each valid plan ID to the fingerprint in that
call's immutable operator observation. Both parents' receipts must agree, covering
recipes, child count, seeds and the resource envelope. `catalog` displays these
cards without making inference calls. Changing a payload changes its fingerprint.

The protocol templates live in `schemas/communication.schema.json` and
`schemas/selection.schema.json`. Partner and plan enums are specialized from
the observation before submission; the template placeholders are never offered.
Constrained schemas use required nullable private fields. Parsing also accepts
omitted private fields and missing consent for diagnostic/fixture compatibility.
Schemas bound public messages to 128 Unicode characters and each private note to
64; peer aliases, plan IDs and consent strings are bounded too. The parser enforces
the public/private character limits as well as byte guards, including initial notes.
Excess text returns `oversized_response`; fields are not silently shortened.
One complete JSON markdown fence is accepted and counted in
`fenced_json_responses`; surrounding prose, guessed targets and unknown plan IDs
are not repaired. An obsolete echoed fingerprint is ignored, never used as consent.

## Memory and token budgets

Every configuration declares `memory`: `recent_rounds` preceding rounds plus
current-round events, `max_public_events`, `max_prompt_tokens`,
`context_window_tokens`, and `safety_margin_tokens`. Owner-private state persists,
including `learned_preference`; the procedure and full offered plan cards stay intact.
The full public transcript is retained in the output, while each call receives only
its recorded window. Oldest complete messages/pair events are removed first to fit
the byte guard and then the tokenizer budget. A base prompt that cannot fit gives
`context_limit` without generation. Output tokens and the safety margin are reserved.
Do not intentionally set a budget larger than the actual server slot.

Example configurations reserve 4096 output tokens, 3968 prompt tokens and 128
safety tokens within 8192 context tokens. Compact, maximally Unicode-escaped text
at the public/private character limits fits the examples' 4096-byte response guard.
This is not a worst-case token guarantee: tokenization, repeated whitespace and
model reasoning can consume additional tokens. XGrammar exposes whitespace limits
but allows flexible whitespace by default; pin and test the actual decoder/server
configuration ([compiler API](https://xgrammar.mlc.ai/docs/latest/api/python/grammar_compiler.html)).
Use a non-reasoning deployment or explicitly validate its reasoning budget/template
before research use. The client does not guess model-specific reasoning options.

HTTP responses retain the choice's `finish_reason` in the operator record. A
`length` finish produces `generation_limit` even if the content is complete JSON;
partial content is retained for diagnostics, with no public message, memory update
or nomination applied. Null content with `length` is recorded as empty text.
Missing/null reasons are recorded as null and counted as `unreported`; fixtures
also have unreported reasons. They do not prove that generation completed normally.
Other bounded reason strings are retained and content follows normal parsing.
Invalid reason metadata gives `malformed_content`. Servers that omit termination
metadata cannot reliably distinguish token exhaustion from formatting failure.

Tokenization and generation share one timeout after worker acquisition.
Tokenizer/HTTP/parse errors become `tokenization_failure`; timeout remains
`timeout`. There is no approximate-token fallback for HTTP backends. Fixtures use
a declared deterministic byte-plus-16 accounting rule, not a real-model tokenizer.
Operator calls retain each successful count, model capacity, prompt hash, final
projection, omitted-event count and response-schema hash. Replay reconstructs
trimming from those receipts without contacting the tokenizer. External counts
are trusted server measurements, not authenticated/recomputed offline; fixture
counts are recomputed and checked. Owners may receive different fitting projections
of the same frozen underlying public snapshot.

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
serialized observation at 4 MiB, and the configurable private-state byte guard at
64 KiB; the stricter per-field character limits also apply. Oversized observations skip inference with `context_limit`;
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
`finish_reason_counts` counts responses with available content (including length
termination); transport/HTTP failures have no finish-reason receipt. All-call and
selection status counts include `generation_limit` in their existing denominators.
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
cargo build --locked
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
```

Tests cover frozen snapshots, owner-only projections, bounded concurrent calls,
real HTTP to disposable loopback test servers, failures, exact consent, sibling
parentage, persistence, replay and CLI workflows. They establish protocol behavior,
not real-model preferences or fusion quality. Local and hosted validation are both
enabled. The user restored GitHub CI after making the repository public; main now
requires the current-stable `check` job. See the sprint receipts for tested heads.

[INT-0002](intents/INT-0002-evaluated-dare-descendants.md) supplies real checked
checkpoints, a pinned external merger and evaluation before child admission.
[INT-0003](intents/INT-0003-controlled-evolutionary-study.md) supplies causal
comparisons and generational studies. [Method research](sprints/s0/sprint-research/fusion-method-review.md)
and [sibling research](sprints/s0/sprint-research/sibling-batch-review.md) explain
why the current catalog permits exact single-child or opt-in sibling plans.

The v0.4 record format is schema version 4. It adds per-call seeds, configured
response bounds and optional private peer memory. It rejects older records rather
than silently reinterpret their seeds, schemas or outcomes.
Use each archived record's matching code revision for replay. Historical evidence
in `docs/sprints/s0` is preserved unchanged.

See the [variance study](research/variance-review.md), [useful-protocol assessment](research/useful-protocol-review.md)
and [unfrozen next-study preregistration](../PREREG.md). No training or real fusion
has been executed by this follow-up.

## Reproducible sampling and private memory (v0.4)

Temperature is declared in each experiment's `decoding.temperature`. The existing
examples retain greedy decoding as controls; `stochastic-local-experiment.json`
illustrates temperature 0.7. A seed does not guarantee identical real-server output
across hardware/software changes; offline replay validates the recorded result.

Every call logs and sends its own u64 seed. Encoding is UTF-8 compact JSON of
`["lagoon-call-seed-v1", run_seed, round, phase, step, handle]`, where phase is
`"communication"` or `"selection"`, step is an integer or null, and handle is the
operator handle. SHA-256's first eight bytes are interpreted big-endian. The same
seed is used for token counting and generation. Replay rejects a changed seed.

Optional `string_limits` has `communication` and `selection` objects, each with
`public_message_chars` and `private_note_chars` (1–4096 Unicode characters).
Defaults remain 128/64. These values enter runtime schemas, schema fingerprints,
observations and parser checks. Selection has no public-message output. Limits
constrain new text; a longer note validly written in another phase is retained.
The separate UTF-8 response, context and total-private-state byte limits still apply.
Changing limits does not justify reducing the output-token reserve without G1.

With `peer_memory: true`, private updates may include `peers`, an object mapping
known peer aliases to `{"trust": 0..10, "note": "up to 64 Unicode characters"}`.
Only other configured participants are allowed. Omitted/null maps retain the
ledger; a supplied map replaces it, and `{}` clears it. Entries and aggregate bytes
are bounded. The owner writes all entries; the harness never invents a reputation.
Peer observations and public transcripts do not include another owner's ledger.
