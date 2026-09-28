# INT-0005 — Instrument observability and analysis

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0005
- **State:** proposed
- **Work evidence:** none
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** [Roadmap](../roadmap.md)

## Intent

Extend the reproducible instrument for social selection after G0. Preserve the
existing privacy and procedural-consent boundary while enabling meaningful
sampling, bounded memory, participant retirement and auditable analysis.

## Acceptance criteria

- Derive and record distinct per-call seeds from a versioned, unambiguous SHA-256
  encoding of run seed, round, phase, step and handle; declare temperature per arm.
- Configure, fingerprint and enforce phase-specific string limits with old defaults.
- Persist optional owner-written alias/trust/note ledgers with bounded size and
  schema, parser, replay and privacy-canary coverage.
- Pin server/model/template/grammar provenance, commit real vLLM and llama.cpp
  conformance receipts, and justify output reserves from measured tokens plus margin.
- Implement selectable matched exit including eligibility, reduced enums,
  singleton handling, retirement announcements and replay.
- Export choices, messages, events, outcomes and profiles with auditable joins and
  visibility. Python conditional logit and OLS recover known fixture parameters
  through the entire export pipeline before real data; invalid choices and voluntary
  abstention are not silently conflated or dropped.

## Rationale

An analysis is only as useful as its information timing, denominators and replay
receipts. Python is explicitly requested for the scientific analysis ecosystem;
Rust remains the instrument language.

## Alternatives

Reusing one seed or greedy-only decoding does not provide declared stochastic
sampling. Enlarging prompts without real conformance evidence risks truncation.
Excluding zero-pair or refusal outcomes creates selection bias.

## Consequences

These are future features, not claims about v0.3. Server execution needs compatible
hardware and pinned models. Real Study A additionally requires pilot and registration
gates. Study B and C retain separate evaluation and operator-executed fusion gates.

## Transition history

- 2026-09-28: created as `proposed`; follows the realized first harness and is
  scheduled after G0 rather than modifying terminal INT-0001.
