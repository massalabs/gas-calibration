use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_blake3(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.blake3(\"{}\");",
        generate_string(rng.gen_range(5..1000)),
    ));
}
