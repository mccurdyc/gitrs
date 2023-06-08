use anyhow::{anyhow, Result};
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
