pub fn generate_abi_get_bytecode(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    if !*call_already_prep {
        preparation_calls.push(format!(
            "env.set_bytecode(\"{}\", env.get_op_data(toBytes(\"empty_main_sc\")), null);",
            address_sc
        ));
        preparation_calls.push(format!(
            "env.transfer_coins(\"{}\", env.make_native_amount(100,0), null);",
            address_sc
        ));
        *call_already_prep = true;
    }
    calls.push("  env.get_bytecode(null);".to_string());
}
