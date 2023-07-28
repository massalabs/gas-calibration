use std::{collections::HashMap, process::Command, time::Duration};

use clap::Parser;
use which::which;

use crate::calculation::compile_and_write_results;

mod args;
mod calculation;
mod execute_batch_sc;
mod execution;
mod sc_generation;

fn main() {
    let args = args::Args::parse();
    let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(1);
    let nb_wasm_scs = 0;

    let npm_path = which("npm").expect("npm not found in PATH");
    Command::new(npm_path.clone())
        .arg("update")
        .current_dir("./src/sc_generation/template")
        .output()
        .expect("failed to execute process");

    Command::new(npm_path.clone())
        .arg("install")
        .current_dir("./src/sc_generation/template")
        .output()
        .expect("failed to execute process");

    let env_path = args
        .as_sdk_env_path
        .unwrap_or(String::from("./src/sc_generation/template/env_wasmv1.ts"));
    // Copy the env file to the current directory
    //TODO: Improve
    std::fs::copy(
        "./src/sc_generation/template/env_wasmv1.ts",
        "./src/sc_generation/template/env_wasmv1.ts.bak",
    )
    .unwrap();
    //std::fs::copy(env_path.clone(), "./src/sc_generation/template/env.ts").unwrap();
    let abis = sc_generation::abis::get_abis(&env_path);
    if args.only_generate {
        let datastore = sc_generation::generation::generate_op_datastore();
        //let datastore = sc_generation::read_existing_op_datastore();
        sc_generation::generate_scs(nb_scs_by_abi, 300, datastore.clone(), &env_path);
        std::fs::copy(
            "./src/sc_generation/template/env_wasmv1.ts.bak",
            "./src/sc_generation/template/env_wasmv1.ts",
        )
        .unwrap();
        return;
    }
    let op_datastore = if args.skip_generation_scs {
        //sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        sc_generation::read_existing_op_datastore()
    } else {
        let datastore = sc_generation::generation::generate_op_datastore();
        //let datastore = sc_generation::read_existing_op_datastore();
        sc_generation::generate_scs(nb_scs_by_abi, 300, datastore.clone(), &env_path);
        sc_generation::build_scs(nb_scs_by_abi, abis);
        sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        datastore
    };
    std::fs::copy(
        "./src/sc_generation/template/env_wasmv1.ts.bak",
        "./src/sc_generation/template/env_wasmv1.ts",
    )
    .unwrap();
    let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    execution::execute_abi_scs(&mut full_results, nb_scs_by_abi, op_datastore, &env_path);
    compile_and_write_results(full_results, u32::MAX, Duration::from_millis(300), true);

    // Not executing WAT SCs, as the new runtime does not support them out of the box
    /*let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    execution::execute_wasm_scs(&mut full_results, nb_wasm_scs);
    compile_and_write_results(full_results, u32::MAX, Duration::from_millis(300), false);*/
}
