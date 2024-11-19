use std::{
    fs::{self, File},
    process,
};

use massa_models::datastore::Datastore;
use rand::{rngs::ThreadRng, Rng};
use std::io::Write;

use crate::{
    config::{output_dir, template_dir},
    AbisType,
};

use super::abi_wasmv1_generation::generate_bytes;

fn static_address() -> String {
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    // String::from("AU12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
    // We need to have a SC Address to be able to set its bytecode
    String::from("AS12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
}

pub fn generate_op_datastore(abi_type: &AbisType) -> Datastore {
    match abi_type {
        AbisType::AS => generate_op_datastore_as(),
        AbisType::WasmV1 => generate_op_datastore_wasmv1(),
    }
}

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn generate_op_datastore_as() -> Datastore {
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
        let key = String::from("empty_main_sc_as")
            .encode_utf16()
            .collect::<Vec<u16>>()
            .align_to::<u8>()
            .1
            .to_vec();

        let path = template_dir(&AbisType::AS).join("empty_main_sc_as.wasm");
        match fs::read(&path) {
            Ok(bytes) => datastore.insert(key, bytes),
            Err(e) => panic!("{:?} {}", path, e),
        }
    };
    let mut output =
        File::create(output_dir(&AbisType::AS).join("op_datastore.json"))
            .unwrap();
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

fn generate_op_datastore_wasmv1() -> Datastore {
    let mut rng = rand::thread_rng();
    let mut datastore: Datastore = Datastore::new();
    let nb_entries = 100;

    for _ in 0..nb_entries {
        let rng_key_bytes = generate_bytes(rng.gen_range(10..64));
        let rng_value_bytes = generate_bytes(rng.gen_range(1..1000));
        datastore.insert(rng_key_bytes, rng_value_bytes);
    }

    let key = String::from("empty_main_sc_wasmv1").into_bytes();
    match std::fs::read(
        template_dir(&AbisType::WasmV1).join("empty_main_sc_wasmv1.wasm_add"),
    ) {
        Ok(bytes) => datastore.insert(key, bytes),
        Err(e) => panic!("{}", e),
    };

    let mut output =
        File::create(output_dir(&AbisType::WasmV1).join("op_datastore.json"))
            .unwrap();
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
    abi_type: &AbisType,
    abi: Vec<String>,
    limit_per_calls: u64,
    op_datastore: &Datastore,
) -> (Vec<String>, Vec<String>) {
    let mut rng = rand::thread_rng();
    let mut calls = Vec::new();
    let mut preparation_calls = Vec::new();
    let address_sc = static_address();
    let nb_calls = rng.gen_range(1..limit_per_calls);
    let mut call_already_prep = false;
    let mut def_call_already_prep = 0;

    for _ in 0..nb_calls {
        match abi_type {
            AbisType::AS => {
                generate_call_as(
                    &abi,
                    op_datastore,
                    &mut rng,
                    &mut calls,
                    &mut preparation_calls,
                    &address_sc,
                    &mut call_already_prep,
                    &mut def_call_already_prep,
                );
            }
            AbisType::WasmV1 => {
                generate_call_wasmv1(
                    &abi,
                    op_datastore,
                    &mut calls,
                    &mut preparation_calls,
                    &address_sc,
                    &mut call_already_prep,
                    &mut def_call_already_prep,
                );
            }
        }
    }

    let mut final_preparation_calls = Vec::new();
    for call in preparation_calls {
        final_preparation_calls.insert(0, call);
    }
    (final_preparation_calls, calls)
}

fn generate_call_as(
    abi: &[String],
    op_datastore: &Datastore,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    address_sc: &str,
    call_already_prep: &mut bool,
    def_call_cancel_counter: &mut u64,
) {
    use super::abi_generation::*;
    match abi[0].as_str() {
        "print" => generate_abi_print(rng, calls),
        "call" => generate_abi_call(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "localCall" => generate_abi_local_call(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "localExecution" => {
            generate_abi_local_execution(calls, call_already_prep)
        }
        "getBytecode" => generate_abi_get_bytecode(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "getBytecodeOf" => generate_abi_get_bytecode_of(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "callerHasWriteAccess" => generate_abi_caller_has_write_access(calls),
        "functionExists" => generate_abi_function_exists(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "remainingGas" => generate_abi_remaining_gas(calls),
        "createSC" => generate_abi_create_sc(calls),
        "getKeys" => generate_abi_get_keys(calls),
        "getKeysOf" => generate_abi_get_keys_of(address_sc, calls),
        "set" => generate_abi_set(rng, calls),
        "setOf" => generate_abi_set_of(address_sc, rng, calls),
        "get" => generate_abi_get(rng, calls, preparation_calls),
        "getOf" => {
            generate_abi_get_of(address_sc, rng, calls, preparation_calls)
        }
        "del" => generate_abi_del(rng, calls, preparation_calls),
        "deleteOf" => {
            generate_abi_del_of(address_sc, rng, calls, preparation_calls)
        }
        "append" => generate_abi_append(rng, calls, preparation_calls),
        "appendOf" => {
            generate_abi_append_of(address_sc, rng, calls, preparation_calls)
        }
        "has" => generate_abi_has(rng, calls, preparation_calls),
        "hasOf" => {
            generate_abi_has_of(address_sc, rng, calls, preparation_calls)
        }
        "ownedAddresses" => generate_abi_owned_addresses(calls),
        "callStack" => generate_abi_call_stack(calls),
        "generateEvent" => generate_abi_generate_event(rng, calls),
        "transferCoins" => generate_abi_transfer_coins(rng, calls),
        "transferCoinsOf" => {
            generate_abi_transfer_coins_of(address_sc, rng, calls)
        }
        "balance" => generate_abi_balance(calls),
        "balanceOf" => generate_abi_balance_of(address_sc, calls),
        "callCoins" => generate_abi_call_coins(calls),
        "blake3" => generate_abi_blake3(rng, calls),
        "sha256" => generate_abi_hash_sha256(rng, calls),
        "isSignatureValid" => generate_abi_is_signature_valid(rng, calls),
        "publicKeyToAddress" => generate_abi_public_key_to_address(calls),
        "time" => generate_abi_time(calls),
        "unsafeRandom" => generate_abi_unsafe_random(calls),
        "sendMessage" => generate_abi_send_message(address_sc, rng, calls),
        "currentPeriod" => generate_abi_current_period(calls),
        "currentThread" => generate_abi_current_thread(calls),
        "setBytecode" => generate_abi_set_bytecode(calls, call_already_prep),
        "setBytecodeOf" => {
            generate_abi_set_bytecode_of(address_sc, calls, call_already_prep)
        }
        "validateAddress" => generate_abi_validate_address(address_sc, calls),
        "getOpKeys" => generate_abi_get_op_keys(calls),
        "hasOpKey" => generate_abi_has_op_key(op_datastore, rng, calls),
        "getOpData" => generate_abi_get_op_data(op_datastore, rng, calls),
        "seed" => calls.push("seed();".to_string()),
        "deferredCallQuote" => generate_abi_deferred_call_quote(rng, calls),
        "deferredCallRegister" => {
            generate_abi_deferred_call_register(address_sc, rng, calls)
        }
        "deferredCallExists" => generate_abi_deferred_call_exists(rng, calls),
        "deferredCallCancel" => generate_abi_deferred_call_cancel(
            address_sc,
            rng,
            calls,
            preparation_calls,
            def_call_cancel_counter,
        ),
        "Date.now" => calls.push("Date.now();".to_string()),
        _ => {
            println!(
                "Assembly script ABI: {} don't have any generation function.",
                abi[0].as_str()
            );
            // println!(
            //     "Please add one in src/sc_generation/generation.rs:{}",
            //     line!()
            // );
            // println!("Calibrating process aborted.");
            // process::exit(1);
        }
    }
}

fn generate_call_wasmv1(
    abi: &[String],
    op_datastore: &Datastore,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    address_sc: &str,
    call_already_prep: &mut bool,
    def_call_already_prep: &mut u64,
) {
    use super::abi_wasmv1_generation::*;
    match abi[0].as_str() {
        "abi_set_ds_value" => generate_abi_set_ds_value(calls),
        "abi_get_ds_value" => {
            generate_abi_get_ds_value(calls, preparation_calls)
        }
        "abi_delete_ds_entry" => {
            generate_abi_delete_ds_entry(calls, preparation_calls)
        }
        "abi_append_ds_value" => {
            generate_abi_append_ds_value(calls, preparation_calls)
        }
        "abi_ds_entry_exists" => {
            generate_abi_ds_entry_exists(calls, preparation_calls)
        }
        "abi_get_balance" => generate_abi_get_balance(calls),
        "abi_get_bytecode" => generate_abi_get_bytecode(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "abi_set_bytecode" => {
            generate_abi_set_bytecode(calls, call_already_prep)
        }
        "abi_get_ds_keys" => generate_abi_get_ds_keys(calls),
        "abi_get_op_keys" => generate_abi_get_op_keys(calls),
        "abi_op_entry_exists" => {
            generate_abi_op_entry_exists(op_datastore, calls)
        }
        "abi_get_op_data" => {
            generate_abi_get_op_data(op_datastore, calls, call_already_prep)
        }
        "abi_call" => generate_abi_call(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "abi_create_sc" => generate_abi_create_sc(calls, call_already_prep),
        "abi_transfer_coins" => generate_abi_transfer_coins(calls),
        "abi_generate_event" => generate_abi_generate_event(calls),
        "abi_abort" => generate_abi_abort(),
        "abi_get_current_slot" => generate_abi_get_current_slot(calls),
        "abi_hash_sha256" => generate_abi_hash_sha256(calls),
        "abi_hash_keccak256" => generate_abi_hash_keccak256(calls),
        "abi_hash_blake3" => generate_abi_hash_blake3(calls),
        "abi_get_remaining_gas" => generate_abi_get_remaining_gas(calls),
        "abi_get_owned_addresses" => generate_abi_get_owned_addresses(calls),
        "abi_get_deferred_call_quote" => {
            generate_abi_deferred_call_quote(calls);
        }
        "abi_deferred_call_register" => {
            generate_abi_deferred_call_register(address_sc, calls);
        }
        "abi_deferred_call_cancel" => generate_abi_deferred_call_cancel(
            address_sc,
            calls,
            preparation_calls,
            def_call_already_prep,
        ),
        "abi_deferred_call_exists" => generate_abi_deferred_call_exists(calls),
        "abi_get_call_stack" => generate_abi_get_call_stack(calls),
        "abi_address_from_public_key" => {
            generate_abi_address_from_public_key(calls)
        }
        "abi_unsafe_random" => generate_abi_unsafe_random(calls),
        "abi_get_call_coins" => generate_abi_get_call_coins(calls),
        "abi_get_native_time" => generate_abi_get_native_time(calls),
        "abi_send_async_message" => {
            generate_abi_send_async_message(address_sc, calls)
        }
        "abi_get_origin_operation_id" => {
            generate_abi_get_origin_operation_id(calls)
        }
        "abi_local_execution" => {
            generate_abi_local_execution(calls, call_already_prep)
        }
        "abi_caller_has_write_access" => {
            generate_abi_caller_has_write_access(calls)
        }
        "abi_check_native_amount" => generate_abi_check_native_amount(calls),
        "abi_add_native_amount" => generate_abi_add_native_amount(calls),
        "abi_sub_native_amount" => generate_abi_sub_native_amount(calls),
        "abi_scalar_mul_native_amount" => {
            generate_abi_scalar_mul_native_amount(calls)
        }
        "abi_scalar_div_rem_native_amount" => {
            generate_abi_scalar_div_rem_native_amount(calls)
        }
        "abi_div_rem_native_amount" => {
            generate_abi_div_rem_native_amount(calls)
        }
        "abi_native_amount_to_string" => {
            generate_abi_native_amount_to_string(calls)
        }
        "abi_native_amount_from_string" => {
            generate_abi_native_amount_from_string(calls)
        }
        "abi_base58_check_to_bytes" => {
            generate_abi_base58_check_to_bytes(calls)
        }
        "abi_bytes_to_base58_check" => {
            generate_abi_bytes_to_base58_check(calls)
        }
        "abi_check_address" => generate_abi_check_address(calls),
        "abi_check_pubkey" => generate_abi_check_pubkey(calls),
        "abi_check_signature" => generate_abi_check_signature(calls),
        "abi_get_address_category" => generate_abi_get_address_category(calls),
        "abi_get_address_version" => generate_abi_get_address_version(calls),
        "abi_get_pubkey_version" => generate_abi_get_pubkey_version(calls),
        "abi_get_signature_version" => {
            generate_abi_get_signature_version(calls)
        }
        "abi_checked_add_native_time" => {
            generate_abi_checked_add_native_time(calls)
        }
        "abi_checked_sub_native_time" => {
            generate_abi_checked_sub_native_time(calls)
        }
        "abi_checked_mul_native_time" => {
            generate_abi_checked_mul_native_time(calls)
        }
        "abi_checked_scalar_div_native_time" => {
            generate_abi_checked_scalar_div_native_time(calls)
        }
        "abi_checked_div_native_time" => {
            generate_abi_checked_div_native_time(calls)
        }
        "abi_compare_address" => generate_abi_compare_address(calls),
        "abi_compare_native_amount" => {
            generate_abi_compare_native_amount(calls)
        }
        "abi_compare_native_time" => generate_abi_compare_native_time(calls),
        "abi_compare_pub_key" => generate_abi_compare_pub_key(calls),
        "abi_verify_signature" => generate_abi_verify_signature(calls),
        "abi_local_call" => generate_abi_local_call(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "abi_function_exists" => generate_abi_function_exists(
            address_sc,
            calls,
            preparation_calls,
            call_already_prep,
        ),
        "abi_evm_verify_signature" => generate_abi_evm_verify_signature(calls),
        "abi_evm_get_address_from_pubkey" => {
            generate_abi_evm_get_address_from_pubkey(calls)
        }
        "abi_evm_get_pubkey_from_signature" => {
            generate_abi_evm_get_pubkey_from_signature(calls)
        }
        "abi_is_address_eoa" => generate_abi_is_address_eoa(calls),
        _ => {
            println!(
                "WasmV1 ABI: {} don't have any generation function.",
                abi[0].as_str()
            );
            println!(
                "Please add one in src/sc_generation/generation.rs:{}",
                line!()
            );
            println!("Calibrating process aborted.");
            process::exit(1);
        }
    }
}

pub fn generate_instruction(
    limit_per_calls: u64,
) -> (Vec<String>, Vec<String>) {
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
