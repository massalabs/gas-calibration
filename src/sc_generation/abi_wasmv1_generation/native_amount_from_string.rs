use rand::{rngs::ThreadRng, Rng};

use super::generate_native_amount_string;

pub fn generate_abi_native_amount_from_string(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.native_amount_from_string(\"{}\");",
        generate_native_amount_string(rng.gen_range(1..100), 0),
    ));
}