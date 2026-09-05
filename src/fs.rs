use crate::repo;
use anyhow::{Result, anyhow};
use git2::{Cred, RemoteCallbacks};
use ssh2;
use home;
use log::{debug, info};
use std::collections::HashMap;
use std::{fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

const GITRS_ROOT_DEFAULT: &str = "src";

pub fn sync(root: PathBuf, repos: &HashMap<String, repo::Repo>, clean_only: &bool) -> Result<()> {
    sync_with_fn(root, repos, clean_only, clone_ssh)
}

fn sync_with_fn(
    root: PathBuf,
    repos: &HashMap<String, repo::Repo>,
    _clean_only: &bool,
    clone_fn: fn(&str, &Path) -> Result<()>,
) -> Result<()> {
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

        // TODO: consider fetching updates for all repos here.
        if let Some(s) = f.to_str() {
            if !repos.contains_key(s) {
                // TODO: prompt for input if there are uncommitted changes.
                fs::remove_dir_all(d)?;
            }
        };
    }

    debug!("Looping repositories: {:?}", repos);

    // If directory doesn't exist, clone it.
    for r in repos.values() {
        debug!("On repository: {:?}", r.get_name());

        if !root.join(r.get_name()).exists() {
            clone_fn(r.get_url(), root.join(r.get_name()).as_path())?;
        }
    }

    Ok(())
}

/// clone_ssh clones a git repository to a specified path.
///
/// One thing to note is that clone_ssh does NOT respect your SSH config because
/// clone_ssh underneath uses libssh2 which does not respect your SSH config by default.
///
// TODO: support parsing a user's ssh config.
fn clone_ssh(url: &str, dst: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, username, _allowed_types| {
        Cred::ssh_key_from_agent(
            username.unwrap(),
        )
    });

    callbacks.certificate_check(|cert, hostname| {
        // GitHub serves ECDSA by "default" or in higher order because it's ECDSA is more
        // widely accepted by clients than ED25519 and RSA is more legacy.
        let raw = cert.as_hostkey()
            // and_then defines a new Option and "flattens" the result
            // it's lazy.
            .and_then(|hostkey| hostkey.hostkey())
            .ok_or_else(|| Err(anyhow::anyhow!("issue extracting or encoding the host key")))?;

            // TODO: read known hosts
        let s = ssh2::Session::new().or_else(|e| Err(anyhow::anyhow!("failed to create ssh session - {}", e)))?;
        let kh = s.known_hosts().or_else(|e| Err(anyhow::anyhow!("failed to create known hosts - {}", e)))?;
        kh.check(hostname, raw);

        Ok(git2::CertificateCheckStatus::CertificateOk)
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    info!("cloning: {}", url);
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
    use repo;
    use tempfile::{TempDir, tempdir};
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
        unsafe {
            env::set_var("HOME", root.path().as_os_str());
        }

        let want = home::home_dir()
            .expect("couldn't get user's HOME directory")
            .join("src");

        let got = init(None).expect("init failed");

        assert_eq!(want.exists(), true);
        assert_eq!(got, want);

        unsafe {
            env::set_var("HOME", old_home);
        }
        cleanup(root);
    }

    // #[test]
    // fn test_sync_add_repo_dir_doesnt_exists() {
    //     let root = setup();
    //
    //     let got = sync(
    //         root.path().to_path_buf(),
    //         &HashMap::from([(
    //             "github.com/a/a".to_string(),
    //             repo::Repo::new()
    //                 .name("github.com/a/a".to_string())
    //                 .expect("sync name failed")
    //                 .url("github.com/a/a".to_string())
    //                 .expect("sync url failed")
    //                 .pin(false)
    //                 .sha("".to_string())
    //                 .to_owned(),
    //         )]),
    //         &false,
    //     );
    //     assert_eq!(got.is_err(), false);
    //
    //     cleanup(root);
    // }
    //
    // #[test]
    // fn test_sync_add_repo_dir_exists() {
    //     let root = setup();
    //     cleanup(root);
    //     unimplemented!("test_sync");
    // }
    //
    // #[test]
    // fn test_sync_remove_repo_dir_exists() {
    //     let root = setup();
    //     cleanup(root);
    //     unimplemented!("test_sync");
    // }
    //
    // #[test]
    // fn test_sync_remove_repo_dir_doesnt_exists() {
    //     let root = setup();
    //     cleanup(root);
    //     unimplemented!("test_sync");
    // }
}
