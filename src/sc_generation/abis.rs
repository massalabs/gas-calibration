pub fn get_abis(file_path: &String) -> Vec<Vec<String>> {
    let abis_string =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut abis: Vec<Vec<String>> = abis_string
        .lines()
        .filter_map(|line| {
            if line.starts_with("    export declare function") {
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
        .map(|function| function.split('(').map(String::from).collect())
        .map(|function_array: Vec<String>| {
            let mut abi = vec![function_array[0].clone()];
            abi.extend(function_array[1].split("): ").map(String::from));
            abi
        })
        .map(|function_array| {
            let mut abi = vec![function_array[0].clone()];
            abi.extend(function_array[1].split(", ").map(String::from));
            abi.push(function_array[2].clone());
            abi
        })
        .collect();
    abis.push(vec![String::from("seed")]);
    abis.push(vec![String::from("Date.now")]);
    abis
}
