use std::{io::Read, str::FromStr, time::Duration, collections::HashMap};

use massa_execution_worker::InterfaceImpl;
use massa_models::{address::Address, datastore::Datastore};
use massa_sc_runtime::run_main;
use std::fs::File;

pub fn execute_batch_sc(
    first_sc_index: u32,
    last_sc_index: u32,
    op_datastore: Datastore,
) -> Duration {
    let mut bytecodes = Vec::new();
    for i in first_sc_index..last_sc_index {
        let filename = format!("./src/sc_generation/template/build/SC_{}.wasm", i);
        let mut file = File::open(&filename).expect(&format!("Failed to open {}", filename));
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
    for bytecode in bytecodes {
        let start = std::time::Instant::now();
        run_main(&bytecode, u64::MAX, &interface).unwrap();
        
        let end = std::time::Instant::now();
        total_execution_time += end - start;
    }
    total_execution_time
}
