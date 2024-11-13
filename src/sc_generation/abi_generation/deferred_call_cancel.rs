use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_deferred_call_cancel(
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.deferredCallCancel(\"{}\");",
        generate_string(rng.gen_range(5..25))
    ));
}
