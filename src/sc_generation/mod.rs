use std::fs::File;
use std::io::{Write, Read};
use std::process::Command;
use std::str::FromStr;

use massa_models::address::Address;
use massa_models::config::THREAD_COUNT;
use massa_models::datastore::Datastore;
use massa_signature::KeyPair;
use pbr::ProgressBar;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub mod abis;

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

pub fn read_existing_op_datastore() -> Datastore {
    let mut file = File::open("./src/sc_generation/template/op_datastore.json")
        .expect("Failed to open op_datastore.json");
    let mut op_datastore_json = String::new();
    file.read_to_string(&mut op_datastore_json)
        .expect("Failed to read op_datastore.json");
    let datastore_vec: Vec<(Vec<u8>, Vec<u8>)> = serde_json::from_str(&op_datastore_json).unwrap();
    let mut datastore = Datastore::new();
    for (key, value) in datastore_vec {
        datastore.insert(key, value);
    }
    datastore
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
    write!(output, "{}", serde_json::to_string(&datastore.clone().into_iter().collect::<Vec<(Vec<u8>, Vec<u8>)>>()).unwrap()).unwrap();
    datastore
}

fn generate_nb_each_call(max_nb: u64, max_call_index: u64) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut call_indexes = Vec::new();
    for i in 0..max_call_index {
        let nb_this_call = rng.gen_range(0..max_nb);
        for _ in 0..nb_this_call {
            call_indexes.push(i as usize);
        }
    }
    call_indexes
} 

fn generate_calls(
    abis: Vec<Vec<String>>,
    limit_per_calls: u64,
    op_datastore: Datastore,
) -> Vec<String> {
    let mut rng = rand::thread_rng();

    let mut calls = Vec::new();
    let mut saved_key = String::new();
    let mut calls_to_add = Vec::new();
    let call_indexes = generate_nb_each_call(limit_per_calls, abis.len() as u64);
    for (index_call, abi_index) in call_indexes.iter().enumerate() {
        let abi = abis[*abi_index].clone();
        if abi[0] == "call" {
            continue;
        }
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
                            calls_to_add.push((
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

    for (i, (abi, key, index_call)) in calls_to_add.iter().enumerate() {
        let call = format!("env.{}({});", abi, key);
        let index_call = (*index_call + i as usize) / 2;
        if index_call <= 0 {
            calls.insert(0, call);
        } else {
            calls.insert(rng.gen_range::<usize, _>(0..index_call as usize), call);
        }
    }
    calls
}

pub fn generate_scs(nb_sc: u32, limit_per_calls_per_sc: u64, op_datastore: Datastore) {
    let abis = abis::get_abis();
    println!("Generating {} smart contracts", nb_sc);
    let mut pb = ProgressBar::new(nb_sc as u64);
    (0..nb_sc).into_par_iter().for_each(|i| {
        let op_datastore_clone = op_datastore.clone();
        let abis_clone = abis.clone();
        let calls = generate_calls(abis_clone, limit_per_calls_per_sc, op_datastore_clone.clone());
        let template_index = format!(
            "import {{env}} from './env';

export function main(): void {{
    {}
}}",
            calls.join("\n")
        );
        let mut output = File::create("./src/sc_generation/template/index.ts").unwrap();
        write!(output, "{}", template_index).unwrap();
        let mut temp = File::create(format!("./src/sc_generation/template/build/SC_{}.ts", i)).unwrap();
        write!(temp, "{}", template_index).unwrap();
        Command::new("npm")
            .arg("run")
            .arg("build")
            .env("SC_NAME", format!("SC_{}", i))
            .current_dir("./src/sc_generation/template")
            .output()
            .unwrap();
        //pb.inc();
    });
    pb.finish_print(&format!("Generated {} smart contracts", nb_sc));
}
