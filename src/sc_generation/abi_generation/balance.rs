pub fn generate_abi_balance(calls: &mut Vec<String>) {
    calls.push(format!("env.balance();"));
}
