use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_native_amount_from_string(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.native_amount_from_string(\"{}\");",
        generate_string(rng.gen_range(1..1000)),
    ));
}
