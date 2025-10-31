mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    env, io,
    path::{Path, PathBuf},
    time::Duration,
};
use toolbox_core::registry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let module_labels = load_module_labels();
    let result = run(&mut terminal, &module_labels);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Toolbox TUI error: {err}");
    }

    Ok(())
}

fn load_module_labels() -> Vec<String> {
    let modules_dir = env::var("WPSI_UTILS_MODULE_DIR").unwrap_or_else(|_| "modules".to_string());
    let modules_path = PathBuf::from(&modules_dir);
    match registry::discover_modules(&modules_path) {
        Ok(modules) if !modules.is_empty() => modules
            .into_iter()
            .map(|module| {
                let relative = module.root.strip_prefix(&modules_path).unwrap_or(&module.root);
                format!("{} ({}) – {}", module.name, module.category, display_path(relative))
            })
            .collect(),
        Ok(_) => default_modules(),
        Err(err) => {
            eprintln!("Failed to discover modules in {}: {err}", modules_path.display());
            default_modules()
        }
    }
}

fn default_modules() -> Vec<String> {
    vec!["Example Module A".into(), "Example Module B".into(), "Example Module C".into()]
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}

fn run<B: ratatui::prelude::Backend>(
    terminal: &mut Terminal<B>,
    modules: &[String],
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, modules))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
