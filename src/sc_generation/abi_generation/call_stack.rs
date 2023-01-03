pub fn generate_abi_call_stack(calls: &mut Vec<String>) {
    calls.push(format!("env.callStack();"));
}
