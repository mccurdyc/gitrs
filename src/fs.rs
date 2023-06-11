use crate::repo;
use anyhow::{anyhow, Result};
use git2::{Cred, RemoteCallbacks};
use home;
use log::{debug, error};
use std::collections::HashMap;
use std::{env, fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

const ENV_GITRS_ROOT: &str = "GITRS_ROOT";
const GITRS_ROOT_DEFAULT: &str = "src";

pub fn sync(root: PathBuf, repos: &HashMap<String, repo::Repo>, _clean_only: &bool) -> Result<()> {
    for entry in WalkDir::new(root.as_path())
        .min_depth(3) // forces it to look at full paths only
        .max_depth(3)
        .contents_first(true)
    {
        // If the directory doesn't exist in the config, delete it.
        // This forces you to declare the repos.
        let e = entry?;
        let d = e.path();
        let f = d.strip_prefix(root.as_path())?;
        debug!("Using directory: {:?}", d);

        // TODO (mccurdyc): consider fetching updates for all repos here.
        if let Some(s) = f.to_str() {
            if !repos.contains_key(s) {
                // TODO (mccurdyc): prompt for input if there are uncommitted changes.
                fs::remove_dir_all(d)?;
            }
        };
    }

    debug!("Looping repositories: {:?}", repos);

    // If directory doesn't exist, clone it.
    for r in repos.values() {
        debug!("On repository: {:?}", r.get_name());

        if !root.join(r.get_name()).exists() {
            clone_ssh(r.get_url(), root.join(r.get_name()).as_path())?;
        }
    }

    Ok(())
}

// https://docs.rs/git2/latest/git2/build/struct.RepoBuilder.html
fn clone_ssh(url: &str, dst: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, username, _allowed_types| {
        let mut ssh_privkey = PathBuf::new();
        let mut ssh_privkey_pass = String::from("");

        // default
        if let Some(h) = home::home_dir() {
            ssh_privkey = h.join(".ssh/id_rsa");
        }

        // default
        if let Ok(pw) = env::var("SSH_PRIVKEY_PASS") {
            ssh_privkey_pass = pw;
        }

        if !ssh_privkey.exists() {
            ssh_privkey = PathBuf::from(env::var("SSH_PRIVKEY_PATH").expect("$HOME/.ssh/id_rsa doesn't exists, you must specify an ssh private key path via SSH_PRIVKEY_PATH"));
            if !ssh_privkey.exists() {
                error!("$SSH_PRIVKEY_PATH doesn't exists");
            }
        }

        // https://libgit2.org/libgit2/#HEAD/group/credential/git_credential_ssh_key_from_agent
        Cred::ssh_key(
            username.unwrap(),
            None,
            &ssh_privkey.as_path(),
            Some(ssh_privkey_pass.as_str()),
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    debug!("Using clone url: {}", url);
    match builder.clone(url, dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn init(p: Option<PathBuf>) -> Result<PathBuf> {
    let binding = root(p);
    let r = binding.as_path();
    debug!("Initializing root: {:?}", r);
    fs::create_dir_all(r)?;
    Ok(r.to_path_buf())
}

// TODO write tests for this function
fn root(p: Option<PathBuf>) -> PathBuf {
    if let Some(r) = p {
        return r;
    }

    if let Ok(v) = env::var(ENV_GITRS_ROOT) {
        return PathBuf::from(v);
    }

    // defaults to $HOME/src
    let h = home::home_dir().expect("couldn't get user's HOME directory");
    return h.join(PathBuf::from(GITRS_ROOT_DEFAULT));
}
