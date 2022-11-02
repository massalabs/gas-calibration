use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use rand::Rng;

mod abis;

fn generate_u8_array(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut array = Vec::new();
    for _ in 0..length {
        array.push(rng.gen());
    }
    array
}

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..'z'));
    }
    string
}

fn generate_calls(abis: Vec<Vec<String>>, limit_calls: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();

    let nb_calls = rng.gen_range(0..limit_calls);
    let mut calls = Vec::new();
    for _ in 0..nb_calls {
        let abi = abis[rng.gen_range(0..abis.len())].clone();
        let mut call = format!("env.{}", abi[0].clone());
        call.push('(');
        for i in 1..abi.len() - 1 {
            if abi[i] == "" {
                break;
            }
            let mut splitted_params = abi[i].as_str().split(": ");
            let arg = match (splitted_params.next().unwrap(), splitted_params.next().unwrap()) {
                (_, "string") => format!("\"{}\"", generate_string(rng.gen_range(0..100))),
                (_, "u64") => rng.gen::<u64>().to_string(),
                (_, "u8") => rng.gen::<u8>().to_string(),
                (_, "boolean") => rng.gen::<bool>().to_string(),
                (_, "StaticArray<u8>") => format!("{:#?}", generate_u8_array(rng.gen_range(0..100))),
                _ => panic!("Unknown type"),
            };
            call.push_str(&arg);
            call.push(',');
        }
        if call.ends_with(',') {
            call.pop();
        }
        call.push(')');
        call.push(';');
        println!("{}", call);
        calls.push(call);
    }
    calls
}

pub fn generate_scs(nb_sc: u32, limit_calls: u32) {
    let abis = abis::get_abis();
    for i in 0..nb_sc {
    let calls = generate_calls(abis.clone(), limit_calls);
    let template_index = format!(
        "import {{env}} from './env';

export function main(): void {{
    {}
}}", calls.join("\n")
    );
    let mut output = File::create("./src/sc_generation/template/index.ts").unwrap();
    write!(output, "{}", template_index).unwrap();
    println!("{:#?}",    std::env::current_dir());
    Command::new("npm")
        .arg("run")
        .arg("build")
        .env("SC_NAME", format!("SC_{}", i))
        .current_dir("./src/sc_generation/template").spawn().unwrap().wait().unwrap();
    }
}
