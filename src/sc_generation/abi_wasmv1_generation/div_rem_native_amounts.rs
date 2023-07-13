use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_div_rem_native_amounts(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.div_rem_native_amounts(env.make_native_amount({}, 8), env.make_native_amount({}, 8));",
        rng.gen_range(1_u64..10_000_000_000),
        rng.gen_range(1_u64..10_000_000_000)
    ));
}
