use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_native_amount_to_string(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.native_amount_to_string(env.make_native_amount({}, 0));",
        rng.gen_range(1..100),
    ));
}
