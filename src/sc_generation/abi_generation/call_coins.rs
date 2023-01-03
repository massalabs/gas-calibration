pub fn generate_abi_call_coins(calls: &mut Vec<String>) {
    calls.push(format!("env.callCoins();"));
}
