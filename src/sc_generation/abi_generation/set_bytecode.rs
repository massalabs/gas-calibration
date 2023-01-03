use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_set_bytecode(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.setBytecode(toBytes(\"{}\"));",
        generate_string(rng.gen_range(5..10000))
    ));
}
