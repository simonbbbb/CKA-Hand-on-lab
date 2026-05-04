use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, Gauge, Paragraph,
    },
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 16, 28)));
    f.render_widget(bg, size);

    let chunks = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(10),    // Domain progress
        Constraint::Length(3),  // Overall summary
        Constraint::Length(1),  // Footer
    ])
    .split(size);

    // Header
    let header_block = Block::default()
        .title(" Progress Tracker ")
        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let header_inner = header_block.inner(chunks[0]);
    f.render_widget(header_block, chunks[0]);

    let header_content = Line::from(Span::styled(
        "  Track your preparation progress across all CKA exam domains",
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(Paragraph::new(header_content), header_inner);

    // Domain progress cards
    let domain_area = chunks[1];
    let row_height = 4u16;
    let domain_chunks = Layout::vertical(
        app.domains
            .iter()
            .map(|_| Constraint::Length(row_height))
            .chain(std::iter::once(Constraint::Min(0)))
            .collect::<Vec<_>>(),
    )
    .split(domain_area);

    for (i, domain) in app.domains.iter().enumerate() {
        let (completed, total) = app.get_domain_progress(i);
        let pct = if total > 0 { (completed * 100) / total } else { 0 };
        let is_weak = total > 0 && completed < total / 2;

        let border_color = if pct == 100 && total > 0 {
            Color::Green
        } else if is_weak {
            Color::Red
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title(Title::from(Span::styled(
                format!(
                    " {} ({}%) ",
                    domain.name,
                    domain.weight
                ),
                Style::default()
                    .fg(if pct == 100 && total > 0 {
                        Color::Green
                    } else {
                        Color::White
                    })
                    .add_modifier(Modifier::BOLD),
            )))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(Color::Rgb(16, 16, 28)));

        let inner = block.inner(domain_chunks[i]);
        f.render_widget(block, domain_chunks[i]);

        // Progress info line
        let info_line = Line::from(vec![
            Span::styled(
                format!("  {}/{} tasks  ", completed, total),
                Style::default().fg(Color::White),
            ),
            if is_weak {
                Span::styled(
                    "WEAK AREA - needs more practice",
                    Style::default().fg(Color::Red),
                )
            } else if pct == 100 && total > 0 {
                Span::styled("COMPLETE", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            },
        ]);

        let info_para = Paragraph::new(info_line);
        f.render_widget(info_para, Rect::new(inner.x, inner.y, inner.width, 1));

        // Gauge
        let gauge_area = Rect::new(inner.x, inner.y + 1, inner.width, 1);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(if pct == 100 {
                Color::Green
            } else if is_weak {
                Color::Red
            } else {
                Color::Cyan
            }).bg(Color::Rgb(30, 30, 50)))
            .percent(pct as u16)
            .label(Span::styled(
                format!("{}%", pct),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ));
        f.render_widget(gauge, gauge_area);
    }

    // Overall summary
    let (total_completed, total_tasks) = app.get_total_progress();
    let total_pct = if total_tasks > 0 {
        (total_completed * 100) / total_tasks
    } else {
        0
    };

    let summary_block = Block::default()
        .title(" Overall Readiness ")
        .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let summary_inner = summary_block.inner(chunks[2]);
    f.render_widget(summary_block, chunks[2]);

    let readiness_label = if total_pct >= 80 {
        "Exam Ready!"
    } else if total_pct >= 50 {
        "Keep Practicing"
    } else if total_pct > 0 {
        "Early Stage"
    } else {
        "Not Started"
    };

    let summary_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(if total_pct >= 80 {
                    Color::Green
                } else if total_pct >= 50 {
                    Color::Cyan
                } else {
                    Color::Yellow
                })
                .bg(Color::Rgb(30, 30, 50)),
        )
        .percent(total_pct as u16)
        .label(Span::styled(
            format!(
                "{}/{} tasks -- {}",
                total_completed, total_tasks, readiness_label
            ),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(summary_gauge, summary_inner);

    // Footer
    let footer = Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Back to Dashboard", Style::default().fg(Color::DarkGray)),
    ]);
    let footer_para = Paragraph::new(footer);
    f.render_widget(footer_para, chunks[3]);
}
