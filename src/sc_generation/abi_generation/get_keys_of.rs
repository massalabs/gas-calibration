pub fn generate_abi_get_keys_of(address_sc: &str, calls: &mut Vec<String>) {
    calls.push(format!("env.getKeysOf(\"{}\", new StaticArray<u8>(0));", address_sc));
}
