use massa_models::datastore::Datastore;
use rand::Rng;

pub fn generate_abi_get_op_data(op_datastore: &Datastore, calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let mut call_added = false;
    while !call_added {
        let index_key = rng.gen_range(0..op_datastore.len() - 2);
        let key_bytes = op_datastore
            .clone()
            .into_iter()
            .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
            .get(index_key)
            .unwrap()
            .0
            .clone();
        let key: Vec<u16> = key_bytes
            .chunks_exact(2)
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect();
        let key = key.as_slice();
        if String::from_utf16_lossy(key) != "empty_main_sc" {
            calls.push(format!(
                "{{let key_bytes = new Uint8Array({});
                key_bytes.set({:?});
                env.get_op_data(key_bytes);}}",
                key_bytes.len(),
                key_bytes
            ));
            call_added = true;
        }
    }
}
