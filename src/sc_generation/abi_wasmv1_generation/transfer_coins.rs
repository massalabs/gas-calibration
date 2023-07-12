use rand::{rngs::ThreadRng, Rng};

use super::generate_address;

pub fn generate_abi_transfer_coins(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.transfer_coins(\"{}\", env.make_native_amount({}, 0), null);",
        generate_address(),
        rng.gen_range(1..100)
    ));
}
