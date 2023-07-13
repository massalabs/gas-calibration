use rand::Rng;

pub fn generate_abi_unsafe_random(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!("  env.unsafe_random({});", rng.gen_range(5..1000)));
}
