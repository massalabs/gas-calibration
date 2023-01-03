pub fn generate_abi_set_bytecode(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        calls.push(format!(
            "let bytecode = env.getOpData(toBytes(\"empty_main_sc\"));"
        ));
        *call_already_prep = true;
    }
    calls.push(format!("env.setBytecode(bytecode);",));
}
