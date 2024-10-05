use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    commands::default::DefaultCommand,
    utils::cmd::Command,
};

fn main() -> Result<()> {
    let args = Args::parse();

    let subcommand = match args.subcommand {
        None => {
            let _ = DefaultCommand::new_and_run(args.path, args.repository_only)?;
            return Ok(());
        }
        Some(subcommand) => subcommand,
    };

    let _ = match subcommand {
        SubCommands::Remotes(cmd) => cmd.run(args.path, args.repository_only)?,
    };

    Ok(())
}
