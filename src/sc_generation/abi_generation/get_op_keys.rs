pub fn generate_abi_get_op_keys(calls: &mut Vec<String>) {
    calls.push(format!("env.getOpKeys();"));
}
