use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    commands::{
        default::git_ls,
        remotes::{git_ls_with_remotes, RemotesArgs},
    },
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
            let _ = git_ls_with_remotes(
                args.path,
                args.repository_only,
                RemotesArgs { domain, user },
            )?;
        }
    };

    Ok(())
}
