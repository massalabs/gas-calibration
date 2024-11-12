use massa_models::config::THREAD_COUNT;
use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_deferred_call_quote(
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.getDeferredCallQuote({}, {}, {}, {});",
        rng.gen_range(100..1_000),
        rng.gen_range(0..THREAD_COUNT),
        rng.gen_range(100_000_000..1_000_000_000),
        rng.gen_range(0..2000)
    ));
}
