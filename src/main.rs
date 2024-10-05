use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    commands::{default::DefaultCommand, remotes::RemotesCommand},
    utils::cmd::Command,
};

fn main() -> Result<()> {
    let args = Args::parse();

    let subcommand = match args.subcommand {
        None => {
            let _ = DefaultCommand::run(args.path, args.repository_only, DefaultCommand {})?;
            return Ok(());
        }
        Some(subcommand) => subcommand,
    };

    match subcommand {
        SubCommands::Remotes { domain, user } => {
            let _ = RemotesCommand::run(
                args.path,
                args.repository_only,
                RemotesCommand { domain, user },
            )?;
        }
    };

    Ok(())
}
