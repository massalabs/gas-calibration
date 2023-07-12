pub fn generate_abi_compare_address(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.compare_address(\"{}\", \"{}\");",
        address_sc, address_sc
    ));
}
