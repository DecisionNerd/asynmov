use rand::{Rng, SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Normal, Uniform};
use rayon::prelude::*;
use serde::Deserialize;

/// A heterogeneous value that can appear as a `choice` option in TOML.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChoiceValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

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
        values: Vec<ChoiceValue>,
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

/// Typed columnar output — one variant per wire type.
#[derive(Debug)]
pub enum Column {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Bool(Vec<bool>),
    Utf8(Vec<String>),
}

/// Columnar result: one Column per AttrSpec plus a leading "id" Int column.
pub struct GeneratedData {
    pub ids: Vec<u64>,
    /// (column_name, column_data), id column is index 0.
    pub columns: Vec<(String, Column)>,
}

// ── Sampling helpers ──────────────────────────────────────────────────────────

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

// ── Per-column typed generation ───────────────────────────────────────────────

/// Determine the output Column variant for a given spec and generate all values.
fn generate_column(seed: u64, scale: u64, col_idx: usize, spec: &AttrSpec) -> Column {
    match &spec.dist {
        AttrDist::UniformInt { low, high } => {
            Column::Int((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                // advance past earlier columns
                advance_rng(&mut rng, col_idx);
                rng.random_range(*low..=*high)
            }).collect())
        }
        AttrDist::UniformFloat { low, high } => {
            Column::Float((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                Uniform::new(*low, *high).unwrap().sample(&mut rng)
            }).collect())
        }
        AttrDist::Normal { mean, std_dev, min, max } => {
            Column::Float((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                let mut v = Normal::new(*mean, *std_dev).unwrap().sample(&mut rng);
                if let Some(mn) = min { v = v.max(*mn); }
                if let Some(mx) = max { v = v.min(*mx); }
                v
            }).collect())
        }
        AttrDist::Choice { values, weights } => {
            choice_column(seed, scale, col_idx, values, weights)
        }
    }
}

/// Choice columns are heterogeneous — dispatch to the dominant type.
fn choice_column(seed: u64, scale: u64, col_idx: usize, values: &[ChoiceValue], weights: &[f64]) -> Column {
    if values.is_empty() {
        return Column::Utf8(vec![String::new(); scale as usize]);
    }

    // Determine output type from the first value.
    match &values[0] {
        ChoiceValue::Bool(_) => {
            let bools: Vec<bool> = values.iter().map(|v| matches!(v, ChoiceValue::Bool(true))).collect();
            Column::Bool((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                let idx = pick(& mut rng, values.len(), weights);
                bools[idx]
            }).collect())
        }
        ChoiceValue::Int(_) => {
            let ints: Vec<i64> = values.iter().map(|v| match v {
                ChoiceValue::Int(i) => *i,
                ChoiceValue::Float(f) => *f as i64,
                _ => 0,
            }).collect();
            Column::Int((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                ints[pick(&mut rng, values.len(), weights)]
            }).collect())
        }
        ChoiceValue::Float(_) => {
            let floats: Vec<f64> = values.iter().map(|v| match v {
                ChoiceValue::Float(f) => *f,
                ChoiceValue::Int(i) => *i as f64,
                _ => 0.0,
            }).collect();
            Column::Float((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                floats[pick(&mut rng, values.len(), weights)]
            }).collect())
        }
        ChoiceValue::Str(_) => {
            let strs: Vec<String> = values.iter().map(|v| match v {
                ChoiceValue::Str(s) => s.clone(),
                other => format!("{other:?}"),
            }).collect();
            Column::Utf8((0..scale).map(|id| {
                let mut rng = entity_rng(seed, id);
                advance_rng(&mut rng, col_idx);
                strs[pick(&mut rng, values.len(), weights)].clone()
            }).collect())
        }
    }
}

#[inline]
fn entity_rng(seed: u64, id: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed.wrapping_add(id).wrapping_mul(6364136223846793005))
}

/// Fast-forward an RNG past `n` columns worth of state so each column gets
/// independent draws without needing row-major allocation.
#[inline]
fn advance_rng(rng: &mut SmallRng, n: usize) {
    for _ in 0..n {
        let _: u64 = rng.random();
    }
}

#[inline]
fn pick(rng: &mut SmallRng, len: usize, weights: &[f64]) -> usize {
    if weights.is_empty() {
        rng.random_range(0..len)
    } else {
        weighted_index(rng, weights)
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Generate `scale` entities in parallel, returning columnar data.
pub fn generate(seed: u64, scale: u64, specs: &[AttrSpec]) -> GeneratedData {
    let mut columns: Vec<(String, Column)> = specs
        .par_iter()
        .enumerate()
        .map(|(col_idx, spec)| {
            (spec.name.clone(), generate_column(seed, scale, col_idx, spec))
        })
        .collect();

    // Stable column order matches spec order.
    columns.sort_by_key(|(name, _)| {
        specs.iter().position(|s| &s.name == name).unwrap_or(usize::MAX)
    });

    let ids: Vec<u64> = (0..scale).collect();
    columns.insert(0, ("id".to_string(), Column::Int(ids.iter().map(|&id| id as i64).collect())));

    GeneratedData { ids, columns }
}
