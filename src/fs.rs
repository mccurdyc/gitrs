use crate::repo;
use anyhow::Result;
use home;
use std::collections::HashMap;
use std::{env, fs, path::PathBuf};

const ENV_GITRS_ROOT: &str = "GITRS_ROOT";
const GITRS_ROOT_DEFAULT: &str = "/src";

pub fn sync(_repos: HashMap<&str, repo::Repo>, _clean_only: &bool) -> Result<()> {
    //  If dir doesn't exist, run git clone.
    //  If dir DOES exist, but missing from config, rm the dir.
    //  TODO - implemented
    unimplemented!();
}

pub fn init(p: Option<PathBuf>) -> Result<PathBuf> {
    let binding = root(p);
    let r = binding.as_path();
    fs::create_dir_all(r)?;
    Ok(r.to_path_buf())
}

fn root(p: Option<PathBuf>) -> PathBuf {
    if let Some(r) = p {
        return r;
    }

    if let Ok(v) = env::var(ENV_GITRS_ROOT) {
        return PathBuf::from(v);
    }

    // defaults to $HOME/src
    let mut h = home::home_dir().expect("couldn't get user's HOME directory");
    h.push(PathBuf::from(GITRS_ROOT_DEFAULT));
    return h;
}
