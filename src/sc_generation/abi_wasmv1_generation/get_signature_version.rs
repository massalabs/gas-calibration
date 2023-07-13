use super::generate_signature;

pub fn generate_abi_get_signature_version(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.get_signature_version(\"{}\");",
        generate_signature(),
    ));
}
