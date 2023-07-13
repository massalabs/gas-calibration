use rand::{rngs::ThreadRng, Rng};

use super::generate_address;

pub fn generate_abi_transfer_coins(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.transfer_coins(\"{}\", env.make_native_amount({}, 9), null);",
        generate_address(),
        rng.gen_range(1_u64..100_000_000_000)
    ));
}
