use super::generate_address;

pub fn generate_abi_check_address(calls: &mut Vec<String>) {
    calls.push(format!("  env.check_address(\"{}\");", generate_address()));
}
