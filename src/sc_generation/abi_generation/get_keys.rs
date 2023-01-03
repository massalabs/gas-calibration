pub fn generate_abi_get_keys(calls: &mut Vec<String>) {
    calls.push("env.getKeys();".to_string());
}
