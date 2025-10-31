use crate::app_state::{App, Focus};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

const ACCENT: Color = Color::Rgb(232, 199, 95);
const ACCENT_DIM: Color = Color::Rgb(180, 150, 72);
const PANEL_BG: Color = Color::Rgb(28, 31, 38);
const PANEL_BG_ALT: Color = Color::Rgb(22, 24, 30);
const TEXT_PRIMARY: Color = Color::Rgb(225, 225, 220);
const TEXT_MUTED: Color = Color::Rgb(150, 153, 160);

pub fn draw(f: &mut Frame, app: &App) {
    f.render_widget(Clear, f.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());

    render_title_bar(f, layout[0]);
    render_header(f, layout[1], app);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(48), Constraint::Length(30)].as_ref())
        .split(layout[2]);

    render_categories(f, body_chunks[0], app);
    render_modules(f, body_chunks[1], app);
    render_actions(f, body_chunks[2], app);

    render_footer(f, layout[3], app);
}

fn render_title_bar(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(PANEL_BG_ALT).fg(TEXT_PRIMARY))
        .title(Span::styled(
            "utilities",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ));

    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled("Linux Toolbox", Style::default().fg(TEXT_PRIMARY)),
        Span::raw("  -  "),
        Span::styled("powered by wpsi_utils", Style::default().fg(TEXT_MUTED)),
    ]))
    .alignment(Alignment::Center)
    .block(block);

    f.render_widget(paragraph, area);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2)].as_ref())
        .split(area);

    let banner = Paragraph::new(Line::from(vec![Span::styled(
        "* Applications Setup * Gaming * Security * System Setup * Utilities *",
        Style::default().fg(ACCENT_DIM).add_modifier(Modifier::ITALIC | Modifier::DIM),
    )]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT_DIM))
            .style(Style::default().bg(PANEL_BG_ALT).fg(TEXT_PRIMARY)),
    );

    let lower_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(header_chunks[1]);

    let search = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Search", Style::default().fg(ACCENT)),
            Span::raw("  "),
            Span::styled("Press / to search", Style::default().fg(TEXT_MUTED)),
        ]),
        Line::from(vec![Span::styled(app.status(), Style::default().fg(TEXT_PRIMARY))]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT))
            .style(Style::default().bg(PANEL_BG).fg(TEXT_PRIMARY)),
    );

    let version = Paragraph::new(vec![
        Line::from(Span::styled(
            "Linux Toolbox - 0.1.0-preview",
            Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Stay sharp. Scripts run at your own risk.",
            Style::default().fg(TEXT_MUTED),
        )),
    ])
    .alignment(Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT_DIM))
            .style(Style::default().bg(PANEL_BG).fg(TEXT_PRIMARY)),
    );

    f.render_widget(banner, header_chunks[0]);
    f.render_widget(search, lower_chunks[0]);
    f.render_widget(version, lower_chunks[1]);
}

fn render_categories(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .categories()
        .iter()
        .enumerate()
        .map(|(index, category)| {
            let selected = index == app.category_index();
            let focused = selected && app.focus() == Focus::Categories;

            let (prefix, style) = if focused {
                ("[>]", Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD))
            } else if selected {
                ("[*]", Style::default().fg(ACCENT).bg(PANEL_BG_ALT).add_modifier(Modifier::BOLD))
            } else {
                ("[ ]", Style::default().fg(TEXT_MUTED))
            };

            let line = Line::from(vec![
                Span::styled(format!("{} ", prefix), style),
                Span::styled(category.clone(), style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .title(Span::styled(
            "Applications Setup",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ))
        .borders(Borders::ALL)
        .border_style(border_style(app.focus() == Focus::Categories))
        .style(Style::default().bg(PANEL_BG).fg(TEXT_PRIMARY));

    let list = List::new(if items.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "(no categories)",
            Style::default().fg(TEXT_MUTED),
        )))]
    } else {
        items
    })
    .block(block);

    f.render_widget(list, area);
}

fn render_modules(f: &mut Frame, area: Rect, app: &App) {
    let modules = app.current_modules();
    let items: Vec<ListItem> = if modules.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "(no modules discovered)",
            Style::default().fg(TEXT_MUTED),
        )))]
    } else {
        modules
            .into_iter()
            .enumerate()
            .map(|(index, module)| {
                let selected = index == app.module_index();
                let focused = selected && app.focus() == Focus::Modules;

                let name_style = if focused {
                    Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)
                } else if selected {
                    Style::default().fg(ACCENT).bg(PANEL_BG_ALT).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(TEXT_PRIMARY)
                };

                let desc_style = if focused {
                    Style::default().fg(Color::Black).bg(ACCENT)
                } else if selected {
                    Style::default().fg(ACCENT_DIM)
                } else {
                    Style::default().fg(TEXT_MUTED)
                };

                let mut lines = vec![Line::from(vec![
                    Span::styled(format!("{} ", if selected { ">" } else { " " }), name_style),
                    Span::styled(module.name.clone(), name_style),
                ])];

                if !module.description.is_empty() {
                    lines.push(Line::from(Span::styled(
                        format!("    {}", module.description),
                        desc_style,
                    )));
                }

                if let Ok(relative) = module.root.strip_prefix(app.modules_root()) {
                    lines.push(Line::from(Span::styled(
                        format!("    {}", relative.display()),
                        desc_style,
                    )));
                }

                ListItem::new(lines)
            })
            .collect()
    };

    let title = app.current_category_name().unwrap_or("Modules").to_string();

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ))
        .borders(Borders::ALL)
        .border_style(border_style(app.focus() == Focus::Modules))
        .style(Style::default().bg(PANEL_BG).fg(TEXT_PRIMARY));

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_actions(f: &mut Frame, area: Rect, app: &App) {
    let actions = app.current_actions();
    let items: Vec<ListItem> = if actions.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "(no actions)",
            Style::default().fg(TEXT_MUTED),
        )))]
    } else {
        actions
            .into_iter()
            .enumerate()
            .map(|(index, (name, command))| {
                let selected = index == app.action_index();
                let focused = selected && app.focus() == Focus::Actions;

                let name_style = if focused {
                    Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)
                } else if selected {
                    Style::default().fg(ACCENT).bg(PANEL_BG_ALT).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(TEXT_PRIMARY)
                };

                let tag_style = if focused {
                    Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)
                } else if selected {
                    Style::default().fg(ACCENT).bg(PANEL_BG_ALT).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(ACCENT_DIM).add_modifier(Modifier::BOLD)
                };

                let command_style = if focused {
                    Style::default().fg(Color::Black).bg(ACCENT)
                } else {
                    Style::default().fg(TEXT_MUTED)
                };

                let tag = short_tag(&name);

                let line = Line::from(vec![
                    Span::styled(format!("{:<3} ", tag), tag_style),
                    Span::styled(name, name_style),
                    Span::raw("  "),
                    Span::styled(command, command_style),
                ]);

                ListItem::new(line)
            })
            .collect()
    };

    let block = Block::default()
        .title(Span::styled(
            "Important Actions",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ))
        .borders(Borders::ALL)
        .border_style(border_style(app.focus() == Focus::Actions))
        .style(Style::default().bg(PANEL_BG).fg(TEXT_PRIMARY));

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .title(Span::styled("Tab list", Style::default().fg(ACCENT)))
        .style(Style::default().bg(PANEL_BG_ALT).fg(TEXT_PRIMARY));

    let key_style = Style::default().fg(ACCENT).add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::styled("[g]", key_style),
            Span::raw(" Show tabs    "),
            Span::styled("[Ctrl+C]", key_style),
            Span::raw(" Exit toolbox    "),
            Span::styled("[Enter]", key_style),
            Span::raw(" Run selected    "),
            Span::styled("[k]", key_style),
            Span::raw(" Move up"),
        ]),
        Line::from(vec![
            Span::styled("[j]", key_style),
            Span::raw(" Move down    "),
            Span::styled("[Tab]", key_style),
            Span::raw(" Next panel    "),
            Span::styled("[Shift+Tab]", key_style),
            Span::raw(" Previous panel    "),
            Span::styled("[q]", key_style),
            Span::raw(" Quit"),
        ]),
        Line::from(vec![
            Span::styled("Status:", Style::default().fg(TEXT_MUTED)),
            Span::raw(" "),
            Span::styled(app.status(), Style::default().fg(TEXT_PRIMARY)),
        ]),
    ];

    let footer = Paragraph::new(lines).wrap(Wrap { trim: true }).block(footer_block);

    f.render_widget(footer, area);
}

fn border_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(ACCENT)
    } else {
        Style::default().fg(ACCENT_DIM)
    }
}

fn short_tag(name: &str) -> String {
    let mut tag = String::new();
    for part in name.split_whitespace() {
        if let Some(ch) = part.chars().next() {
            tag.push(ch.to_ascii_uppercase());
        }
        if tag.len() >= 3 {
            break;
        }
    }

    while tag.len() < 3 {
        tag.push(' ');
    }

    tag
}
