use std::{collections::HashMap, time::Duration};

use clap::Parser;

use crate::calculation::compile_and_write_results;

mod args;
mod calculation;
mod execute_batch_sc;
mod execution;
mod sc_generation;

fn main() {
    let args = args::Args::parse();
    let nb_scs_by_abi: u32 = 100;
    let nb_wasm_scs = 500;
    let abis = sc_generation::abis::get_abis();
    let op_datastore = if args.skip_generation_scs {
        sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        sc_generation::read_existing_op_datastore()
    } else {
        let datastore = sc_generation::generation::generate_op_datastore();
        sc_generation::generate_scs(nb_scs_by_abi, 300, datastore.clone());
        sc_generation::build_scs(nb_scs_by_abi, abis);
        sc_generation::generate_wasm_scs(nb_wasm_scs, 300);
        datastore
    };
    let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    execution::execute_abi_scs(&mut full_results, nb_scs_by_abi, op_datastore);
    compile_and_write_results(full_results, u32::MAX, Duration::from_millis(300), true);
    let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    execution::execute_wasm_scs(&mut full_results, nb_wasm_scs);
    compile_and_write_results(full_results, u32::MAX, Duration::from_millis(300), false);
}
