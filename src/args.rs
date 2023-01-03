use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Skip generation of SCs if it has already be done before
    #[arg(long, short, action)]
    pub skip_generation_scs: bool,
    /// Override number of SCs by ABI
    #[arg(long, short, action)]
    pub nb_scs_by_abi: Option<u32>,
    /// Override path to as-sdk env
    #[arg(long, short, action)]
    pub as_sdk_env_path: Option<String>,
}
