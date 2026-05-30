#![deny(clippy::all)]

use napi_derive::napi;
use asynmov_core::{
    config::WorldConfig,
    generators::entities::Column,
    rng, generate,
};

#[napi]
pub fn make_seed(seed: Option<u32>) -> u32 {
    rng::make_seed(seed.map(|s| s as u64)) as u32
}

/// Parse a TOML config string and validate it. Throws on error.
#[napi]
pub fn validate_config(toml_str: String) -> napi::Result<()> {
    WorldConfig::from_toml(&toml_str)
        .map(|_| ())
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Parse a TOML config string and generate `scale` entities.
/// Returns a JSON string (array of row objects).
#[napi]
pub fn generate_from_toml(toml_str: String, scale: u32) -> napi::Result<String> {
    let cfg = WorldConfig::from_toml(&toml_str)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let seed = rng::make_seed(cfg.world.seed);
    let data = generate(seed, scale as u64, &cfg.attributes);

    let n = data.ids.len();
    let mut rows: Vec<serde_json::Map<String, serde_json::Value>> =
        (0..n).map(|_| serde_json::Map::new()).collect();

    for (name, col) in &data.columns {
        match col {
            Column::Int(v)   => v.iter().enumerate().for_each(|(i, &val)| { rows[i].insert(name.clone(), val.into()); }),
            Column::Float(v) => v.iter().enumerate().for_each(|(i, &val)| { rows[i].insert(name.clone(), val.into()); }),
            Column::Bool(v)  => v.iter().enumerate().for_each(|(i, &val)| { rows[i].insert(name.clone(), val.into()); }),
            Column::Utf8(v)  => v.iter().enumerate().for_each(|(i, val)| { rows[i].insert(name.clone(), val.clone().into()); }),
        }
    }

    serde_json::to_string(&rows)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
