use rand::Rng;

use super::generate_string;

pub fn generate_abi_ds_entry_exists(calls: &mut Vec<String>, preparation_calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let key = generate_string(rng.gen_range(5..32));
    preparation_calls.push(format!(
        "env.set_data(toBytes(\"{}\"), toBytes(\"{}\"), null);",
        key,
        generate_string(rng.gen_range(1..1000))
    ));
    calls.push(format!("  env.ds_entry_exists(toBytes(\"{}\"), null);", key));
}
