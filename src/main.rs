use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    commands::{none_info::git_ls, remotes::ls_remotes},
};

fn main() -> Result<()> {
    let args = Args::parse();

    let subcommand = match args.subcommand {
        None => {
            let _ = git_ls(args.path, args.repository_only)?;
            return Ok(());
        }
        Some(subcommand) => subcommand,
    };

    match subcommand {
        SubCommands::Remotes { domain, user } => {
            ls_remotes(args.path, args.repository_only, domain, user)
        }
    }?;

    Ok(())
}
