pub fn generate_abi_get_address_category(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!("  env.get_address_category(\"{}\");", address_sc));
}
