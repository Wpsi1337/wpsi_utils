mod app_state;
mod ui;

use app_state::App;
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
    let modules_dir = env::var("WPSI_UTILS_MODULE_DIR").unwrap_or_else(|_| "modules".to_string());
    let modules_path = PathBuf::from(&modules_dir);
    let modules = load_modules(&modules_path);
    let mut app = App::new(modules, modules_path);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Toolbox TUI error: {err}");
    }

    Ok(())
}

fn load_modules(path: &Path) -> Vec<registry::Module> {
    match registry::discover_modules(path) {
        Ok(modules) => modules,
        Err(err) => {
            eprintln!("Failed to discover modules in {}: {err}", path.display());
            Vec::new()
        }
    }
}

fn run<B: ratatui::prelude::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab => app.focus_next(),
                    KeyCode::BackTab => app.focus_prev(),
                    KeyCode::Right => app.focus_next(),
                    KeyCode::Left => app.focus_prev(),
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::Enter => app.activate(),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
