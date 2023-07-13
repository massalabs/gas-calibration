use rand::Rng;

use super::generate_string;

pub fn generate_abi_blake3_hash(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.blake3_hash(toBytes(\"{}\"));",
        generate_string(rng.gen_range(5..1000)),
    ));
}
