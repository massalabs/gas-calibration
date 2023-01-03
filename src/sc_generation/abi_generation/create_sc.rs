pub fn generate_abi_create_sc(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.createSC(env.getOpData(toBytes(\"empty_main_sc\")));"
    ));
}
