pub fn generate_abi_current_period(calls: &mut Vec<String>) {
    calls.push(format!("env.currentPeriod();"));
}
