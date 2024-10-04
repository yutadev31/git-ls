use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<SubCommands>,

    #[arg(global = true, default_value_t = String::from("."))]
    pub path: String,

    #[arg(short('r'), long, global = true, default_value_t = false)]
    pub repository_only: bool,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Remotes {
        #[arg(short('d'), long, default_value_t = String::new())]
        domain: String,

        #[arg(short('u'), long, default_value_t = String::new())]
        user: String,
    },
}
