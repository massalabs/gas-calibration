use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_print(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.print(\"{}\");",
        generate_string(rng.gen_range(0..255))
    ));
}
