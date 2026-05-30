use rand::{RngCore, SeedableRng, rngs::SmallRng};

pub fn make_seed(seed: Option<u64>) -> u64 {
    match seed {
        Some(s) => s,
        None => SmallRng::from_os_rng().next_u64(),
    }
}
