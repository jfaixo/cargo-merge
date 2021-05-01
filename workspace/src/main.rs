#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! # Cargo merge
//!
//! A cargo subcommand that merges your crate source code into a single file.
//!
//! The initial purpose of this command is to merge your whole crate as a single source file that can be used on competitive programming platforms.
//!
//! It works by expanding module imports by detecting them with regex, rewriting some "use" statements in the process.
//!
//! ## Features
//! - merge the library crate of the package
//! - merge modules from the main binary crate of the package
//! - merge external crates
//! - path based crates (crates that are declared as dependencies like this : `my-crate = { path = "path/to/crate" }` )
//! - silence the standard error output by cleansing source code from `eprint!` and `eprintln!` macros
//!
//! ## Examples
//! Working basic examples can be found in the [integration tests data](https://github.com/jfaixo/cargo-merge/tree/main/workspace/tests_data).
//!
//! ## Install
//! Just run the following command:
//! ```bash
//! cargo install cargo-merge
//! ```
//!
//! ## Usage
//! Simply call the cargo sub command inside your crate folder hierarchy (it can be any folder below the one containing your `Cargo.toml` file):
//! ```bash
//! cargo merge
//! ```
//!
//! This will generate a merged file in `target/merge/merged.rs`.
//!
//! ## Options
//!
//! | Long flag | Short flag | Description |
//! |-|-|-|
//! | `-s` | `--silence-standard-error-output` | Remove all the usages of `eprint!` and `eprintln!` macros from your code. |
//!
//! ## Credits
//! This little project is heavily inspired by [rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler).
//! It has the same approach and has the same goal, but I find the cargo subcommand approach less intrusive.
//! Also, I'll hopefully also maintain this project actively.
//!

use structopt::StructOpt;
use cargo_merge::opts::Opts;
use cargo_merge::merge::Merge;
use log::LevelFilter;

#[doc(hidden)]
/// Main entry point
fn main() {
    // Parse CLI arguments
    let mut args: Vec<String> = std::env::args().collect();
    // Handle the case when the first argument is in fact "merge" because called as a cargo subcommand
    if args.len() > 1 && args[1] == "merge" {
        args.remove(1);
    }
    let opts = Opts::from_iter(args);

    // Initialize logging
    if opts.debug {
        simple_logging::log_to_stderr(LevelFilter::Debug);
    }
    else {
        simple_logging::log_to_stderr(LevelFilter::Info);
    }

    // Run the merge logic
    let merge = Merge::new(opts);
    merge.run();
}
