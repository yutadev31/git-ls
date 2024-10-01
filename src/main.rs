mod utils;
use std::{collections::HashMap, env::args, path::PathBuf};

use colored::Colorize;
use git2::Repository;

use crate::utils::{get_dir_items, get_git_url, home_dir_mark, GitUrl};

fn print_ls_item(path: &str, is_repo: bool, name: Option<String>, url: Option<GitUrl>) {
    if !is_repo {
        println!("{}", path.green());
        return;
    }

    match url {
        None => {
            println!("{}", path.red());
        }
        Some(url) => {
            let url = format!(
                "{}/{}/{}",
                url.domain.blue(),
                url.user.yellow(),
                url.repo.green()
            );
            println!("{:<30}{}", path.red(), url)
        }
    }
}

fn main() {
    let args = args();
    let args: Vec<String> = args.collect();

    let path = if args.len() < 3 {
        "./"
    } else {
        args[2].as_str()
    };

    let mut paths: Vec<PathBuf> = get_dir_items(path);
    paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

    for path in paths {
        let dir_path = path.clone();
        let dir_path = dir_path.to_str().unwrap();
        let mut dir_path = home_dir_mark(dir_path);

        let repo = match Repository::open(path) {
            Ok(data) => data,
            Err(_) => {
                print_ls_item(dir_path.as_str(), false, None, None);
                continue;
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
            let name = if remotes.len() == 1 {
                None
            } else {
                Some(name.to_string())
            };

            match get_git_url(url) {
                Some(url) => {
                    print_ls_item(dir_path.as_str(), true, name, Some(url));
                }
                None => {
                    print_ls_item(dir_path.as_str(), false, None, None);
                }
            }

            dir_path = String::new();
        }
    }
}
