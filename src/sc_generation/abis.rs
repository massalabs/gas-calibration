use std::{collections::HashMap, fmt, path::Path};

use massa_models::datastore::Datastore;

use crate::execution::execute_abi_scs;

use super::{
    build_scs, generate_scs, generation::generate_op_datastore,
    read_existing_op_datastore,
};

pub(crate) enum AbisType {
    AS,
    WasmV1,
}

impl fmt::Display for AbisType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbisType::AS => write!(f, "assembly script"),
            AbisType::WasmV1 => write!(f, "WasmV1"),
        }
    }
}

pub(crate) struct Abis {
    pub abis_type: AbisType,
    pub abis: Vec<Vec<String>>,
    pub datastore: Datastore,
}

pub fn get_as_abis<P: AsRef<Path>>(file_path: &P) -> Abis {
    Abis {
        abis_type: AbisType::AS,
        abis: get_abis(file_path),
        datastore: Datastore::default(),
    }
}

pub fn get_wasmv1_abis<P: AsRef<Path>>(file_path: &P) -> Abis {
    Abis {
        abis_type: AbisType::WasmV1,
        abis: get_abis(file_path),
        datastore: Datastore::default(),
    }
}

fn get_abis<P: AsRef<Path>>(file_path: &P) -> Vec<Vec<String>> {
    println!("############################################################");
    println!("Reading ABIs from {}", file_path.as_ref().to_str().unwrap());

    let abis_string = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let content: Vec<&str> = abis_string.lines().collect();
    // Group prototypes splitted in multiple lines
    let mut abis_string_vec: Vec<String> = Vec::new();
    let mut in_abi_definition = false;

    for line in content {
        if line.starts_with("  export declare function") {
            in_abi_definition = true;
            abis_string_vec.push(line.to_string());
        } else if in_abi_definition {
            if line.starts_with("    ") || line.ends_with(';') {
                abis_string_vec
                    .last_mut()
                    .unwrap()
                    .push_str(&format!(" {}", &line.trim_start()));
            } else {
                in_abi_definition = false;
            }
        }
    }

    let abis: Vec<Vec<String>> = abis_string_vec
        .iter()
        .filter_map(|line| {
            if line.starts_with("  export declare function") {
                Some(
                    line.trim_start()
                        .split("export declare function ")
                        .last()
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .map(|line| &line[0..line.len() - 1])
        .map(|function| {
            function
                .split('(')
                .map(|s| String::from(s.trim_start()))
                .collect()
        })
        .map(|function_array: Vec<String>| {
            let mut abi = vec![function_array[0].clone()];
            abi.extend(
                function_array[1]
                    .split("): ")
                    .map(|s| String::from(s.trim_end_matches([',', ' ']))),
            );
            abi
        })
        .map(|function_array| {
            let mut abi = vec![function_array[0].clone()];
            abi.extend(function_array[1].split(", ").map(String::from));
            abi.push(function_array[2].clone());
            abi
        })
        .collect();

    abis
}

impl Abis {
    pub fn generate_op_datastore(&mut self) {
        self.datastore = generate_op_datastore(&self.abis_type);
    }

    pub fn read_existing_op_datastore(&mut self) {
        self.datastore = read_existing_op_datastore(&self.abis_type);
    }

    pub fn generate_scs(&self, nb_scs: u32, nb_instructions: u64) {
        generate_scs(
            nb_scs,
            nb_instructions,
            &self.datastore,
            &self.abis_type,
            &self.abis,
        );
    }

    pub fn build_scs(&self, nb_scs: u32) {
        build_scs(nb_scs, &self.abis_type, &self.abis);
    }

    pub fn execute_abi_scs(
        &self,
        nb_scs_per_abi: u32,
    ) -> HashMap<String, Vec<f64>> {
        let mut full_results: HashMap<String, Vec<f64>> = HashMap::new();
        execute_abi_scs(
            &mut full_results,
            nb_scs_per_abi,
            &self.datastore,
            &self.abis_type,
            &self.abis,
        );
        full_results
    }
}
