use rand::Rng;

use super::generate_bytes;

pub fn generate_abi_set_ds_value(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    let rng_key_bytes = generate_bytes(rng.gen_range(5..32));
    let rng_value1_bytes = generate_bytes(rng.gen_range(1..1000));

    calls.push(format!(
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
}
