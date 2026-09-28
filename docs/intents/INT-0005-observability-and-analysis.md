# INT-0005 — Instrument observability and analysis

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0005
- **State:** active
- **Work evidence:** [T-022 through T-025 plan](../sprints/s1/sprint-plans/instrument-extension.md)
- **Completion evidence:** none
- **Code evidence:** [Configuration](../../src/config.rs); [runner](../../src/runner.rs); [export](../../src/export.rs); [analysis](../../analysis/analyze.py)
- **Test evidence:** [Sprint 1 partial acceptance](../sprints/s1/sprint-tests/test-report.md); [recovery receipt](../../analysis/recovery-receipt.json)
- **Documentation evidence:** [Usage](../usage.md); [conformance kit](../../conformance/README.md); [analysis guide](../../analysis/README.md); [roadmap](../roadmap.md)

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

The v0.4 instrument implements the seed, memory, retirement and export features.
Actual server conformance still needs compatible hardware and pinned models.
Real Study A additionally requires pilot and registration
gates. Study B and C retain separate evaluation and operator-executed fusion gates.

## Transition history

- 2026-09-28: created as `proposed`; follows the realized first harness and is
  scheduled after G0 rather than modifying terminal INT-0001.
- 2026-09-28: `proposed` → `planned`; G0 verified with current-stable hosted checks
  after the user's Zenodo deferral; instrument tasks scheduled in Sprint 1 extension.
- 2026-09-28: `planned` → `active`; T-022 begins. Real-server conformance remains
  separately required for G1 and is not inferred from synthetic tests.

## Current evidence and remaining work

T-022–25 pass local/hosted tests and fixed planted-parameter recovery. T-018 must
still supply actual vLLM/llama.cpp receipts and justify the output reserve before G1;
the intent remains active. T-020 covers empirical coverage and the separate pilot
before freeze. Profiles are intentionally unavailable until INT-0004 implements
their semantics; neither empty profile tables nor operator feature annotations
constitute a social-layer implementation.
