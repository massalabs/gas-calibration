use std::{fs, path::PathBuf};

use crate::AbiType;

const ROOT_TEMPLATE_DIR: &str = "./src/sc_generation/template";

pub fn template_dir(abi_type: &AbiType) -> PathBuf {
    match abi_type {
        AbiType::AS => PathBuf::from(ROOT_TEMPLATE_DIR).join("as"),
        AbiType::WasmV1 => PathBuf::from(ROOT_TEMPLATE_DIR).join("wasmv1"),
    }
}

pub fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("calibration")
}

pub fn output_dir(abi_type: &AbiType) -> PathBuf {
    let path = match abi_type {
        AbiType::AS => root_dir().join("as"),
        AbiType::WasmV1 => root_dir().join("wasmv1"),
    };

    fs::create_dir_all(&path).unwrap();
    path
}
pub fn generate_dir(abi_type: &AbiType) -> PathBuf {
    let path = output_dir(abi_type).join("src");
    fs::create_dir_all(&path).unwrap();
    path
}

pub fn build_dir(abi_type: &AbiType) -> PathBuf {
    let path = output_dir(abi_type).join("build");
    fs::create_dir_all(&path).unwrap();
    path
}

pub(crate) fn wasmv1_env_path() -> std::path::PathBuf {
    let wasmv1_env_path = template_dir(&AbiType::WasmV1).join("env_wasmv1.ts");
    if !wasmv1_env_path.exists() {
        panic!("env_wasmv1.ts not found in template directory");
    }
    wasmv1_env_path
}

pub(crate) fn as_env_path() -> std::path::PathBuf {
    let as_env_path = template_dir(&AbiType::AS).join("env.ts");

    if !as_env_path.exists() {
        panic!("env.ts not found in template directory");
    }
    as_env_path
}
