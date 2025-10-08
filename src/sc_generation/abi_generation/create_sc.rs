pub fn generate_abi_create_sc(calls: &mut Vec<String>) {
    calls.push(
        "env.createSC(env.getOpData(toBytes(\"empty_main_sc_as\")));"
            .to_string(),
    );
}
