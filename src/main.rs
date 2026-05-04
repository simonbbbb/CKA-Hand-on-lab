mod app;
mod task;
mod ui;

use std::io;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, Screen};
use task::loader::TaskLoader;

#[derive(Parser, Debug)]
#[command(name = "cka-lab", version = "2.0.0")]
#[command(about = "The definitive hands-on CKA exam prep platform")]
struct Args {
    /// Path to the lab directory containing domain folders
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Start in exam mode
    #[arg(short, long, default_value_t = false)]
    exam: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load tasks
    let loader = TaskLoader::new(&args.path);
    let domains = loader.load_domains()?;

    // Create app
    let initial_screen = if args.exam {
        Screen::ExamMode
    } else {
        Screen::Dashboard
    };
    let mut app = App::new(domains, initial_screen);

    // Run the main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => {
                        if matches!(app.current_screen, Screen::Dashboard) {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('Q') => {
                        return Ok(());
                    }
                    KeyCode::Char('S') => {
                        if matches!(app.current_screen, Screen::ExamMode) {
                            app.exam_submit();
                        }
                    }
                    KeyCode::Esc => {
                        app.go_back();
                    }
                    KeyCode::Up => {
                        app.move_up();
                    }
                    KeyCode::Down => {
                        app.move_down();
                    }
                    KeyCode::Left => {
                        app.move_left();
                    }
                    KeyCode::Right => {
                        app.move_right();
                    }
                    KeyCode::Enter => {
                        app.select();
                    }
                    KeyCode::Char('h') => {
                        app.reveal_hint();
                    }
                    KeyCode::Char('v') => {
                        if matches!(app.current_screen, Screen::ExamMode) {
                            app.exam_mark_answer();
                        } else {
                            app.verify();
                        }
                    }
                    KeyCode::Char('s') => {
                        app.show_solution();
                    }
                    KeyCode::Char('y') => {
                        app.confirm(true);
                    }
                    KeyCode::Char('n') => {
                        app.confirm(false);
                    }
                    KeyCode::Char('e') => {
                        if matches!(app.current_screen, Screen::ExamMode) {
                            // Already in exam, don't restart
                        } else {
                            app.start_exam();
                        }
                    }
                    KeyCode::Char('p') => {
                        app.show_progress();
                    }
                    KeyCode::Char('1') => {
                        app.jump_to_domain(0);
                    }
                    KeyCode::Char('2') => {
                        app.jump_to_domain(1);
                    }
                    KeyCode::Char('3') => {
                        app.jump_to_domain(2);
                    }
                    KeyCode::Char('4') => {
                        app.jump_to_domain(3);
                    }
                    KeyCode::Char('5') => {
                        app.jump_to_domain(4);
                    }
                    KeyCode::Tab => {
                        app.next_task();
                    }
                    KeyCode::BackTab => {
                        app.prev_task();
                    }
                    _ => {}
                }
            }
        }

        // Update exam timer
        if matches!(app.current_screen, Screen::ExamMode) {
            app.tick_exam_timer();
        }
    }
}
