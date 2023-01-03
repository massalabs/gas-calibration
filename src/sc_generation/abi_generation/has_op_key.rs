use massa_models::datastore::Datastore;
use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_has_op_key(
    op_datastore: &Datastore,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    let index_key = rng.gen_range(0..op_datastore.len() - 2);
    let key = op_datastore
        .clone()
        .into_iter()
        .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
        .get(index_key)
        .unwrap()
        .0
        .clone();
    let key: Vec<u16> = key
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();
    let key = key.as_slice();
    let key = format!("toBytes(\"{}\")", String::from_utf16_lossy(key));

    calls.push(format!("env.hasOpKey({});", key));
}
