use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use asynmov_core::{
    config::WorldConfig,
    generators::entities::Column,
    rng, generate,
};

#[pyfunction]
#[pyo3(signature = (seed=None))]
fn make_seed(seed: Option<u64>) -> u64 {
    rng::make_seed(seed)
}

/// Parse and validate a TOML config string. Raises ValueError on failure.
#[pyfunction]
fn validate_config(toml_str: &str) -> PyResult<()> {
    WorldConfig::from_toml(toml_str)
        .map(|_| ())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Parse a TOML config string and generate `scale` entities, returning a polars DataFrame.
#[pyfunction]
fn generate_from_toml(toml_str: &str, scale: u64) -> PyResult<PyDataFrame> {
    let cfg = WorldConfig::from_toml(toml_str)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let seed = rng::make_seed(cfg.world.seed);
    let data = generate(seed, scale, &cfg.attributes);

    let columns: Vec<polars::prelude::Column> = data.columns.into_iter().map(|(name, col)| {
        match col {
            Column::Int(v)   => Series::new(name.into(), v).into(),
            Column::Float(v) => Series::new(name.into(), v).into(),
            Column::Bool(v)  => Series::new(name.into(), v).into(),
            Column::Utf8(v)  => Series::new(name.into(), v).into(),
        }
    }).collect();

    let df = DataFrame::new(columns)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    Ok(PyDataFrame(df))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_seed, m)?)?;
    m.add_function(wrap_pyfunction!(validate_config, m)?)?;
    m.add_function(wrap_pyfunction!(generate_from_toml, m)?)?;
    Ok(())
}
