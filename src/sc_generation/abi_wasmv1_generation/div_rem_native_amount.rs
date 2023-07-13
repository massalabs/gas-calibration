use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_div_rem_native_amount(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.div_rem_native_amount(env.make_native_amount({}, 9), {});",
        rng.gen_range(1_u64..100_000_000_000),
        rng.gen_range(1..1000)
    ));
}
