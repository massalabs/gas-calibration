pub fn generate_abi_get_owned_addresses(calls: &mut Vec<String>) {
    calls.push("  env.get_owned_addresses();".to_string());
}
