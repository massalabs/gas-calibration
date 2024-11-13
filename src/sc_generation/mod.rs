use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::Command;

use assembly_script::write_sc_as;
use massa_models::datastore::Datastore;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use wasmv1::write_sc_wasmv1;

use crate::config::generate_dir;
use crate::sc_generation::generation::generate_calls;
use crate::AbiType;

use self::generation::generate_instruction;

mod abi_generation;
mod abi_wasmv1_generation;

pub mod abis;
pub mod generation;

mod assembly_script;
mod wasmv1;

use which::which;

pub fn read_existing_op_datastore() -> Datastore {
    let mut file = File::open("./src/sc_generation/template/op_datastore.json")
        .expect("Failed to open op_datastore.json");
    let mut op_datastore_json = String::new();
    file.read_to_string(&mut op_datastore_json)
        .expect("Failed to read op_datastore.json");
    let datastore_vec: Vec<(Vec<u8>, Vec<u8>)> =
        serde_json::from_str(&op_datastore_json).unwrap();
    let mut datastore = Datastore::new();
    for (key, value) in datastore_vec {
        datastore.insert(key, value);
    }
    datastore
}

fn write_sc(calls: Vec<String>, abi_type: &AbiType, file_name: &str) {
    let template_index = match abi_type {
        AbiType::AS => write_sc_as(calls),
        AbiType::WasmV1 => write_sc_wasmv1(calls),
    };
    let mut output =
        File::create("./src/sc_generation/template/index.ts").unwrap();

    let output_dir = generate_dir(abi_type);

    fs::create_dir_all(&output_dir).unwrap();

    write!(output, "{}", template_index).unwrap();
    let sc_filename = format!("SC_{}.ts", file_name);
    let mut src = File::create(output_dir.join(sc_filename)).unwrap();
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

pub fn generate_scs(
    nb_sc_per_abi: u32,
    limit_per_calls_per_sc: u64,
    op_datastore: &Datastore,
    abi_type: &AbiType,
    abis: &[Vec<String>],
) {
    println!(
        "Generating {} smart contracts for each `{}` abi",
        nb_sc_per_abi, abi_type
    );
    let mut pb = pbr::ProgressBar::new(abis.len() as u64);
    for (index_abi, abi) in abis.iter().enumerate() {
        (0..nb_sc_per_abi).into_par_iter().for_each(|i| {
            // if index_abi < 3 || index_abi > 3 {
            //     return;
            // }
            // let op_datastore_clone = op_datastore.clone();
            let (preparation_calls, calls) = generate_calls(
                abi_type,
                abi.clone(),
                limit_per_calls_per_sc,
                op_datastore,
            );
            if !preparation_calls.is_empty() {
                write_sc(
                    preparation_calls,
                    abi_type,
                    &format!(
                        "preparation_{}",
                        ((index_abi as u32 * nb_sc_per_abi) + i)
                    ),
                );
            }
            write_sc(
                calls,
                abi_type,
                &((index_abi as u32 * nb_sc_per_abi) + i).to_string(),
            );
        });
        pb.inc();
    }
    pb.finish_print("End of SC generation");
}

pub fn build_scs(nb_sc_per_abi: u32, abi_type: &AbiType, abis: &[Vec<String>]) {
    println!(
        "building {} smart contracts...",
        nb_sc_per_abi * abis.len() as u32
    );
    (0..(nb_sc_per_abi * abis.len() as u32))
        .into_par_iter()
        .for_each(|i| {
            // if i < 3 * nb_sc_per_abi || i > 3 * nb_sc_per_abi {
            //     return;
            // }
            let npm_path = which("npm").expect("npm not found in PATH");
            let build_script = if cfg!(target_os = "windows") {
                "build_windows"
            } else {
                "build"
            };

            let cur_dir = generate_dir(abi_type);
            // dbg!(&cur_dir);

            Command::new(npm_path.clone())
                .arg("run")
                .arg(build_script)
                .env("SC_NAME", format!("SC_preparation_{}", i))
                .current_dir(&cur_dir)
                .output()
                .expect("failed to execute process");
            // std::io::stderr().write_all(&output1.stderr).unwrap();
            let output = Command::new(npm_path)
                .arg("run")
                .arg(build_script)
                .env("SC_NAME", format!("SC_{}", i))
                .current_dir(&cur_dir)
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
