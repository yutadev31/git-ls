use clap::{arg, command, Parser, Subcommand};

use crate::commands::remotes::RemotesCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<SubCommands>,

    /// Parent directory of the repositories
    #[arg(global = true, default_value_t = String::from("."))]
    pub path: String,

    /// Shows repository only
    #[arg(short('r'), long, global = true, default_value_t = false)]
    pub repository_only: bool,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Remotes(RemotesCommand),
}
