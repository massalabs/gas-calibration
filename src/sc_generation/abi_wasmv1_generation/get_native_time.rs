pub fn generate_abi_get_native_time(calls: &mut Vec<String>) {
    calls.push("  env.get_native_time();".to_string());
}
