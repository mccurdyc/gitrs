use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::repo;

const CONFIG_VERSION: &str = "v1beta";

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    version: String,
    root: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    file_contents: Vec<u8>,
    metadata: Metadata,
    repos: Vec<repo::Repo>,
}

impl Config {
    pub fn new(r: PathBuf) -> Self {
        Config {
            file_contents: Vec::new(),
            metadata: Metadata {
                version: CONFIG_VERSION.to_owned(),
                root: r,
            },
            repos: Vec::new(),
        }
    }

    /// load loads or creates and then loads a config file.
    pub fn load(&mut self, f: &Path) -> Result<Self> {
        let mut bind = self.metadata.root.clone();
        bind.push(f);
        let p = bind.as_path();

        // if the config file doesn't exist, create it.
        let mut f = OpenOptions::new().read(true).open(p).unwrap_or_else(|_| {
            // TODO: could these panics be propogated up instead?
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(p)
                .expect("Couldn't open file");
            serde_yaml::to_writer(&mut f, self).expect("Couldn't write to file");
            f
        });

        f.read_to_end(&mut self.file_contents)?;
        let cfg: Config = serde_yaml::from_reader(f)?;
        Ok(cfg)
    }

    /// add adds a repo to the config and indicates whether or not the repo
    /// should be pinned at the first fetched commit sha.
    ///
    /// Pinning will prevent future fs::sync calls from checking for updates.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn add(&self, _repo: &String, _pin: &bool) -> Result<()> {
        // TODO - implement
        return Err(anyhow!("not implemented"));
    }

    /// remove removes a repo from the config.
    ///
    /// Removing a repo from the config will indicate future fs::sync calls
    /// to ensure the repo directory is removed from the GITRS_ROOT directory.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn remove(&self, _repo: &String) -> Result<()> {
        // TODO - implement
        return Err(anyhow!("not implemented"));
    }

    /// list_repos lists the repositories.
    pub fn list_repos(&self) -> Result<Vec<repo::Repo>> {
        // TODO - implement
        return Err(anyhow!("not implemented"));
    }
}
