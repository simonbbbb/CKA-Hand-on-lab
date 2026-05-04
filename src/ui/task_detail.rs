use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::task::Difficulty;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 16, 28)));
    f.render_widget(bg, size);

    let task = match app.get_current_task() {
        Some(t) => t.clone(),
        None => return,
    };

    let domain = &app.domains[app.selected_domain];
    let key = format!("{}-{}", app.selected_domain, app.selected_item);
    let is_completed = app.progress.get(&key).copied().unwrap_or(false);

    // Dynamic layout: if we have a verify result, add a section for it
    let has_result = app.verify_result.is_some();

    let chunks = if has_result {
        Layout::vertical([
            Constraint::Length(3),   // Header
            Constraint::Min(7),     // Description
            Constraint::Length(5),   // Hints
            Constraint::Length(3),   // Verify result
            Constraint::Length(3),   // Action bar
            Constraint::Length(1),   // Footer
        ])
        .split(size)
    } else {
        Layout::vertical([
            Constraint::Length(3),   // Header
            Constraint::Min(10),     // Description
            Constraint::Length(5),   // Hints
            Constraint::Length(3),   // Action bar
            Constraint::Length(1),   // Footer
        ])
        .split(size)
    };

    draw_header(f, chunks[0], &task.title, &domain.name, &task.difficulty, is_completed);
    draw_description(f, chunks[1], &task.description, app.scroll_offset, app.solution_visible, &task.solution);
    draw_hints(f, chunks[2], &task.hints, app.hints_revealed, app.solution_visible);

    if has_result {
        draw_verify_result(f, chunks[3], app);
        draw_actions(f, chunks[4], app.solution_visible, app.hints_revealed, task.hints.len());
        draw_footer(f, chunks[5]);
    } else {
        draw_actions(f, chunks[3], app.solution_visible, app.hints_revealed, task.hints.len());
        draw_footer(f, chunks[4]);
    }
}

fn draw_header(
    f: &mut Frame,
    area: Rect,
    title: &str,
    domain_name: &str,
    difficulty: &Difficulty,
    completed: bool,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let difficulty_color = match difficulty {
        Difficulty::Easy => Color::Green,
        Difficulty::Medium => Color::Yellow,
        Difficulty::Hard => Color::Red,
    };

    let status = if completed {
        Span::styled(
            " COMPLETED",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            " IN PROGRESS",
            Style::default().fg(Color::Yellow),
        )
    };

    let content = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            domain_name,
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  [{}]", difficulty),
            Style::default().fg(difficulty_color).add_modifier(Modifier::BOLD),
        ),
        status,
    ]);

    let para = Paragraph::new(content);
    f.render_widget(para, inner);
}

fn draw_description(f: &mut Frame, area: Rect, description: &str, _scroll_offset: u16, solution_visible: bool, solution: &str) {
    let block = Block::default()
        .title(" Task Description ")
        .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    // Task description
    for line in description.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("kubectl") {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Green),
            )));
        } else if trimmed.starts_with('-') {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Rgb(200, 200, 200)),
            )));
        } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed.contains('.') {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Rgb(220, 220, 240)),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::White),
            )));
        }
    }

    // If solution is visible, show it in a highlighted section
    if solution_visible {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "--- Solution ---",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        for line in solution.lines() {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Rgb(150, 255, 150)),
            )));
        }
    }

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((_scroll_offset, 0));
    f.render_widget(paragraph, inner);
}

fn draw_hints(f: &mut Frame, area: Rect, hints: &[String], revealed: usize, solution_visible: bool) {
    let border_color = if revealed > 0 || solution_visible {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let title_text = if solution_visible {
        " Hints & Solution ".to_string()
    } else if revealed > 0 {
        format!(" Hints ({}/{}) ", revealed, hints.len())
    } else {
        " Hints (press h to reveal) ".to_string()
    };

    let block = Block::default()
        .title(title_text)
        .title_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    if revealed == 0 && !solution_visible {
        lines.push(Line::from(Span::styled(
            format!(" Press 'h' to reveal hints ({} available)", hints.len()),
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, hint) in hints.iter().enumerate().take(revealed) {
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" Hint {}: ", i + 1),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    hint.clone(),
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]));
        }

        if revealed < hints.len() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!(" ... {} more hints available (press h)", hints.len() - revealed),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(paragraph, inner);
}

fn draw_actions(f: &mut Frame, area: Rect, solution_visible: bool, hints_revealed: usize, total_hints: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let items = vec![
        Span::styled(" h", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!(" Hint ({}/{}) ", hints_revealed, total_hints),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(" v", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled(" Verify ", Style::default().fg(Color::DarkGray)),
        if solution_visible {
            Span::styled(" s", Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(" s", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        },
        if solution_visible {
            Span::styled(" Solution (shown) ", Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(" Show Solution ", Style::default().fg(Color::DarkGray))
        },
        Span::styled(" Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" Next Task ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Back", Style::default().fg(Color::DarkGray)),
    ];

    let content = Line::from(items);
    let para = Paragraph::new(content).alignment(Alignment::Center);
    f.render_widget(para, inner);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let keys = Line::from(vec![
        Span::styled(" Up/Down", Style::default().fg(Color::DarkGray)),
        Span::styled(" Scroll ", Style::default().fg(Color::DarkGray)),
    ]);
    let para = Paragraph::new(keys);
    f.render_widget(para, area);
}

fn draw_verify_result(f: &mut Frame, area: Rect, app: &App) {
    let result = match &app.verify_result {
        Some(r) => r,
        None => return,
    };

    let (fg_color, border_color, label) = if result.passed {
        (Color::Green, Color::Green, " VERIFICATION PASSED ")
    } else {
        (Color::Red, Color::Red, " VERIFICATION FAILED ")
    };

    let block = Block::default()
        .title(label)
        .title_style(Style::default().fg(fg_color).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines: Vec<Line> = result
        .feedback
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                format!(" {}", l),
                Style::default().fg(if result.passed { Color::Green } else { Color::Red }),
            ))
        })
        .collect();

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

/// Draw the confirmation dialog overlay
pub fn draw_confirm(f: &mut Frame, app: &App) {
    // First draw the task detail behind
    draw(f, app);

    // Now draw the confirmation overlay
    let size = f.area();
    let popup_width = 50u16;
    let popup_height = 7u16;

    let popup_area = Rect::new(
        (size.width.saturating_sub(popup_width)) / 2,
        (size.height.saturating_sub(popup_height)) / 2,
        popup_width.min(size.width),
        popup_height.min(size.height),
    );

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Confirm ")
        .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Rgb(30, 16, 16)));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Are you sure you want to reveal the solution?",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  Try solving it yourself first!",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" Yes  ", Style::default().fg(Color::DarkGray)),
            Span::styled(" n", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(" No (go back)", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}
