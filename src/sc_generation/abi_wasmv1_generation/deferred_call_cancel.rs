use rand::Rng;

use super::generate_string;

pub fn generate_abi_deferred_call_cancel(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.deferred_call_cancel(\"{}\");",
        generate_string(rng.gen_range(5..25)),
    ));
}
