use anyhow::Result;

use clap::Parser;
use git_ls::{
    args::{Args, SubCommands},
    commands::{
        none_info::{proc_none_info, NoneInfoArgs},
        remotes::ls_remotes,
    },
    utils::cmd::loop_dirs,
};

fn main() -> Result<()> {
    let args = Args::parse();

    let subcommand = match args.subcommand {
        None => {
            let _ = loop_dirs(
                args.path,
                args.repository_only,
                NoneInfoArgs {},
                proc_none_info,
            )?;

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
