use std::{fs::File, str::FromStr};

use massa_models::{address::Address, config::THREAD_COUNT, datastore::Datastore};
use massa_signature::KeyPair;
use rand::Rng;
use std::io::Write;

fn _generate_u8_array(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut array = Vec::new();
    for _ in 0..length {
        array.push(rng.gen());
    }
    array
}

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn static_public_key() -> String {
    let keypair =
        KeyPair::from_str("S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY").unwrap();
    keypair.get_public_key().to_bs58_check()
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    // Address: A12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M
}

fn static_address() -> String {
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    String::from("A12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M")
}

fn generate_address() -> String {
    let keypair = KeyPair::generate();

    Address::from_public_key(&keypair.get_public_key()).to_string()
}

pub fn generate_op_datastore() -> Datastore {
    let mut rng = rand::thread_rng();
    let mut datastore: Datastore = Datastore::new();
    let nb_entries = 100;
    for _ in 0..nb_entries + 1 {
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
    let mut saved_key = String::new();
    let mut preparation_calls = Vec::new();

    let nb_calls = rng.gen_range(0..limit_per_calls);
    for index_call in 0..nb_calls {
        let mut call = format!("env.{}", abi[0].clone());
        call.push('(');
        for i in 1..abi.len() - 1 {
            if abi[i].is_empty() {
                break;
            }
            let mut splitted_params = abi[i].as_str().split(": ");
            let arg = match (
                splitted_params.next().unwrap(),
                splitted_params.next().unwrap(),
            ) {
                ("to", "string") => format!("\"{}\"", generate_address()),
                ("address" | "from", "string") => format!("\"{}\"", static_address()),
                ("publicKey", "string") => format!("\"{}\"", static_public_key()),
                ("key", "StaticArray<u8>") => {
                    if abi[0] == "getOpData" || abi[0] == "hasOpKey" {
                        let index_key = rng.gen_range(0..op_datastore.len());
                        //op things
                        // Get key at index_key in datastore
                        let key = op_datastore
                            .clone()
                            .into_iter()
                            .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
                            .get(index_key)
                            .unwrap()
                            .0
                            .clone();
                        let key: Vec<u16> = key
                            .chunks_exact(2)
                            .into_iter()
                            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                            .collect();
                        let key = key.as_slice();
                        format!("toBytes(\"{}\")", String::from_utf16_lossy(&key))
                    } else {
                        // Storage things
                        let mut key = generate_string(rng.gen_range(5..32));
                        if abi[0] == "set" {
                            saved_key = key.clone();
                        } else if abi[0] == "get"
                            || abi[0] == "getOf"
                            || abi[0] == "append"
                            || abi[0] == "appendOf"
                            || abi[0] == "del"
                            || abi[0] == "deleteOf"
                        {
                            if saved_key.is_empty() {
                                preparation_calls.push((
                                    "set",
                                    format!(
                                        "toBytes(\"{}\"), toBytes(\"{}\")",
                                        key,
                                        generate_string(rng.gen_range(1..1000))
                                    ),
                                    index_call,
                                ));
                            } else {
                                key = saved_key.clone();
                                saved_key = String::new();
                            }
                        }
                        format!("toBytes(\"{}\")", key)
                    }
                }
                ("bytecode", "string") => format!(
                    "\"{}\"",
                    base64::encode(generate_string(rng.gen_range(0..1000)))
                ),
                (_, "string") => format!("\"{}\"", generate_string(rng.gen_range(0..255))),
                (_, "StaticArray<u8>") => {
                    format!("toBytes(\"{}\")", generate_string(rng.gen_range(0..1000)))
                }
                ("amount", "u64") => rng
                    .gen_range::<u64, _>(100_000_000..1_000_000_000)
                    .to_string(),
                ("coins", "u64") => rng
                    .gen_range::<u64, _>(100_000_000..1_000_000_000)
                    .to_string(),
                ("maxGas", "u64") => rng.gen_range::<u64, _>(100_000..300_000).to_string(),
                ("rawFee", "u64") => rng.gen_range::<u64, _>(1..4).to_string(),
                ("validityStartPeriod", "u64") => rng.gen_range::<u64, _>(10..100).to_string(),
                ("validityEndPeriod", "u64") => rng.gen_range::<u64, _>(100..1000).to_string(),
                (_, "u64") => rng.gen::<u64>().to_string(),
                ("thread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                ("validityStartThread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                ("validityEndThread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                (_, "u8") => rng.gen::<u8>().to_string(),
                (_, "boolean") => rng.gen::<bool>().to_string(),
                (a, b) => {
                    panic!("Unknown type: {} {}", a, b);
                }
            };
            call.push_str(&arg);
            call.push(',');
        }
        if call.ends_with(',') {
            call.pop();
        }
        call.push(')');
        call.push(';');
        calls.push(call);
    }

    let mut final_preparation_calls = Vec::new();
    for (_i, (abi, key, _index_call)) in preparation_calls.iter().enumerate() {
        let call = format!("env.{}({});", abi, key);
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
