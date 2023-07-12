use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_bytes_to_base58_check(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.bytes_to_base58_check(toBytes(\"{}\"));",
        generate_string(rng.gen_range(5..1000)),
    ));
}
