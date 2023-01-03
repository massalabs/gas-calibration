pub fn generate_abi_caller_has_write_access(calls: &mut Vec<String>) {
    calls.push(format!("env.callerHasWriteAccess();"));
}
