use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

use crate::repo::Repo;

const CONFIG_VERSION: &str = "v1beta";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Metadata {
    version: String,
    root: PathBuf,
    #[serde(skip_serializing, skip_deserializing)]
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    metadata: Metadata,
    repos: HashMap<String, Repo>,
}

impl Config {
    pub fn new(r: PathBuf, c: PathBuf) -> Result<Self, anyhow::Error> {
        let p = r.join(c);

        let cfg = Config {
            metadata: Metadata {
                version: CONFIG_VERSION.to_owned(),
                root: r,
                path: p,
            },
            repos: HashMap::new(),
        };

        Ok(cfg)
    }

    /// create creates the config file.
    pub fn create(&mut self) -> Result<Config> {
        create_dir_all(self.metadata.root.as_path()).context("Failed to create dir")?;

        if !self.metadata.path.exists() {
            File::create(self.metadata.path.as_path()).context("Failed to create")?;
            self.write().context("Failed to write")?
        }

        self.read(self.path())
    }

    /// write writes the config file.
    fn write(&mut self) -> Result<()> {
        println!("{:?}", self.metadata.path.as_path()); // path gets moved
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.metadata.path.as_path())
            .context("Couldn't open file")?;
        Ok(serde_yaml::to_writer(f, &self)?)
    }

    /// read reads the config file.
    pub fn read(&self, p: PathBuf) -> Result<Config> {
        let f = File::open(p.clone())?;

        let mut cfg: Config = serde_yaml::from_reader(f)?;
        cfg.metadata.path = p;
        Ok(cfg)
    }

    /// add adds a repo to the config and indicates whether or not the repo
    /// should be pinned at the first fetched commit sha.
    ///
    /// Pinning will prevent future fs::sync calls from checking for updates.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn add(&mut self, repo: String, pin: bool) -> Result<()> {
        let mut binding = Repo::new();
        let r = binding.name(repo.clone())?.pin(pin);

        self.repos.insert(repo, r.to_owned());
        self.write()
    }

    /// remove removes a repo from the config.
    ///
    /// Removing a repo from the config will indicate future fs::sync calls
    /// to ensure the repo directory is removed from the GITRS_ROOT directory.
    /// (This statement is a bit of package bleed, consider removing).
    pub fn remove(&self, _repo: &String) -> Result<()> {
        // TODO - implement
        unimplemented!();
    }

    // Naming conventions https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter

    pub fn root(&self) -> PathBuf {
        self.metadata.root.to_path_buf()
    }

    pub fn path(&self) -> PathBuf {
        self.metadata.path.to_path_buf()
    }

    pub fn repos(&self) -> &HashMap<String, Repo> {
        &self.repos
    }

    pub fn repos_mut(&mut self) -> &mut HashMap<String, Repo> {
        &mut self.repos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let got = Config::new(PathBuf::from("/foo"), PathBuf::from("test.yaml"))
            .expect("failed to create 'got'");

        assert_eq!(got.metadata.root, PathBuf::from("/foo"));
        assert_eq!(got.metadata.path, PathBuf::from("/foo/test.yaml"));
        assert_eq!(got.repos.len(), 0);
        assert_eq!(got.metadata.version, String::from(CONFIG_VERSION));
    }

    #[test]
    fn test_load_doesnt_exist_yet() {
        let root = tempdir().expect("Failed to create tempdir");
        // Note: use of path().to_path_buf() is to prevent moves.
        // I copied this pattern from tempdir's tests - https://github.com/Stebalien/tempfile/blob/a2b45b3363ddf31efcd4920462d6ec3e0ef9a909/tests/tempdir.rs#L72
        let mut got = Config::new(
            root.path().to_path_buf(),
            root.path().to_path_buf().join("test.yaml"),
        )
        .expect("expected successful creation of got value");

        got.create().expect("expected to create test.yaml");

        // dir and file should exist now
        assert_eq!(root.path().exists(), true);
        assert_eq!(root.path().join("test.yaml").exists(), true);

        // should be the default config values
        let want = Config::new(
            root.path().to_path_buf(),
            root.path().to_path_buf().join("test.yaml"),
        )
        .expect("expected successful creation of want value");

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
        let p = root.path().to_path_buf().join("test.yaml");
        let f = File::create(p).expect("Failed to create file");
        serde_yaml::to_writer(
            f,
            &Config {
                metadata: Metadata {
                    version: "hello".to_string(),
                    root: PathBuf::from("/foo"),
                    path: PathBuf::from("/foo/test.yaml"),
                },
                repos: HashMap::from([
                    (
                        "github.com/org/a".to_string(),
                        repo::Repo::new()
                            .name("github.com/org/a".to_string())
                            .expect("test repo name 'a' not working")
                            .pin(false)
                            .sha("sha".to_string())
                            .to_owned(),
                    ),
                    (
                        "github.com/org/b".to_string(),
                        repo::Repo::new()
                            .name("github.com/org/b".to_string())
                            .expect("test repo name 'b' not working")
                            .pin(true)
                            .sha("shasha".to_string())
                            .to_owned(),
                    ),
                ]),
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
