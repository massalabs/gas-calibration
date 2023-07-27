pub fn generate_abi_get_origin_operation_id(calls: &mut Vec<String>) {
    calls.push("  env.get_origin_operation_id();".to_string());
}
