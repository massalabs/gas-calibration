use rand::Rng;

use super::{generate_pub_key, generate_signature, generate_string};

pub fn generate_abi_verify_signature(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.verify_signature(\"{}\", toBytes(\"{}\"), \"{}\");",
        generate_signature(),
        generate_string(rng.gen_range(5..1000)),
        generate_pub_key(),
    ));
}
