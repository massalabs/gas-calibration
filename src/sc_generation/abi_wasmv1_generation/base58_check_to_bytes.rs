use rand::{rngs::ThreadRng, Rng};

use super::generate_b58_check_string;

pub fn generate_abi_base58_check_to_bytes(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.base58_check_to_bytes(\"{}\");",
        generate_b58_check_string(rng.gen_range(5..1000)),
    ));
}
