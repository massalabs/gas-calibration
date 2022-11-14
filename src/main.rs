use std::collections::HashMap;

use rand::Rng;
use clap::Parser;

use crate::calculation::compile_and_write_results;

mod args;
mod calculation;
mod execute_batch_sc;
mod sc_generation;

fn main() {
    let args = args::Args::parse();
    let mut rng = rand::thread_rng();
    let nb_scs_by_abi: u32 = 100;
    let abis = sc_generation::abis::get_abis();
    let op_datastore = if args.skip_generation_scs {
        sc_generation::read_existing_op_datastore()
    } else {
        let datastore = sc_generation::generate_op_datastore();
        sc_generation::generate_scs(nb_scs_by_abi, 300, datastore.clone());
        sc_generation::build_scs(nb_scs_by_abi, abis.clone());
        datastore
    };
    println!("Executing {} SCs per abis", nb_scs_by_abi);
    let mut pb = pbr::ProgressBar::new(abis.len() as u64);
    let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
    for (abi_index, _) in abis.iter().enumerate() {
        let mut full_stats = Vec::new();
        let mut executed = 0;
        while executed < nb_scs_by_abi {
            let nb_exec = rng.gen_range(1..6);
            let stats = execute_batch_sc::execute_batch_sc(
                executed + (abi_index as u32 * nb_scs_by_abi),
                std::cmp::min(nb_scs_by_abi, executed + nb_exec) + (abi_index as u32 * nb_scs_by_abi),
                op_datastore.clone(),
            );
            executed += nb_exec;
            full_stats.push(stats);
        }
        let results = calculation::calculate_times(abi_index, full_stats);
        for (key, value) in results.iter() {
            full_results.entry(key.clone()).or_insert(Vec::new()).push(*value);
        }
        pb.inc();
    }
    pb.finish_print("Finished executing SCs");
    compile_and_write_results(full_results);
}
