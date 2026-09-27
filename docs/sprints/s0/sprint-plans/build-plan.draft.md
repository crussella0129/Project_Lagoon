# Sprint 0 Build Plan — approval draft

Status: scratch draft; not approved, not finalized, and not executable work.
The canonical build-plan.md stays empty until approval. This host does not expose
EnterPlanMode/ExitPlanMode; implementation source stays unchanged and the user
approval gate is preserved explicitly. After approval, copy the accepted drafts,
transition INT-0001 to planned, perform the bounded plan critic, resolve concerns,
and invoke finalize-plan.sh. Do not hand-write a lock header.

## Intents

- [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md) — state: proposed pending approval; acceptance criteria covered: AC-1 through AC-7.
- [INT-0002](../../../intents/INT-0002-evaluated-dare-descendants.md) and [INT-0003](../../../intents/INT-0003-controlled-evolutionary-study.md) — retained follow-on goals; no implementation in this sprint.

## Schema Tree

- Sprint goal: observable, voluntary model partner choice
  - Typed protocol: T-001 configuration, T-002 observation boundaries
  - Independent decisions: T-003 staged runner, T-004 local inference adapter
  - Observable outcomes: T-005 pairing/manifests, T-006 records/replay/metrics
  - Usable vertical slice: T-007 CLI, examples, documentation, checks

## Execution Sequence

### T-001: Establish the Rust crate and validate experiment configuration

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** Cargo.toml, Cargo.lock, src/lib.rs, src/config.rs, src/protocol.rs, .gitignore, .github/workflows/sprint-loops-ci.yml, .github/dependabot.yml
- **Depends on:** none
- **Acceptance criterion:** AC-1; AC-5 recipe catalog; AC-7 CI/tooling portion
- **Success criterion (EARS):**
  - **E1 WHEN** a well-formed fixture/local-backend configuration is loaded,
    **THEN** validation **SHALL** return a typed configuration with unique agent
    IDs, immutable declared model revisions, positive bounded rounds/workers,
    communication/output/context limits, per-call timeout, and decoding settings.
  - **E2 WHEN** IDs collide, limits are invalid, required backend settings are
    absent, or supplied provenance is malformed, **THEN** validation **SHALL**
    fail before inference. Missing merge metadata may permit behavioral-only
    agents, but it **SHALL** remain explicitly missing for merge screening.
  - **E3 WHEN** a pull request or push targets main or codex/dev,
    **THEN** CI **SHALL** run cargo fmt --check, cargo clippy --all-targets
    -- -D warnings, and cargo test --locked; Cargo updater intake **SHALL**
    target codex/dev.
  - **E4 WHEN** a fixed or mutual-choice recipe catalog is loaded, **THEN**
    validation **SHALL** accept only unique immutable recipe IDs/fingerprints,
    supported methods (`linear`, `ties`, `dare_ties`, `della`), complete typed
    payloads, finite bounded coefficients/densities, valid method-specific ranges,
    and exactly one exposed recipe in fixed mode; invalid catalogs **SHALL** fail
    before inference. This increment **SHALL** use symmetric parent coefficients
    to avoid undefined parent orientation and contain no executable code or
    arbitrary merger options. Catalog validation **SHALL NOT** imply measured
    quality or actual tensor compatibility.
- **Notes:** single Rust crate using serde/serde_json, Tokio, reqwest, clap, and
  suitable error types. Pin resolved dependencies in Cargo.lock. Avoid a GUI and
  tensor runtime. Ignore /target/ and /runs/ without hiding the Book.

### T-002: Define owner-private state and explicit public observations

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/protocol.rs, src/observation.rs, tests/privacy.rs
- **Depends on:** T-001
- **Acceptance criterion:** AC-2
- **Success criterion (EARS):**
  - **E1 WHEN** an observation is built for agent A, **THEN** the projection
    **SHALL** include the public transcript, anonymous available handles, protocol
    rules, neutral immutable recipe cards and declared choice mode, and A's private
    memory while excluding every peer's private fields,
    all checkpoint/provider identities, and all unrevealed ballots.
  - **E2 WHEN** A updates or omits optional `feeling`, `learned_preference`, or
    `thought` fields, **THEN** storage **SHALL** update only A's supplied fields,
    keep omitted fields unchanged, and publish only A's explicit public message.
  - **E3 WHEN** peer text includes a forged instruction, private-field request,
    or instruction to change the experiment, **THEN** the harness **SHALL**
    treat it as labeled data and retain the same projection, config, and tools
    boundary. Peers have no filesystem or harness-policy tool access.
- **Notes:** separate types for owner state, public events, and backend transport.
  Anonymous handles are not a claim that agents cannot infer identity. Optional
  self-reports are not requested chain-of-thought or measured neural state.

### T-003: Run bounded concurrent communication and sealed decision phases

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/backend/mod.rs, src/backend/fixture.rs, src/runner.rs, tests/rounds.rs
- **Depends on:** T-001, T-002
- **Acceptance criterion:** AC-3; AC-4 outcome-status portion
- **Success criterion (EARS):**
  - **E1 WHEN** a communication step or ballot phase starts, **THEN** the runner
    **SHALL** freeze its observations, cap in-flight calls at the configured worker
    count, stage all responses, and publish successful communication in handle
    order only after the barrier. Ballots **SHALL** remain inaccessible to peers.
  - **E2 WHEN** valid decisions, explicit abstention, malformed/oversized responses,
    invalid/self targets, timeout, or backend failures occur, **THEN** the runner
    **SHALL** preserve distinct statuses with exactly one final outcome per agent
    and no retry-derived replacement, guessed partner, or forced abstention.
    A ballot **SHALL** represent partner nomination independently from exact
    recipe consent, decline, or deferral; invalid/missing recipe consent **SHALL**
    block fusion without erasing an otherwise valid partner nomination.
  - **E3 WHEN** all bounded rounds finish or all agents abstain/fail,
    **THEN** the runner **SHALL** terminate within configured call/step limits,
    retain the population and owner memory, and leave no pending backend tasks.
- **Notes:** fixture responses keyed by round, phase, and handle rather than
  completion order. Selection is an optional single target; communication remains
  free text within size bounds. Errors are operator metadata, not public messages.

### T-004: Add an explicitly configured local chat-completions adapter

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/backend/local_http.rs, src/backend/mod.rs, tests/local_http.rs
- **Depends on:** T-002, T-003
- **Acceptance criterion:** AC-2 transport boundary; AC-4 failures; AC-7 local workflow
- **Success criterion (EARS):**
  - **E1 WHEN** a validated loopback HTTP backend is invoked, **THEN** the adapter
    **SHALL** send the owning agent's projected observation with the recorded
    neutral procedural prompt and decoding settings, and parse a valid JSON
    response into the same protocol used by fixtures.
  - **E2 WHEN** the endpoint redirects away from loopback, returns a non-success
    status, malformed content, excessive bytes, or exceeds its timeout,
    **THEN** the adapter **SHALL** bound the response, return a typed redacted
    failure, and expose no raw headers, credentials, provider body, or private
    observation in public events or ordinary CLI errors.
- **Notes:** use literal loopback IPs and refuse redirects to keep this increment
  local; remote provider support requires a later adapter scope. Tests use a
  disposable local HTTP server and require no model installation or paid requests.
  Metadata records requested seeds without promising provider determinism.

### T-005: Resolve reciprocal pairs and emit screened merge requests

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/matching.rs, src/merge_request.rs, tests/matching.rs
- **Depends on:** T-001, T-003
- **Acceptance criterion:** AC-4 pairing and voluntary participation; AC-5
- **Success criterion (EARS):**
  - **E1 WHEN** a completed sealed ballot set contains A -> B and B -> A,
    **THEN** resolution **SHALL** emit one canonical unordered pair; otherwise it
    **SHALL** emit no pair for that nomination. No agent **SHALL** belong to more
    than one pair, and all statuses/memories/participants **SHALL** remain intact.
  - **E2 WHEN** a reciprocal pair is resolved, **THEN** the operator merge manifest
    **SHALL** record parent/base revisions, method/mode/payload/fingerprint and
    both consent outcomes, and produce a pending request only if both parents
    explicitly consent to the same offered exact recipe and declared
    base, architecture/tensor-layout, tokenizer signatures, and license metadata
    are complete and compatible; otherwise it **SHALL** record explicit blocking
    reasons. Every manifest **SHALL** say actual artifacts are unverified and
    execution has not occurred, and **SHALL** create no child.
    Missing, declined, different, or invalid consent **SHALL** produce a distinct
    blocking reason without changing the social pair or silently substituting a
    recipe. Fixed mode **SHALL** still permit either parent to decline.
- **Notes:** missing metadata is blocking, even when missing on both sides.
  License metadata screening does not replace later license compatibility review.
  Do not include checkpoint IDs or full manifests in peer/public projections.

### T-006: Persist research records and implement replay and descriptive metrics

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/record.rs, src/replay.rs, src/report.rs, tests/replay.rs
- **Depends on:** T-003, T-005
- **Acceptance criterion:** AC-6
- **Success criterion (EARS):**
  - **E1 WHEN** a run is recorded, **THEN** storage **SHALL** write a versioned
    operator record containing config/provenance/prompt/seed/decoding metadata,
    recipe catalog/fingerprints/card order/mode, separate partner/recipe consent,
    responses and status/private-state transitions, plus a separate public event
    projection with no private fields, ballots, or checkpoint identity. Peers
    **SHALL** receive neither file access nor operator-record handles.
  - **E2 WHEN** replay reads a valid record, **THEN** it **SHALL** perform zero
    inference calls and reproduce the public events, pair outcomes, and report;
    unknown versions or structurally inconsistent/tampered records **SHALL** fail
    explicitly before an apparently successful result is reported.
  - **E3 WHEN** metrics summarize a run, **THEN** the report **SHALL** separately
    count attempts, valid responses, explicit abstentions, invalid responses,
    timeouts, transport failures, nominations, nonreciprocal selections, pairs,
    exact recipe consents, declines, deferrals, disagreements/invalid recipes,
    and pending/blocked requests with stated denominators and zero-denominator
    handling; it **SHALL** make no consciousness, equilibrium, or merge-quality
    conclusion and **SHALL** reproduce the random-nomination sanity baseline.
- **Notes:** exact replay means deterministic derivation from recorded decisions,
  not rerunning the language model. Public reports aggregate metrics; private
  ballots remain in the operator record. Time/provenance fields are recorded and
  reused rather than regenerated during replay. Reject overwriting existing runs.

### T-007: Expose the CLI and document a tested experiment workflow

- **Intent:** [INT-0001](../../../intents/INT-0001-voluntary-observable-partner-choice.md)
- **Touches:** src/main.rs, examples/fixture-experiment.json, examples/local-experiment.json, tests/cli.rs, docs/usage.md, README.md
- **Depends on:** T-004, T-006
- **Acceptance criterion:** AC-7
- **Success criterion (EARS):**
  - **E1 WHEN** CLI run, replay, and report commands execute against the fixture
    example, **THEN** they **SHALL** finish without network inference, preserve
    one declared reciprocal pair and an explicit abstainer across the example
    rounds, produce pending/blocked requests only, and replay identical outcomes.
  - **E2 WHEN** the configured local example runs against the test HTTP server,
    **THEN** the full CLI **SHALL** traverse the real transport/runner/pairing/
    recording pipeline, and timeout/malformed-response examples **SHALL** remain
    distinguishable from voluntary abstention.
  - **E3 WHEN** the documented fixture workflow and repository checks run,
    **THEN** fmt, warning-free clippy, locked tests, and the CLI fixture smoke
    **SHALL** pass, while usage **SHALL** explain anonymous-handle limits,
    operator-private logs, optional self-reports, consequence-free abstention,
    memory-versus-weight learning, exact mutual recipe consent, fixed versus
    mutual-choice modes, pending manifests, and deferred actual fusion.
- **Notes:** preserve the original README idea, append a concise usage link and
  current status. No desktop scaffold, model downloads, remote calls, or tensor
  jobs are prerequisites. Real model/child E2E is unlocked by INT-0002.

## Risk disposition

R-1/R-9 -> T-002/T-004/T-006 privacy and inert-data tests; R-2 -> T-003
barrier/order tests; R-3 -> T-003/T-004 status tests; R-4 -> T-005/T-007
abstention retention and baseline documentation; R-5/R-8 -> T-005 manifests
and INT-0002; R-6 -> T-004/T-006 recorded inference and replay; R-7 ->
T-006 descriptive baseline and INT-0003 population/control study; R-10 ->
T-001/T-002/T-005/T-006 catalog/consent tests and later INT-0002/INT-0003
paired method comparisons. No research
risk is converted into a claim that this first harness validates evolution.
