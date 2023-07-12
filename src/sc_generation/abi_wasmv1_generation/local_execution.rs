pub fn generate_abi_local_execution(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        let prep_call = String::from("let bytecode = env.get_op_data(toBytes(\"empty_main_sc\"));");
        calls.push(prep_call);
        *call_already_prep = true;
    }
    let call = String::from("env.local_execution(bytecode, \"main\", new Uint8Array(0));");
    calls.push(call);
}
