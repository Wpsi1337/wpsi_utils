//! Configuration models and loaders for the toolbox scaffold.

use crate::{Error, Result};
use std::path::Path;

/// High-level configuration for the toolbox runtime.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Modules to auto-run once the toolbox starts.
    pub auto_execute: Vec<String>,
    /// Whether to skip confirmation prompts.
    pub skip_confirmation: bool,
    /// Whether to bypass size checks.
    pub size_bypass: bool,
}

/// Attempt to load configuration from the provided path.
///
/// Expected TOML keys:
/// - `auto_execute` as an array of module identifiers
/// - `skip_confirmation` as a boolean
/// - `size_bypass` as a boolean
pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let _ = path.as_ref();
    // TODO: parse TOML once the configuration format is finalized.
    Err(Error::Unimplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "placeholder"]
    fn loads_example_config() {
        let path = Path::new("../config/example_config.toml");
        let cfg = load_config(path).expect("TODO: replace with real parsing");
        assert!(cfg.auto_execute.is_empty());
    }
}
