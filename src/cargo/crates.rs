//! Crates metadata fetching

#[allow(unused_imports)]
use crate::cargo::project::dumb_manifest;
use cargo_toml::{DepsSet, Manifest};
use crates_io_api::Error as CratesIOError;
use crates_io_api::{AsyncClient, CrateResponse};
use futures::future::try_join_all;
use reqwest::header::InvalidHeaderValue as HeaderError;
use reqwest::Error as HttpError;
use std::{fmt, io};
use tokio::runtime;

/// Meta module error
#[derive(Debug)]
pub enum Error {
    CratesIOErr(CratesIOError),
    StdIOErr(io::Error),
    HttpErr(HttpError),
    HeaderErr(HeaderError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use custom impl of fmt from underlining error
        match *self {
            Error::CratesIOErr(ref e) => e.fmt(f),
            Error::StdIOErr(ref e) => e.fmt(f),
            Error::HttpErr(ref e) => e.fmt(f),
            Error::HeaderErr(ref e) => e.fmt(f),
        }
    }
}

// From CratesIOError to Error
impl From<CratesIOError> for Error {
    fn from(err: CratesIOError) -> Error {
        Error::CratesIOErr(err)
    }
}

// From CratesIOError to Error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::StdIOErr(err)
    }
}

// From HttpError to Error
impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::HttpErr(err)
    }
}

// From HeaderError to Error
impl From<HeaderError> for Error {
    fn from(err: HeaderError) -> Error {
        Error::HeaderErr(err)
    }
}

/// Get dependencies metadata from [crates.io](https://crates.io)
///
/// Takes a Manifest struct as argument, for each dependency, get the metadata from crates.io and return a `Vec` of `CrateResponse`
pub fn deps_from_crates_io(manifest: Manifest) -> Result<Vec<CrateResponse>, Error> {
    // merge all deps in an array
    let deps: [DepsSet; 3] = [
        manifest.dependencies,
        manifest.dev_dependencies,
        manifest.build_dependencies,
    ];

    // create client
    let client = AsyncClient::new(
        "Rustars (https://github.com/papey/rustars)",
        std::time::Duration::from_millis(100),
    )?;

    // init a runtime for async task
    let mut rt = runtime::Runtime::new()?;

    let tasks = async {
        // init tasks vector
        let mut tasks = Vec::new();
        for subdeps in deps.iter() {
            for key in subdeps.keys() {
                // push all tasks
                tasks.push(client.get_crate(key))
            }
        }
        // wait for tasks completion
        return try_join_all(tasks).await;
    };

    // return Results
    rt.block_on(tasks).map_err(|e| Error::from(e))
}

mod test {
    //! Test submodule
    #[allow(unused_imports)]
    use super::{deps_from_crates_io, dumb_manifest};

    #[test]
    fn test_deps_from_crates_io() {
        // get dumb manifest
        let manifest = dumb_manifest();
        // get crate io data
        let deps = deps_from_crates_io(manifest).unwrap();

        // assert on data name
        assert_eq!(deps[0].crate_data.name, "futures");
        assert_eq!(deps[1].crate_data.name, "log")
    }
}
