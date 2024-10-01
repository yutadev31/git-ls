mod utils;
use std::{env::args, path::PathBuf};

use colored::Colorize;
use git2::Repository;

use crate::utils::{get_dir_items, get_git_url, home_dir_mark, GitUrl};

fn print_ls_item(path: &str, is_repo: bool, url: Option<GitUrl>) {
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
        let dir_path = home_dir_mark(dir_path);

        let repo = match Repository::open(path) {
            Ok(data) => data,
            Err(_) => {
                print_ls_item(dir_path.as_str(), false, None);
                continue;
            }
        };

        let remote = match repo.find_remote("origin") {
            Ok(data) => data,
            Err(_) => {
                print_ls_item(dir_path.as_str(), true, None);
                continue;
            }
        };

        let url = remote.url().unwrap();

        match get_git_url(url) {
            Some(url) => {
                print_ls_item(dir_path.as_str(), true, Some(url));
            }
            None => {
                print_ls_item(dir_path.as_str(), false, None);
            }
        }
    }
}
