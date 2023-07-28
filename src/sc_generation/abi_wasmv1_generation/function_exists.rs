use super::get_bytecode;

pub fn generate_abi_function_exists(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    if !*call_already_prep {
        let bytecode_bytes = get_bytecode();
        preparation_calls.push(format!(
            "let bytecode_bytes = new Uint8Array({});
            bytecode_bytes.set({:?});
            env.set_bytecode(bytecode_bytes, \"{}\");
            env.transfer_coins(\"{}\", env.make_native_amount(100,0), null);",
            bytecode_bytes.len(),
            bytecode_bytes,
            address_sc,
            address_sc
        ));
        *call_already_prep = true;
    }

    let call = format!("env.functionExists(\"{}\", \"main\");", address_sc);
    calls.push(call);
}
