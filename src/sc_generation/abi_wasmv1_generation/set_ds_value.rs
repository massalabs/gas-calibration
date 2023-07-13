use rand::Rng;

use super::generate_string;

pub fn generate_abi_set_ds_value(calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    calls.push(format!(
        "env.set_ds_value(toBytes(\"{}\"), toBytes(\"{}\"), null);",
        generate_string(rng.gen_range(5..32)),
        generate_string(rng.gen_range(1..1000))
    ));
}
