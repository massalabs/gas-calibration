use std::collections::HashMap;

use massa_models::datastore::Datastore;
use rand::Rng;

use crate::{calculation, execute_batch_sc, AbiType};

pub fn execute_abi_scs(
    full_results: &mut HashMap<String, Vec<f64>>,
    nb_scs_per_abi: u32,
    op_datastore: &Datastore,
    abi_type: &AbiType,
    abis: &[Vec<String>],
) {
    println!("Executing {} SCs per abis", nb_scs_per_abi);
    let mut rng = rand::thread_rng();
    let mut pb = pbr::ProgressBar::new(abis.len() as u64);

    for (abi_index, _) in abis.iter().enumerate() {
        let mut full_stats = Vec::new();
        let mut executed = 0;

        while executed < nb_scs_per_abi {
            let nb_exec = rng.gen_range(1..6);
            let stats = execute_batch_sc::execute_batch_sc(
                executed + (abi_index as u32 * nb_scs_per_abi),
                std::cmp::min(nb_scs_per_abi, executed + nb_exec)
                    + (abi_index as u32 * nb_scs_per_abi),
                op_datastore,
                abi_type,
            );
            executed += nb_exec;
            full_stats.push(stats);
        }

        let results = calculation::calculate_times(full_stats, true);
        for (key, value) in results.iter() {
            full_results.entry(key.clone()).or_default().push(*value);
        }
        pb.inc();
    }
    pb.finish_print("Finished executing ABI SCs");
}

pub fn execute_wasm_scs(
    full_results: &mut HashMap<String, Vec<f64>>,
    nb_contracts: u32,
    abi_type: &AbiType,
) {
    println!("Executing {} SCs WASM", nb_contracts);
    let mut rng = rand::thread_rng();
    let mut pb = pbr::ProgressBar::new(nb_contracts as u64);
    let mut executed = 0;
    let mut full_stats = Vec::new();

    while executed < nb_contracts {
        let nb_exec = rng.gen_range(1..6);
        let stats = execute_batch_sc::execute_batch_sc(
            executed,
            std::cmp::min(nb_contracts, executed + nb_exec),
            &Datastore::new(),
            abi_type,
        );
        executed += nb_exec;
        full_stats.push(stats);
        pb.add(nb_exec as u64);
    }
    // Remove values with 0 in full_stats
    let full_stats = full_stats
        .into_iter()
        .map(|(x, d)| (x.into_iter().filter(|(_k, v)| v != &0).collect(), d))
        .collect();

    let results = calculation::calculate_times(full_stats, false);
    for (key, value) in results.iter() {
        full_results.entry(key.clone()).or_default().push(*value);
    }
    pb.finish_print("Finished executing WASM SCs");
}
