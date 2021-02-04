# Cargo bundle

A cargo subcommand that bundles your crate source code into a single file.

The initial purpose of this command is to bundle your whole crate as a single source file that can be used on competitive programming platforms.

It works by expanding module imports by detecting them with regex, rewriting some "use" statements in the process.

## Install
Just run the following command:
```bash
cargo install cargo-bundle
```

## Usage
Simply call the cargo sub command inside your crate folder hierarchy (it can be any folder below the one containing your `Cargo.toml` file):
```bash
cargo bundle
```

This will generate a bundle file in `target/bundle/bundle.rs`.

## Options

| Long flag | Short flag | Description |
|-|-|-|
| `-s` | `--silence-standard-error-output` | Remove all the usages of `eprint!` and `eprintln!` macros from your code. |

## Credits
This little project is heavily inspired by [rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler).
It has the same approach and has the same goal, but I find the cargo subcommand approach less intrusive.
Also, I'll hopefully also maintain this project actively.
                                                                                                            