mod opts;
mod bundle;

use structopt::StructOpt;
use crate::opts::Opts;
use crate::bundle::Bundle;
use log::LevelFilter;

/// Main entry point
fn main() {
    // Parse CLI arguments
    let mut args: Vec<String> = std::env::args().collect();
    // Handle the case when the first argument is in fact "bundle" because called as a cargo subcommand
    if args.len() > 1 && args[1] == "bundle" {
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

    // Run the bundle logic
    let bundle = Bundle::new(opts);
    bundle.run();
}
