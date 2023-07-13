use std::fs::File;

use massa_models::datastore::Datastore;
use rand::Rng;
use std::io::Write;

use super::abi_wasmv1_generation::*;

fn static_address() -> String {
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    // String::from("AU12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
    // We need to have a SC Address to be able to set its bytecode
    String::from("AS12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
}

pub fn generate_op_datastore() -> Datastore {
    let mut rng = rand::thread_rng();
    let mut datastore: Datastore = Datastore::new();
    let nb_entries = 100;
    for _ in 0..nb_entries {
        unsafe {
            let key = generate_string(rng.gen_range(5..32))
                .encode_utf16()
                .collect::<Vec<u16>>()
                .align_to::<u8>()
                .1
                .to_vec();
            let value = generate_string(rng.gen_range(5..100))
                .encode_utf16()
                .collect::<Vec<u16>>()
                .align_to::<u8>()
                .1
                .to_vec();
            datastore.insert(key, value);
        }
    }
    unsafe {
        let key = String::from("empty_main_sc")
            .encode_utf16()
            .collect::<Vec<u16>>()
            .align_to::<u8>()
            .1
            .to_vec();
        match std::fs::read("./src/sc_generation/template/empty_main_sc_wasmv1.wasm_add") {
            Ok(bytes) => datastore.insert(key, bytes),
            Err(e) => panic!("{}", e),
        };
    };
    let mut output = File::create("./src/sc_generation/template/op_datastore.json").unwrap();
    write!(
        output,
        "{}",
        serde_json::to_string(
            &datastore
                .clone()
                .into_iter()
                .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
        )
        .unwrap()
    )
    .unwrap();
    datastore
}

// Return type: preparation calls, calls
pub fn generate_calls(
    abi: Vec<String>,
    limit_per_calls: u64,
    op_datastore: Datastore,
) -> (Vec<String>, Vec<String>) {
    let mut rng = rand::thread_rng();
    let mut calls = Vec::new();
    let mut preparation_calls = Vec::new();
    let address_sc = static_address();

    let nb_calls = rng.gen_range(0..limit_per_calls);
    let mut call_already_prep = false;
    for _ in 0..nb_calls {
        match abi[0].as_str() {
            "abi_set_data" => generate_abi_set_data(&mut rng, &mut calls),
            "abi_get_data" => generate_abi_get_data(&mut rng, &mut calls, &mut preparation_calls),
            "abi_delete_data" => {
                generate_abi_delete_data(&mut rng, &mut calls, &mut preparation_calls)
            }
            "abi_append_data" => {
                generate_abi_append_data(&mut rng, &mut calls, &mut preparation_calls)
            }
            "abi_has_data" => generate_abi_has_data(&mut rng, &mut calls, &mut preparation_calls),
            "abi_get_balance" => generate_abi_get_balance(&mut calls),
            "abi_get_bytecode" => generate_abi_get_bytecode(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "abi_set_bytecode" => generate_abi_set_bytecode(&mut calls, &mut call_already_prep),
            "abi_get_keys" => generate_abi_get_keys(&mut calls),
            "abi_get_op_keys" => generate_abi_get_op_keys(&mut calls),
            "abi_has_op_key" => generate_abi_has_op_key(&op_datastore, &mut rng, &mut calls),
            "abi_get_op_data" => generate_abi_get_op_data(&op_datastore, &mut rng, &mut calls),
            "abi_call" => generate_abi_call(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "abi_create_sc" => generate_abi_create_sc(&mut calls),
            "abi_transfer_coins" => generate_abi_transfer_coins(&mut rng, &mut calls),
            "abi_generate_event" => generate_abi_generate_event(&mut rng, &mut calls),
            "abi_abort" => generate_abi_abort(),
            "abi_get_current_slot" => generate_abi_get_current_slot(&mut calls),
            "abi_hash_sha256" => generate_abi_hash_sha256(&mut rng, &mut calls),
            "abi_hash_keccak256" => generate_abi_hash_keccak256(&mut rng, &mut calls),
            "abi_blake3_hash" => generate_abi_blake3_hash(&mut rng, &mut calls),
            "abi_verify_evm_signature" => generate_abi_verify_evm_signature(&mut calls),
            "abi_get_remaining_gas" => generate_abi_get_remaining_gas(&mut calls),
            "abi_get_owned_addresses" => generate_abi_get_owned_addresses(&mut calls),
            "abi_get_call_stack" => generate_abi_get_call_stack(&mut calls),
            "abi_address_from_public_key" => generate_abi_address_from_public_key(&mut calls),
            "abi_unsafe_random" => generate_abi_unsafe_random(&mut rng, &mut calls),
            "abi_get_call_coins" => generate_abi_get_call_coins(&mut calls),
            "abi_get_native_time" => generate_abi_get_native_time(&mut calls),
            "abi_send_async_message" => {
                generate_abi_send_async_message(&address_sc, &mut rng, &mut calls)
            }
            "abi_get_origin_operation_id" => generate_abi_get_origin_operation_id(&mut calls),
            "abi_local_execution" => {
                generate_abi_local_execution(&mut calls, &mut call_already_prep)
            }
            "abi_caller_has_write_access" => generate_abi_caller_has_write_access(&mut calls),
            "abi_check_native_amount" => generate_abi_check_native_amount(&mut rng, &mut calls),
            "abi_add_native_amounts" => generate_abi_add_native_amounts(&mut rng, &mut calls),
            "abi_sub_native_amounts" => generate_abi_sub_native_amounts(&mut rng, &mut calls),
            "abi_mul_native_amount" => generate_abi_mul_native_amount(&mut rng, &mut calls),
            "abi_div_rem_native_amount" => generate_abi_div_rem_native_amount(&mut rng, &mut calls),
            "abi_div_rem_native_amounts" => {
                generate_abi_div_rem_native_amounts(&mut rng, &mut calls)
            }
            "abi_native_amount_to_string" => {
                generate_abi_native_amount_to_string(&mut rng, &mut calls)
            }
            "abi_native_amount_from_string" => {
                generate_abi_native_amount_from_string(&mut rng, &mut calls)
            }
            "abi_base58_check_to_bytes" => generate_abi_base58_check_to_bytes(&mut rng, &mut calls),
            "abi_bytes_to_base58_check" => generate_abi_bytes_to_base58_check(&mut rng, &mut calls),
            "abi_check_address" => generate_abi_check_address(&mut calls),
            "abi_check_pubkey" => generate_abi_check_pubkey(&mut calls),
            "abi_check_signature" => generate_abi_check_signature(&mut calls),
            "abi_get_address_category" => generate_abi_get_address_category(&mut calls),
            "abi_get_address_version" => generate_abi_get_address_version(&mut calls),
            "abi_get_pubkey_version" => generate_abi_get_pubkey_version(&mut calls),
            "abi_get_signature_version" => generate_abi_get_signature_version(&mut calls),
            "abi_checked_add_native_time" => {
                generate_abi_checked_add_native_time(&mut rng, &mut calls)
            }
            "abi_checked_sub_native_time" => {
                generate_abi_checked_sub_native_time(&mut rng, &mut calls)
            }
            "abi_checked_mul_native_time" => {
                generate_abi_checked_mul_native_time(&mut rng, &mut calls)
            }
            "abi_checked_scalar_div_native_time" => {
                generate_abi_checked_scalar_div_native_time(&mut rng, &mut calls)
            }
            "abi_checked_div_native_time" => {
                generate_abi_checked_div_native_time(&mut rng, &mut calls)
            }
            "abi_compare_address" => generate_abi_compare_address(&mut calls),
            "abi_compare_native_amount" => generate_abi_compare_native_amount(&mut rng, &mut calls),
            "abi_compare_native_time" => generate_abi_compare_native_time(&mut rng, &mut calls),
            "abi_compare_pub_key" => generate_abi_compare_pub_key(&mut calls),
            "abi_verify_signature" => generate_abi_verify_signature(&mut rng, &mut calls),
            /*
            "localCall" => generate_abi_local_call(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "functionExists" => generate_abi_function_exists(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),*/
            _ => {
                panic!("ABI: {} don't have any generation function. Please add one in src/sc_generation/generation.rs", abi[0].as_str())
            }
        }
    }

    let mut final_preparation_calls = Vec::new();
    for call in preparation_calls {
        final_preparation_calls.insert(0, call);
    }
    (final_preparation_calls, calls)
}

pub fn generate_instruction(limit_per_calls: u64) -> (Vec<String>, Vec<String>) {
    let mut rng = rand::thread_rng();
    let nb_calls = rng.gen_range(0..limit_per_calls);
    let nb_init = rng.gen_range(0..limit_per_calls / 10);
    let mut instructions = Vec::new();
    let mut setup_instructions = Vec::new();

    let operations = [
        "i32.add",
        "i32.sub",
        "i32.mul",
        "i32.div_s",
        "local.get",
        "local.set",
        "global.get",
        "global.set",
        "if",
    ];
    let mut nb_drop = 0;
    let mut local_initialized = vec![];
    for i in 0..nb_init {
        let local_init = format!("(local ${} i32)", i);
        instructions.push(local_init);
        let global_init = format!(
            "(global ${} (mut i32) (i32.const {}))",
            i,
            rng.gen_range(0..i32::MAX)
        );
        setup_instructions.push(global_init.clone());
    }
    for _ in 0..nb_calls {
        let left_operand = rng.gen_range(1..i32::MAX);
        let right_operand = rng.gen_range(1..i32::MAX);
        let gen_first_operand = rng.gen_range(0..2) == 1;
        let instruction = match (
            rng.gen_range(0..operations.len()),
            nb_drop,
            gen_first_operand,
        ) {
            (idx, 0, _) if idx < 4 => {
                nb_drop += 1;
                format!(
                    "i32.const {}\ni32.const {} \n{}",
                    left_operand, right_operand, operations[idx]
                )
            }
            (idx, _, false) if idx < 4 => {
                format!("i32.const {} \n{}", right_operand, operations[idx])
            }
            (idx, _, true) if idx < 4 => {
                nb_drop += 1;
                format!(
                    "i32.const {}\ni32.const {} \n{}",
                    left_operand, right_operand, operations[idx]
                )
            }
            (4, _, _) => {
                if local_initialized.is_empty() {
                    continue;
                } else {
                    nb_drop += 1;
                    let local = rng.gen_range(0..local_initialized.len());
                    format!("{} ${}", operations[4], local)
                }
            }
            (5, _, _) => {
                if nb_init == 0 {
                    continue;
                }
                if local_initialized.len() < nb_init as usize {
                    local_initialized.push(true);
                    format!(
                        "i32.const {}\n{} ${}",
                        left_operand,
                        operations[5],
                        local_initialized.len() - 1
                    )
                } else {
                    format!(
                        "i32.const {}\n{} ${}",
                        left_operand,
                        operations[5],
                        rng.gen_range(0..local_initialized.len())
                    )
                }
            }
            (6, _, _) => {
                if setup_instructions.is_empty() {
                    continue;
                }
                nb_drop += 1;
                format!(
                    "{} ${}",
                    operations[6],
                    rng.gen_range(0..setup_instructions.len())
                )
            }
            (7, _, _) => {
                if setup_instructions.is_empty() {
                    continue;
                }
                format!(
                    "i32.const {}\n{} ${}",
                    rng.gen_range(0..i32::MAX),
                    operations[7],
                    rng.gen_range(0..setup_instructions.len())
                )
            }
            (8, _, _) => {
                format!(
                    "
                i32.const {}\n
                ({}\n
                    (then\n
                    )\n
                    (else\n
                    )\n
                  )",
                    rng.gen_range(0..i32::MAX),
                    operations[8]
                )
            }
            _ => {
                panic!("Unknown operation");
            }
        };
        instructions.push(instruction);
    }

    for _ in 0..nb_drop {
        instructions.push("drop".to_string());
    }
    (setup_instructions, instructions)
}
