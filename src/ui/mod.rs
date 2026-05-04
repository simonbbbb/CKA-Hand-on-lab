mod dashboard;
mod exam;
mod progress;
mod task_detail;
mod task_list;

use ratatui::Frame;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        crate::app::Screen::Dashboard => {
            dashboard::draw(f, app);
        }
        crate::app::Screen::TaskList => {
            task_list::draw(f, app);
        }
        crate::app::Screen::TaskDetail => {
            task_detail::draw(f, app);
        }
        crate::app::Screen::ExamMode => {
            exam::draw(f, app);
        }
        crate::app::Screen::Progress => {
            progress::draw(f, app);
        }
        crate::app::Screen::ConfirmSolution => {
            task_detail::draw_confirm(f, app);
        }
        crate::app::Screen::DomainList => {
            // Redirect to dashboard for now
            dashboard::draw(f, app);
        }
    }
}
