use super::get_bytecode;

pub fn generate_abi_get_bytecode(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    if !*call_already_prep {
        let bytecode_bytes = get_bytecode();
        preparation_calls.push(format!(
            "let bytecode_bytes = new Uint8Array({});
            bytecode_bytes.set({:?});",
            bytecode_bytes.len(),
            bytecode_bytes
        ));
        preparation_calls.push(format!(
            "env.transfer_coins(\"{}\", env.make_native_amount(100,0), null);",
            address_sc
        ));
        *call_already_prep = true;
    }

    calls.push("  env.get_bytecode(null);".to_string());
}
