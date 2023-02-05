/*! A commmand line tool to star a list of Github repositories taken from dependencies of a `Cargo.toml` file

# Overview

Rustars is a command line tool that reads a `Cargo.toml` file in order to star asssociated
github repositories. It first read deps from manifest file, get dependencies metadata from
[crates.io](https://crates.io) and then stars repositories on [github.com](https://github.com)

# Built with

- [cargo_toml](https://crates.io/crates/cargo_toml)
- [hubcaps](https://github.com/softprops/hubcaps)
- [crates_io_api](https://github.com/theduke/crates_io_api)
- [clap](https://github.com/clap-rs/clap)

*/

extern crate log;

use clap::{App, Arg};
use log::info;
use std::process;

mod app;
mod cargo;
mod github;

/// Rustars entrypoint
///
/// Handle argument parsing, main routines running and error feedback
fn main() {
    env_logger::init();

    // parse args
    let matches = App::new("Rustars")
        .version("0.1.2")
        .author("Wilfried OLLIVIER")
        .about(
            "A commmand line tool to star a list of Github repositories taken from
dependencies of a Cargo.toml file.",
        )
        .arg(
            Arg::with_name("loglevel")
                .env("RUST_LOG")
                .help("Set loglevel using RUST_LOG environment variable")
                .default_value("ERROR"),
        )
        .arg(
            Arg::with_name("manifest")
                .short("m")
                .long("manifest")
                .env("MANIFEST")
                .help("Path to manifest (Cargo.toml) file")
                .default_value("Cargo.toml"),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .env("TOKEN")
                .help("Github API Token (needs public_repo scope)"),
        )
        .get_matches();

    info!("Starting main routine");

    // start main routine, handle error if any
    if let Err(e) = app::run(matches) {
        eprintln!("Application error : {}", e);

        process::exit(1);
    }
}
