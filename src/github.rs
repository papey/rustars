//! Github routines and structs
//!
//! Module containing routines to work with Github url and repos

use futures::future::try_join_all;
use futures_util::future::TryFutureExt;
use hubcaps::errors::Error as HubCapsErr;
use hubcaps::{Credentials, Github};
use log::{error, info};
use std::{fmt, io};
use tokio::runtime;

/// Parsing repository error
///
/// Used when url can not be parsed as a github repository
#[derive(Debug)]
pub struct RepoUrlError {
    url: String,
}

impl fmt::Display for RepoUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Can't get user and name from repository URL {}",
            self.url
        )
    }
}

/// Stargazing repository error
///
/// Used when a request failed to star a repo
#[derive(Debug)]
pub struct StargazError {
    err: HubCapsErr,
    repo: Repo,
}

impl fmt::Display for StargazError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error stargazing repository : {}, {}",
            self.repo, self.err
        )
    }
}

/// Meta module error
///
/// Wraps all kind of errors related to this module
#[derive(Debug)]
pub enum Error {
    HubCapsErr(hubcaps::Error),
    StargazErr(StargazError),
    RepoUrlErr(RepoUrlError),
    StdIOErr(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use custom impl of fmt from underlining error
        match *self {
            Error::HubCapsErr(ref e) => e.fmt(f),
            Error::StargazErr(ref e) => e.fmt(f),
            Error::RepoUrlErr(ref e) => e.fmt(f),
            Error::StdIOErr(ref e) => e.fmt(f),
        }
    }
}

// From HubCapsError to Error
impl From<StargazError> for Error {
    fn from(err: StargazError) -> Error {
        Error::StargazErr(err)
    }
}

// From CratesIOError to Error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::StdIOErr(err)
    }
}

// From hubcaps::Error to Error
impl From<hubcaps::Error> for Error {
    fn from(err: hubcaps::Error) -> Error {
        Error::HubCapsErr(err)
    }
}

/// Simple repository metadata structure
///
/// Contains a user and a repository name
#[derive(Debug)]
pub struct Repo {
    user: String,
    name: String,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "user: {}, name: {}", self.user, self.name)
    }
}

/// Custom type reprensenting Vector of Result containing a Repo
type VecResRepo = Vec<Result<Repo, Error>>;

/// Star all repositories given as arguments
///
/// Stars all the repositories passed as an iterator of string elements using the provided github token (with public_repo scope)
pub fn star_repos<T>(token: String, urls: T) -> Result<(), Error>
where
    T: std::iter::Iterator<Item = String>,
{
    let gh = Github::new(
        "Rustars (https://github.com/papey/rustars)",
        Credentials::Token(token),
    )?;

    let starrer = gh.activity().stars();

    // convert urls into repo structs, filter ok and errs
    let (repos, errs): (VecResRepo, VecResRepo) = urls
        .map(|url| repo_from_url(url))
        .partition(|r| r.is_ok() == true);

    // log errs
    errs.into_iter()
        .map(Result::unwrap_err)
        .for_each(|e| error!("{}", e));

    // init a runtime for async task
    let rt = runtime::Runtime::new().unwrap();

    // star repos using futures
    // create tasks
    let starize = async {
        // just rely on type inference in order to not 🤯
        // TODO: look for other possible solutions
        #[allow(unused_assignments)]
        let mut tasks = Vec::new();
        tasks = repos
            .into_iter()
            .map(Result::unwrap)
            .map(|r| {
                info!("Async stargazing of {}/{}", &r.user, &r.name);
                starrer
                    .star(&r.user, &r.name)
                    .map_err(|e| Error::StargazErr(StargazError { err: e, repo: r }))
            })
            .collect();

        return try_join_all(tasks).await;
    };

    rt.block_on(starize).map_err(|e| Error::from(e))?;

    Ok(())
}

/// Parse a repository url and return a `Repo` wrapped inside a `Result`
fn repo_from_url(url: String) -> Result<Repo, Error> {
    // filter is done before, for sure url contains https://github.com
    // input data : https://github.com/user/name
    // split on / to get ["https:", "", "github.com", "user", "name"]
    // skip the 3 first occurences
    let parts: Vec<&str> = url.split("/").skip(3).collect::<Vec<_>>();

    // if there is 2 elements at least in the split
    if parts.len() >= 2 {
        return Ok(Repo {
            user: String::from(parts[0]),
            name: String::from(parts[1]),
        });
    } else {
        // if not, there is an error
        return Err(Error::RepoUrlErr(RepoUrlError { url: url.clone() }));
    }
}

mod test {
    //! Test submodule
    #[allow(unused_imports)]
    use super::repo_from_url;

    #[test]
    fn test_repo_from_url() {
        // example url
        let url = "https://github.com/papey/rustars";

        let res = repo_from_url(String::from(url)).unwrap();

        assert_eq!(res.user, "papey");
        assert_eq!(res.name, "rustars")
    }
}
