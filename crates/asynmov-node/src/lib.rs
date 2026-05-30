#![deny(clippy::all)]

use napi_derive::napi;
use asynmov_core::{generators::entities, rng};

#[napi]
pub fn make_seed(seed: Option<u32>) -> u32 {
    rng::make_seed(seed.map(|s| s as u64)) as u32
}

/// Generate `scale` entities using Monte Carlo sampling.
///
/// `specs_json` is a JSON array of attribute spec objects.
/// Returns a JSON string (array of entity objects).
#[napi]
pub fn generate_entities(seed: u32, scale: u32, specs_json: String) -> napi::Result<String> {
    let specs: Vec<entities::AttrSpec> = serde_json::from_str(&specs_json)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_string(&entities::generate(seed as u64, scale as u64, &specs))
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
