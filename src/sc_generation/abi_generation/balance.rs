pub fn generate_abi_balance(calls: &mut Vec<String>) {
    calls.push("env.balance();".to_string());
}
