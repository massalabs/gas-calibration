pub fn generate_abi_call(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    let mut call = String::from("env.call(");

    if !*call_already_prep {
        preparation_calls.push(format!(
            "env.set_bytecode(\"{}\", env.get_op_data(toBytes(\"empty_main_sc\")));",
            address_sc
        ));
        preparation_calls.push(format!(
            "env.transfer_coins(\"{}\", env.make_native_amount(100, 0));",
            address_sc
        ));
        *call_already_prep = true;
    }

    call.push_str(&format!(
        "\"{}\", \"main\", new Uint8Array(0), env.make_native_amount(0, 0));",
        address_sc
    ));
    calls.push(call);
}
