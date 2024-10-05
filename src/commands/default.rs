use anyhow::Result;
use git2::Repository;

use crate::utils::{
    cmd::{loop_dirs, CommandArgs},
    output::print_item,
};

#[derive(Clone)]
pub struct DefaultArgs {}

impl CommandArgs for DefaultArgs {}

pub fn git_ls(path: String, repository_only: bool) -> Result<()> {
    let _ = loop_dirs(path, repository_only, DefaultArgs {}, proc_none_info)?;
    Ok(())
}

fn proc_none_info(path: &str, _: Repository, _: DefaultArgs) -> Result<()> {
    print_item(path, true);
    Ok(())
}
