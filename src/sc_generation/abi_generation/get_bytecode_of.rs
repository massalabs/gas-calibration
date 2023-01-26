pub fn generate_abi_get_bytecode_of(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    if !*call_already_prep {
        preparation_calls.push(format!(
            "env.setBytecodeOf(\"{}\", env.getOpData(toBytes(\"empty_main_sc\")));",
            address_sc
        ));
        preparation_calls.push(format!(
            "env.transferCoins(\"{}\", 10_000_000_000);",
            address_sc
        ));
        *call_already_prep = true;
    }
    calls.push(format!("env.getBytecodeOf(\"{}\");", address_sc));
}
