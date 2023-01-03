pub fn generate_abi_current_thread(calls: &mut Vec<String>) {
    calls.push("env.currentThread();".to_string());
}
