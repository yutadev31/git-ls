use anyhow::{Ok, Result};

use crate::utils::{cmd::Command, output::Output};

#[derive(Clone)]
pub struct DefaultCommand {}

impl DefaultCommand {
    pub fn new_and_run(path: String, repository_only: bool) -> Result<()> {
        let _ = Self {}.run(path, repository_only)?;
        Ok(())
    }
}

impl Output for DefaultCommand {}

impl Command for DefaultCommand {}
