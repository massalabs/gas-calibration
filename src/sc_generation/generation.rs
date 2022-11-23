use std::{fs::File, str::FromStr};

use massa_models::{address::Address, config::THREAD_COUNT, datastore::Datastore};
use massa_signature::KeyPair;
use rand::Rng;
use std::io::Write;

fn generate_u8_array(length: usize) -> Vec<u8> {
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
        string.push(rng.gen_range('a'..'z'));
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
        let key = generate_u8_array(rng.gen_range(5..32));
        let value = generate_u8_array(rng.gen_range(1..100));
        datastore.insert(key, value);
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
            if abi[i] == "" {
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
                ("key", "string") => {
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
                                    "\"{}\", \"{}\"",
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
                    format!("\"{}\"", key)
                }
                ("bytecode", "string") => format!(
                    "\"{}\"",
                    base64::encode(generate_string(rng.gen_range(0..1000)))
                ),
                (_, "string") => format!("\"{}\"", generate_string(rng.gen_range(0..1000))),
                ("amount", "u64") => rng
                    .gen_range::<u64, _>(100_000_000..1_000_000_000)
                    .to_string(),
                ("coins", "u64") => rng
                    .gen_range::<u64, _>(100_000_000..1_000_000_000)
                    .to_string(),
                ("maxGas", "u64") => rng.gen_range::<u64, _>(100_000..300_000).to_string(),
                ("gasPrice", "u64") => rng.gen_range::<u64, _>(1..4).to_string(),
                ("validityStartPeriod", "u64") => rng.gen_range::<u64, _>(10..100).to_string(),
                ("validityEndPeriod", "u64") => rng.gen_range::<u64, _>(100..1000).to_string(),
                (_, "u64") => rng.gen::<u64>().to_string(),
                ("thread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                ("validityStartThread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                ("validityEndThread", "u8") => rng.gen_range(0..THREAD_COUNT).to_string(),
                (_, "u8") => rng.gen::<u8>().to_string(),
                (_, "boolean") => rng.gen::<bool>().to_string(),
                (_, "StaticArray<u8>") => {
                    let key_index = rng.gen_range(0..100);
                    let key = op_datastore.keys().collect::<Vec<&Vec<u8>>>()[key_index];
                    format!("{:#?}", key)
                }
                _ => panic!("Unknown type"),
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

pub fn generate_instruction(limit_per_calls: u64) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let nb_calls = rng.gen_range(0..limit_per_calls);
    let mut instructions = Vec::new();

    for _ in 0..nb_calls {
        let left_operand = rng.gen_range(1..i32::MAX);
        let right_operand = rng.gen_range(1..i32::MAX);
        let instruction = match rng.gen_range(0..4) {
            0 => format!("{} + {}", left_operand, right_operand),
            1 => format!("{} - {}", left_operand, right_operand),
            2 => format!("{} * {}", left_operand, right_operand),
            3 => format!("{} / {}", left_operand, right_operand),
            _ => panic!("Unknown instruction"),
        };
        instructions.push(instruction);
    }
    return instructions;
}
