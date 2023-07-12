use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_hash_keccak256(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    let data = generate_string(rng.gen_range(5..32));
    calls.push(format!("  env.hash_keccak256(toBytes(\"{}\"));", data));
}
