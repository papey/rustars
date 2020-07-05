//! Rustars core

use crate::cargo::crates;
use crate::cargo::project;
use crate::github;
use log::debug;
use log::info;
use std::fmt;

/// Missing argument error
///
/// Used when an arguments from clap is None
#[derive(Debug)]
pub struct NoneError {
    name: String,
}

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There is no value associated with {} argument",
            self.name
        )
    }
}

/// Meta module error
///
/// Wraps all kind of errors related to this module
#[derive(Debug)]
pub enum Error {
    CargoErr(project::Error),
    CratesErr(crates::Error),
    GithubErr(github::Error),
    NoneErr(NoneError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use custom impl of fmt from underlining error
        match *self {
            Error::CargoErr(ref e) => e.fmt(f),
            Error::CratesErr(ref e) => e.fmt(f),
            Error::GithubErr(ref e) => e.fmt(f),
            Error::NoneErr(ref e) => e.fmt(f),
        }
    }
}

// From NoneError to Error
impl From<NoneError> for Error {
    fn from(err: NoneError) -> Error {
        Error::NoneErr(err)
    }
}
// From project::Error to Error
impl From<project::Error> for Error {
    fn from(err: project::Error) -> Error {
        Error::CargoErr(err)
    }
}

// From crates::Error to Error
impl From<crates::Error> for Error {
    fn from(err: crates::Error) -> Error {
        Error::CratesErr(err)
    }
}

// From github::Error to Error
impl From<github::Error> for Error {
    fn from(err: github::Error) -> Error {
        Error::GithubErr(err)
    }
}

/// Main routine
///
/// Called after argument parsing, this function wraps all the main tasks of the application
pub fn run(matches: clap::ArgMatches) -> Result<(), Error> {
    info!("Reading manifest file");

    // read manifest file
    let manifest =
        project::read_manifest_from_path(matches.value_of("manifest").ok_or(NoneError {
            name: "manifest".to_string(),
        })?)?;

    debug!("{:?}", manifest);

    info!("Fetching deps metadata from crates.io");

    // loop over all deps, get github repo if present
    let deps = crates::deps_from_crates_io(manifest)?;

    info!("Filtering deps to get links to github repos");

    debug!("{:?}", deps);

    // get github repos from all deps
    let urls = deps
        .iter()
        .filter_map(|d| d.crate_data.repository.clone())
        .filter(|r| r.contains("https://github.com"));

    info!("Stargazing repos");

    // For all repos, give stars ⭐
    github::star_repos(
        matches
            .value_of("token")
            .ok_or(NoneError {
                name: "token".to_string(),
            })?
            .to_string(),
        urls,
    )?;

    info!("All dependencies starred");

    Ok(())
}
