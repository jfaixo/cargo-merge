use structopt::StructOpt;
use serde_derive::Deserialize;

/// Represents the various options
#[derive(Debug, StructOpt, Deserialize)]
#[structopt(name = "cargo merge", about = "Merges the source code of a crate into a single file")]
pub struct Opts {
    /// Remove all the usages of eprint! and eprintln! macros
    #[structopt(short = "s", long="silence-standard-error-output")]
    pub remove_error_output: bool,
    /// Debug mode (for cargo-merge development purpose)
    #[structopt(short = "d", long="debug")]
    pub debug: bool,
}
