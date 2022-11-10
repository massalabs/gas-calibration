use std::{collections::HashMap, io::{Read, Stdout}, str::FromStr, time::Duration};

use massa_execution_worker::InterfaceImpl;
use massa_models::{address::Address, datastore::Datastore};
use massa_sc_runtime::run_main_gc;
use pbr::ProgressBar;
use std::fs::File;

pub fn execute_batch_sc(
    first_sc_index: u32,
    last_sc_index: u32,
    op_datastore: Datastore,
    pb: &mut ProgressBar<Stdout>,
) -> (HashMap<String, u64>, Duration) {
    let mut bytecodes = Vec::new();
    for i in first_sc_index..last_sc_index {
        let filename = format!("./src/sc_generation/template/build/SC_{}.wasm", i);
        let file = File::open(&filename);
        if file.is_err() {
            continue;
        }
        let mut file = file.unwrap();
        let mut bytecode = vec![];
        file.read_to_end(&mut bytecode)
            .expect(&format!("Failed to read {}", filename));
        bytecodes.push(bytecode);
    }
    let interface = InterfaceImpl::new_default(
        Address::from_str("A12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M").unwrap(),
        Some(op_datastore),
    );
    let mut total_execution_stats: HashMap<String, u64> = HashMap::default();
    let mut total_execution_time = Duration::from_secs(0);
    let bytecodes_len = bytecodes.len() as u64;
    for bytecode in bytecodes {
        let start = std::time::Instant::now();
        let mut results = run_main_gc(&bytecode, u64::MAX, &interface).unwrap();
        let time_exec = start.elapsed();
        results.0.insert(String::from("Size"), bytecode.len() as u64);
        total_execution_time += time_exec;
        for (key, value) in results.0 {
            let entry = total_execution_stats.entry(key).or_insert(0);
            *entry += value;
        }
        pb.inc();
    }
    total_execution_stats.insert(String::from("Launch"), bytecodes_len);
    (total_execution_stats, total_execution_time)
}
