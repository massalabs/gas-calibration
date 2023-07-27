use super::get_bytecode;

pub fn generate_abi_set_bytecode(calls: &mut Vec<String>, call_already_prep: &mut bool) {
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
    calls.push("  env.set_bytecode(bytecode_bytes, null);".to_string());
}
