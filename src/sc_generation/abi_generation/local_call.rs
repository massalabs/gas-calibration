pub fn generate_abi_local_call(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    let mut call = String::from("env.localCall(");

    if !*call_already_prep {
        preparation_calls.push(format!(
            "env.setBytecodeOf(\"{}\", env.getOpData(toBytes(\"empty_main_sc\")));",
            address_sc
        ));
        preparation_calls.push(format!(
            "env.transferCoins(\"{}\", 10_000_000_000);",
            address_sc
        ));
        *call_already_prep = true;
    }

    call.push_str(&format!(
        "\"{}\", \"main\", new StaticArray<u8>(0));",
        address_sc
    ));
    calls.push(call);
}
