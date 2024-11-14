use std::{
    collections::HashMap, io::Read, process, str::FromStr, time::Duration,
};

use massa_execution_worker::InterfaceImpl;
use massa_models::{address::Address, datastore::Datastore};
use massa_sc_runtime::{
    run_main_gc, Compiler, CondomLimits, GasCosts, RuntimeModule,
};
// use rand::Rng;
use std::fs::File;

use crate::{config::build_dir, AbisType};

pub fn execute_batch_sc(
    first_sc_index: u32,
    last_sc_index: u32,
    op_datastore: &Datastore,
    abi_type: &AbisType,
) -> (HashMap<String, u64>, Duration) {
    // Optional preparation SC and SC
    let mut bytecodes: Vec<(Option<Vec<u8>>, Vec<u8>)> = Vec::new();
    for i in first_sc_index..last_sc_index {
        let filename = format!("SC_{}.wasm", i);

        println!("Executing {} for abi `{}`", filename, abi_type);

        let build_dir = build_dir(abi_type);
        let sc_file_path = build_dir.join(filename.clone());
        let file = File::open(&sc_file_path);
        if file.is_err() {
            println!("Failed to open {:?}", sc_file_path);
            process::exit(1);
        }
        let mut file = file.unwrap();
        let mut bytecode = match abi_type {
            AbisType::AS => vec![],
            AbisType::WasmV1 => vec![1_u8],
        };
        file.read_to_end(&mut bytecode)
            .unwrap_or_else(|_| panic!("Failed to read {}", filename));
        // TODO: Change here
        let preparation_bytecode = if let Ok(mut file) =
            File::open(build_dir.join(format!("SC_preparation_{}.wasm", i)))
        {
            let mut bytecode = match abi_type {
                AbisType::AS => vec![],
                AbisType::WasmV1 => {
                    vec![1_u8]
                }
            };
            file.read_to_end(&mut bytecode)
                .unwrap_or_else(|_| panic!("Failed to read {}", filename));
            Some(bytecode)
        } else {
            None
        };
        bytecodes.push((preparation_bytecode, bytecode));
    }

    let mut total_execution_stats: HashMap<String, u64> = HashMap::default();
    let mut total_execution_time = Duration::from_secs(0);
    let bytecodes_len = bytecodes.len() as u64;
    // let mut nb_compiled = 0;
    for (preparation_bytecode, bytecode) in bytecodes {
        // let mut rng = rand::thread_rng();
        // let need_compile = rng.gen_bool(0.5);
        let interface = InterfaceImpl::new_default(
            Address::from_str(
                "AS12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M",
            )
            .unwrap(),
            Some(op_datastore.clone()),
        );

        if let Some(preparation_bytecode) = preparation_bytecode {
            if let Err(e) = run_main_gc(
                &interface,
                RuntimeModule::new(
                    &preparation_bytecode,
                    GasCosts::default(),
                    Compiler::CL,
                    CondomLimits::default(),
                )
                .unwrap(),
                &[],
                u64::MAX,
                GasCosts::default(),
                CondomLimits::default(),
            ) {
                println!("Failed to execute preparation bytecode: {:?}", e);
                continue;
            }
        }

        let module = RuntimeModule::new(
            &bytecode,
            GasCosts::default(),
            Compiler::CL,
            CondomLimits::default(),
        )
        .unwrap();
        let start = std::time::Instant::now();

        let results = match run_main_gc(
            &interface,
            module,
            &[],
            u64::MAX,
            GasCosts::default(),
            CondomLimits::default(),
        ) {
            Ok(results) => results,
            Err(e) => {
                println!("Failed to execute bytecode: {:?}", e);
                continue;
            }
        };

        // println!("Results:");
        // println!("");
        // println!("Counters:");
        // for (key, value) in &results.counters {
        // println!("key: {:?}, value: {:?}", key, value);
        // }
        //    nb_compiled += 1;
        //    (start, results)
        // } else {
        //    let module = RuntimeModule::new(&bytecode, u64::MAX,
        // GasCosts::default()).unwrap();    let start =
        // std::time::Instant::now();    let results =
        //        run_main_gc(&interface, module, &[], u64::MAX,
        // GasCosts::default()).unwrap();    (start, results)
        // };
        // println!("Results: {:?}", results);
        let mut time_exec = start.elapsed();
        // println!("Time: {:?}", time_exec);
        for (_key, value) in results.timers {
            // println!("time_exec: {:?}", time_exec);
            // println!("key: {:?}, value: {:?}", _key, value);
            // time_exec -= Duration::from_secs_f64(value);
            time_exec =
                match time_exec.checked_sub(Duration::from_secs_f64(value)) {
                    Some(new_time_exec) => new_time_exec,
                    None => {
                        println!(
                            "Time exec overflow: {:?}, {:?}",
                            time_exec,
                            Duration::from_secs_f64(value)
                        );
                        Duration::from_secs(0)
                    }
                };
        }
        // Size ignored for now because we saw that it doesn't change a lot
        // results
        //     .counters
        //     .insert(String::from("Size"), bytecode.len() as u64);
        // println!("Time: {:?}", time_exec);
        total_execution_time += time_exec;
        for (key, value) in results.counters {
            let entry = total_execution_stats.entry(key).or_insert(0);
            *entry += value;
        }
    }
    // total_execution_stats.insert(String::from("Compile"), nb_compiled);
    total_execution_stats.insert(String::from("Launch"), bytecodes_len);
    (total_execution_stats, total_execution_time)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sc_generation::read_existing_op_datastore;

    use super::*;

    #[test]
    fn test_run_sc() {
        let op_datastore = read_existing_op_datastore(&AbisType::AS);

        let interface = InterfaceImpl::new_default(
            Address::from_str(
                "AS12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M",
            )
            .unwrap(),
            Some(op_datastore.clone()),
        );

        let sc_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            // .join("calibration/as/toto.wasm");
            .join("calibration/as/build/SC_9.wasm");
        // let sc_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        //     .join("calibration/wasmv1/build/SC_preparation_1.wasm_add");

        let mut file = File::open(&sc_file_path)
            .expect("Failed to open SC preparation file");

        let mut preparation_bytecode = Vec::new();

        file.read_to_end(&mut preparation_bytecode)
            .expect("Failed to read SC preparation file");

        let res = run_main_gc(
            &interface,
            RuntimeModule::new(
                &preparation_bytecode,
                GasCosts::default(),
                Compiler::CL,
                CondomLimits::default(),
            )
            .unwrap(),
            &[],
            u64::MAX,
            GasCosts::default(),
            CondomLimits::default(),
        );

        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);
            }
        }
    }
}
