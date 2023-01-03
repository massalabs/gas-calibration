use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_del_of(
    address_sc: &str,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
) {
    let key = generate_string(rng.gen_range(5..32));
    preparation_calls.push(format!(
        "env.set(toBytes(\"{}\"), toBytes(\"{}\"));",
        key,
        generate_string(rng.gen_range(1..1000))
    ));
    calls.push(format!(
        "env.deleteOf(\"{}\", toBytes(\"{}\"));",
        address_sc, key
    ));
}
