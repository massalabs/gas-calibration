use rand::Rng;

use super::generate_bytes;

pub fn generate_abi_get_ds_value(calls: &mut Vec<String>, preparation_calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_key_bytes = generate_bytes(rng.gen_range(10..64));
    let rng_value1_bytes = generate_bytes(rng.gen_range(1..1000));

    preparation_calls.push(format!(
        "{{let key_bytes = new Uint8Array({});
        key_bytes.set({:?});
        let value1_bytes = new Uint8Array({});
        value1_bytes.set({:?});
        env.set_ds_value(key_bytes, value1_bytes, null);}}",
        rng_key_bytes.len(),
        rng_key_bytes,
        rng_value1_bytes.len(),
        rng_value1_bytes,
    ));
    calls.push(format!(
        "{{let key_bytes = new Uint8Array({});
        key_bytes.set({:?});
        env.get_ds_value(key_bytes, null);}}",
        rng_key_bytes.len(),
        rng_key_bytes
    ));
}
