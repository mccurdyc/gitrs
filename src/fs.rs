use crate::repo;
use anyhow::Result;
use git2::Repository;
use home;
use std::collections::HashMap;
use std::{env, fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

const ENV_GITRS_ROOT: &str = "GITRS_ROOT";
const GITRS_ROOT_DEFAULT: &str = "/src";

pub fn sync(root: PathBuf, repos: &HashMap<String, repo::Repo>, _clean_only: &bool) -> Result<()> {
    for entry in WalkDir::new(root.as_path())
        .min_depth(3) // forces it to look at full paths only
        .max_depth(3)
        .contents_first(true)
    {
        // If the directory doesn't exist in the config, delete it.
        // This forces you to declare the repos.
        let e = entry?;
        let p = e.path();
        let f = p.strip_prefix(root.as_path())?;
        println!("f: {:?}", f);

        // TODO (mccurdyc): consider fetching updates for all repos here.
        if let Some(s) = f.to_str() {
            if !repos.contains_key(s) {
                // TODO (mccurdyc): prompt for input if there are uncommitted changes.
                fs::remove_dir(p)?;
            }
        };
    }
    println!("repos: {:?}", repos);

    // If directory doesn't exist, clone it.
    for r in repos.values() {
        println!("r: {:?}", r.get_name());

        if !Path::new(r.get_name()).exists() {
            Repository::clone(r.get_url(), Path::new(r.get_name()))?;
        }
    }

    Ok(())
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
