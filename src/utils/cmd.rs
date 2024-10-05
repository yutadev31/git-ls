use std::path::PathBuf;

use anyhow::Result;
use git2::Repository;

use super::{
    fs::{get_dir_items, home_dir_mark},
    git::open_repository,
    output::print_item,
};

pub trait Command: Clone {
    fn run(path: String, repository_only: bool, args: Self) -> Result<()> {
        let _ = Self::loop_dirs(path, repository_only, args, Self::proc)?;
        Ok(())
    }

    fn loop_dirs<F: Fn(&str, Repository, Self) -> Result<()>>(
        path: String,
        repository_only: bool,
        args: Self,
        f: F,
    ) -> Result<()> {
        let mut paths: Vec<PathBuf> = get_dir_items(path.as_str())?;

        paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

        for path in paths {
            let dir_path = path.clone();
            let dir_path = dir_path.to_str().unwrap();
            let dir_path = home_dir_mark(dir_path).expect("");

            let repo = match open_repository(path.to_str().unwrap()).inspect_err(|_| {
                if !repository_only {
                    print_item(dir_path.as_str(), false);
                }
            }) {
                Ok(repo) => repo,
                Err(_) => continue,
            };

            let _ = f(dir_path.as_str(), repo, args.clone());
        }

        Ok(())
    }

    fn proc(path: &str, _: Repository, _: Self) -> Result<()> {
        print_item(path, true);
        Ok(())
    }
}
