use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_checked_sub_native_time(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.checked_sub_native_time(env.make_native_time({}), env.make_native_time({}));",
        rng.gen_range(1000000000000_u64..1689242786000),
        rng.gen_range(1_u64..1000000000000),
    ));
}
