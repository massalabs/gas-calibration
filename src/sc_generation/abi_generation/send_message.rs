use massa_models::config::THREAD_COUNT;
use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_send_message(address_sc: &str, rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.sendMessage(\"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, toBytes(\"{}\"), \"\", new StaticArray<u8>(0));",
        address_sc,
        generate_string(rng.gen_range(5..100)),
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(1_000..10_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000..300_000),
        rng.gen_range(1..3),
        rng.gen_range(100_000_000..1_000_000_000),
        generate_string(rng.gen_range(0..1000))
    ));
}
