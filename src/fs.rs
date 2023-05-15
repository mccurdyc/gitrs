use crate::repo;
use home;
use std::{env, fs, io::Error, io::ErrorKind, path::PathBuf};

const ENV_GITRS_ROOT: &str = "GITRS_ROOT";
const GITRS_ROOT_DEFAULT: &str = "/src";

pub fn sync(repos: Vec<repo::Repo>, _clean_only: &bool) -> Result<(), Error> {
    return Err(Error::new(ErrorKind::Other, "not implemented"));
}

pub fn init(p: Option<PathBuf>) -> Result<PathBuf, Error> {
    let r = root(p);
    if let Err(e) = fs::create_dir_all(r.clone()) {
        return Err(e);
    }
    Ok(r)
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
