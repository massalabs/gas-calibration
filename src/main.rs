use std::{
    fs, path::Path, process::Command, time::Duration,
};

use clap::Parser;
use config::{as_env_path, output_dir, template_dir, wasmv1_env_path};
use sc_generation::{
    abis::{self, AbisType},
    generate_wasm_scs,
};
use which::which;

use crate::calculation::compile_and_write_results;

mod args;
mod calculation;
mod config;
mod execute_batch_sc;
mod execution;
mod sc_generation;

fn main() {
    let args = args::Args::parse();
    // let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(100);
    let nb_scs_by_abi: u32 = args.nb_scs_by_abi.unwrap_or(1);
    let nb_instructions = 300;
    let nb_wasm_scs = 0;

    let as_env_path = as_env_path();
    let wasmv1_env_path = wasmv1_env_path();

    initialize_calibration_environment(&as_env_path, &wasmv1_env_path);

    let mut abis_list = [
        abis::get_as_abis(&as_env_path),
        abis::get_wasmv1_abis(&wasmv1_env_path),
    ];

    if args.only_generate {
        for abis in abis_list.iter_mut() {
            abis.read_existing_op_datastore();
            abis.generate_scs(nb_scs_by_abi, nb_instructions);
        }
        return;
    }

    if args.skip_generation_scs {
        for abis in abis_list.iter_mut() {
            abis.read_existing_op_datastore();
        }
    } else {
        for abis in abis_list.iter_mut() {
            abis.generate_op_datastore();
            abis.generate_scs(nb_scs_by_abi, nb_instructions);
            abis.build_scs(nb_scs_by_abi);

            generate_wasm_scs(nb_wasm_scs, 300);
        }
    }

    for abi in abis_list {
        let full_results = abi.execute_abi_scs(nb_scs_by_abi);
        compile_and_write_results(
            full_results,
            u32::MAX,
            Duration::from_millis(300),
            &abi.abis_type,
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
    let as_output_dir = output_dir(&AbisType::AS);
    let as_env_path_output = &as_output_dir.join("env.ts");
    fs::copy(as_env_path, as_env_path_output).unwrap();

    println!("############################################################");
    println!("Set wasmv1 env");
    let wasmv1_output_dir = output_dir(&AbisType::WasmV1);
    let wasmv1_env_path_output = &wasmv1_output_dir.join("env_wasmv1.ts");
    fs::copy(wasmv1_env_path, wasmv1_env_path_output).unwrap();

    let files = ["package.json", "helpers.ts"];
    for abi_type in &[AbisType::AS, AbisType::WasmV1] {
        let output_dir = output_dir(abi_type);
        for file in &files {
            fs::copy(template_dir(abi_type).join(file), output_dir.join(file))
                .unwrap();
        }
    }

    // npm install
    let npm_path = which("npm").expect("npm not found in PATH");
    npm_install_update(&npm_path, &AbisType::AS);
    npm_install_update(&npm_path, &AbisType::WasmV1);
}

fn npm_install_update(npm_path: &Path, abi_type: &AbisType) {
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
