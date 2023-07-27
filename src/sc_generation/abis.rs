pub fn get_abis(file_path: &String) -> Vec<Vec<String>> {
    let abis_string =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");
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
                    .map(|s| String::from(s.trim_end_matches(&[',', ' ']))),
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
