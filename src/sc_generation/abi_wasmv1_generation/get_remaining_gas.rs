pub fn generate_abi_get_remaining_gas(calls: &mut Vec<String>) {
    calls.push("  env.get_remaining_gas();".to_string());
}
