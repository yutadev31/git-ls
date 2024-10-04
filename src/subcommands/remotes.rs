use std::{collections::HashMap, path::PathBuf};

use anyhow::{Error, Result};
use colored::Colorize;

use crate::utils::{
    fs::{get_dir_items, home_dir_mark},
    git::{get_git_url, open_repository, GitUrl},
    output::{print_item, print_item_with_info},
};

fn proc_dir(
    path: PathBuf,
    repository_only: bool,
    remote_only: bool,
    domain: String,
    user: String,
) -> Result<()> {
    let dir_path = path.clone();
    let dir_path = dir_path.to_str().unwrap();
    let mut dir_path = home_dir_mark(dir_path).expect("");

    let repo = open_repository(path.to_str().unwrap()).inspect_err(|_| {
        if !repository_only {
            print_item(dir_path.as_str(), false);
        }
    })?;

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

                print_remote_item(dir_path.as_str(), name.to_string(), url);
            }
            None => {
                if remote_only {
                    return Err(Error::msg("Remotes of this repository is not found"));
                }

                print_item(dir_path.as_str(), false);
            }
        }

        dir_path = String::new();
    }

    Ok(())
}

pub fn ls_remotes(path: String, repository_only: bool, domain: String, user: String) -> Result<()> {
    let remote_only = !domain.is_empty() || !user.is_empty();
    let repository_only = repository_only || remote_only;
    let path = path.as_str();

    let mut paths: Vec<PathBuf> = get_dir_items(path)?;

    paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

    for path in paths {
        let _ = proc_dir(
            path,
            repository_only,
            remote_only,
            domain.clone(),
            user.clone(),
        );
    }

    Ok(())
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
