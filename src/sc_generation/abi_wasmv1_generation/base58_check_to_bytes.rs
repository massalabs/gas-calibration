use rand::Rng;

use super::generate_b58_check_string;

pub fn generate_abi_base58_check_to_bytes(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.base58_check_to_bytes(\"{}\");",
        generate_b58_check_string(rng.gen_range(5..1000)),
    ));
}
