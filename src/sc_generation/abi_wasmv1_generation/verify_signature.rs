use rand::{rngs::ThreadRng, Rng};

use super::{generate_string, static_public_key};

pub fn generate_abi_verify_signature(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.verify_signature(\"{}\", toBytes(\"{}\"), \"{}\");",
        generate_string(rng.gen_range(5..1000)),
        generate_string(rng.gen_range(5..1000)),
        static_public_key(),
    ));
}