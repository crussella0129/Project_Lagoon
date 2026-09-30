# Sprint 1 end-to-end results

## Hosted execution

Authoritative GitHub Actions [run 36378653944](https://github.com/crussella0129/Project_Lagoon/actions/runs/36378653944),
head **`bc5cec1472eda883c0278ebe1d7a4f58cb222d61`**, concluded **success** on
2026-09-28 at 04:41:21 UTC. The required `check` job passed formatting, warning-free
Clippy, every Rust target, locked Python setup, ruff, both Python suites and full
planted recovery on Ubuntu 24.04. The synthetic recovery specification/report are
retained as the run's `synthetic-recovery-receipts` artifact. This independently
executes the Linux binary and CSV/analysis boundary checked locally on Windows.

The earlier [run 36378555673](https://github.com/crussella0129/Project_Lagoon/actions/runs/36378555673)
failed in action setup before tests: the publisher has `v10.2.0` but no `v10`
reference. The successful run uses the verified release. No test was removed.
Phase 0 had already passed [run 36374666278](https://github.com/crussella0129/Project_Lagoon/actions/runs/36374666278)
on `46fb66f2d00583592b62179fb16debc730f33886`, satisfying amended G0 before features.

## Genuine limits and unlocks

- **Real inference / G1:** not executed. T-018 / INT-0005 requires pinned actual
  vLLM and llama.cpp receipts, template/model provenance and a justified reserve.
  Local vLLM is absent; Docker's Linux engine is unavailable. The kit, declarations
  and loopback test doubles cannot substitute for that evidence. No reserve reduction.
- **Zenodo:** explicitly deferred by the user to T-017 before archival release.
  No activation or DOI claim. The earlier browser initializer failed before login;
  the user removed this dependency from G0. INT-0006 remains active.
- **Study A / G2:** the fixed synthetic scenario passes, but social profiles/events,
  empirical coverage, a separate pilot, freeze and registration remain T-019/20/21.
- **Study B/C:** training, external fusion, containment verification and generational
  admission remain separate INT-0002/3 work and G3/G4 gates.
- **Main protection:** active rules read back correctly; a deliberately invalid
  merge was not attempted. The main-targeted sprint PR is created only after Book
  closure, per the workflow; its required check is inspected at that checkpoint.

The successful code head above is the reproducible implementation receipt.
Subsequent sprint closeout commits attach this evidence and lifecycle state; they
do not assert a new scientific result or retroactively claim an unexecuted server.
