pub fn generate_abi_call_stack(calls: &mut Vec<String>) {
    calls.push("env.callStack();".to_string());
}
