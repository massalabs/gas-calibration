use super::generate_signature;

pub fn generate_abi_check_signature(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.check_signature(\"{}\");",
        generate_signature(),
    ));
}
