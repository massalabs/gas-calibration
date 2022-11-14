use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Skip generation of SCs if it has already be done before
   #[arg(long, short, action)]
   pub skip_generation_scs: bool,
}
