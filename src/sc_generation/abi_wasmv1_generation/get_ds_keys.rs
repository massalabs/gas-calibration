pub fn generate_abi_get_ds_keys(calls: &mut Vec<String>) {
    calls.push("  env.get_ds_keys(new Uint8Array(0), null);".to_string());
}
