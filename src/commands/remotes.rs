use clap::Args;
use std::collections::HashMap;

use anyhow::Result;
use colored::Colorize;
use git2::Repository;

use crate::utils::{
    cmd::Command,
    git::{get_git_url, GitUrl},
    output::Output,
};

#[derive(Clone, Debug, Args)]
pub struct RemotesCommand {
    #[arg(short('d'), long, default_value_t = String::new())]
    pub domain: String,

    #[arg(short('u'), long, default_value_t = String::new())]
    pub user: String,
}

impl RemotesCommand {
    fn print_item_with_remote_url(&self, path: &str, name: String, url: GitUrl) {
        let name = format!("{:>7}{}", name.red(), ":".white());
        let url = format!(
            "{:>8}{}/{}/{}",
            name.red(),
            url.domain.blue(),
            url.user.yellow(),
            url.repo.green()
        );

        self.clone().print_item_with_info(path, url);
    }
}

impl Output for RemotesCommand {}

impl Command for RemotesCommand {
    fn proc(self, path: &str, repo: Repository) -> Result<()> {
        let domain = self.domain.clone();
        let user = self.user.clone();
        let remote_only = !domain.is_empty() || !user.is_empty();
        let mut path = path;

        let remote_names = repo.remotes().unwrap();
        let remote_names: Vec<&str> = remote_names.iter().map(|s| s.unwrap()).collect();
        let mut remotes: HashMap<String, String> = HashMap::new();
        for name in remote_names {
            let url = repo.find_remote(name).unwrap().url().unwrap().to_string();
            remotes.insert(name.to_string(), url);
        }

        for (name, url) in &remotes {
            let url = get_git_url(url);

            match url {
                None => {
                    if remote_only {
                        continue;
                    }

                    self.clone().print_repo(path);
                }
                Some(url) => {
                    if !domain.is_empty() && !domain.eq(&url.domain) {
                        continue;
                    }

                    if !user.is_empty() && !user.eq(&url.user) {
                        continue;
                    }

                    self.print_item_with_remote_url(path, name.to_string(), url);
                }
            }

            path = "";
        }

        Ok(())
    }
}
