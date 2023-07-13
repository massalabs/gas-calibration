use rand::Rng;

use super::generate_string;

pub fn generate_abi_generate_event(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.generate_event(\"{}\");",
        generate_string(rng.gen_range(0..255))
    ));
}
