use super::static_public_key;

pub fn generate_abi_get_pubkey_version(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.get_pubkey_version(\"{}\");",
        static_public_key(),
    ));
}
