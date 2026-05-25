use pyo3::prelude::*;

mod rng;
mod generators;

/// Generate `scale` entities using Monte Carlo sampling.
///
/// `specs_json` is a JSON array of attribute spec objects, e.g.:
///   [{"name":"age","type":"uniform_int","low":18,"high":80}, ...]
///
/// Returns a JSON string (array of entity objects).
#[pyfunction]
fn generate_entities(seed: u64, scale: u64, specs_json: &str) -> PyResult<String> {
    let specs: Vec<generators::entities::AttrSpec> = serde_json::from_str(specs_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let entities = generators::entities::generate(seed, scale, &specs);

    serde_json::to_string(&entities)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

/// Asynmov Rust core extension module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rng::make_seed, m)?)?;
    m.add_function(wrap_pyfunction!(generate_entities, m)?)?;
    Ok(())
}
