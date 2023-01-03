pub fn generate_abi_remaining_gas(calls: &mut Vec<String>) {
    calls.push(format!("env.remainingGas();"));
}
