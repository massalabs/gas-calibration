pub fn generate_abi_get_balance(calls: &mut Vec<String>) {
    calls.push("  env.get_balance(null);".to_string());
}
