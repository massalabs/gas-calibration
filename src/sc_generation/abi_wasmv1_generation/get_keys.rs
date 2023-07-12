pub fn generate_abi_get_keys(calls: &mut Vec<String>) {
    calls.push("  env.get_keys(new Uint8Array(0), null);".to_string());
}
