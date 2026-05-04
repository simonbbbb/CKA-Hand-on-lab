use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, Gauge, Paragraph, Wrap,
    },
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 16, 28)));
    f.render_widget(bg, size);

    let exam = match &app.exam_state {
        Some(e) => e,
        None => return,
    };

    if exam.submitted {
        draw_results(f, app, exam);
        return;
    }

    let remaining = exam.remaining_secs();
    let minutes = remaining / 60;
    let seconds = remaining % 60;
    let is_low = remaining < 600;

    let timer_color = if is_low { Color::Red } else { Color::Cyan };
    let elapsed_pct = ((exam.duration_secs - remaining) * 100 / exam.duration_secs) as u16;

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Min(10),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .split(size);

    // Timer header
    let timer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(timer_color))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let timer_inner = timer_block.inner(chunks[0]);
    f.render_widget(timer_block, chunks[0]);

    let timer_label = format!(
        " EXAM MODE  |  {}:{:02} remaining  |  Question {}/{} ",
        minutes,
        seconds,
        exam.current_question + 1,
        exam.total_questions
    );

    let timer_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(timer_color)
                .bg(Color::Rgb(30, 30, 50)),
        )
        .percent(elapsed_pct)
        .label(Span::styled(
            timer_label,
            Style::default()
                .fg(if is_low { Color::White } else { timer_color })
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(timer_gauge, timer_inner);

    // Score bar
    let marked = exam.answers.get(exam.current_question).copied().unwrap_or(false);
    let status_text = if marked { "[MARKED]" } else { "[unmarked]" };
    let status_color = if marked { Color::Green } else { Color::DarkGray };

    let score_content = Line::from(vec![
        Span::styled("  Score: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", exam.score),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" / {}", exam.total_questions),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled("   |   ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("Answered: {}", exam.answers.iter().filter(|a| **a).count()),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled("   |   ", Style::default().fg(Color::DarkGray)),
        Span::styled(status_text, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]);
    let score_para = Paragraph::new(score_content);
    f.render_widget(score_para, chunks[1]);

    // Current question
    let exam_tasks = app.get_exam_tasks();
    if let Some((domain_idx, task)) = exam_tasks.get(exam.current_question) {
        let domain_name = app
            .domains
            .get(*domain_idx)
            .map(|d| d.name.as_str())
            .unwrap_or("Unknown");

        let question_block = Block::default()
            .title(Title::from(Span::styled(
                format!(" {} - {} ", domain_name, task.title),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Rgb(16, 16, 28)));

        let question_inner = question_block.inner(chunks[2]);
        f.render_widget(question_block, chunks[2]);

        let desc_lines: Vec<Line> = task
            .description
            .lines()
            .map(|line| {
                Line::from(Span::styled(
                    line.to_string(),
                    Style::default().fg(Color::White),
                ))
            })
            .collect();

        let para = Paragraph::new(desc_lines).wrap(Wrap { trim: false });
        f.render_widget(para, question_inner);
    } else {
        let no_questions = Paragraph::new("\n  No questions available.")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(no_questions, chunks[2]);
    }

    // Navigation bar
    let nav_content = Line::from(vec![
        Span::styled(" Up/Down", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Navigate ", Style::default().fg(Color::DarkGray)),
        Span::styled(" v", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled(" Mark Done ", Style::default().fg(Color::DarkGray)),
        Span::styled(" S", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Submit ", Style::default().fg(Color::DarkGray)),
        Span::styled(" Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit Exam", Style::default().fg(Color::DarkGray)),
    ]);
    let nav_para = Paragraph::new(nav_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .style(Style::default().bg(Color::Rgb(16, 16, 28))),
        );
    f.render_widget(nav_para, chunks[3]);

    // Footer
    let footer = Line::from(Span::styled(
        " CKA Exam Simulator -- 120 minutes, real conditions",
        Style::default().fg(Color::DarkGray),
    ));
    let footer_para = Paragraph::new(footer);
    f.render_widget(footer_para, chunks[4]);
}

fn draw_results(f: &mut Frame, app: &App, exam: &crate::app::ExamState) {
    let size = f.area();

    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(size);

    // Title
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  EXAM COMPLETE",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    ]);
    f.render_widget(title, chunks[0]);

    // Score
    let pct = if exam.total_questions > 0 {
        exam.score * 100 / exam.total_questions as u32
    } else {
        0
    };
    let passed = pct >= 66;
    let score_color = if passed { Color::Green } else { Color::Red };

    let score_block = Block::default()
        .title(Title::from(Span::styled(
            " Your Score ",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(score_color))
        .style(Style::default().bg(Color::Rgb(16, 16, 28)));

    let score_inner = score_block.inner(chunks[1]);
    f.render_widget(score_block, chunks[1]);

    let score_gauge = Gauge::default()
        .gauge_style(Style::default().fg(score_color).bg(Color::Rgb(30, 30, 50)))
        .percent(pct as u16)
        .label(Span::styled(
            format!("{}/{} ({}%) - {}", exam.score, exam.total_questions, pct, if passed { "PASS" } else { "FAIL" }),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(score_gauge, score_inner);

    // Per-domain breakdown
    let mut domain_lines: Vec<Line> = vec![Line::from("")];
    let mut task_idx = 0usize;
    for domain in &app.domains {
        let task_count = domain.tasks.len();
        let domain_marked: usize = (task_idx..task_idx + task_count)
            .filter(|&i| exam.answers.get(i).copied().unwrap_or(false))
            .count();
        let domain_pct = if task_count > 0 {
            domain_marked * 100 / task_count
        } else {
            0
        };
        domain_lines.push(Line::from(vec![
            Span::styled(
                format!("  {:30} ", domain.name),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{}/{} ({}%)", domain_marked, task_count, domain_pct),
                Style::default().fg(if domain_pct == 100 { Color::Green } else { Color::Yellow }),
            ),
        ]));
        task_idx += task_count;
    }
    let breakdown = Paragraph::new(domain_lines);
    f.render_widget(breakdown, chunks[2]);

    // Footer
    let footer = Line::from(vec![
        Span::styled("  Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" Return to Dashboard", Style::default().fg(Color::DarkGray)),
    ]);
    f.render_widget(Paragraph::new(footer), chunks[3]);
}
