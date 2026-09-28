# INT-0007 — Supported and protected builds

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0007
- **State:** active
- **Work evidence:** [T-014 build plan](../sprints/s1/sprint-plans/build-plan.md)
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** none

## Intent

Maintain an explicit, tested Rust support baseline and prevent unchecked updates
to `main`. This follows the realized INT-0001 without reopening its history.
The user's clarified instruction supersedes both minimum-version bisection and
a numbered pinned baseline. Support current stable only, select `stable` in the
toolchain file and CI, and test weekly Dependabot updates against that channel.
Cargo's numeric `rust-version` field is omitted, not set to an invalid `current`.

## Acceptance criteria

- **AC-1:** The README declares current stable support; local toolchain and CI select
  `stable`, Cargo makes no numbered support promise, and local formatting, clippy,
  all 45 existing tests and hosted checks pass against the locked dependencies.
- **AC-2:** An active `main` ruleset requires a successful GitHub Actions `check`
  job, with no bypass actors, preserving deletion and force-push protection.
- **AC-3:** Both Dependabot ecosystems use weekly grouped updates; repository URLs
  and the T-008 hosted-CI status reflect Project_Lagoon and available CI.

## Rationale

A support promise needs a matching check. Following stable avoids a stale numbered
minimum and matches the user's explicit maintenance preference.

## Alternatives

Keeping a numbered minimum while testing only stable creates a stale promise.
Searching older releases was superseded by the user's instruction. Downgrading dependencies solely
to preserve 1.85 is unnecessary. Replacing the existing ruleset would lose history;
update its disabled configuration instead.

## Consequences

Older Rust compilers are unsupported. Dependency changes must pass the stable job.
Changes to `main` remain subject to the existing human-approval merge policy.
Dependabot has no compiler support-floor option; its scheduled Cargo/action updates
are checked by the same stable workflow. Local users run `rustup update stable`.

## Transition history

- 2026-09-28: created as `proposed` from improvement-plan Phase 0.
- 2026-09-28: `proposed` → `planned`; implementation authorized by "take this and
  implement", with current Rust explicitly authorized in the follow-up.
- 2026-09-28: revised while `planned` after "make the support floor 'current'";
  use rolling stable only. The [plan amendment](../sprints/s1/sprint-plans/plan-amendment.md)
  supersedes numbered-floor and two-job clauses without rewriting locked history.
- 2026-09-28: `planned` → `active`; T-014 implementation and validation started.
