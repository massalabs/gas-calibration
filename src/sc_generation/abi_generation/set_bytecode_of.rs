use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_set_bytecode_of(
    address_sc: &str,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.setBytecodeOf(\"{}\", toBytes(\"{}\"));",
        address_sc,
        generate_string(rng.gen_range(5..10000))
    ));
}
