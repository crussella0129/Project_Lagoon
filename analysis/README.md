# Research exports and analysis

The Rust instrument exports replay-validated research CSV tables. Python supplies
audited conditional-logit and OLS estimators plus a fixed synthetic recovery test.
This analysis is exploratory infrastructure. It is not a registered Study A analysis,
a power calculation or evidence of real-model preferences.

```sh
cargo run --locked -- export --record runs/demo/operator-record.json --output runs/demo-tables
uv sync --project analysis --locked --python 3.12
uv run --project analysis python -m unittest discover -s analysis -v
uv run --project analysis python analysis/recover.py --binary target/debug/lovers-lagoon --output target/recovery
```

On Windows use `target/debug/lovers-lagoon.exe`. Build the binary with `cargo build
--locked` before Python integration tests. Set `LAGOON_BINARY` if it is elsewhere.
Every output directory must be new. Export tables preserve raw text, including
newlines and formula-like strings; load as text/CSV data rather than spreadsheet
formulas. Exports contain sealed nominations and are **trusted research data**,
even though private notes, peer ledgers and operator identities are omitted.

## Table contract (export version 1)

`manifest.json` records the full input record fingerprint, configuration fingerprint,
versions, pairing mode, row counts and each table's SHA-256. Record fingerprints
include original timestamps/receipts, so two separately captured runs can have
different IDs even with identical configuration. Digests establish consistency,
not authenticity. The Python loader checks digests, join keys and choice-set shape.

| Table | Unit and missingness |
|---|---|
| `choices.csv` | Record × round × chooser × eligible candidate, plus `__outside__`; one-based shown position, empty outside position; `chosen` is 0/1 for valid decisions and empty for technical failures |
| `messages.csv` | Published communication only; UTF-8 text round-trips through standard CSV quoting |
| `events.csv` | Reciprocal pairs and whether they retired; future social events are not fabricated |
| `outcomes.csv` | Actual calls, alias, phase, call seed, status, finish reason, elapsed time, eligibility, normalized nomination and consent state |
| `profiles.csv` | Header only until profiles are implemented; the manifest explicitly says unavailable |

Matched-exit tables contain only calls that happened. A remaining singleton does
not receive an invented choice set. Zero nominations, missing measurements,
technical failures and voluntary outside choices remain distinct.

## Features and models

Scientific traits are supplied in a separate, versioned feature CSV, never inferred
from private notes. Choice features must have exactly one row per exported
`record_id,round,chooser,candidate`, including failures and the outside option.
OLS features use `record_id,round,candidate`. Include named numeric columns,
`source` and `visible_from_round`, the round when the underlying information first
became available to the chooser. External ratings may measure that visible
information without publishing ratings to participants. Caller-supplied provenance
must be reviewed: a column declaration does not establish actual visibility.

Joins reject missing/extra/duplicate keys, nonfinite data, post-decision visibility
and unidentified designs. Standardization, missing-profile rules, interactions,
quadratic terms and outside-option coding must be explicitly constructed in the
feature file and frozen for the intended study. No automatic imputation occurs.
For example, a chooser's loyalty alone is constant within a choice set and cannot
be estimated in conditional logit; `loyalty × candidate reputation` can be.

```sh
uv run --project analysis python analysis/analyze.py choice --exports runs/demo-tables --features choice-features.csv --columns looks skill reputation loyalty_reputation outside --output choice-fit.json
uv run --project analysis python analysis/analyze.py gradients --exports runs/demo-tables --features candidate-features.csv --columns looks skill reputation --output gradients.json
```

Use multiple independent seeds: both fits require at least two seed clusters.
`ConditionalLogit` has no intercept; it retains the outside option and conditions
on exactly one choice per set. Technical failures stop fitting by default; explicit
`--failure-policy exclude_reported` excludes entire failed sets while retaining
their counts in the output. This is not a solution to informative missingness.
Structurally uninformative singleton sets are reported separately.

OLS regresses each candidate's nomination count divided by the round's population
mean on supplied traits, retaining unchosen candidates. This is a **relative
nomination proxy**, not offspring count or genetic fitness. It refuses incomplete
decisions and rounds with zero nominations, where the ratio is undefined. An
actual study needs a frozen zero-outcome policy; it must not silently drop them.

Both implementations cluster scores/residuals by **run seed**, including records
sharing a seed, and report normal sandwich intervals (99% by default). At least
two clusters permits computation, not reliable inference. The fixed fixture uses
eight; real-study cluster counts, small-sample corrections or run-level bootstrap,
coverage, selection bias and multiplicity require pilot validation before G2.

## Planted recovery

`recovery-spec.json` declares eight seeds, 40 rounds, eight participants, coefficients,
99% intervals and error limits before execution. All ballots traverse the real
Rust runner, operator record, replay-checked exporter and CSV reader. Features are
explicit fixture metadata; they are not observations supplied to a model.

The conditional-logit generator uses Gaussian looks/skill/reputation, an independent
chooser loyalty and their reputation interaction, plus a distinct outside utility.
OLS uses a separate linear-probability generator: peer weight is
`1 + beta * (trait_j - mean_of_eligible_traits)`, divided by `n-1`. Centered uniform
traits keep every probability positive. Its expected relative nomination count is
`1 + beta * [n(n-2)/(n-1)^2 * trait_j]`; that adjusted trait is the planted OLS design.
Nonlinear softmax coefficients are not misidentified as linear selection gradients.

Passing requires every planted coefficient inside its interval, absolute error
within the fixed limits, finite identified converged fits and no technical failures.
The report contains estimates, intervals and outcome counts. A single fixed
scenario per estimator checks implementation; it is **not interval-coverage
calibration**. Preserve failures and never tune seeds or thresholds after a result.
G2 additionally needs real-server G1, the social layer and a separate exploratory
pilot before freezing or registration.

Primary API references: [ConditionalLogit](https://www.statsmodels.org/stable/generated/statsmodels.discrete.conditional_models.ConditionalLogit.html)
and [OLS fitting](https://www.statsmodels.org/stable/generated/statsmodels.regression.linear_model.OLS.fit.html).
