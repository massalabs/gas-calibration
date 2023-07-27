use super::get_bytecode;

pub fn generate_abi_create_sc(calls: &mut Vec<String>, call_already_prep: &mut bool) {
    if !*call_already_prep {
        let bytecode_bytes = get_bytecode();
        calls.push(format!(
            "  let bytecode_bytes = new Uint8Array({});
              bytecode_bytes.set({:?});",
            bytecode_bytes.len(),
            bytecode_bytes
        ));
        *call_already_prep = true;
    }

    calls.push("  env.create_sc(bytecode_bytes);".to_string());
}
