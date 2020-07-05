# Rustars

A commmand line tool to star a list of Github repositories taken from
dependencies of a Cargo.toml file.

Do not forget to thanks a project maintainer anymore !

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)

### Installing

#### From source

Clone this repo and run

```sh
cargo build --release
```

### Usage

```sh
./target/release/rustars --help
```

By default, Rustars get the manifest file from current working directory. In
order to specify another manifest file, use the `manifest` option.

## Running the tests

```sh
cargo test
```

## Built With

- [cargo_toml](https://crates.io/crates/cargo_toml) - A Manifest file reader
- [hubcaps](https://github.com/softprops/hubcaps) - A Rust interface for GitHub
- [crates_io_api](https://github.com/theduke/crates_io_api) - API client for crates.io
- [clap](https://github.com/clap-rs/clap) - A command line argument parser

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

- **Wilfried OLLIVIER** - _Main author_ - [Papey](https://github.com/papey)

## License

[LICENSE](LICENSE) file for details

## Notes

The names come from the "verlan" (a french argotic dialect) of the word "stars"
