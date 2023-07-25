use super::get_bytecode;

pub fn generate_abi_local_execution(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        let bytecode_bytes = get_bytecode();

        calls.push(format!(
            "let bytecode_bytes = new Uint8Array({});
        bytecode_bytes.set({:?});",
            bytecode_bytes.len(),
            bytecode_bytes
        ));

        *call_already_prep = true;
    }
    let call = String::from("env.local_execution(bytecode_bytes, \"main\", new Uint8Array(0));");
    calls.push(call);
}
