use rand::Rng;

use super::generate_string;

pub fn generate_abi_hash_blake3(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.hash_blake3(toBytes(\"{}\"));",
        generate_string(rng.gen_range(5..1000)),
    ));
}
