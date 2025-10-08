use rand::rngs::ThreadRng;

pub fn generate_abi_deferred_call_exists(
    _rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(
        "env.deferredCallExists(\"D17MpSPsmYL3eDj7x3uSE1KUNJf49JtTgzqMLxsxLWF6wLZbNoRtFPNp5uRWaCZA\");".to_string());
}
