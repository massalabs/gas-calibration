use massa_models::datastore::Datastore;
use rand::Rng;

pub fn generate_abi_op_entry_exists(op_datastore: &Datastore, calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let index_key = rng.gen_range(0..op_datastore.len() - 2);
    let key_bytes = op_datastore
        .clone()
        .into_iter()
        .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
        .get(index_key)
        .unwrap()
        .0
        .clone();

    calls.push(format!(
        "{{let key_bytes = new Uint8Array({});
        key_bytes.set({:?});
        env.op_entry_exists(key_bytes);}}",
        key_bytes.len(),
        key_bytes
    ));
}
