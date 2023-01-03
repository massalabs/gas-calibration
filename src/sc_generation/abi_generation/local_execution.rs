pub fn generate_abi_local_execution(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        let prep_call = String::from("let bytecode = env.getOpData(toBytes(\"empty_main_sc\"));");
        calls.push(prep_call);
        *call_already_prep = true;
    }
    let call = String::from("env.localExecution(bytecode, \"main\", new StaticArray<u8>(0));");
    calls.push(call);
}
