use super::generate_pub_key;

pub fn generate_abi_get_pubkey_version(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.get_pubkey_version(\"{}\");",
        generate_pub_key(),
    ));
}
