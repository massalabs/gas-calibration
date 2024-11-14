use std::{fs, path::PathBuf};

use crate::AbisType;

const ROOT_TEMPLATE_DIR: &str = "./src/sc_generation/template";

pub fn template_dir(abi_type: &AbisType) -> PathBuf {
    match abi_type {
        AbisType::AS => PathBuf::from(ROOT_TEMPLATE_DIR).join("as"),
        AbisType::WasmV1 => PathBuf::from(ROOT_TEMPLATE_DIR).join("wasmv1"),
    }
}

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("calibration")
}

pub fn output_dir(abi_type: &AbisType) -> PathBuf {
    let path = match abi_type {
        AbisType::AS => root_dir().join("as"),
        AbisType::WasmV1 => root_dir().join("wasmv1"),
    };

    fs::create_dir_all(&path).unwrap();
    path
}

pub fn src_dir(abi_type: &AbisType) -> PathBuf {
    let path = output_dir(abi_type).join("src");
    fs::create_dir_all(&path).unwrap();
    path
}

pub fn build_dir(abi_type: &AbisType) -> PathBuf {
    let path = output_dir(abi_type).join("build");
    fs::create_dir_all(&path).unwrap();
    path
}

pub(crate) fn wasmv1_env_path() -> std::path::PathBuf {
    let wasmv1_env_path = template_dir(&AbisType::WasmV1).join("env_wasmv1.ts");
    if !wasmv1_env_path.exists() {
        panic!("env_wasmv1.ts not found in template directory");
    }
    wasmv1_env_path
}

pub(crate) fn as_env_path() -> std::path::PathBuf {
    let as_env_path = template_dir(&AbisType::AS).join("env.ts");

    if !as_env_path.exists() {
        panic!("env.ts not found in template directory");
    }
    as_env_path
}
