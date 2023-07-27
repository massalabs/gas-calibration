pub fn generate_abi_get_call_stack(calls: &mut Vec<String>) {
    calls.push("  env.get_call_stack();".to_string());
}
