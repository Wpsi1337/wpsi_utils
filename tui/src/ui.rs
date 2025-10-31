use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the placeholder TUI frame with static module listings.
pub fn draw(f: &mut Frame, modules: &[&str]) {
    let size = f.size();
    let area = centered_rect(60, 60, size);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from("Modules (placeholder):".bold()));
    for module in modules {
        lines.push(Line::from(format!("  • {module}")));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Press q to exit."));

    let block = Block::default()
        .title("Toolbox (TODO)")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(Text::from(lines))
        .block(block)
        .alignment(ratatui::layout::Alignment::Left);

    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
