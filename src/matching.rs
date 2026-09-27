use crate::protocol::{Outcome, Pair, Status};
use std::collections::BTreeMap;

pub fn reciprocal_pairs(outcomes: &BTreeMap<String, Outcome>) -> Vec<Pair> {
    outcomes
        .iter()
        .filter_map(|(a, outcome)| {
            if outcome.status != Status::Valid {
                return None;
            }
            let b = outcome.ballot.as_ref()?.partner.as_ref()?;
            if a >= b {
                return None;
            }
            let other = outcomes.get(b)?;
            if other.status == Status::Valid && other.ballot.as_ref()?.partner.as_ref() == Some(a) {
                Some(Pair {
                    agents: [a.clone(), b.clone()],
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{Ballot, Consent};

    #[test]
    fn reciprocal_pairs_are_exclusive_and_order_independent() {
        let handles = ["a", "b", "c", "d"];
        for code in 0..256 {
            let mut value = code;
            let outcomes: BTreeMap<_, _> = handles
                .iter()
                .map(|a| {
                    let choice = value % 4;
                    value /= 4;
                    let options: Vec<_> = handles.iter().filter(|b| *b != a).copied().collect();
                    let partner = options.get(choice).map(|s| s.to_string());
                    (
                        (*a).into(),
                        Outcome {
                            status: if partner.is_some() {
                                Status::Valid
                            } else {
                                Status::Abstained
                            },
                            public_message: None,
                            private_update: None,
                            ballot: Some(Ballot {
                                partner,
                                consent: Consent::Missing,
                            }),
                        },
                    )
                })
                .collect();
            let pairs = reciprocal_pairs(&outcomes);
            let used: std::collections::BTreeSet<_> =
                pairs.iter().flat_map(|p| p.agents.iter()).collect();
            assert_eq!(used.len(), pairs.len() * 2);
            for pair in &pairs {
                assert_eq!(
                    outcomes[&pair.agents[0]]
                        .ballot
                        .as_ref()
                        .unwrap()
                        .partner
                        .as_ref(),
                    Some(&pair.agents[1])
                );
                assert_eq!(
                    outcomes[&pair.agents[1]]
                        .ballot
                        .as_ref()
                        .unwrap()
                        .partner
                        .as_ref(),
                    Some(&pair.agents[0])
                );
            }
            let reversed = outcomes
                .iter()
                .rev()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            assert_eq!(pairs, reciprocal_pairs(&reversed));
        }
        let cycle: [(&str, &str); 3] = [("a", "b"), ("b", "c"), ("c", "a")];
        let outcomes = cycle
            .into_iter()
            .map(|(a, b)| {
                (
                    a.into(),
                    Outcome {
                        status: Status::Valid,
                        public_message: None,
                        private_update: None,
                        ballot: Some(Ballot {
                            partner: Some(b.into()),
                            consent: Consent::Decline,
                        }),
                    },
                )
            })
            .collect();
        assert!(reciprocal_pairs(&outcomes).is_empty());
    }
}
