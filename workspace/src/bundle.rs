use std::path::PathBuf;
use std::path::Path;
use std::{fs, io};
use toml::Value;
use log::{debug};
use crate::opts::Opts;
use std::io::BufRead;
use regex::Regex;
use std::fs::File;
use std::fmt::Write;
use colored::Colorize;

const CARGO_TOML : &str = "Cargo.toml";
const SIMPLE_CRATE_MAIN_RS : &str = "src/main.rs";
const SIMPLE_CRATE_LIB_RS : &str = "src/lib.rs";
const SIMPLE_CRATE_MAIN : &str = "src/main";
const SIMPLE_CRATE_LIB : &str = "src/lib";
const BUNDLE_OUTPUT_PATH: &str = "target/bundle/";
const BUNDLE_OUTPUT_FILE_NAME: &str = "bundle.rs";

const REGEX_COMMENT : &str = r"^\s*//";
const REGEX_MOD : &str = r"^\s*(pub\s+)?mod\s+(.*)\s*;\s*$";
const REGEX_USE : &str = r"^\s*use\s+(.*)\s*;\s*$";
const REGEX_EPRINT: &str = r"^\s*eprint(ln)?!";

pub struct Bundle {
    comment_regex: Regex,
    mod_regex: Regex,
    use_regex: Regex,
    eprint_regex: Regex,
    opts: Opts,
}

impl Bundle {
    pub fn new(opts: Opts) -> Bundle {
        Bundle {
            comment_regex: Regex::new(REGEX_COMMENT).expect("Unable to compile the comment regex"),
            mod_regex: Regex::new(REGEX_MOD).expect("Unable to compile the mod regex"),
            use_regex: Regex::new(REGEX_USE).expect("Unable to compile the use regex"),
            eprint_regex: Regex::new(REGEX_EPRINT).expect("Unable to compile the eprint regex"),
            opts
        }
    }

    /// Main bundle entrypoint
    pub fn run(&self) {
        // Detect the package root
        let package_root_path = detect_package_root();

        // Read into the Cargo.toml the package name, which is also the default crate name
        let package_name = get_package_name(&package_root_path);

        println!("    {} crate {} ({})", "Bundling".green().bold(), package_name, package_root_path.to_str().unwrap());

        // Holds the single file output built
        let mut output_string = String::new();

        // If there is a lib crate in this package, process it
        if Path::new(SIMPLE_CRATE_LIB_RS).exists() {
            writeln!(output_string, "{}", self.inject_crate(PathBuf::from(SIMPLE_CRATE_LIB), package_name.as_str()).as_str()).unwrap();
        }
        // Simple bin crate case
        if Path::new(SIMPLE_CRATE_MAIN_RS).exists() {
            writeln!(output_string, "{}", self.inject_crate(PathBuf::from(SIMPLE_CRATE_MAIN), package_name.as_str()).as_str()).unwrap();
        }

        // Ensure that the folders are created
        let output_path = package_root_path.join(BUNDLE_OUTPUT_PATH);
        fs::create_dir_all(&output_path).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", output_path));

        // Write to disk
        let output_file_path = output_path.join(BUNDLE_OUTPUT_FILE_NAME);
        fs::write(&output_file_path, output_string)
            .unwrap_or_else(|_| panic!("There was an issue while writing to file: {}", BUNDLE_OUTPUT_PATH));

        println!("     {} crate {} into `{}` ", "Bundled".green().bold(), package_name, output_file_path.to_str().unwrap());
    }

    fn inject_crate(&self, crate_path: PathBuf, package_name: &str) -> String {
        self.inject_modules(crate_path, package_name, true)
    }


    /// Inject a module into the output file, recursively injecting nested modules
    fn inject_modules(&self, full_module_path: PathBuf, current_module_name: &str, is_root_module: bool) -> String {
        let mut output_string = String::new();

        // Find whether this module is defined with a lib.rs file, or directly by a file named as the module
        let mut possible_module_file_paths = Vec::new();

        // Rust file case
        let mut rs_file = full_module_path.clone();
        if !is_root_module {
            rs_file.set_file_name(current_module_name);
        }
        rs_file.set_extension("rs");
        possible_module_file_paths.push(rs_file);

        // mod.rs file case
        let mut mod_file = full_module_path.clone();
        mod_file.set_file_name("mod.rs");
        possible_module_file_paths.push(mod_file);

        // Find if any one exists
        let module_file_descriptor = possible_module_file_paths.iter().map(|possible_module_file_paths| {
                debug!("Trying to open: {:?}", possible_module_file_paths);
                File::open(possible_module_file_paths)
            })
            .find(|file_descriptor| { file_descriptor.is_ok() });

        match module_file_descriptor {
            Some(module_file_descriptor) => {
                // Use the found file, read and inject it
                let module_file_descriptor = module_file_descriptor.unwrap_or_else(|_| panic!("Unable to open module file: {:?}", full_module_path));
                let lines = io::BufReader::new(module_file_descriptor).lines();
                for line in lines {
                    if let Ok(line) = line {
                        if self.comment_regex.is_match(&line) {
                            // If the line is a comment
                            writeln!(output_string, "{}", line).unwrap();
                        }
                        else if let Some(module_name) = self.use_regex.captures(&line) {
                            // If the line is a use statement, rewrite it
                            let module_name = module_name.get(1).unwrap().as_str().trim();
                            let modified_module_name = if is_root_module {
                                module_name.replace(current_module_name, "crate")
                            }
                            else {
                                module_name.to_string()
                            };
                            writeln!(output_string, "use {};", modified_module_name).unwrap();
                        }
                        else if let Some(module_name) = self.mod_regex.captures(&line) {
                            // If the line is a module import, process it
                            let module_name = module_name.get(2).unwrap().as_str().trim();
                            // Open the module closure
                            writeln!(output_string, "pub mod {} {{", module_name).unwrap();

                            // Inject the module content recursively
                            let full_module_path = if is_root_module {
                                let mut path = PathBuf::new();
                                path.push(&full_module_path);
                                path.pop();
                                path.push(module_name);
                                path
                            }
                            else {
                                Path::new(&full_module_path).join(module_name)
                            };
                            writeln!(output_string, "{}", self.inject_modules(full_module_path, &module_name, false)).unwrap();

                            // Close the closure
                            writeln!(output_string, "}}").unwrap();
                        }
                        else if self.eprint_regex.is_match(&line) {
                            // Output the line only if the flag to remove error printing is not set
                            if !self.opts.remove_error_output {
                                writeln!(output_string, "{}", line).unwrap();
                            }
                        }
                        else {
                            // Just output the line
                            writeln!(output_string, "{}", line).unwrap();
                        }
                    }
                }

                output_string
            }
            None => {
                panic!("File not found for module: {:?}", full_module_path);
            }
        }
    }
}


/// Try to find the package root by detecting the Cargo.toml file
fn detect_package_root() -> PathBuf {
    let mut current_folder = std::env::current_dir().unwrap();
    loop {
        // If we found the package root, return it
        let cargo_toml_path = current_folder.as_path().join(CARGO_TOML);
        if cargo_toml_path.exists() {
            debug!("Package root detected at: {:?}", current_folder);
            return current_folder;
        }

        // Else, go up
        match current_folder.parent() {
            None => { break; }
            Some(parent_folder) => { current_folder = PathBuf::from(parent_folder); }
        }
    }

    // Error case, we did not find the package root
    panic!("Rust package root not found.")
}

fn get_package_name(package_root_path: &PathBuf) -> String {
    let cargo_toml = fs::read_to_string(package_root_path.join(CARGO_TOML))
        .expect("Could not read Cargo.toml content");

    let cargo_toml = cargo_toml.parse::<Value>()
        .expect("Could not parse Cargo.toml content");

    let mut package_name = cargo_toml["package"]["name"].to_string();
    // Remove eventual quotes
    package_name = package_name.replace("\"", "").replace("-", "_");

    debug!("Package name: {}", package_name);
    package_name
}
