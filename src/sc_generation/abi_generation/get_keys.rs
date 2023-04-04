pub fn generate_abi_get_keys(calls: &mut Vec<String>) {
    calls.push("env.getKeys(new StaticArray<u8>(0));".to_string());
}
