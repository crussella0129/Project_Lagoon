//! A closed, neutral finite-population toy study, never a weight merger.
use clap::Parser;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;

#[derive(Clone, Debug, Parser, Serialize)]
struct Settings {
    #[arg(long, default_value_t = 9)]
    seed: u64,
    #[arg(long, default_value_t = 64)]
    population: usize,
    #[arg(long, default_value_t = 4096)]
    coordinates: usize,
    #[arg(long, default_value_t = 6)]
    generations: usize,
    #[arg(long, default_value_t = 8)]
    replicates: usize,
}

impl Settings {
    fn valid(&self) -> bool {
        (2..=256).contains(&self.population)
            && (1..=65536).contains(&self.coordinates)
            && (1..=20).contains(&self.generations)
            && (1..=128).contains(&self.replicates)
            && self.population as u64
                * self.coordinates as u64
                * self.generations as u64
                * self.replicates as u64
                * 5
                <= 150_000_000
    }
}

#[derive(Clone, Copy)]
enum Method {
    Average { drop: f64 },
    Crossover { block: usize },
}

const METHODS: [(&str, Method); 5] = [
    ("linear", Method::Average { drop: 0.0 }),
    ("dare_average_p0.5", Method::Average { drop: 0.5 }),
    ("dare_average_p0.9", Method::Average { drop: 0.9 }),
    ("coordinate_crossover", Method::Crossover { block: 1 }),
    ("block64_crossover", Method::Crossover { block: 64 }),
];

/// Equal-mean, equal-variance parents, independent masks; R excludes a lambda scale.
fn iid_ratio(drop: f64, correlation: f64) -> f64 {
    1.0 / (2.0 * (1.0 - drop)) + correlation / 2.0
}

/// Exact conditional expectation with N independently sampled offspring and
/// two distinct uniformly sampled parents per child. V uses divisor N.
fn finite_next(n: usize, variance: f64, mean_squared: f64, method: Method) -> (f64, f64) {
    let n = n as f64;
    let child_variance = match method {
        Method::Average { drop } => {
            let q = 1.0 - drop;
            (1.0 / (2.0 * q) - 1.0 / (2.0 * (n - 1.0))) * variance + drop / (2.0 * q) * mean_squared
        }
        Method::Crossover { .. } => variance,
    };
    (
        (n - 1.0) / n * child_variance,
        mean_squared + child_variance / n,
    )
}

fn moments(pool: &[Vec<f64>]) -> (f64, f64) {
    let n = pool.len() as f64;
    let d = pool[0].len();
    let mut variance = 0.0;
    let mut mean_squared = 0.0;
    for coordinate in 0..d {
        let mean = pool.iter().map(|x| x[coordinate]).sum::<f64>() / n;
        variance += pool
            .iter()
            .map(|x| (x[coordinate] - mean).powi(2))
            .sum::<f64>()
            / n;
        mean_squared += mean * mean;
    }
    (variance / d as f64, mean_squared / d as f64)
}

fn generation(pool: &[Vec<f64>], method: Method, rng: &mut ChaCha8Rng) -> Vec<Vec<f64>> {
    (0..pool.len())
        .map(|_| {
            let a = rng.random_range(0..pool.len());
            let b0 = rng.random_range(0..pool.len() - 1);
            let b = if b0 >= a { b0 + 1 } else { b0 };
            let mut from_a = true;
            pool[a]
                .iter()
                .zip(&pool[b])
                .enumerate()
                .map(|(coordinate, (&x, &y))| match method {
                    Method::Average { drop } => {
                        let q = 1.0 - drop;
                        let ma = f64::from(rng.random_bool(q));
                        let mb = f64::from(rng.random_bool(q));
                        (ma * x + mb * y) / (2.0 * q)
                    }
                    Method::Crossover { block } => {
                        if coordinate % block == 0 {
                            from_a = rng.random_bool(0.5);
                        }
                        if from_a { x } else { y }
                    }
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Serialize, PartialEq)]
struct Row {
    generation: usize,
    population_variance: f64,
    variance_ratio_to_initial: f64,
    mean_squared: f64,
    second_moment: f64,
    expected_finite_variance_ratio: f64,
    expected_finite_mean_squared: f64,
}

#[derive(Debug, Serialize, PartialEq)]
struct Treatment {
    name: String,
    independent_zero_mean_one_generation_ratio: f64,
    rows: Vec<Row>,
}

fn study(settings: &Settings) -> Vec<Treatment> {
    let mut sums = vec![vec![(0.0, 0.0); settings.generations + 1]; METHODS.len()];
    for replicate in 0..settings.replicates {
        let mut initial_rng =
            ChaCha8Rng::seed_from_u64(settings.seed.wrapping_add(replicate as u64 * 104729));
        let initial: Vec<Vec<f64>> = (0..settings.population)
            .map(|_| {
                (0..settings.coordinates)
                    .map(|_| StandardNormal.sample(&mut initial_rng))
                    .collect()
            })
            .collect();
        for (index, (_, method)) in METHODS.iter().enumerate() {
            let mut rng = ChaCha8Rng::seed_from_u64(
                settings
                    .seed
                    .wrapping_add(1_000_003 * (index as u64 + 1))
                    .wrapping_add(replicate as u64 * 104729),
            );
            let mut pool = initial.clone();
            for (g, sum) in sums[index].iter_mut().enumerate() {
                let (v, m) = moments(&pool);
                sum.0 += v / settings.replicates as f64;
                sum.1 += m / settings.replicates as f64;
                if g < settings.generations {
                    pool = generation(&pool, *method, &mut rng);
                }
            }
        }
    }
    METHODS
        .iter()
        .enumerate()
        .map(|(index, (name, method))| {
            let initial_v = sums[index][0].0;
            let (mut expected_v, mut expected_m) = sums[index][0];
            let rows = sums[index]
                .iter()
                .enumerate()
                .map(|(g, &(v, m))| {
                    let row = Row {
                        generation: g,
                        population_variance: v,
                        variance_ratio_to_initial: v / initial_v,
                        mean_squared: m,
                        second_moment: v + m,
                        expected_finite_variance_ratio: expected_v / initial_v,
                        expected_finite_mean_squared: expected_m,
                    };
                    (expected_v, expected_m) =
                        finite_next(settings.population, expected_v, expected_m, *method);
                    row
                })
                .collect();
            Treatment {
                name: name.to_string(),
                independent_zero_mean_one_generation_ratio: match method {
                    Method::Average { drop } => iid_ratio(*drop, 0.0),
                    Method::Crossover { .. } => 1.0,
                },
                rows,
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    if !settings.valid() {
        return Err("invalid settings or work bound exceeded (150M coordinate operations)".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "settings":settings,"study":"neutral closed population; distinct parents; iid normal initial deltas; no fitness, mutation, TIES, or neural networks",
            "results":study(&settings)
        }))?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_mask_enumeration_includes_parent_correlation() {
        for drop in [0.0, 0.5, 0.9] {
            let q = 1.0 - drop;
            for correlation in [-1.0, 0.0, 1.0] {
                let mut second = 0.0;
                for x in [-1.0, 1.0] {
                    for y in [-1.0, 1.0] {
                        let parent_probability = (1.0 + correlation * x * y) / 4.0;
                        for ma in [0.0, 1.0] {
                            for mb in [0.0, 1.0] {
                                let mask_probability = (if ma == 1.0 { q } else { drop })
                                    * (if mb == 1.0 { q } else { drop });
                                let c = (ma * x + mb * y) / (2.0 * q);
                                second += parent_probability * mask_probability * c * c;
                            }
                        }
                    }
                }
                assert!((second - iid_ratio(drop, correlation)).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn finite_population_drift_and_nonzero_mean_are_explicit() {
        let (v, m) = finite_next(64, 1.0, 0.0, Method::Average { drop: 0.0 });
        assert_eq!(v, 31.0 / 64.0);
        assert!(m > 0.0);
        assert_eq!(
            finite_next(64, 1.0, 0.0, Method::Crossover { block: 1 }).0,
            63.0 / 64.0
        );
        assert_eq!(
            finite_next(2, 1.0, 0.0, Method::Average { drop: 0.0 }),
            (0.0, 0.0)
        );
        assert!(finite_next(64, 1.0, 1.0, Method::Average { drop: 0.5 }).0 > 1.0);
    }

    #[test]
    fn study_is_reproducible_bounded_and_finite() {
        let settings = Settings {
            seed: 9,
            population: 8,
            coordinates: 64,
            generations: 3,
            replicates: 2,
        };
        assert!(settings.valid());
        let results = study(&settings);
        assert_eq!(results, study(&settings));
        assert!(
            results
                .iter()
                .flat_map(|t| &t.rows)
                .all(|r| r.population_variance.is_finite() && r.mean_squared.is_finite())
        );
        assert!(
            !Settings {
                population: 1,
                ..settings.clone()
            }
            .valid()
        );
        assert!(
            !Settings {
                population: 256,
                coordinates: 65536,
                generations: 20,
                replicates: 128,
                ..settings
            }
            .valid()
        );
    }
}
