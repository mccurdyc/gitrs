use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Repo<'a> {
    name: &'a str,
    #[serde(skip_serializing, skip_deserializing)]
    url: String,
    pin: bool,
    sha: &'a str,
}

// Modeling after OpenOptions. This is so that Repo struct fields can change, but
// not affect the new() constructor interface.
impl<'a> Repo<'a> {
    pub fn new() -> Self {
        Repo {
            name: "",
            url: "".to_owned(),
            pin: false,
            sha: "",
        }
    }

    pub fn name(&mut self, name: &'a str) -> Result<&mut Self> {
        self.name = name;
        self.url(name)?;

        Ok(self)
    }

    // url changes the name of the format "github.com/<org>/<name>" to Git SSH
    // protocol format.
    //
    // Only supports SSH cloning, [similar to Go](https://cs.opensource.google/go/go/+/refs/heads/master:src/cmd/go/internal/get/get.go%3Bdrc=91b8cc0dfaae12af1a89e2b7ad3da10728883ee1%3Bl=423).
    // https://cs.opensource.google/go/go/+/refs/heads/master:src/cmd/go/internal/vcs/vcs.go%3Bl=301%3Bdrc=7ad92e95b56019083824492fbec5bb07926d8ebd
    fn url(&mut self, name: &str) -> Result<&mut Self> {
        let v: Vec<&str> = name.split('/').collect();

        if name.contains(':') || name.contains('@') || v.len() != 3 {
            return Err(anyhow!(
                "Invalid repo name: name should be of the format <host>/<org>/<repo>"
            ));
        }

        self.url = format!("git@{}:{}/{}.git", v[0], v[1], v[2]);
        Ok(self)
    }

    pub fn pin(&mut self, pin: bool) -> &mut Self {
        self.pin = pin;
        self
    }

    pub fn sha(&mut self, sha: &'a str) -> &mut Self {
        self.sha = sha;
        self
    }

    pub fn to_owned(&mut self) -> Self {
        self.clone()
    }
}
