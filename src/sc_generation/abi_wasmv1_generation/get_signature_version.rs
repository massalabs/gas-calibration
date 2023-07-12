use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_get_signature_version(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.get_signature_version(\"{}\");",
        generate_string(rng.gen_range(5..1000)),
    ));
}
