use massa_models::datastore::Datastore;
use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_get_op_data(
    op_datastore: &Datastore,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
) {
    calls.push(format!(
        "env.getOpData(toBytes(\"{}\"));",
        rng.gen_range(0..op_datastore.len() - 2)
    ));
}
