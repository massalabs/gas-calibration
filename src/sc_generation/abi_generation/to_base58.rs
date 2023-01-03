use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_to_base58(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.toBase58(\"{}\");",
        generate_string(rng.gen_range(5..1000)),
    ));
}
