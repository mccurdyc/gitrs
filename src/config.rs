use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::path::PathBuf;

use crate::repo;

const CONFIG_VERSION: &str = "v1beta";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Metadata {
    version: String,
    root: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config<'a> {
    metadata: Metadata,
    #[serde(borrow)]
    repos: Vec<repo::Repo<'a>>,
}

impl<'a> Config<'a> {
    pub fn new(r: PathBuf) -> Self {
        Config {
            metadata: Metadata {
                version: CONFIG_VERSION.to_owned(),
                root: r,
            },
            repos: Vec::new(),
        }
    }

    /// load loads or creates and then loads a config file.
    pub fn load(&mut self, p: PathBuf) -> Result<()> {
        if !p.exists() {
            let f = File::create(p.as_path())
                .with_context(|| format!("Failed to create file {}", p.display()))?;
            serde_yaml::to_writer(f, &self)?; // serialize
        }
        Ok(())
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
    use crate::repo;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let got = Config::new(PathBuf::from("/foo"));

        assert_eq!(got.metadata.root, PathBuf::from("/foo"));
        assert_eq!(got.repos.len(), 0);
        assert_eq!(got.metadata.version, String::from(CONFIG_VERSION));
    }

    #[test]
    fn test_load_doesnt_exist_yet() {
        let root = tempdir().expect("Failed to create tempdir");
        // Note: use of path().to_path_buf() is to prevent moves.
        // I copied this pattern from tempdir's tests - https://github.com/Stebalien/tempfile/blob/a2b45b3363ddf31efcd4920462d6ec3e0ef9a909/tests/tempdir.rs#L72
        let p = root.path().to_path_buf().join(".gitrs.yaml");

        let mut got = Config::new(root.path().to_path_buf());
        got.load(p).expect("expected config");

        // dir and file should exist now
        assert_eq!(root.path().exists(), true);
        assert_eq!(root.path().join(".gitrs.yaml").exists(), true);

        // should be the default config values
        let want = Config::new(root.path().to_path_buf());
        assert_eq!(got, want);

        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        root.close().expect("Failed to close tempdir");
    }

    #[test]
    fn test_load_already_exists() {
        let root = tempdir().expect("Failed to create tempdir");
        // Note: use of path().to_path_buf() is to prevent moves.
        // I copied this pattern from tempdir's tests - https://github.com/Stebalien/tempfile/blob/a2b45b3363ddf31efcd4920462d6ec3e0ef9a909/tests/tempdir.rs#L72
        let p = root.path().to_path_buf().join(".gitrs.yaml");
        let f = File::create(p).expect("Failed to create file");
        serde_yaml::to_writer(
            f,
            &Config {
                metadata: Metadata {
                    version: "hello".to_string(),
                    root: PathBuf::from("/foo"),
                },
                repos: vec![
                    repo::Repo::new().name("a").pin(false).sha("sha").to_owned(),
                    repo::Repo::new()
                        .name("b")
                        .pin(true)
                        .sha("shasha")
                        .to_owned(),
                ],
            },
        )
        .expect("Failed to serialize Config");

        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        root.close().expect("Failed to close tempdir");
    }
}
