pub fn generate_abi_get_keys(calls: &mut Vec<String>) {
    calls.push(format!("env.getKeys();"));
}
