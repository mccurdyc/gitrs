use crate::repo;
use anyhow::{anyhow, Result};
use git2::{Cred, RemoteCallbacks};
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
        // TODO: use leveled logging.
        println!("[DEBUG] - f: {:?}", f);

        // TODO (mccurdyc): consider fetching updates for all repos here.
        if let Some(s) = f.to_str() {
            if !repos.contains_key(s) {
                // TODO (mccurdyc): prompt for input if there are uncommitted changes.
                fs::remove_dir(p)?;
            }
        };
    }
    // TODO: use leveled logging.
    println!("[DEBUG] - repos: {:?}", repos);

    // If directory doesn't exist, clone it.
    for r in repos.values() {
        // TODO: use leveled logging.
        println!("[DEBUG] - r: {:?}", r.get_name());

        if !Path::new(r.get_name()).exists() {
            clone_ssh(r.get_url(), root.join(r.get_name()).as_path())?;
        }
    }

    Ok(())
}

// https://docs.rs/git2/latest/git2/build/struct.RepoBuilder.html
// TODO: make the SSH params configurable.
fn clone_ssh(url: &str, dst: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    // TODO: fix this
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        // https://libgit2.org/libgit2/#HEAD/group/credential/git_credential_ssh_key_from_agent
        Cred::ssh_key(
            username_from_url.unwrap(),
            Some(Path::new(&format!(
                "{}/.ssh/fastly_rsa.pub",
                env::var("HOME").unwrap()
            ))),
            Path::new(&format!("{}/.ssh/fastly_rsa", env::var("HOME").unwrap())),
            Some(env::var("SSH_PASS").unwrap().as_str()),
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    println!("[DEBUG] - u: {}", url);
    match builder.clone(url, dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
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
