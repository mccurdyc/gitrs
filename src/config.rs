use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

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
    pub fn load(&mut self, p: PathBuf) -> Result<Config> {
        let v = self.read_or_create(p)?;
        let cfg: Config = serde_yaml::from_slice(v.as_slice())?;
        Ok(cfg)
    }

    fn read_or_create(&mut self, p: PathBuf) -> Result<Vec<u8>> {
        if !p.exists() {
            let f = File::create(p.as_path())
                .with_context(|| format!("Failed to create file {}", p.display()))?;
            serde_yaml::to_writer(f, &self)?; // serialize
        }

        let d = fs::read(p)?;
        Ok(d)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let got = Config::new(PathBuf::from("/foo"));

        assert_eq!(got.metadata.root, PathBuf::from("/foo"));
        assert_eq!(got.file_contents.len(), 0);
        assert_eq!(got.repos.len(), 0);
        assert_eq!(got.metadata.version, String::from(CONFIG_VERSION));
    }

    #[test]
    fn test_load() {
        let _got = Config::new(PathBuf::from("/foo"));
    }
}