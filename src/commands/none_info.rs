use anyhow::Result;
use git2::Repository;

use crate::utils::{
    cmd::{loop_dirs, CommandArgs},
    output::print_item,
};

#[derive(Clone)]
pub struct NoneInfoArgs {}

impl CommandArgs for NoneInfoArgs {}

fn proc_none_info(path: &str, _: Repository, _: NoneInfoArgs) {
    print_item(path, true);
}

pub fn git_ls(path: String, repository_only: bool) -> Result<()> {
    let _ = loop_dirs(path, repository_only, NoneInfoArgs {}, proc_none_info)?;
    Ok(())
}
