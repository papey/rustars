//! Manifest parsing

use cargo_toml::Manifest;
use std::fmt;
use std::path::Path;

/// Custom error for Project module
///
/// Used to wrap error and
#[derive(Debug)]
pub struct Error {
    path: String,
    error: cargo_toml::Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use custom impl of fmt from underlining error
        write!(f, "Error with Manifest file {} : {}", self.path, self.error)
    }
}

/// Read a Manifest file from a Path
pub fn read_manifest_from_path(path: &str) -> Result<Manifest, Error> {
    Manifest::from_path_with_metadata(Path::new(path)).map_err(|e| Error {
        path: String::from(path),
        error: e,
    })
}

/// Create a dumb manifest file for tests
#[allow(dead_code)]
pub fn dumb_manifest() -> Manifest {
    let toml = r#"
    [package]
    name = "dumb"
    version = "0.1.0"
    authors = ["Dumbo Dumb"]
    edition = "2018"

    [dependencies]
    log = "0.4"
    futures = "0.3"
"#;

    Manifest::from_str(toml).unwrap()
}

mod test {
    //! Test submodule
    #[allow(unused_imports)]
    use super::read_manifest_from_path;

    #[test]
    fn test_read_manifest_from_path() {
        // get manifest from rustars Cargofile
        let manifest = read_manifest_from_path("Cargo.toml").unwrap();

        // assert on dependencies
        assert!(manifest.dependencies.contains_key("log"));
        assert!(manifest.dependencies.contains_key("tokio"));
    }
}
