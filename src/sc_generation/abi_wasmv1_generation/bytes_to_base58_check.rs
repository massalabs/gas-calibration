use rand::Rng;

use super::generate_string;

pub fn generate_abi_bytes_to_base58_check(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let s = generate_string(rng.gen_range(2..500));
    let bytes: Vec<_> = unsafe { s.encode_utf16()
        .collect::<Vec<u16>>()
        .align_to::<u8>()
        .1
        .to_vec() };

    calls.push(format!(
        "{{let bytes = new Uint8Array({});
        bytes.set({:?});
        env.bytes_to_base58_check(bytes);}}",
        bytes.len(),
        bytes,
    ));
}
