use super::generate_address;

pub fn generate_abi_compare_address(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.compare_address(\"{}\", \"{}\");",
        generate_address(),
        generate_address()
    ));
}
