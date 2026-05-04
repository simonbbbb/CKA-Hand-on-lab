use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::task::Difficulty;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 16, 28)));
    f.render_widget(bg, size);

    let domain = match app.domains.get(app.selected_domain) {
        Some(d) => d,
        None => return,
    };

    let chunks = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(5),     // Task list
        Constraint::Length(1),  // Footer
    ])
    .split(size);

    draw_header(f, chunks[0], &domain.name, domain.weight);
    draw_task_list(f, chunks[1], app);
    draw_footer(f, chunks[2]);
}

fn draw_header(f: &mut Frame, area: Rect, name: &str, weight: u8) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let content = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ({}% of exam)", weight),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            "Up/Down: Navigate   Enter: Open   Esc: Back",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let para = Paragraph::new(content);
    f.render_widget(para, inner);
}

fn draw_task_list(f: &mut Frame, area: Rect, app: &App) {
    let domain = match app.domains.get(app.selected_domain) {
        Some(d) => d,
        None => return,
    };

    if domain.tasks.is_empty() {
        let empty_msg = Paragraph::new("\n  No tasks found for this domain.\n  Make sure the README.md exists in the domain directory.")
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        f.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = domain
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_selected = i == app.selected_item;
            let key = format!("{}-{}", app.selected_domain, i);
            let is_completed = app.progress.get(&key).copied().unwrap_or(false);

            let difficulty_color = match task.difficulty {
                Difficulty::Easy => Color::Green,
                Difficulty::Medium => Color::Yellow,
                Difficulty::Hard => Color::Red,
            };

            let status_icon = if is_completed {
                Span::styled(
                    " \u{2713} ",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(" \u{25CB} ", Style::default().fg(Color::DarkGray))
            };

            let id_span = Span::styled(
                format!("Task {:>2} ", task.id.split('-').last().unwrap_or("?")),
                Style::default().fg(if is_selected { Color::Cyan } else { Color::White }),
            );

            let badge = Span::styled(
                format!("[{}] ", task.difficulty),
                Style::default()
                    .fg(difficulty_color)
                    .add_modifier(Modifier::BOLD),
            );

            let title_span = Span::styled(
                &task.title,
                Style::default().fg(if is_selected {
                    Color::White
                } else {
                    Color::Gray
                }),
            );

            let line = if is_selected {
                Line::from(vec![
                    Span::styled(" > ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    status_icon,
                    id_span,
                    badge,
                    title_span,
                ])
            } else {
                Line::from(vec![
                    Span::raw("   "),
                    status_icon,
                    id_span,
                    badge,
                    title_span,
                ])
            };

            let bg_color = if is_selected {
                Color::Rgb(25, 35, 55)
            } else {
                Color::Rgb(16, 16, 28)
            };

            ListItem::new(line).style(Style::default().bg(bg_color))
        })
        .collect();

    let list = List::new(items).style(Style::default().bg(Color::Rgb(16, 16, 28)));

    // We need a mutable ListState for scrolling
    let mut state = ListState::default();
    state.select(Some(app.selected_item));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let keys = Line::from(vec![
        Span::styled(" Up/Down", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Navigate ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Enter", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Open Task ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Next Task ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Back", Style::default().fg(Color::DarkGray)),
    ]);
    let para = Paragraph::new(keys);
    f.render_widget(para, area);
}
