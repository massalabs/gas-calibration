use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_compare_native_amount(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.compare_native_amount(env.make_native_amount({}, 0), env.make_native_amount({}, 0));",
        rng.gen_range(1..100),
        rng.gen_range(1..100)
    ));
}
