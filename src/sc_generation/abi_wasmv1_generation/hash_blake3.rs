use rand::Rng;

use super::generate_bytes;

pub fn generate_abi_hash_blake3(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_bytes = generate_bytes(rng.gen_range(2..500));

    calls.push(format!(
        "{{let bytes = new Uint8Array({});
        bytes.set({:?});
        env.hash_blake3(bytes);}}",
        rng_bytes.len(),
        rng_bytes,
    ));
}
