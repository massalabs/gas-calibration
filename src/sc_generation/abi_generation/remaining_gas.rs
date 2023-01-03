pub fn generate_abi_remaining_gas(calls: &mut Vec<String>) {
    calls.push("env.remainingGas();".to_string());
}
