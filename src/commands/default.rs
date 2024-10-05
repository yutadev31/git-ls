use crate::utils::{cmd::Command, output::Output};

#[derive(Clone)]
pub struct DefaultCommand {}

impl Output for DefaultCommand {}

impl Command for DefaultCommand {}
