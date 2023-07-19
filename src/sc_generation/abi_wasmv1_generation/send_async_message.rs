use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::{generate_bytes, generate_string};

pub fn generate_abi_send_async_message(address_sc: &str, calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_bytes = generate_bytes(rng.gen_range(0..500));

    calls.push(format!(
        "{{let key_bytes = new Uint8Array({});
        key_bytes.set({:?});
        env.send_async_message(\"{}\", \"{}\", env.make_slot({}, {}), env.make_slot({}, {}), {}, {}, {}, key_bytes, env.make_send_async_message_filter_null());}}",
        rng_bytes.len(),
        rng_bytes,
        address_sc,
        generate_string(rng.gen_range(5..100)),
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(1_000..10_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000..300_000),
        rng.gen_range(1..3),
        rng.gen_range(100_000_000..1_000_000_000),
    ));
}
