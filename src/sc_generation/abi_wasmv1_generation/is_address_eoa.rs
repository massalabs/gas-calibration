use super::generate_address;

pub fn generate_abi_is_address_eoa(calls: &mut Vec<String>) {
    calls.push(format!("env.is_address_eoa(\"{}\");", generate_address(),));
}
