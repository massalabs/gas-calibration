use rand::{rngs::ThreadRng, Rng};

use super::generate_address;

pub fn generate_abi_transfer_coins(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.transferCoins(\"{}\", {});",
        generate_address(),
        rng.gen_range(100_000_000..1_000_000_000)
    ));
}
