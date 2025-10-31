//! Module registry placeholders.

use crate::{Error, Result};
use std::path::{Path, PathBuf};

/// Metadata describing a module that will eventually ship with the toolbox.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub script_kind: String,
    pub enabled: bool,
}

/// Registry handle that knows where module metadata lives on disk.
#[derive(Debug, Clone)]
pub struct Registry {
    pub modules_dir: PathBuf,
}

impl Registry {
    /// Create a new registry pointing at the provided modules directory.
    pub fn new(modules_dir: PathBuf) -> Self {
        Self { modules_dir }
    }

    /// Enumerate modules using the placeholder discovery routine.
    pub fn modules(&self) -> Result<Vec<Module>> {
        discover_modules(&self.modules_dir)
    }
}

/// Look for modules beneath the path.
pub fn discover_modules(dir: impl AsRef<Path>) -> Result<Vec<Module>> {
    let _ = dir.as_ref();
    // TODO: walk the filesystem and parse module.toml files.
    Ok(vec![])
}
