use pyo3::prelude::*;
use rand::{RngCore, SeedableRng, rngs::SmallRng};

/// Return a u64 seed derived from an optional user-supplied seed.
/// Passing None draws from the OS entropy source.
#[pyfunction]
pub fn make_seed(seed: Option<u64>) -> u64 {
    match seed {
        Some(s) => s,
        None => SmallRng::from_os_rng().next_u64(),
    }
}
