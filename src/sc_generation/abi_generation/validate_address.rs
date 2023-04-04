pub fn generate_abi_validate_address(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.validateAddress(\"{}\");",
        address_sc,
    ));
}
