use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use toolbox_core::registry;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Focus {
    Categories,
    Modules,
    Actions,
}

pub struct App {
    categories: Vec<String>,
    modules_by_category: BTreeMap<String, Vec<registry::Module>>,
    category_index: usize,
    module_index: usize,
    action_index: usize,
    focus: Focus,
    status: String,
    modules_root: PathBuf,
}

impl App {
    pub fn new(modules: Vec<registry::Module>, modules_root: PathBuf) -> Self {
        let mut map: BTreeMap<String, Vec<registry::Module>> = BTreeMap::new();

        if modules.is_empty() {
            let mut default_module = registry::Module::default();
            default_module.id = "example-module".into();
            default_module.name = "Example Module".into();
            default_module.description = "Placeholder module – add your own".into();
            default_module.category = "Examples".into();
            default_module.script_kind = "bash".into();
            default_module.root = modules_root.join("examples");
            default_module.actions = HashMap::from([(
                "run-placeholder".into(),
                "echo 'Replace this with your script'".into(),
            )]);
            map.entry(default_module.category.clone()).or_default().push(default_module);
        } else {
            for module in modules {
                map.entry(module.category.clone()).or_default().push(module);
            }
        }

        for modules in map.values_mut() {
            modules.sort_by(|a, b| a.name.cmp(&b.name));
        }

        let categories = map.keys().cloned().collect::<Vec<_>>();

        let mut app = Self {
            categories,
            modules_by_category: map,
            category_index: 0,
            module_index: 0,
            action_index: 0,
            focus: Focus::Categories,
            status: String::from("Ready. Use Tab to switch panels."),
            modules_root,
        };
        app.ensure_indices();
        app
    }

    pub fn modules_root(&self) -> &Path {
        &self.modules_root
    }

    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    pub fn current_category_name(&self) -> Option<&str> {
        self.categories.get(self.category_index).map(|category| category.as_str())
    }

    pub fn category_index(&self) -> usize {
        self.category_index
    }

    pub fn module_index(&self) -> usize {
        self.module_index
    }

    pub fn action_index(&self) -> usize {
        self.action_index
    }

    pub fn focus(&self) -> Focus {
        self.focus
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn current_modules(&self) -> Vec<&registry::Module> {
        self.categories
            .get(self.category_index)
            .and_then(|category| self.modules_by_category.get(category))
            .map(|modules| modules.iter().collect())
            .unwrap_or_default()
    }

    pub fn current_actions(&self) -> Vec<(String, String)> {
        self.current_modules()
            .get(self.module_index)
            .map(|module| {
                let mut actions: Vec<_> = module
                    .actions
                    .iter()
                    .map(|(name, path)| (name.clone(), path.clone()))
                    .collect();
                actions.sort_by(|a, b| a.0.cmp(&b.0));
                actions
            })
            .unwrap_or_default()
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Focus::Categories => {
                if self.current_modules_len() > 0 {
                    Focus::Modules
                } else if self.current_actions_len() > 0 {
                    Focus::Actions
                } else {
                    Focus::Categories
                }
            }
            Focus::Modules => {
                if self.current_actions_len() > 0 {
                    Focus::Actions
                } else {
                    Focus::Categories
                }
            }
            Focus::Actions => Focus::Categories,
        };
    }

    pub fn focus_prev(&mut self) {
        self.focus = match self.focus {
            Focus::Categories => Focus::Actions,
            Focus::Modules => Focus::Categories,
            Focus::Actions => {
                if self.current_modules_len() > 0 {
                    Focus::Modules
                } else {
                    Focus::Categories
                }
            }
        };
    }

    pub fn move_up(&mut self) {
        match self.focus {
            Focus::Categories => {
                if self.category_index > 0 {
                    self.category_index -= 1;
                    self.module_index = 0;
                    self.action_index = 0;
                }
            }
            Focus::Modules => {
                if self.module_index > 0 {
                    self.module_index -= 1;
                    self.action_index = 0;
                }
            }
            Focus::Actions => {
                if self.action_index > 0 {
                    self.action_index -= 1;
                }
            }
        }
        self.ensure_indices();
    }

    pub fn move_down(&mut self) {
        match self.focus {
            Focus::Categories => {
                if self.category_index + 1 < self.categories.len() {
                    self.category_index += 1;
                    self.module_index = 0;
                    self.action_index = 0;
                }
            }
            Focus::Modules => {
                let len = self.current_modules_len();
                if len > 0 && self.module_index + 1 < len {
                    self.module_index += 1;
                    self.action_index = 0;
                }
            }
            Focus::Actions => {
                let len = self.current_actions_len();
                if len > 0 && self.action_index + 1 < len {
                    self.action_index += 1;
                }
            }
        }
        self.ensure_indices();
    }

    pub fn activate(&mut self) {
        if self.focus != Focus::Actions {
            self.status = String::from("Select an action and press Enter to run it.");
            return;
        }

        if let Some((name, command)) = self.current_actions().get(self.action_index).cloned() {
            self.status = format!("TODO: run `{command}` ({name})");
        } else {
            self.status = String::from("No actions available for this module.");
        }
    }

    fn current_modules_len(&self) -> usize {
        self.categories
            .get(self.category_index)
            .and_then(|category| self.modules_by_category.get(category))
            .map(|modules| modules.len())
            .unwrap_or(0)
    }

    fn current_actions_len(&self) -> usize {
        self.categories
            .get(self.category_index)
            .and_then(|category| self.modules_by_category.get(category))
            .and_then(|modules| modules.get(self.module_index))
            .map(|module| module.actions.len())
            .unwrap_or(0)
    }

    fn ensure_indices(&mut self) {
        if self.categories.is_empty() {
            self.category_index = 0;
            self.module_index = 0;
            self.action_index = 0;
            return;
        }

        if self.category_index >= self.categories.len() {
            self.category_index = self.categories.len() - 1;
        }

        let modules_len = self.current_modules_len();
        if modules_len == 0 {
            self.module_index = 0;
            self.action_index = 0;
        } else if self.module_index >= modules_len {
            self.module_index = modules_len - 1;
        }

        let actions_len = self.current_actions_len();
        if actions_len == 0 {
            self.action_index = 0;
        } else if self.action_index >= actions_len {
            self.action_index = actions_len - 1;
        }
    }
}
