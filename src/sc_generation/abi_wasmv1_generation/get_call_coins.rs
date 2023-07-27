pub fn generate_abi_get_call_coins(calls: &mut Vec<String>) {
    calls.push("  env.get_call_coins();".to_string());
}
