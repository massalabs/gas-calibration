use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_set_of(address_sc: &str, rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.setOf(\"{}\", toBytes(\"{}\"), toBytes(\"{}\"));",
        address_sc,
        generate_string(rng.gen_range(5..32)),
        generate_string(rng.gen_range(1..1000))
    ));
}
