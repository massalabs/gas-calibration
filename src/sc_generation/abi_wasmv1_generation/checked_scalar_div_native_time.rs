use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_checked_scalar_div_native_time(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.checked_scalar_div_native_time(env.make_native_time({}), {});",
        rng.gen_range(1..100000),
        rng.gen_range(1..100)
    ));
}
