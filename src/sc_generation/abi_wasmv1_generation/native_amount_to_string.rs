use rand::Rng;

pub fn generate_abi_native_amount_to_string(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.native_amount_to_string(env.make_native_amount({}, 8));",
        rng.gen_range(1_u64..10_000_000_000),
    ));
}
