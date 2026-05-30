use std::collections::HashMap;

use rand::{Rng, SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Normal, Uniform};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AttrDist {
    UniformInt { low: i64, high: i64 },
    UniformFloat { low: f64, high: f64 },
    Normal {
        mean: f64,
        std_dev: f64,
        min: Option<f64>,
        max: Option<f64>,
    },
    Choice {
        values: Vec<Value>,
        #[serde(default)]
        weights: Vec<f64>,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttrSpec {
    pub name: String,
    #[serde(flatten)]
    pub dist: AttrDist,
}

#[derive(Debug, Serialize)]
pub struct Entity {
    pub id: u64,
    #[serde(flatten)]
    pub attrs: HashMap<String, Value>,
}

fn sample_attr(rng: &mut SmallRng, dist: &AttrDist) -> Value {
    match dist {
        AttrDist::UniformInt { low, high } => Value::from(rng.random_range(*low..=*high)),
        AttrDist::UniformFloat { low, high } => {
            Value::from(Uniform::new(*low, *high).unwrap().sample(rng))
        }
        AttrDist::Normal { mean, std_dev, min, max } => {
            let mut v = Normal::new(*mean, *std_dev).unwrap().sample(rng);
            if let Some(mn) = min { v = v.max(*mn); }
            if let Some(mx) = max { v = v.min(*mx); }
            Value::from(v)
        }
        AttrDist::Choice { values, weights } => {
            if values.is_empty() {
                return Value::Null;
            }
            let idx = if weights.is_empty() {
                rng.random_range(0..values.len())
            } else {
                weighted_index(rng, weights)
            };
            values[idx].clone()
        }
    }
}

fn weighted_index(rng: &mut SmallRng, weights: &[f64]) -> usize {
    let total: f64 = weights.iter().sum();
    let mut r = rng.random::<f64>() * total;
    for (i, w) in weights.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return i;
        }
    }
    weights.len() - 1
}

pub fn generate(seed: u64, scale: u64, specs: &[AttrSpec]) -> Vec<Entity> {
    (0..scale)
        .into_par_iter()
        .map(|id| {
            let entity_seed = seed.wrapping_add(id).wrapping_mul(6364136223846793005);
            let mut rng = SmallRng::seed_from_u64(entity_seed);
            let mut attrs = HashMap::with_capacity(specs.len());
            for spec in specs {
                attrs.insert(spec.name.clone(), sample_attr(&mut rng, &spec.dist));
            }
            Entity { id, attrs }
        })
        .collect()
}
