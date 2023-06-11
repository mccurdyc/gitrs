use crate::repo;
use anyhow::{anyhow, Result};
use git2::{Cred, RemoteCallbacks};
use home;
use log::{debug, error};
use std::collections::HashMap;
use std::{env, fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

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

fn root(p: Option<PathBuf>) -> PathBuf {
    if let Some(r) = p {
        return r;
    }

    // defaults to $HOME/src
    let h = home::home_dir().expect("couldn't get user's HOME directory");
    return h.join(PathBuf::from(GITRS_ROOT_DEFAULT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    extern crate log;
    use env_logger;

    fn setup() -> TempDir {
        // https://github.com/rust-cli/env_logger/blob/19e92ece73472ca3a0269c61c4f44399c6ea2366/examples/in_tests.rs#L21
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();

        tempdir().expect("Failed to create tempdir")
    }

    fn cleanup(root: TempDir) {
        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        root.close().expect("Failed to close tempdir");
    }

    #[test]
    fn test_init_from_input() {
        let root = setup();
        let p = root.path().to_path_buf();

        init(Some(p.clone())).expect("init failed");
        assert_eq!(p.exists(), true);

        cleanup(root);
    }

    #[test]
    fn test_init_from_default() {
        let root = setup();
        let old_home = env::var("HOME").expect("failed to get old home");
        env::set_var("HOME", root.path().as_os_str());

        let want = home::home_dir()
            .expect("couldn't get user's HOME directory")
            .join("src");

        let got = init(None).expect("init failed");

        assert_eq!(want.exists(), true);
        assert_eq!(got, want);

        env::set_var("HOME", old_home);
        cleanup(root);
    }
}
