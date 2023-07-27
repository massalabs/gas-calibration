pub fn generate_abi_get_current_slot(calls: &mut Vec<String>) {
    calls.push("  env.get_current_slot();".to_string());
}
