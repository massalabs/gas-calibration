use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_generate_event(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.generateEvent(\"{}\");",
        generate_string(rng.gen_range(0..255))
    ));
}
