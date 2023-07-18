use rand::Rng;

use super::generate_string;

pub fn generate_abi_get_ds_value(calls: &mut Vec<String>, preparation_calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let key = generate_string(rng.gen_range(5..32));
    preparation_calls.push(format!(
        "env.set_ds_value(toBytes(\"{}\"), toBytes(\"{}\"), null);",
        key,
        generate_string(rng.gen_range(1..1000))
    ));
    calls.push(format!("  env.get_ds_value(toBytes(\"{}\"), null);", key));
}