use anyhow::{Context, Error};
use clap::{Parser, Subcommand};
use env_logger::{Builder, Env, Target};
extern crate log;
use std::env;
use std::path::PathBuf;

pub mod config;
pub mod fs;
pub mod repo;

/// A simple, opinionated, tool, written in Rust, for declaretively managing Git repos on your machine.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Directory that you want gitrs to sync to
    #[arg(global = true, long, value_name = "FILE")]
    root: Option<PathBuf>,
    // Path to your gitrs config file.
    #[arg(global = true, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add repository to config to be managed by gitrs.
    Add {
        repo: String,
        // TODO - implement
        #[arg(short, long)]
        pin: bool,
    },
    /// Remove repository from the filesystem and from being managed by gitrs.
    Remove { repo: String },
    /// Sync fetches a repository if it exists, clones it if it doesn't, and
    /// removes it if it exists, but the config no longer has a record for it.
    Sync {
        // TODO - implement
        /// Force a clean-only sync i.e., don't fetch updates or try to clone missing repos.
        #[arg(short, long)]
        clean_only: bool,
    },
}

fn main() -> anyhow::Result<(), Error> {
    let env = Env::default().default_filter_or("info");

    Builder::from_env(env)
        .default_format()
        .target(Target::Stdout)
        .init();

    let cli = Cli::parse();
    run(cli)
}

fn run(mut c: Cli) -> anyhow::Result<(), Error> {
    if let Ok(root) = env::var("GITRS_ROOT") {
        c.root = Some(PathBuf::from(root));
    }

    let r = fs::init(c.root).expect("failed to initialize root");
    let mut cfg = config::Config::new(r, PathBuf::from(".gitrs.yaml"))?;

    // NOTE: reads the entire gitrs config into memory. this is notable because writes of the
    // config file truncate contents first.
    cfg = match cfg.path().exists() {
        true => cfg.read(cfg.path()).expect("failed to read config"),
        false => cfg.create().expect("failed to create config"),
    };

    match &c.command {
        Commands::Add { repo, pin } => {
            cfg.add(repo.to_string(), *pin)
                .with_context(|| format!("failed to add repo: {}", repo))?;
        }
        Commands::Remove { repo } => {
            cfg.remove(repo.to_string())
                .with_context(|| format!("failed to remove repo: {}", repo))?;
        }
        Commands::Sync { clean_only } => {
            fs::sync(cfg.root(), cfg.repos(), clean_only).context("failed to sync repos")?;
        }
    }
    Ok(())
}
