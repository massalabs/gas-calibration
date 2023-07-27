use massa_models::datastore::Datastore;
use rand::seq::IteratorRandom;

pub fn generate_abi_get_op_data(
    op_datastore: &Datastore,
    calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    let mut rng = rand::thread_rng();

    if !*call_already_prep {
        while let Some((key_bytes, _)) = op_datastore.iter().choose(&mut rng) {
            let key = String::from_utf8_lossy(key_bytes);
            if key != "empty_main_sc" {
                calls.push(format!(
                    "let key_bytes = new Uint8Array({});
                    key_bytes.set({:?});",
                    key_bytes.len(),
                    key_bytes
                ));
                break;
            }
        }
        *call_already_prep = true;
    }

    calls.push(String::from("env.get_op_data(key_bytes);"));
}
