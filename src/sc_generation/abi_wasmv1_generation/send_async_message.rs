use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::generate_string;

pub fn generate_abi_send_async_message(address_sc: &str, calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.send_async_message(\"{}\", \"{}\", env.make_slot({}, {}), env.make_slot({}, {}), {}, {}, {}, toBytes(\"{}\"), env.make_send_async_message_filter_null());",
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
