use rand::Rng;

use super::generate_native_amount_string;

pub fn generate_abi_native_amount_from_string(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.native_amount_from_string(\"{}\");",
        generate_native_amount_string(rng.gen_range(1_u64..10_000_000_000), 8),
    ));
}
