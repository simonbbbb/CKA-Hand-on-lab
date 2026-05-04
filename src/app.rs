use std::time::Instant;

use crate::task::verifier::{TaskVerifier, VerificationResult};
use crate::task::{Domain, Task};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Screen {
    Dashboard,
    DomainList,
    TaskList,
    TaskDetail,
    ExamMode,
    Progress,
    ConfirmSolution,
}

pub struct ExamState {
    pub start_time: Instant,
    pub duration_secs: u64,
    pub current_question: usize,
    pub score: u32,
    pub total_questions: usize,
    pub answers: Vec<bool>,
    pub submitted: bool,
}

impl ExamState {
    pub fn new(total_questions: usize) -> Self {
        Self {
            start_time: Instant::now(),
            duration_secs: 120 * 60, // 120 minutes
            current_question: 0,
            score: 0,
            total_questions,
            answers: vec![false; total_questions],
            submitted: false,
        }
    }

    pub fn remaining_secs(&self) -> u64 {
        let elapsed = self.start_time.elapsed().as_secs();
        self.duration_secs.saturating_sub(elapsed)
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_secs() == 0
    }
}

pub struct App {
    pub current_screen: Screen,
    pub domains: Vec<Domain>,
    pub selected_domain: usize,
    pub selected_item: usize,
    pub scroll_offset: u16,
    pub hints_revealed: usize,
    pub solution_visible: bool,
    pub progress: std::collections::HashMap<String, bool>,
    pub exam_state: Option<ExamState>,
    pub dashboard_cursor: (usize, usize), // (row, col) in the 2x3 grid
    pub confirm_pending: bool,
    pub verifier: TaskVerifier,
    pub verify_result: Option<VerificationResult>,
}

impl App {
    pub fn new(domains: Vec<Domain>, initial_screen: Screen) -> Self {
        Self {
            current_screen: initial_screen,
            domains,
            selected_domain: 0,
            selected_item: 0,
            scroll_offset: 0,
            hints_revealed: 0,
            solution_visible: false,
            progress: std::collections::HashMap::new(),
            exam_state: None,
            dashboard_cursor: (0, 0),
            confirm_pending: false,
            verifier: TaskVerifier::new(),
            verify_result: None,
        }
    }

    pub fn go_back(&mut self) {
        if self.confirm_pending {
            self.confirm_pending = false;
            return;
        }
        match self.current_screen {
            Screen::Dashboard => {}
            Screen::DomainList => {
                self.current_screen = Screen::Dashboard;
            }
            Screen::TaskList => {
                self.current_screen = Screen::Dashboard;
                self.selected_item = 0;
            }
            Screen::TaskDetail => {
                self.current_screen = Screen::TaskList;
                self.selected_item = 0;
                self.hints_revealed = 0;
                self.solution_visible = false;
                self.scroll_offset = 0;
                self.verify_result = None;
            }
            Screen::ExamMode => {
                self.exam_state = None;
                self.current_screen = Screen::Dashboard;
            }
            Screen::Progress => {
                self.current_screen = Screen::Dashboard;
            }
            Screen::ConfirmSolution => {
                self.current_screen = Screen::TaskDetail;
                self.confirm_pending = false;
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.current_screen {
            Screen::Dashboard => {
                if self.dashboard_cursor.0 > 0 {
                    self.dashboard_cursor.0 -= 1;
                }
                self.update_selected_domain_from_cursor();
            }
            Screen::TaskList => {
                if self.selected_item > 0 {
                    self.selected_item -= 1;
                }
            }
            Screen::ExamMode => {
                if let Some(ref mut exam) = self.exam_state {
                    if exam.current_question > 0 {
                        exam.current_question -= 1;
                    }
                }
            }
            Screen::TaskDetail => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn move_down(&mut self) {
        match self.current_screen {
            Screen::Dashboard => {
                let max_row = if self.domains.len() > 4 { 2 } else { 1 };
                if self.dashboard_cursor.0 < max_row {
                    self.dashboard_cursor.0 += 1;
                    let items_in_last_row = if self.domains.len() % 2 == 0 {
                        2
                    } else {
                        1
                    };
                    if self.dashboard_cursor.0 == max_row
                        && self.dashboard_cursor.1 >= items_in_last_row
                    {
                        self.dashboard_cursor.1 = items_in_last_row - 1;
                    }
                    self.update_selected_domain_from_cursor();
                }
            }
            Screen::TaskList => {
                if let Some(domain) = self.domains.get(self.selected_domain) {
                    if self.selected_item < domain.tasks.len().saturating_sub(1) {
                        self.selected_item += 1;
                    }
                }
            }
            Screen::ExamMode => {
                if let Some(ref mut exam) = self.exam_state {
                    if exam.current_question < exam.total_questions.saturating_sub(1) {
                        exam.current_question += 1;
                    }
                }
            }
            Screen::TaskDetail => {
                self.scroll_offset += 1;
            }
            _ => {}
        }
    }

    pub fn move_left(&mut self) {
        if matches!(self.current_screen, Screen::Dashboard) {
            if self.dashboard_cursor.1 > 0 {
                self.dashboard_cursor.1 -= 1;
                self.update_selected_domain_from_cursor();
            }
        }
    }

    pub fn move_right(&mut self) {
        if matches!(self.current_screen, Screen::Dashboard) {
            if self.dashboard_cursor.1 < 1 {
                // Check that there's an item at this position
                let idx = self.dashboard_cursor.0 * 2 + 1;
                if idx < self.domains.len() + 1 {
                    // +1 for exam card
                    self.dashboard_cursor.1 = 1;
                    self.update_selected_domain_from_cursor();
                }
            }
        }
    }

    fn update_selected_domain_from_cursor(&mut self) {
        let idx = self.dashboard_cursor.0 * 2 + self.dashboard_cursor.1;
        // idx 0..4 = domains, idx 5 = exam, idx 6 = progress
        if idx < self.domains.len() {
            self.selected_domain = idx;
        }
    }

    pub fn select(&mut self) {
        match self.current_screen {
            Screen::Dashboard => {
                let idx = self.dashboard_cursor.0 * 2 + self.dashboard_cursor.1;
                if idx < self.domains.len() {
                    self.selected_domain = idx;
                    self.selected_item = 0;
                    self.current_screen = Screen::TaskList;
                } else if idx == self.domains.len() {
                    // Exam card
                    self.start_exam();
                } else {
                    // Progress card
                    self.show_progress();
                }
            }
            Screen::TaskList => {
                if self.get_current_tasks().is_empty() {
                    return;
                }
                self.hints_revealed = 0;
                self.solution_visible = false;
                self.scroll_offset = 0;
                self.current_screen = Screen::TaskDetail;
            }
            _ => {}
        }
    }

    pub fn jump_to_domain(&mut self, idx: usize) {
        if idx < self.domains.len() {
            self.selected_domain = idx;
            self.selected_item = 0;
            self.current_screen = Screen::TaskList;
        }
    }

    pub fn reveal_hint(&mut self) {
        if matches!(self.current_screen, Screen::TaskDetail) {
            self.hints_revealed = self.hints_revealed.saturating_add(1);
        }
    }

    pub fn verify(&mut self) {
        if matches!(self.current_screen, Screen::TaskDetail) {
            if let Some(task) = self.get_current_task() {
                let result = self.verifier.verify(task);
                let passed = result.passed;
                self.verify_result = Some(result);
                if passed {
                    let key = format!("{}-{}", self.selected_domain, self.selected_item);
                    self.progress.insert(key, true);
                }
            }
        }
    }

    pub fn show_solution(&mut self) {
        if matches!(self.current_screen, Screen::TaskDetail) && !self.solution_visible {
            self.confirm_pending = true;
            self.current_screen = Screen::ConfirmSolution;
        }
    }

    pub fn confirm(&mut self, yes: bool) {
        if self.confirm_pending {
            self.confirm_pending = false;
            if yes {
                self.solution_visible = true;
            }
            self.current_screen = Screen::TaskDetail;
        }
    }

    pub fn start_exam(&mut self) {
        let total: usize = self.domains.iter().map(|d| d.tasks.len()).sum();
        self.exam_state = Some(ExamState::new(total));
        self.selected_item = 0;
        self.current_screen = Screen::ExamMode;
    }

    pub fn show_progress(&mut self) {
        self.current_screen = Screen::Progress;
    }

    pub fn tick_exam_timer(&mut self) {
        if let Some(ref mut exam) = self.exam_state {
            if exam.is_expired() && !exam.submitted {
                exam.submitted = true;
            }
        }
    }

    pub fn exam_mark_answer(&mut self) {
        if let Some(ref mut exam) = self.exam_state {
            if !exam.submitted {
                let idx = exam.current_question;
                if idx < exam.answers.len() {
                    exam.answers[idx] = !exam.answers[idx];
                    if exam.answers[idx] {
                        exam.score += 1;
                    } else {
                        exam.score = exam.score.saturating_sub(1);
                    }
                }
            }
        }
    }

    pub fn exam_submit(&mut self) {
        if let Some(ref mut exam) = self.exam_state {
            exam.submitted = true;
        }
    }

    pub fn next_task(&mut self) {
        if matches!(self.current_screen, Screen::TaskList) {
            let max = self.get_current_tasks().len();
            if self.selected_item < max.saturating_sub(1) {
                self.selected_item += 1;
            }
        } else if matches!(self.current_screen, Screen::ExamMode) {
            if let Some(ref mut exam) = self.exam_state {
                if exam.current_question < exam.total_questions.saturating_sub(1) {
                    exam.current_question += 1;
                }
            }
        } else if matches!(self.current_screen, Screen::TaskDetail) {
            let max = self.get_current_tasks().len();
            if self.selected_item < max.saturating_sub(1) {
                self.selected_item += 1;
                self.hints_revealed = 0;
                self.solution_visible = false;
                self.scroll_offset = 0;
                self.verify_result = None;
            }
        }
    }

    pub fn prev_task(&mut self) {
        if matches!(self.current_screen, Screen::ExamMode) {
            if let Some(ref mut exam) = self.exam_state {
                if exam.current_question > 0 {
                    exam.current_question -= 1;
                }
            }
        } else if self.selected_item > 0 {
            self.selected_item -= 1;
            if matches!(self.current_screen, Screen::TaskDetail) {
                self.hints_revealed = 0;
                self.solution_visible = false;
                self.scroll_offset = 0;
                self.verify_result = None;
            }
        }
    }

    pub fn get_current_tasks(&self) -> &[Task] {
        self.domains
            .get(self.selected_domain)
            .map(|d| d.tasks.as_slice())
            .unwrap_or(&[])
    }

    pub fn get_current_task(&self) -> Option<&Task> {
        self.get_current_tasks().get(self.selected_item)
    }

    pub fn get_domain_progress(&self, domain_idx: usize) -> (usize, usize) {
        if let Some(domain) = self.domains.get(domain_idx) {
            let completed = domain
                .tasks
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    let key = format!("{}-{}", domain_idx, i);
                    self.progress.get(&key).copied().unwrap_or(false)
                })
                .count();
            (completed, domain.tasks.len())
        } else {
            (0, 0)
        }
    }

    pub fn get_total_progress(&self) -> (usize, usize) {
        let total: usize = self.domains.iter().map(|d| d.tasks.len()).sum();
        let completed: usize = self.domains
            .iter()
            .enumerate()
            .flat_map(|(di, d)| {
                d.tasks.iter().enumerate().filter_map(move |(ti, _)| {
                    let key = format!("{}-{}", di, ti);
                    if self.progress.get(&key).copied().unwrap_or(false) {
                        Some(1)
                    } else {
                        None
                    }
                })
            })
            .count();
        (completed, total)
    }

    pub fn get_exam_tasks(&self) -> Vec<(usize, &Task)> {
        // Collect all tasks across domains in exam order
        let mut tasks = Vec::new();
        for (di, domain) in self.domains.iter().enumerate() {
            for task in &domain.tasks {
                tasks.push((di, task));
            }
        }
        tasks
    }
}
