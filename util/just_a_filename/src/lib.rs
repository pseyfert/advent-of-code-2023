use clap::{Parser, ValueHint};
use std::path::PathBuf;

pub mod prelude {
    pub use clap::Parser;
}

#[derive(Parser, Debug)]
#[clap(name = "exercise")]
pub struct Cli {
    #[clap(value_name = "INPUT_FILE", value_hint = ValueHint::FilePath, default_value = "../input")]
    pub path: PathBuf,
}
