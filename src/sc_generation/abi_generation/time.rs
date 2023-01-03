pub fn generate_abi_time(calls: &mut Vec<String>) {
    calls.push("env.time();".to_string());
}
