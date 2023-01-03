pub fn generate_abi_current_thread(calls: &mut Vec<String>) {
    calls.push(format!("env.currentThread();"));
}
