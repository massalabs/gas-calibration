use super::generate_pub_key;

pub fn generate_abi_address_from_public_key(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.address_from_public_key(\"{}\");",
        generate_pub_key()
    ));
}
