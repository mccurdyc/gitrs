use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Repo {
    name: String,
    url: String,
    pin: bool,
    sha: String,
}

// Modeling after OpenOptions. This is so that Repo struct fields can change, but
// not affect the new() constructor interface.
impl Repo {
    pub fn new() -> Self {
        Repo {
            name: "".to_owned(),
            url: "".to_owned(),
            pin: false,
            sha: "".to_owned(),
        }
    }

    pub fn name(&mut self, name: String) -> Result<&mut Self> {
        self.name = name.clone();
        self.url(name.clone())?;

        Ok(self)
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    // url changes the name of the format "github.com/<org>/<name>" to Git SSH
    // protocol format.
    //
    // Only supports SSH cloning, [similar to Go](https://cs.opensource.google/go/go/+/refs/heads/master:src/cmd/go/internal/get/get.go%3Bdrc=91b8cc0dfaae12af1a89e2b7ad3da10728883ee1%3Bl=423).
    // https://cs.opensource.google/go/go/+/refs/heads/master:src/cmd/go/internal/vcs/vcs.go%3Bl=301%3Bdrc=7ad92e95b56019083824492fbec5bb07926d8ebd
    pub fn url(&mut self, name: String) -> Result<&mut Self> {
        let v: Vec<&str> = name.as_str().split('/').collect();

        debug!("Split repo vec: {:?}", v);
        if name.contains(':') || name.contains('@') || v.len() != 3 {
            return Err(anyhow!(
                "Invalid repo name: name should be of the format <host>/<org>/<repo>"
            ));
        }

        self.url = format!("git@{}:{}/{}.git", v[0], v[1], v[2]);
        Ok(self)
    }

    pub fn get_url(&self) -> &str {
        self.url.as_str()
    }

    pub fn pin(&mut self, pin: bool) -> &mut Self {
        self.pin = pin;
        self
    }

    pub fn sha(&mut self, sha: String) -> &mut Self {
        self.sha = sha;
        self
    }

    pub fn to_owned(&mut self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate log;
    use env_logger;

    fn setup() -> Repo {
        // https://github.com/rust-cli/env_logger/blob/19e92ece73472ca3a0269c61c4f44399c6ea2366/examples/in_tests.rs#L21
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();

        Repo::new()
    }

    #[test]
    fn test_url_invalid_name() {
        let mut r = setup();

        // contains ":"
        let got = r.url("a:a/a/a".to_string());
        assert_eq!(got.is_err(), true);

        // contains "@"
        let got = r.url("a@a/a/a".to_string());
        assert_eq!(got.is_err(), true);

        // <3
        let got = r.url("a/a".to_string());
        assert_eq!(got.is_err(), true);
    }

    #[test]
    fn test_url_valid_name() {
        let mut r = setup();

        let got = r
            .url("github.com/a/a".to_string())
            .expect("failed to set url");
        assert_eq!(got.get_url(), "git@github.com:a/a.git");
    }
}
