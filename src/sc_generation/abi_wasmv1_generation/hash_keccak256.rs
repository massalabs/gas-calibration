use rand::Rng;

use super::generate_bytes;

pub fn generate_abi_hash_keccak256(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_bytes = generate_bytes(rng.gen_range(1..1000));

    calls.push(format!(
        "{{let bytes = new Uint8Array({});
        bytes.set({:?});
        env.hash_keccak256(bytes);}}",
        rng_bytes.len(),
        rng_bytes,
    ));
}
