use rand::Rng;

use super::generate_address;

pub fn generate_abi_transfer_coins(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.transfer_coins(\"{}\", env.make_native_amount({}, 8), null);",
        generate_address(),
        rng.gen_range(1_u64..100_000_000)
    ));
}
