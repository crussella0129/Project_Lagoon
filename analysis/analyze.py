"""Audited choice/relative-nomination models; this is not a frozen study analysis."""

import argparse
import hashlib
import json
import warnings
from pathlib import Path

import numpy as np
import pandas as pd
import statsmodels.api as sm
from scipy.stats import norm
from statsmodels.discrete.conditional_models import ConditionalLogit
from statsmodels.tools.sm_exceptions import ConvergenceWarning

SET_KEYS = ["record_id", "round", "chooser"]
ROW_KEYS = SET_KEYS + ["candidate"]
VALID = {"valid", "abstained"}
FAILURES = {
    "invalid_response",
    "invalid_target",
    "oversized_response",
    "context_limit",
    "timeout",
    "transport_failure",
    "http_failure",
    "malformed_content",
    "tokenization_failure",
    "generation_limit",
}


def csv(path):
    return pd.read_csv(path, dtype=str, keep_default_na=False)


def rounds(values):
    value = pd.to_numeric(values, errors="raise")
    if not np.isfinite(value).all() or (value < 0).any() or (value % 1 != 0).any():
        raise ValueError("round values must be finite nonnegative integers")
    return value.astype(int)


def load_choices(paths):
    tables, seen = [], set()
    for path in paths:
        path = Path(path)
        manifest = json.loads((path / "manifest.json").read_text())
        if manifest["export_version"] != 1 or manifest["record_id"] in seen:
            raise ValueError("unsupported export or duplicate record")
        seen.add(manifest["record_id"])
        for name in (
            "choices.csv",
            "messages.csv",
            "events.csv",
            "outcomes.csv",
            "profiles.csv",
        ):
            if (
                hashlib.sha256((path / name).read_bytes()).hexdigest()
                != manifest["sha256"][name]
            ):
                raise ValueError("table digest mismatch")
        table = csv(path / "choices.csv")
        if len(table) != manifest["choice_rows"]:
            raise ValueError("choice row count differs from manifest")
        if not table.record_id.eq(manifest["record_id"]).all():
            raise ValueError("record join mismatch")
        if not table.run_seed.eq(str(manifest["run_seed"])).all():
            raise ValueError("run seed mismatch")
        if table[ROW_KEYS].duplicated().any():
            raise ValueError("duplicate choice row")
        if not table.status.isin(VALID | FAILURES).all():
            raise ValueError("unknown status")
        groups = table.groupby(SET_KEYS, sort=False)
        if len(groups) != manifest["choice_sets"]:
            raise ValueError("missing choice sets")
        for _, group in groups:
            outside = group.candidate.eq("__outside__")
            eligible = len(group) - 1
            if (
                outside.sum() != 1
                or not group.is_outside.eq(outside.astype(int).astype(str)).all()
                or not group.eligible_peers.eq(str(eligible)).all()
                or group.status.nunique() != 1
                or group.run_seed.nunique() != 1
                or group.loc[outside, "shown_position"].iloc[0] != ""
                or sorted(group.loc[~outside, "shown_position"].astype(int))
                != list(range(1, eligible + 1))
                or group.loc[~outside, "candidate"].eq(group.chooser.iloc[0]).any()
            ):
                raise ValueError("incomplete or inconsistent choice set")
            state = group.status.iloc[0]
            if state in VALID:
                if (
                    not group.chosen.isin(["0", "1"]).all()
                    or group.chosen.eq("1").sum() != 1
                ):
                    raise ValueError("a valid choice needs exactly one chosen row")
                if (group.loc[outside, "chosen"].iloc[0] == "1") != (
                    state == "abstained"
                ):
                    raise ValueError("abstention/outside mismatch")
            elif not group.chosen.eq("").all():
                raise ValueError("technical failures must have missing choice flags")
        tables.append(table)
    if not tables:
        raise ValueError("no exports supplied")
    result = pd.concat(tables, ignore_index=True)
    result["round"] = rounds(result["round"])
    return result


def audit(choices):
    sets = choices.drop_duplicates(SET_KEYS)
    return {
        "records": int(choices.record_id.nunique()),
        "seed_clusters": int(choices.run_seed.nunique()),
        "choice_sets": len(sets),
        "status_counts": {
            str(k): int(v) for k, v in sets.status.value_counts().items()
        },
        "failed_sets": int((~sets.status.isin(VALID)).sum()),
        "abstained_sets": int(sets.status.eq("abstained").sum()),
    }


def join_features(rows, features, keys, columns):
    if not columns or len(set(columns)) != len(columns):
        raise ValueError("feature columns must be nonempty and unique")
    if set(columns) & set(rows.columns) or set(columns) & {
        "source",
        "visible_from_round",
        "const",
    }:
        raise ValueError("feature names collide with audit columns")
    features = features.copy()
    features["round"] = rounds(features["round"])
    if features[keys].duplicated().any():
        raise ValueError("duplicate feature key")
    features = features[keys + columns + ["visible_from_round", "source"]]
    joined = rows.merge(
        features, on=keys, how="outer", validate="one_to_one", indicator=True
    )
    if not joined["_merge"].eq("both").all():
        raise ValueError("feature join must cover exactly the exported rows")
    visible = rounds(joined.visible_from_round)
    if (
        not np.isfinite(visible).all()
        or (visible < 0).any()
        or (visible > joined["round"]).any()
    ):
        raise ValueError("feature visibility occurs after the decision")
    if joined.source.isna().any() or joined.source.eq("").any():
        raise ValueError("feature source is required")
    for name in columns:
        joined[name] = pd.to_numeric(joined[name], errors="raise")
    if not np.isfinite(joined[columns].to_numpy(dtype=float)).all():
        raise ValueError("nonfinite feature")
    return joined.drop(columns="_merge")


def estimates(params, covariance, names, alpha):
    if not 0 < alpha < 1:
        raise ValueError("alpha must be in (0, 1)")
    params, covariance = np.asarray(params), np.asarray(covariance)
    diagonal = np.diag(covariance)
    if (
        not np.isfinite(params).all()
        or not np.isfinite(covariance).all()
        or (diagonal <= 0).any()
    ):
        raise ValueError("nonfinite fit or nonpositive uncertainty")
    se = np.sqrt(diagonal)
    margin = norm.ppf(1 - alpha / 2) * se
    return {
        name: {
            "estimate": float(p),
            "se": float(s),
            "lower": float(p - m),
            "upper": float(p + m),
        }
        for name, p, s, m in zip(names, params, se, margin, strict=True)
    }


def fit_choice(choices, features, columns, alpha=0.01, failure_policy="error"):
    info = audit(choices)
    if failure_policy not in ("error", "exclude_reported"):
        raise ValueError("unknown failure policy")
    if info["failed_sets"] and failure_policy == "error":
        raise ValueError("technical failures require an explicit exclusion policy")
    # Join before filtering: missing failures/abstentions cannot disappear silently.
    table = join_features(choices, features, ROW_KEYS, columns)
    table = table.loc[table.status.isin(VALID)].copy()
    sizes = table.groupby(SET_KEYS).candidate.transform("size")
    info["uninformative_singleton_sets"] = int((sizes == 1).sum())
    table = table.loc[sizes > 1].reset_index(drop=True)
    codes = pd.factorize(pd.MultiIndex.from_frame(table[SET_KEYS]), sort=False)[0]
    x = table[columns].astype(float)
    centered = x - x.groupby(codes).transform("mean")
    if len(table) == 0 or np.linalg.matrix_rank(centered.to_numpy()) != len(columns):
        raise ValueError("choice design is not identified within sets")
    model = ConditionalLogit(table.chosen.astype(int), x, groups=codes, missing="raise")
    with warnings.catch_warnings():
        warnings.simplefilter("error", ConvergenceWarning)
        # ConditionalLogit 0.15's wrapper does not forward optimizer kwargs.
        # Newton's supported defaults meet the independently checked score bound.
        fitted = model.fit(method="newton", maxiter=100, disp=False)
    if np.max(np.abs(model.score(fitted.params))) / len(np.unique(codes)) > 1e-5:
        raise ValueError("choice fit did not converge")
    # Sum choice-set scores within a complete run before forming the sandwich.
    group_records = (
        table.assign(group=codes).groupby("group", sort=True).run_seed.first()
    )
    records = group_records.unique()
    if len(records) < 2:
        raise ValueError(
            "at least two independent runs required for cluster covariance"
        )
    scores = np.asarray(
        [model.score_grp(i, fitted.params) for i in range(len(group_records))]
    )
    clustered = np.asarray(
        [scores[group_records.to_numpy() == rid].sum(axis=0) for rid in records]
    )
    bread = np.linalg.inv(-model.hessian(fitted.params))
    covariance = (
        bread @ (clustered.T @ clustered) @ bread * len(records) / (len(records) - 1)
    )
    return {
        "model": "conditional_logit",
        "audit": info,
        "failure_policy": failure_policy,
        "covariance": "run_seed_cluster_sandwich_normal",
        "alpha": alpha,
        "coefficients": estimates(fitted.params, covariance, columns, alpha),
    }


def nomination_outcomes(choices):
    if audit(choices)["failed_sets"]:
        raise ValueError("relative-nomination analysis refuses incomplete decisions")
    peers = choices.loc[choices.candidate.ne("__outside__")].copy()
    peers["chosen"] = peers.chosen.astype(int)
    keys = ["record_id", "round", "candidate"]
    result = (
        peers.groupby(keys, as_index=False)
        .chosen.sum()
        .rename(columns={"chosen": "nominations"})
    )
    means = result.groupby(["record_id", "round"]).nominations.transform("mean")
    if len(result) == 0 or (means <= 0).any():
        raise ValueError(
            "relative nominations are undefined in a zero-nomination round"
        )
    result["relative_nominations"] = result.nominations / means
    result = result.merge(
        choices[["record_id", "run_seed"]].drop_duplicates(),
        on="record_id",
        validate="many_to_one",
    )
    return result


def fit_gradients(choices, features, columns, alpha=0.01):
    keys = ["record_id", "round", "candidate"]
    table = join_features(nomination_outcomes(choices), features, keys, columns)
    x = sm.add_constant(table[columns].astype(float), has_constant="add")
    if np.linalg.matrix_rank(x.to_numpy()) != len(x.columns):
        raise ValueError("OLS design is rank deficient")
    if table.run_seed.nunique() < 2:
        raise ValueError(
            "at least two independent runs required for cluster covariance"
        )
    fitted = sm.OLS(table.relative_nominations, x, missing="raise").fit(
        cov_type="cluster", cov_kwds={"groups": table.run_seed}, use_t=False
    )
    return {
        "model": "ols_relative_nominations",
        "audit": audit(choices),
        "covariance": "run_seed_cluster_sandwich_normal",
        "alpha": alpha,
        "coefficients": estimates(fitted.params, fitted.cov_params(), x.columns, alpha),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("model", choices=["choice", "gradients"])
    parser.add_argument("--exports", type=Path, nargs="+", required=True)
    parser.add_argument("--features", type=Path, required=True)
    parser.add_argument("--columns", nargs="+", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--alpha", type=float, default=0.01)
    parser.add_argument(
        "--failure-policy", choices=["error", "exclude_reported"], default="error"
    )
    args = parser.parse_args()
    choices, features = load_choices(args.exports), csv(args.features)
    if args.model == "choice":
        result = fit_choice(
            choices, features, args.columns, args.alpha, args.failure_policy
        )
    else:
        if args.failure_policy != "error":
            parser.error("gradient analysis requires complete decisions")
        result = fit_gradients(choices, features, args.columns, args.alpha)
    result["study_status"] = "exploratory_instrument; not preregistered"
    with args.output.open("x", encoding="utf-8") as stream:
        json.dump(result, stream, indent=2, allow_nan=False)


if __name__ == "__main__":
    main()
