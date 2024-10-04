use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    subcommands::remotes::ls_remotes,
};

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
