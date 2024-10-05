use std::collections::HashMap;

use anyhow::{Error, Result};
use colored::Colorize;
use git2::Repository;

use crate::utils::{
    cmd::Command,
    git::{get_git_url, GitUrl},
    output::{print_item, print_item_with_info},
};

#[derive(Clone)]
pub struct RemotesCommand {
    pub domain: String,
    pub user: String,
}

impl Command for RemotesCommand {
    fn proc(self, path: &str, repo: Repository) -> Result<()> {
        let domain = self.domain;
        let user = self.user;
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
                Some(url) => {
                    if !domain.is_empty() && !domain.eq(&url.domain) {
                        return Err(Error::msg("Domain is not match"));
                    }

                    if !user.is_empty() && !user.eq(&url.user) {
                        return Err(Error::msg("User is not match"));
                    }

                    print_remote_item(path, name.to_string(), url);
                }
                None => {
                    if remote_only {
                        return Err(Error::msg("Remotes of this repository is not found"));
                    }

                    print_item(path, false);
                }
            }

            path = "";
        }

        Ok(())
    }
}

fn print_remote_item(path: &str, name: String, url: GitUrl) {
    let name = format!("{:>7}{}", name.red(), ":".white());
    let url = format!(
        "{:>8}{}/{}/{}",
        name.red(),
        url.domain.blue(),
        url.user.yellow(),
        url.repo.green()
    );

    print_item_with_info(path, true, url);
}
