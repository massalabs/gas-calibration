pub fn generate_abi_current_period(calls: &mut Vec<String>) {
    calls.push("env.currentPeriod();".to_string());
}
