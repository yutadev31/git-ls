use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use git2::Repository;

use git_ls::utils::{get_dir_items, get_git_url, home_dir_mark, print_ls_item};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("."))]
    path: String,

    #[arg(short('r'), long, default_value_t = false)]
    repository_only: bool,

    #[arg(short('d'), long, default_value_t = String::new())]
    domain: String,

    #[arg(short('u'), long, default_value_t = String::new())]
    user: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let remote_only = !args.domain.is_empty() || !args.user.is_empty();
    let repository_only = args.repository_only || remote_only;
    let path = args.path.as_str();

    let mut paths: Vec<PathBuf> = get_dir_items(path)?;

    paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

    for path in paths {
        let dir_path = path.clone();
        let dir_path = dir_path.to_str().unwrap();
        let mut dir_path = home_dir_mark(dir_path).expect("");

        let repo = match Repository::open(path) {
            Ok(data) => data,
            Err(_) => {
                if repository_only {
                    continue;
                }
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
            let url = get_git_url(url);
            match url {
                Some(url) => {
                    if !args.domain.is_empty() && !args.domain.eq(&url.domain) {
                        continue;
                    }

                    if !args.user.is_empty() && !args.user.eq(&url.user) {
                        continue;
                    }

                    print_ls_item(dir_path.as_str(), true, Some(name.to_string()), Some(url));
                }
                None => {
                    print_ls_item(dir_path.as_str(), false, None, None);
                }
            }

            dir_path = String::new();
        }
    }

    Ok(())
}
