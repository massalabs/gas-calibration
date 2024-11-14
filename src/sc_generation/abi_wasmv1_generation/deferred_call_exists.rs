pub fn generate_abi_deferred_call_exists(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.deferred_call_exists(\"D17MpSPsmYL3eDj7x3uSE1KUNJf49JtTgzqMLxsxLWF6wLZbNoRtFPNp5uRWaCZA\");"
    ));
}
