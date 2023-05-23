use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Repo {
    name: String,
    pin: bool,
    sha: String,
}

impl Repo {
    // get_git_ssh changes the name of the format "github.com/<org>/<name>" to Git SSH
    // protocol format.
    pub fn get_git_ssh(&self) {}
}
