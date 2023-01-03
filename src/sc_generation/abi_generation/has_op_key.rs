use massa_models::datastore::Datastore;
use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_has_op_key(
    op_datastore: &Datastore,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.hasOpKey(toBytes(\"{}\"));",
        rng.gen_range(0..op_datastore.len() - 2)
    ));
}
