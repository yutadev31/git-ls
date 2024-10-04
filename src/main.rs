use anyhow::Result;
use clap::{Parser, Subcommand};

use git_ls::subcommands::remotes::ls_remotes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Remotes {
        #[arg(default_value_t = String::from("."))]
        path: String,

        #[arg(short('r'), long, default_value_t = false)]
        repository_only: bool,

        #[arg(short('d'), long, default_value_t = String::new())]
        domain: String,

        #[arg(short('u'), long, default_value_t = String::new())]
        user: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommands::Remotes {
            path,
            repository_only,
            domain,
            user,
        } => ls_remotes(path, repository_only, domain, user),
    }?;

    Ok(())
}
