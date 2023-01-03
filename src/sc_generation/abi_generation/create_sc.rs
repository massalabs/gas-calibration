pub fn generate_abi_create_sc(calls: &mut Vec<String>) {
    calls.push("env.createSC(env.getOpData(toBytes(\"empty_main_sc\")));".to_string());
}
