pub fn generate_abi_get_op_keys(calls: &mut Vec<String>) {
    calls.push("env.getOpKeys();".to_string());
}
