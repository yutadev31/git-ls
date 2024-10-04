use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
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
