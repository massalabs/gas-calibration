use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_blake3_hash(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.blake3_hash(toBytes(\"{}\"));",
        generate_string(rng.gen_range(5..1000)),
    ));
}
