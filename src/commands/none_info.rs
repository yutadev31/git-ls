use git2::Repository;

use crate::utils::{cmd::CommandArgs, output::print_item};

#[derive(Clone)]
pub struct NoneInfoArgs {}

impl CommandArgs for NoneInfoArgs {}

pub fn proc_none_info(path: &str, _: Repository, _: NoneInfoArgs) {
    print_item(path, true);
}
