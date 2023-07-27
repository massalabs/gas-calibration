use rand::Rng;

pub fn generate_abi_sub_native_amount(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.sub_native_amount(env.make_native_amount({}, 8), env.make_native_amount({}, 8));",
        rng.gen_range(5_000_000_000_u64..10_000_000_000),
        rng.gen_range(1_u64..5_000_000_000)
    ));
}
