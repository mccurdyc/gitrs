use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A simple, opinionated, tool, written in Rust, for declaretively managing Git repos on your machine.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(global = true, long, value_name = "FILE", default_value = "$HOME/src")]
    root: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add repository to config to be managed by gitrs.
    Add { repo: String },
    /// Remove repository from the filesystem and from being managed by gitrs.
    Remove {
        repo: String,
        // TODO
        // #[arg(short, long)]
        // archive: bool,
    },
    /// Sync fetches a repository if it exists, clones it if it doesn't, and
    /// removes it if it exists, but the config no longer has a record for it.
    Sync {
        // TODO
        // /// Force a clean-only sync i.e., don't fetch updates or try to clone missing repos.
        // #[arg(short, long)]
        // clean: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    handle(cli)
}

fn handle(c: Cli) -> anyhow::Result<()> {
    // TODO: if root/.gitrs.yaml doesn't exist, create it.
    if let Some(r) = c.root.as_deref() {
        println!("root: {}", r.display());
    }

    match &c.command {
        Commands::Add { repo } => {
            println!("'add' {repo:?}")
            // TODO: adds a line to the config file.
            // - The line is going to need to specify
            // <https/ssh> <provider>/<org>/<name>
            // This isn't easily-parsable / YAML format
        }
        Commands::Remove { repo } => {
            println!("'remove' {repo:?}")
            // TODO: removes a line from the config file.
            // - Allow just specifying <name> and then have user select if there are
            // multiple results.
        }
        Commands::Sync {} => {
            println!("'sync'")
            // TODO: reads config file.
            // TODO: checks some state/lockfile / uses the GITRS_ROOT as a the state DB
            // - would be nice to not have to traverse the filesystem a bunch when a lot
            // of the time there might be nothing to do.
            // TODO: writes to the state/lockfile.
        }
    }
    Ok(())
}
