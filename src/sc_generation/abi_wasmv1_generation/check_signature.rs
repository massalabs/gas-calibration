use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_check_signature(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.check_signature(\"{}\");",
        generate_string(rng.gen_range(5..1000)),
    ));
}
