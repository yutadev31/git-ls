use std::path::PathBuf;

use anyhow::Result;
use git2::Repository;

use super::{
    fs::{get_dir_items, home_dir_mark},
    git::open_repository,
    output::Output,
};

pub trait Command: Output + Clone {
    fn run(self, path: String, repository_only: bool) -> Result<()> {
        let _ = self.loop_dirs(path, repository_only)?;
        Ok(())
    }

    fn loop_dirs(self, path: String, repository_only: bool) -> Result<()> {
        let mut paths: Vec<PathBuf> = get_dir_items(path.as_str())?;

        paths.sort_by(|a, b| a.to_str().cmp(&b.to_str()));

        let this = self;

        for path in paths {
            let dir_path = path.clone();
            let dir_path = dir_path.to_str().unwrap();
            let dir_path = home_dir_mark(dir_path).expect("");

            let repo = match open_repository(path.to_str().unwrap()).inspect_err(|_| {
                if !repository_only {
                    this.clone().print_dir(dir_path.as_str());
                }
            }) {
                Ok(repo) => repo,
                Err(_) => continue,
            };

            let _ = this.clone().proc(dir_path.as_str(), repo);
        }

        Ok(())
    }

    fn proc(self, path: &str, _: Repository) -> Result<()> {
        self.print_repo(path);
        Ok(())
    }
}
