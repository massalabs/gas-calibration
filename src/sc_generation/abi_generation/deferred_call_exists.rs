use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_deferred_call_exists(
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.deferredCallExists(\"{}\");",
        generate_string(rng.gen_range(5..25))
    ));
}
