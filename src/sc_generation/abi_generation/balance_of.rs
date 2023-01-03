pub fn generate_abi_balance_of(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!("env.balanceOf(\"{}\");", address_sc));
}
