use std::{collections::HashMap, io::Read, str::FromStr, time::Duration};

use massa_execution_worker::InterfaceImpl;
use massa_models::{address::Address, datastore::Datastore};
use massa_sc_runtime::{run_main_gc, GasCosts, RuntimeModule};
//use rand::Rng;
use std::fs::File;

pub fn execute_batch_sc(
    first_sc_index: u32,
    last_sc_index: u32,
    op_datastore: Datastore,
    abi_mode: bool,
) -> (HashMap<String, u64>, Duration) {
    // Optional preparation SC and SC
    let mut bytecodes: Vec<(Option<Vec<u8>>, Vec<u8>)> = Vec::new();
    for i in first_sc_index..last_sc_index {
        let filename = if abi_mode {
            format!("./src/sc_generation/template/build/SC_{}.wasm", i)
        } else {
            format!("./src/sc_generation/template/build/WAT_{}.wat", i)
        };
        //let filename = format!("./src/sc_generation/template/test.wasm");
        let file = File::open(&filename);
        if file.is_err() {
            continue;
        }
        let mut file = file.unwrap();
        let mut bytecode = vec![];
        file.read_to_end(&mut bytecode)
            .unwrap_or_else(|_| panic!("Failed to read {}", filename));
        //TODO: Change here
        let preparation_bytecode = if let Ok(mut file) = File::open(format!(
            "./src/sc_generation/template/build/SC_preparation_{}.wasm",
            i
        )) {
            if abi_mode {
                let mut bytecode = vec![];
                file.read_to_end(&mut bytecode)
                    .unwrap_or_else(|_| panic!("Failed to read {}", filename));
                Some(bytecode)
            } else {
                None
            }
        } else {
            None
        };
        bytecodes.push((preparation_bytecode, bytecode));
    }
    let mut total_execution_stats: HashMap<String, u64> = HashMap::default();
    let mut total_execution_time = Duration::from_secs(0);
    let bytecodes_len = bytecodes.len() as u64;
    //let mut nb_compiled = 0;
    for (preparation_bytecode, bytecode) in bytecodes {
        //let mut rng = rand::thread_rng();
        //let need_compile = rng.gen_bool(0.5);
        let interface = InterfaceImpl::new_default(
            Address::from_str("AU12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M").unwrap(),
            Some(op_datastore.clone()),
        );
        if let Some(preparation_bytecode) = preparation_bytecode {
            run_main_gc(
                &interface,
                RuntimeModule::new(&preparation_bytecode, u64::MAX, GasCosts::default()).unwrap(),
                &[],
                u64::MAX,
                GasCosts::default(),
            )
            .unwrap();
        }
        //let (start, results) = if need_compile {
        let module = RuntimeModule::new(&bytecode, u64::MAX, GasCosts::default()).unwrap();
        let start = std::time::Instant::now();
        let results = run_main_gc(&interface, module, &[], u64::MAX, GasCosts::default()).unwrap();
        //    nb_compiled += 1;
        //    (start, results)
        // } else {
        //    let module = RuntimeModule::new(&bytecode, u64::MAX, GasCosts::default()).unwrap();
        //    let start = std::time::Instant::now();
        //    let results =
        //        run_main_gc(&interface, module, &[], u64::MAX, GasCosts::default()).unwrap();
        //    (start, results)
        // };
        //println!("Results: {:?}", results);
        let mut time_exec = start.elapsed();
        //println!("Time: {:?}", time_exec);
        for (_key, value) in results.timers {
            //println!("key: {:?}, value: {:?}", _key, value);
            time_exec -= Duration::from_secs_f64(value);
        }
        // Size ignored for now because we saw that it doesn't change a lot
        // results
        //     .counters
        //     .insert(String::from("Size"), bytecode.len() as u64);
        //println!("Time: {:?}", time_exec);
        total_execution_time += time_exec;
        for (key, value) in results.counters {
            let entry = total_execution_stats.entry(key).or_insert(0);
            *entry += value;
        }
    }
    //total_execution_stats.insert(String::from("Compile"), nb_compiled);
    total_execution_stats.insert(String::from("Launch"), bytecodes_len);
    (total_execution_stats, total_execution_time)
}
