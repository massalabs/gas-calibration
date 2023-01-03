pub fn generate_abi_unsafe_random(calls: &mut Vec<String>) {
    calls.push("env.unsafeRandom();".to_string());
}
