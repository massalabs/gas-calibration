pub fn generate_abi_create_sc(calls: &mut Vec<String>) {
    calls.push("  env.create_sc(env.get_op_data(toBytes(\"empty_main_sc\")));".to_string());
}
