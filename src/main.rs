use std::{
    collections::HashMap, fmt, fs, path::Path, process::Command, time::Duration,
};

use clap::Parser;
use config::{as_env_path, output_dir, template_dir, wasmv1_env_path};
use sc_generation::abis;
use which::which;

use crate::calculation::compile_and_write_results;

mod args;
mod calculation;
mod execute_batch_sc;
mod execution;
mod sc_generation;

enum AbiType {
    AS,
    WasmV1,
}

impl fmt::Display for AbiType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbiType::AS => write!(f, "assembly script"),
            AbiType::WasmV1 => write!(f, "WasmV1"),
        }
    }
}
mod config;

fn main() {
    let args = args::Args::parse();
    // let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(100);
    let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(1);
    let nb_wasm_scs = 0;

    let as_env_path = as_env_path();
    let wasmv1_env_path = wasmv1_env_path();

    initialize_calibration_environment(&as_env_path, &wasmv1_env_path);

    println!("############################################################");
    println!("Reading ABIs from env.ts");
    let as_abis = abis::get_abis(&as_env_path);
    println!("############################################################");
    println!("Reading ABIs from env_wasmv1.ts");
    let wasmv1_abis = abis::get_abis(&wasmv1_env_path);

    if args.only_generate {
        let datastore = sc_generation::generation::generate_op_datastore();
        for abis in [(AbiType::AS, &as_abis), (AbiType::WasmV1, &wasmv1_abis)] {
            sc_generation::generate_scs(
                nb_scs_by_abi,
                300,
                &datastore,
                &abis.0,
                abis.1,
            );
        }
        return;
    }

    let op_datastore = if args.skip_generation_scs {
        sc_generation::read_existing_op_datastore()
    } else {
        let datastore = sc_generation::generation::generate_op_datastore();
        for abis in [(AbiType::AS, &as_abis), (AbiType::WasmV1, &wasmv1_abis)] {
            sc_generation::generate_scs(
                nb_scs_by_abi,
                300,
                &datastore,
                &abis.0,
                abis.1,
            );
            sc_generation::build_scs(nb_scs_by_abi, &abis.0, abis.1);
            sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        }
        datastore
    };

    for abis in [(AbiType::AS, &as_abis), (AbiType::WasmV1, &wasmv1_abis)] {
        let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
        execution::execute_abi_scs(
            &mut full_results,
            nb_scs_by_abi,
            &op_datastore,
            &abis.0,
            abis.1,
        );
        compile_and_write_results(
            full_results,
            u32::MAX,
            Duration::from_millis(300),
            &abis.0,
        );
    }

    // Not executing WAT SCs, as the new runtime does not support them out of
    // the box
    // let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    // execution::execute_wasm_scs(&mut full_results, nb_wasm_scs);
    // compile_and_write_results(full_results, u32::MAX,
    // Duration::from_millis(300), false);
}

// copy templates to output directory
fn initialize_calibration_environment(
    as_env_path: &Path,
    wasmv1_env_path: &Path,
) {
    println!("############################################################");
    println!("Set assemblyscript env");
    let as_output_dir = output_dir(&AbiType::AS);
    let as_env_path_output = &as_output_dir.join("env.ts");
    fs::copy(as_env_path, as_env_path_output).unwrap();

    println!("############################################################");
    println!("Set wasmv1 env");
    let wasmv1_output_dir = output_dir(&AbiType::WasmV1);
    let wasmv1_env_path_output = &wasmv1_output_dir.join("env_wasmv1.ts");
    fs::copy(wasmv1_env_path, wasmv1_env_path_output).unwrap();

    let files = ["package.json", "helpers.ts"];
    for abi_type in &[AbiType::AS, AbiType::WasmV1] {
        let output_dir = output_dir(abi_type);
        for file in &files {
            fs::copy(template_dir(abi_type).join(file), output_dir.join(file))
                .unwrap();
        }
    }

    // npm install
    let npm_path = which("npm").expect("npm not found in PATH");
    npm_install_update(&npm_path, &AbiType::AS);
    npm_install_update(&npm_path, &AbiType::WasmV1);
}

fn npm_install_update(npm_path: &Path, abi_type: &AbiType) {
    Command::new(npm_path)
        .arg("update")
        .current_dir(output_dir(abi_type))
        .output()
        .expect("failed to execute process");

    Command::new(npm_path)
        .arg("install")
        .current_dir(output_dir(abi_type))
        .output()
        .expect("failed to execute process");
}
