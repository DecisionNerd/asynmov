use pyo3::prelude::*;
use asynmov_core::{generators::entities, rng};

#[pyfunction]
#[pyo3(signature = (seed=None))]
fn make_seed(seed: Option<u64>) -> u64 {
    rng::make_seed(seed)
}

#[pyfunction]
fn generate_entities(seed: u64, scale: u64, specs_json: &str) -> PyResult<String> {
    let specs: Vec<entities::AttrSpec> = serde_json::from_str(specs_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    serde_json::to_string(&entities::generate(seed, scale, &specs))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_seed, m)?)?;
    m.add_function(wrap_pyfunction!(generate_entities, m)?)?;
    Ok(())
}
