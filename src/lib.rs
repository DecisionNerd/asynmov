use pyo3::prelude::*;

mod rng;

/// Asynmov Rust core extension module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rng::make_seed, m)?)?;
    Ok(())
}
