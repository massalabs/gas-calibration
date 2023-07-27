use rand::Rng;

pub fn generate_abi_checked_div_native_time(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.checked_div_native_time(env.make_native_time({}), env.make_native_time({}));",
        rng.gen_range(1_u64..1689242786000),
        rng.gen_range(1_u64..1689242786000)
    ));
}
