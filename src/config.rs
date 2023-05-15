use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::repo;

const CONFIG_VERSION: &str = "v1beta";

struct Metadata<'a> {
    version: &'a str,
    root: PathBuf,
}

pub struct Config<'a> {
    metadata: Metadata<'a>,
    repos: Vec<repo::Repo<'a>>,
}

impl<'a> Config<'a> {
    /// new is a config file constructor method.
    pub fn new(r: PathBuf) -> Self {
        Self {
            metadata: Metadata {
                version: CONFIG_VERSION,
                root: r,
            },
            repos: Vec::new(),
        }
    }

    /// init creates the config file on disk in the GITRS_ROOT directory.
    /// At this point, metadata and general config file structure is added.
    pub fn init(&self) -> Result<(), Error> {
        // TODO - implement
        return Err(Error::new(ErrorKind::Other, "not implemented"));
    }

    /// add adds a repo to the config and indicates whether or not the repo
    /// should be pinned at the first fetched commit sha.
    ///
    /// Pinning will prevent future fs::sync calls from checking for updates.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn add(&self, repo: &String, _pin: &bool) -> Result<(), Error> {
        // TODO - implement
        return Err(Error::new(ErrorKind::Other, "not implemented"));
    }

    /// remove removes a repo from the config.
    ///
    /// Removing a repo from the config will indicate future fs::sync calls
    /// to ensure the repo directory is removed from the GITRS_ROOT directory.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn remove(&self, repo: &String) -> Result<(), Error> {
        // TODO - implement
        return Err(Error::new(ErrorKind::Other, "not implemented"));
    }

    /// list_active_repos lists the repositories that are not pinned.
    pub fn list_active_repos(&self) -> Result<Vec<repo::Repo>, Error> {
        // TODO - implement
        return Err(Error::new(ErrorKind::Other, "not implemented"));
    }
}
