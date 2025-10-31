mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let modules = ["Example Module A", "Example Module B", "Example Module C"];
    let result = run(&mut terminal, &modules);

    disable_raw_mode()?;
    let backend = terminal.into_inner();
    let mut stdout = backend.into_inner();
    execute!(stdout, LeaveAlternateScreen)?;

    if let Err(err) = result {
        eprintln!("TODO: handle TUI errors: {err}");
    }

    Ok(())
}

fn run<B: ratatui::prelude::Backend>(terminal: &mut Terminal<B>, modules: &[&str]) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, modules))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {
                        // TODO: hook up navigation once real module data exists.
                    }
                }
            }
        }
    }

    Ok(())
}
