pub fn generate_abi_owned_addresses(calls: &mut Vec<String>) {
    calls.push(format!("env.ownedAddresses();"));
}
