use rand::Rng;

use super::{generate_bytes, generate_pub_key, generate_signature};

pub fn generate_abi_verify_signature(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_bytes = generate_bytes(rng.gen_range(2..500));

    calls.push(format!(
        "{{let bytes = new Uint8Array({});
        bytes.set({:?});
        env.verify_signature(\"{}\", bytes, \"{}\");}}",
        rng_bytes.len(),
        rng_bytes,
        generate_signature(),
        generate_pub_key(),
    ));
}
