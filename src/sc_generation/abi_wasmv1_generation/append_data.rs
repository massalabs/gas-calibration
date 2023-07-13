use rand::Rng;

use super::generate_string;

pub fn generate_abi_append_data(calls: &mut Vec<String>, preparation_calls: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    let key = generate_string(rng.gen_range(5..32));
    preparation_calls.push(format!(
        "env.set_data(toBytes(\"{}\"), toBytes(\"{}\"), null);",
        key,
        generate_string(rng.gen_range(1..1000))
    ));
    calls.push(format!(
        "env.append_data(toBytes(\"{}\"), toBytes(\"{}\"), null);",
        key,
        generate_string(rng.gen_range(1..1000))
    ));
}
