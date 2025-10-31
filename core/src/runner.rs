//! Command runner abstractions.

use crate::{Error, Result};

/// Trait describing how to execute commands for modules.
pub trait CommandRunner {
    fn run(&self, command: &str) -> Result<()>;
}

/// A command runner that simply reminds the user to implement real logic.
pub struct NoopRunner;

impl CommandRunner for NoopRunner {
    fn run(&self, command: &str) -> Result<()> {
        println!("TODO: run command `{}`", command);
        Err(Error::Unimplemented)
    }
}

impl NoopRunner {
    /// Helper constructor for the noop runner.
    pub fn new() -> Self {
        Self
    }
}
