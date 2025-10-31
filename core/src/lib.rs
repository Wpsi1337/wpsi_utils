//! Core crate exposing placeholder types for the toolbox workspace.

pub mod config;
pub mod registry;
pub mod runner;

pub use config::Config;
pub use registry::{Module, Registry};
pub use runner::{CommandRunner, NoopRunner};

use thiserror::Error;

/// Common error type for the toolbox scaffolding.
#[derive(Debug, Error)]
pub enum Error {
    /// Raised whenever a feature still needs to be implemented by the repo author.
    #[error("Not implemented yet")] 
    Unimplemented,
}

/// Convenient alias for results returned by the core crate.
pub type Result<T> = std::result::Result<T, Error>;
