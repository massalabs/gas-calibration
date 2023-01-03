pub fn generate_abi_set_bytecode(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        calls.push("let bytecode = env.getOpData(toBytes(\"empty_main_sc\"));".to_string());
        *call_already_prep = true;
    }
    calls.push("env.setBytecode(bytecode);".to_string());
}
