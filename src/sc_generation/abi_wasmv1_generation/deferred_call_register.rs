use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::generate_string;

pub fn generate_abi_deferred_call_register(
    address_sc: &str,
    calls: &mut Vec<String>,
) {
    let mut rng = rand::thread_rng();

    calls.push(format!(
        "env.deferred_call_register(\"{}\", \"{}\", {}, {}, {} ,toBytes(\"{}\"), {});",
        address_sc,
        generate_string(rng.gen_range(5..25)),
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000_000..1_000_000_000),
        rng.gen_range(0..2000),
        rng.gen_range(0..1000)
    ));
}
