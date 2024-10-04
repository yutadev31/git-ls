use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use colored::Colorize;
use git2::Repository;

use crate::utils::{
    fs::{get_dir_items, home_dir_mark},
    git::{get_git_url, GitUrl},
};

fn proc_dir(path: PathBuf, repository_only: bool, remote_only: bool, domain: String, user: String) {
    let dir_path = path.clone();
    let dir_path = dir_path.to_str().unwrap();
    let mut dir_path = home_dir_mark(dir_path).expect("");

    let repo = match Repository::open(path) {
        Ok(data) => data,
        Err(_) => {
            if repository_only {
                return;
            }

            print_ls_item(dir_path.as_str(), false, None, None);
            return;
        }
    };

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
                    return;
                }

                if !user.is_empty() && !user.eq(&url.user) {
                    return;
                }

                print_ls_item(dir_path.as_str(), true, Some(name.to_string()), Some(url));
            }
            None => {
                if remote_only {
                    return;
                }

                print_ls_item(dir_path.as_str(), false, None, None);
            }
        }

        dir_path = String::new();
    }
}

pub fn ls_remotes(path: String, repository_only: bool, domain: String, user: String) -> Result<()> {
    let remote_only = !domain.is_empty() || !user.is_empty();
    let repository_only = repository_only || remote_only;
    let path = path.as_str();

    let mut paths: Vec<PathBuf> = get_dir_items(path)?;

    paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

    for path in paths {
        proc_dir(
            path,
            repository_only,
            remote_only,
            domain.clone(),
            user.clone(),
        );
    }

    Ok(())
}

fn print_ls_item(path: &str, is_repo: bool, name: Option<String>, url: Option<GitUrl>) {
    if !is_repo {
        println!("{}", path.white());
        return;
    }

    match url {
        None => {
            println!("{}", path.red());
        }
        Some(url) => {
            let name = match name {
                None => String::new(),
                Some(s) => format!("{:>7}{}", s.red(), ":".white()),
            };

            let url = format!(
                "{:>8}{}/{}/{}",
                name.red(),
                url.domain.blue(),
                url.user.yellow(),
                url.repo.green()
            );
            println!("{:<30}{}", path.red(), url)
        }
    }
}
