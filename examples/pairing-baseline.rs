//! Exact small-pool reference, not the live protocol or a model simulation.
//! Independent uniform non-self nominations, no abstention/failures, fresh draws.
use serde_json::json;

fn choose(n: usize, k: usize) -> f64 {
    (0..k).fold(1.0, |v, i| v * (n - i) as f64 / (i + 1) as f64)
}

fn distribution(n: usize) -> Vec<f64> {
    assert!(n <= 8);
    if n < 2 {
        return vec![1.0];
    }
    // a_r = E[binomial(K,r)] = number of r-disjoint matchings / (n-1)^(2r).
    let mut moments = vec![1.0; n / 2 + 1];
    for r in 1..moments.len() {
        moments[r] = moments[r - 1] * ((n - 2 * r + 2) * (n - 2 * r + 1)) as f64
            / (2 * r * (n - 1).pow(2)) as f64;
    }
    (0..moments.len())
        .map(|k| {
            (k..moments.len())
                .map(|r| {
                    let sign = if (r - k) % 2 == 0 { 1.0 } else { -1.0 };
                    sign * choose(r, k) * moments[r]
                })
                .sum()
        })
        .collect()
}

fn matched_exit(n: usize, rounds: usize) -> Vec<f64> {
    let mut unmatched = vec![0.0; n + 1];
    unmatched[n] = 1.0;
    for _ in 0..rounds {
        let mut next = vec![0.0; n + 1];
        for (remaining, mass) in unmatched.iter().copied().enumerate() {
            if mass == 0.0 {
                continue;
            }
            for (pairs, probability) in distribution(remaining).into_iter().enumerate() {
                next[remaining - 2 * pairs] += mass * probability;
            }
        }
        unmatched = next;
    }
    (0..=n / 2).map(|pairs| unmatched[n - 2 * pairs]).collect()
}

fn summary(probabilities: &[f64]) -> serde_json::Value {
    json!({
        "expected_pairs": probabilities.iter().enumerate().map(|(k,p)| k as f64 * p).sum::<f64>(),
        "probability_at_least_two": probabilities.iter().skip(2).sum::<f64>(),
        "probability_by_pair_count": probabilities,
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&json!({
        "population": 8,
        "assumptions": "Independent uniform non-self nominations; no abstention or failures; fresh draws each round. Matched-exit permanently removes both members from further matching within a seed. Social pairing does not imply reproduction consent.",
        "single_round": summary(&distribution(8)),
        "three_round_matched_exit": summary(&matched_exit(8, 3)),
    })).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_moments_match_full_nomination_enumeration() {
        for n in 2..=5usize {
            let total = (n - 1).pow(n as u32);
            let mut counts = vec![0usize; n / 2 + 1];
            for mut code in 0..total {
                let mut nominations = vec![0; n];
                for (owner, target) in nominations.iter_mut().enumerate() {
                    let choice = code % (n - 1);
                    code /= n - 1;
                    *target = choice + usize::from(choice >= owner);
                }
                let pairs = (0..n)
                    .filter(|&owner| {
                        owner < nominations[owner] && nominations[nominations[owner]] == owner
                    })
                    .count();
                counts[pairs] += 1;
            }
            for (predicted, observed) in distribution(n).into_iter().zip(counts) {
                assert!((predicted - observed as f64 / total as f64).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn retirement_counts_each_participant_once_and_probability_is_normalized() {
        assert_eq!(matched_exit(2, 3), vec![0.0, 1.0]);
        assert_eq!(matched_exit(1, 3), vec![1.0]);
        let single = distribution(8);
        assert!(
            (single
                .iter()
                .enumerate()
                .map(|(k, p)| k as f64 * p)
                .sum::<f64>()
                - 4.0 / 7.0)
                .abs()
                < 1e-12
        );
        for rounds in 0..=3 {
            let result = matched_exit(8, rounds);
            assert!(result.iter().all(|p| *p >= 0.0 && *p <= 1.0));
            assert!((result.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        }
    }
}
