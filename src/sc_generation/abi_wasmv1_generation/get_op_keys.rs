pub fn generate_abi_get_op_keys(calls: &mut Vec<String>) {
    calls.push("  env.get_op_keys(new Uint8Array(0));".to_string());
}
