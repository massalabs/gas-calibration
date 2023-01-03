use super::static_public_key;

pub fn generate_abi_public_key_to_address(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.publicKeyToAddress(\"{}\");",
        static_public_key(),
    ));
}
