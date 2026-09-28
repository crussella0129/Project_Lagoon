# INT-0007 — Supported and protected builds

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0007
- **State:** planned
- **Work evidence:** [T-014 build plan](../sprints/s1/sprint-plans/build-plan.md)
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** none

## Intent

Maintain an explicit, tested Rust support baseline and prevent unchecked updates
to `main`. This follows the realized INT-0001 without reopening its history.
The user's instruction to use current Rust supersedes the improvement plan's
request to search for the oldest compatible compiler. Use current stable 1.98.1
as the declared minimum and pinned CI baseline, plus rolling stable CI.

## Acceptance criteria

- **AC-1:** Cargo and the README declare Rust 1.98.1; local formatting, clippy and
  all 45 existing tests pass, and CI tests the locked dependency set on both the
  pinned baseline and rolling stable.
- **AC-2:** An active `main` ruleset requires successful GitHub Actions `check` and
  `msrv` jobs, with no bypass actors, preserving deletion and force-push protection.
- **AC-3:** Both Dependabot ecosystems use weekly grouped updates; repository URLs
  and the T-008 hosted-CI status reflect Project_Lagoon and available CI.

## Rationale

A support promise needs a matching check. Selecting a modern supported baseline
is cheaper and clearer here than claiming the historical minimum of dependencies.

## Alternatives

Testing only stable misses a stale declared baseline. Searching older releases was
superseded by the user's current-Rust instruction. Downgrading dependencies solely
to preserve 1.85 is unnecessary. Replacing the existing ruleset would lose history;
update its disabled configuration instead.

## Consequences

Older Rust compilers are unsupported. Dependency changes must pass both jobs.
Changes to `main` remain subject to the existing human-approval merge policy.
The pinned job retains the name `msrv` because it tests the declared minimum
supported version, not a claim about the earliest compiler that happens to work.

## Transition history

- 2026-09-28: created as `proposed` from improvement-plan Phase 0.
- 2026-09-28: `proposed` → `planned`; implementation authorized by "take this and
  implement", with current Rust explicitly authorized in the follow-up.
