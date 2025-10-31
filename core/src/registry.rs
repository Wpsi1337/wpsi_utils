//! Module registry utilities.

use crate::Result;
use std::collections::HashMap;
use std::fs;
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
    #[serde(skip)]
    pub root: PathBuf,
    #[serde(default)]
    pub actions: HashMap<String, String>,
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
    let dir = dir.as_ref();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut modules = Vec::new();
    visit_dir(dir, &mut modules)?;
    Ok(modules)
}

fn visit_dir(dir: &Path, modules: &mut Vec<Module>) -> Result<()> {
    let module_file = dir.join("module.toml");
    if module_file.is_file() {
        let mut module: Module = toml::from_str(&fs::read_to_string(&module_file)?)?;
        module.root = dir.to_path_buf();
        modules.push(module);
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dir(&path, modules)?;
        }
    }

    Ok(())
}
