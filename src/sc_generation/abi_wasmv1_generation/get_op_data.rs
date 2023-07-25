use massa_models::datastore::Datastore;
use rand::Rng;

pub fn generate_abi_get_op_data(
    op_datastore: &Datastore,
    calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    let mut rng = rand::thread_rng();

    if !*call_already_prep {
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
            let key = String::from_utf8_lossy(&key_bytes);
            if key != "empty_main_sc" {
                calls.push(format!(
                    "let key_bytes = new Uint8Array({});
                    key_bytes.set({:?});",
                    key_bytes.len(),
                    key_bytes
                ));
                call_added = true;
            }
        }
        *call_already_prep = true;
    }

    calls.push(String::from("env.get_op_data(key_bytes);"));
}
