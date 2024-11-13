use massa_models::config::THREAD_COUNT;
use rand::Rng;

pub fn generate_abi_deferred_call_quote(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.get_deferred_call_quote({}, {}, {}, {});",
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000_000..1_000_000_000),
        rng.gen_range(0..2000)
    ));
}
