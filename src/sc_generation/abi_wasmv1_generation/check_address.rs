pub fn generate_abi_check_address(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!("  env.check_address(\"{}\");", address_sc,));
}
