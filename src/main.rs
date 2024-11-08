use std::{
    collections::HashMap,
    fmt,
    path::Path,
    process::{self, Command},
    time::Duration,
};

use clap::Parser;
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

const TEMPLATE_DIR: &str = "./src/sc_generation/template";

fn main() {
    let args = args::Args::parse();
    // let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(100);
    let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(1);
    let nb_wasm_scs = 0;

    let npm_path = which("npm").expect("npm not found in PATH");
    Command::new(npm_path.clone())
        .arg("update")
        .current_dir(TEMPLATE_DIR)
        .output()
        .expect("failed to execute process");

    Command::new(npm_path.clone())
        .arg("install")
        .current_dir(TEMPLATE_DIR)
        .output()
        .expect("failed to execute process");

    let template_dir = Path::new(TEMPLATE_DIR);
    let as_env_path = template_dir.join("as").join("env.ts");
    let wasmv1_env_path = template_dir.join("wasmv1").join("env_wasmv1.ts");

    if !as_env_path.exists() {
        panic!("env.ts not found in template directory");
    }

    if !wasmv1_env_path.exists() {
        panic!("env_wasmv1.ts not found in template directory");
    }

    let as_abis = abis::get_abis(&as_env_path);
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
            true,
        );
    }
    process::exit(0);

    let env_path = args
        .as_sdk_env_path
        .unwrap_or(String::from("./src/sc_generation/template/env.ts"));
    // .unwrap_or(String::from("./src/sc_generation/template/env_wasmv1.ts"));
    // Copy the env file to the current directory
    // TODO: Improve
    std::fs::copy(
        "./src/sc_generation/template/env.ts",
        "./src/sc_generation/template/env.ts.bak",
    )
    .unwrap();
    // std::fs::copy(env_path.clone(),
    // "./src/sc_generation/template/env.ts").unwrap();
    let abis = sc_generation::abis::get_abis(&env_path);
    if args.only_generate {
        let datastore = sc_generation::generation::generate_op_datastore();
        // let datastore = sc_generation::read_existing_op_datastore();
        sc_generation::generate_scs(
            nb_scs_by_abi,
            300,
            &datastore,
            &AbiType::AS,
            &abis,
        );
        std::fs::copy(
            "./src/sc_generation/template/env.ts.bak",
            "./src/sc_generation/template/env.ts",
        )
        .unwrap();
        return;
    }
    let op_datastore = if args.skip_generation_scs {
        // sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        sc_generation::read_existing_op_datastore()
    } else {
        let datastore = sc_generation::generation::generate_op_datastore();
        // let datastore = sc_generation::read_existing_op_datastore();
        sc_generation::generate_scs(
            nb_scs_by_abi,
            300,
            &datastore,
            &AbiType::AS,
            &abis,
        );
        sc_generation::build_scs(nb_scs_by_abi, &AbiType::AS, &abis);
        sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        datastore
    };
    std::fs::copy(
        "./src/sc_generation/template/env.ts.bak",
        "./src/sc_generation/template/env.ts",
    )
    .unwrap();
    let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    execution::execute_abi_scs(
        &mut full_results,
        nb_scs_by_abi,
        &op_datastore,
        &AbiType::AS,
        &abis,
    );
    compile_and_write_results(
        full_results,
        u32::MAX,
        Duration::from_millis(300),
        true,
    );

    // Not executing WAT SCs, as the new runtime does not support them out of
    // the box
    // let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    // execution::execute_wasm_scs(&mut full_results, nb_wasm_scs);
    // compile_and_write_results(full_results, u32::MAX,
    // Duration::from_millis(300), false);
}
