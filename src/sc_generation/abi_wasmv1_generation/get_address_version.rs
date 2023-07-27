use super::generate_address;

pub fn generate_abi_get_address_version(calls: &mut Vec<String>) {
    calls.push(format!(
        "  env.get_address_version(\"{}\");",
        generate_address()
    ));
}
