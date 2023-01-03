use std::fs::File;

use massa_models::datastore::Datastore;
use rand::Rng;
use std::io::Write;

use super::abi_generation::{
    generate_abi_append, generate_abi_append_of, generate_abi_balance, generate_abi_balance_of,
    generate_abi_call, generate_abi_call_coins, generate_abi_call_stack,
    generate_abi_caller_has_write_access, generate_abi_create_sc, generate_abi_current_period,
    generate_abi_current_thread, generate_abi_del, generate_abi_del_of,
    generate_abi_function_exists, generate_abi_generate_event, generate_abi_get,
    generate_abi_get_bytecode, generate_abi_get_bytecode_for, generate_abi_get_keys,
    generate_abi_get_keys_of, generate_abi_get_of, generate_abi_get_op_data,
    generate_abi_get_op_keys, generate_abi_has, generate_abi_has_of, generate_abi_has_op_key,
    generate_abi_is_signature_valid, generate_abi_local_call, generate_abi_local_execution,
    generate_abi_owned_addresses, generate_abi_print, generate_abi_public_key_to_address,
    generate_abi_remaining_gas, generate_abi_send_message, generate_abi_set,
    generate_abi_set_bytecode, generate_abi_set_bytecode_of, generate_abi_set_of,
    generate_abi_time, generate_abi_to_base58, generate_abi_transfer_coins,
    generate_abi_transfer_coins_of, generate_abi_unsafe_random,
};

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn static_address() -> String {
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    String::from("A12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
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
        match std::fs::read("./src/sc_generation/template/empty_main_sc.wasm") {
            Ok(bytes) => datastore.insert(key, bytes),
            Err(e) => panic!("{}", e),
        }
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
            "print" => generate_abi_print(&mut rng, &mut calls),
            "call" => generate_abi_call(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "localCall" => generate_abi_local_call(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "localExecution" => generate_abi_local_execution(&mut calls, &mut call_already_prep),
            "getBytecode" => generate_abi_get_bytecode(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "getBytecodeFor" => generate_abi_get_bytecode_for(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "callerHasWriteAccess" => generate_abi_caller_has_write_access(&mut calls),
            "functionExists" => generate_abi_function_exists(
                &address_sc,
                &mut calls,
                &mut preparation_calls,
                &mut call_already_prep,
            ),
            "remainingGas" => generate_abi_remaining_gas(&mut calls),
            "createSC" => generate_abi_create_sc(&mut calls),
            "getKeys" => generate_abi_get_keys(&mut calls),
            "getKeysOf" => generate_abi_get_keys_of(&address_sc, &mut calls),
            "set" => generate_abi_set(&mut rng, &mut calls),
            "setOf" => generate_abi_set_of(&address_sc, &mut rng, &mut calls),
            "get" => generate_abi_get(&mut rng, &mut calls, &mut preparation_calls),
            "getOf" => {
                generate_abi_get_of(&address_sc, &mut rng, &mut calls, &mut preparation_calls)
            }
            "del" => generate_abi_del(&mut rng, &mut calls, &mut preparation_calls),
            "delOf" => {
                generate_abi_del_of(&address_sc, &mut rng, &mut calls, &mut preparation_calls)
            }
            "append" => generate_abi_append(&mut rng, &mut calls, &mut preparation_calls),
            "appendOf" => {
                generate_abi_append_of(&address_sc, &mut rng, &mut calls, &mut preparation_calls)
            }
            "has" => generate_abi_has(&mut rng, &mut calls, &mut preparation_calls),
            "hasOf" => {
                generate_abi_has_of(&address_sc, &mut rng, &mut calls, &mut preparation_calls)
            }
            "ownedAddresses" => generate_abi_owned_addresses(&mut calls),
            "callStack" => generate_abi_call_stack(&mut calls),
            "generateEvent" => generate_abi_generate_event(&mut rng, &mut calls),
            "transferCoins" => generate_abi_transfer_coins(&mut rng, &mut calls),
            "transferCoinsOf" => generate_abi_transfer_coins_of(&address_sc, &mut rng, &mut calls),
            "balance" => generate_abi_balance(&mut calls),
            "balanceOf" => generate_abi_balance_of(&address_sc, &mut calls),
            "callCoins" => generate_abi_call_coins(&mut calls),
            "toBase58" => generate_abi_to_base58(&mut rng, &mut calls),
            "isSignatureValid" => generate_abi_is_signature_valid(&mut rng, &mut calls),
            "publicKeyToAddress" => generate_abi_public_key_to_address(&mut calls),
            "time" => generate_abi_time(&mut calls),
            "unsafeRandom" => generate_abi_unsafe_random(&mut calls),
            "sendMessage" => generate_abi_send_message(&address_sc, &mut rng, &mut calls),
            "currentPeriod" => generate_abi_current_period(&mut calls),
            "currentThread" => generate_abi_current_thread(&mut calls),
            "setBytecode" => generate_abi_set_bytecode(&mut rng, &mut calls),
            "setBytecodeOf" => generate_abi_set_bytecode_of(&address_sc, &mut rng, &mut calls),
            "getOpKeys" => generate_abi_get_op_keys(&mut calls),
            "hasOpKey" => generate_abi_has_op_key(&op_datastore, &mut rng, &mut calls),
            "getOpData" => generate_abi_get_op_data(&op_datastore, &mut rng, &mut calls),
            "seed" => calls.push("seed();".to_string()),
            "Date.now" => calls.push("Date.now();".to_string()),
            _ => {}
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

    let operations = vec![
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
        let gen_first_operand = if rng.gen_range(0..2) == 1 {
            true
        } else {
            false
        };
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
