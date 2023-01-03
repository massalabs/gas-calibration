pub fn generate_abi_unsafe_random(calls: &mut Vec<String>) {
    calls.push(format!("env.unsafeRandom();"));
}
