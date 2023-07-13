use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_check_native_amount(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.check_native_amount(env.make_native_amount({}, 8));",
        rng.gen_range(1_u64..10_000_000_000)
    ));
}
