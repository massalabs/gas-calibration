use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::{generate_bytes, generate_string};

pub fn generate_abi_deferred_call_register(
    address_sc: &str,
    calls: &mut Vec<String>,
) {
    let mut rng = rand::thread_rng();

    let rng_key_bytes = generate_bytes(rng.gen_range(10..64));

    calls.push(format!(
        "{{let bytes = new Uint8Array({});
        bytes.set({:?});
        env.deferred_call_register(\"{}\", \"{}\", {}, {}, {} ,bytes, {});}}",
        rng_key_bytes.len(),
        rng_key_bytes,
        address_sc,
        generate_string(rng.gen_range(5..25)),
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000_000..1_000_000_000),
        rng.gen_range(0..1000)
    ));
}
