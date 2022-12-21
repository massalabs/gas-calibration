use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use massa_models::datastore::Datastore;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::sc_generation::generation::generate_calls;

use self::generation::generate_instruction;

pub mod abis;
pub mod generation;

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

fn write_sc(calls: Vec<String>, file_name: String) {
    let template_index = format!(
        "import {{env}} from '../env';
        import {{ toBytes, fromBytes }} from '../helpers';

export function main(): void {{
{}
}}",
        calls.join("\n")
    );
    let mut output = File::create("./src/sc_generation/template/index.ts").unwrap();
    write!(output, "{}", template_index).unwrap();
    let mut src = File::create(format!(
        "./src/sc_generation/template/build/SC_{}.ts",
        file_name
    ))
    .unwrap();
    write!(src, "{}", template_index).unwrap();
}

fn write_wat(setup_calls: Vec<String>, calls: Vec<String>, file_name: String) {
    let template_index = format!(
        "(module
            (memory $0 1)
            (export \"memory\" (memory $0))
            {}
            (func (export \"main\") (result)
{}
        ))",
        setup_calls.join("\n"),
        calls.join("\n")
    );
    let mut src = File::create(format!(
        "./src/sc_generation/template/build/WAT_{}.wat",
        file_name
    ))
    .unwrap();
    write!(src, "{}", template_index).unwrap();
}

pub fn generate_scs(nb_sc_per_abi: u32, limit_per_calls_per_sc: u64, op_datastore: Datastore) {
    let abis = abis::get_abis();
    println!("Generating {} smart contracts for each abi", nb_sc_per_abi);
    let mut pb = pbr::ProgressBar::new(abis.len() as u64);
    for (index_abi, abi) in abis.iter().enumerate() {
        (0..nb_sc_per_abi).into_par_iter().for_each(|i| {
            if index_abi < 3 || index_abi > 7 {
                return;
            }
            let op_datastore_clone = op_datastore.clone();
            let (preparation_calls, calls) =
                generate_calls(abi.clone(), limit_per_calls_per_sc, op_datastore_clone);
            if !preparation_calls.is_empty() {
                write_sc(
                    preparation_calls,
                    format!("preparation_{}", ((index_abi as u32 * nb_sc_per_abi) + i)),
                );
            }
            write_sc(calls, ((index_abi as u32 * nb_sc_per_abi) + i).to_string());
        });
        pb.inc();
    }
    pb.finish_print("Finish generating SCs");
}

pub fn build_scs(nb_sc_per_abi: u32, abis: Vec<Vec<String>>) {
    println!(
        "building {} smart contracts...",
        nb_sc_per_abi * abis.len() as u32
    );
    (0..(nb_sc_per_abi * abis.len() as u32))
        .into_par_iter()
        .for_each(|i| {
            if i < 3 * nb_sc_per_abi || i > 7 * nb_sc_per_abi {
                return;
            }
            let output1 = Command::new("npm")
                .arg("run")
                .arg("build")
                .env("SC_NAME", format!("SC_preparation_{}", i))
                .current_dir("./src/sc_generation/template")
                .output()
                .expect("failed to execute process");
            //std::io::stderr().write_all(&output1.stderr).unwrap();
            let output = Command::new("npm")
                .arg("run")
                .arg("build")
                .env("SC_NAME", format!("SC_{}", i))
                .current_dir("./src/sc_generation/template")
                .output()
                .expect("failed to execute process");
            std::io::stderr().write_all(&output.stderr).unwrap();
        });
}

pub fn generate_wasm_scs(nb_contracts: u32, max_calls_per_contract: u64) {
    (0..nb_contracts).into_par_iter().for_each(|i| {
        let (setup_calls, calls) = generate_instruction(max_calls_per_contract);
        write_wat(setup_calls, calls, format!("{}", i));
    });
}
