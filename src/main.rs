use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    if let Some(r) = c.root.as_deref() {
        println!("root: {}", r.display());
    }

    match &c.command {
        Commands::Add { repo } => {
            println!("'add' {repo:?}")
        }
        Commands::Remove { repo } => {
            println!("'remove' {repo:?}")
        }
        Commands::Sync {} => {
            println!("'sync'")
        }
    }
    Ok(())
}
