use rand::Rng;

use super::generate_string;

pub fn generate_abi_hash_sha256(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let data = generate_string(rng.gen_range(5..32));
    calls.push(format!("  env.hash_sha256(toBytes(\"{}\"));", data));
}
