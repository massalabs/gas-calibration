pub fn generate_abi_create_sc(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        let prep_call = String::from("let bytecode = env.get_op_data(toBytes(\"empty_main_sc\"));");
        calls.push(prep_call);
        *call_already_prep = true;
    }
    calls.push("  env.create_sc(bytecode);".to_string());
}
