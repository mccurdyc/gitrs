use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Repo<'a> {
    name: &'a str,
    pin: bool,
    sha: &'a str,
}

// Modeling after OpenOptions. I did this so that Repo struct fields could change
// and the new() constructor interface didn't. Although, I think I could take
// some generic options arguments instead and set those.
impl<'a> Repo<'a> {
    pub fn new() -> Self {
        Repo {
            name: "",
            pin: false,
            sha: "",
        }
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
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
        *self
    }

    // get_git_ssh changes the name of the format "github.com/<org>/<name>" to Git SSH
    // protocol format.
    pub fn get_git_ssh(&self) {}
}
