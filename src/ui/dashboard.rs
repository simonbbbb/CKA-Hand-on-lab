use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, BorderType, Borders, Gauge, Paragraph,
    },
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 16, 28)));
    f.render_widget(bg, size);

    // Main layout: title | content | footer
    let chunks = Layout::vertical([
        Constraint::Length(9),  // ASCII art + title
        Constraint::Min(10),    // Domain cards
        Constraint::Length(5),  // Progress + keys
    ])
    .split(size);

    draw_header(f, chunks[0]);
    draw_domain_cards(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  _  __     _   _       ____ _               _        ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " | |/ /    | | | |     / ___| |__   ___  ___| | _____",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " | ' /_____| |_| |____| |   | '_ \\ / _ \\/ __| |/ / __|",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " | . \\_____|  _|  ____| |___| | | |  __/\\__ \\   <\\__ \\",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " |_|\\_\\    |_| |_|     \\____|_| |_|\\___||___/_|\\_\\___/",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  CKA ",
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Exam Prep Lab ",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "v2.0",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(logo).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_domain_cards(f: &mut Frame, area: Rect, app: &App) {
    // Split into a 3-row x 2-col grid with spacing
    let rows = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(7),
    ])
    .split(area);

    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

    // Render domain cards (5 domains + 1 exam + 1 progress)
    let items: Vec<(&str, u8, usize, usize)> = app
        .domains
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let (completed, total) = app.get_domain_progress(i);
            (d.name.as_str(), d.weight, completed, total)
        })
        .collect();

    for row_idx in 0..3 {
        let row_area = rows[row_idx];
        let col_areas = cols.split(row_area);

        for col_idx in 0..2 {
            let item_idx = row_idx * 2 + col_idx;
            let area = col_areas[col_idx].inner(Margin::new(1, 0));

            if item_idx < items.len() {
                let (name, weight, completed, total) = &items[item_idx];
                let is_selected = app.dashboard_cursor == (row_idx, col_idx);

                draw_domain_card(f, area, name, *weight, *completed, *total, is_selected);
            } else if item_idx == items.len() {
                // Exam card
                let is_selected = app.dashboard_cursor == (row_idx, col_idx);
                draw_special_card(f, area, "Start Exam Simulator", "Press ENTER or e", Color::Yellow, is_selected);
            } else if item_idx == items.len() + 1 {
                // Progress card
                let is_selected = app.dashboard_cursor == (row_idx, col_idx);
                draw_special_card(f, area, "View Progress", "Press ENTER or p", Color::Magenta, is_selected);
            }
        }
    }
}

fn draw_domain_card(
    f: &mut Frame,
    area: Rect,
    name: &str,
    weight: u8,
    completed: usize,
    total: usize,
    is_selected: bool,
) {
    let border_color = if is_selected {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let border_type = if is_selected {
        BorderType::Thick
    } else {
        BorderType::Plain
    };

    let block = Block::default()
        .title(Title::from(Span::styled(
            format!(" {} ", name),
            Style::default()
                .fg(if is_selected { Color::Cyan } else { Color::White })
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(if is_selected {
            Color::Rgb(20, 30, 50)
        } else {
            Color::Rgb(16, 16, 28)
        }));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let pct = if total > 0 {
        (completed * 100) / total
    } else {
        0
    };

    let content = vec![
        Line::from(vec![
            Span::styled(" Weight: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", weight),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  Tasks: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}/{}", completed, total),
                if completed == total && total > 0 {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
        Line::from(""),
    ];

    let para = Paragraph::new(content);
    f.render_widget(para, inner);

    // Progress bar
    let gauge_area = Rect::new(inner.x, inner.y + 2, inner.width, 1);
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(if pct == 100 { Color::Green } else { Color::Cyan })
                .bg(Color::Rgb(30, 30, 50)),
        )
        .percent(pct as u16)
        .label(Span::styled(
            format!("{}%", pct),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, gauge_area);
}

fn draw_special_card(
    f: &mut Frame,
    area: Rect,
    title: &str,
    subtitle: &str,
    color: Color,
    is_selected: bool,
) {
    let border_type = if is_selected {
        BorderType::Thick
    } else {
        BorderType::Plain
    };

    let block = Block::default()
        .title(Title::from(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(if is_selected { color } else { Color::White })
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(if is_selected { color } else { Color::DarkGray }))
        .style(Style::default().bg(if is_selected {
            Color::Rgb(20, 30, 50)
        } else {
            Color::Rgb(16, 16, 28)
        }));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("    {}", subtitle),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let para = Paragraph::new(content);
    f.render_widget(para, inner);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).split(area);

    // Overall progress bar
    let (completed, total) = app.get_total_progress();
    let pct = if total > 0 { (completed * 100) / total } else { 0 };

    let progress_block = Block::default()
        .title(Title::from(Span::styled(
            " Overall Progress ",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let gauge_inner = progress_block.inner(chunks[0]);
    f.render_widget(progress_block, chunks[0]);

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(if pct == 100 { Color::Green } else { Color::Cyan })
                .bg(Color::Rgb(30, 30, 50)),
        )
        .percent(pct as u16)
        .label(Span::styled(
            format!("{}/{} tasks completed ({}%)", completed, total, pct),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, gauge_inner);

    // Key hints
    let keys = Line::from(vec![
        Span::styled(" 1-5", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Domain ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Arrows", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Navigate ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Enter", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Select ", Style::default().fg(Color::DarkGray)),
        Span::styled(" e", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Exam ", Style::default().fg(Color::DarkGray)),
        Span::styled(" p", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Progress ", Style::default().fg(Color::DarkGray)),
        Span::styled(" q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit", Style::default().fg(Color::DarkGray)),
    ]);
    let keys_para = Paragraph::new(keys);
    f.render_widget(keys_para, chunks[1]);
}
