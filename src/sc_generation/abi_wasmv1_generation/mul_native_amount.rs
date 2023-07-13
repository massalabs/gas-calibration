use rand::Rng;

pub fn generate_abi_mul_native_amount(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.mul_native_amount(env.make_native_amount({}, 8), {});",
        rng.gen_range(1_u64..10_000_000),
        rng.gen_range(1..1000)
    ));
}
