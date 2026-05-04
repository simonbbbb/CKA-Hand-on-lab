pub mod loader;
pub mod verifier;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: u8,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub domain: String,
    pub title: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub time_estimate: String,
    pub weight: u8,
    pub tags: Vec<String>,
    pub hints: Vec<String>,
    pub exam_tips: Vec<String>,
    pub solution_files: Vec<String>,
    pub setup_script: Option<String>,
    pub verify_script: Option<String>,
    pub verify_command: Option<String>,
    pub verify_expected: Option<String>,
    pub prerequisites: Vec<String>,
    pub solution: String,
}

impl Default for Domain {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            weight: 0,
            tasks: Vec::new(),
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: String::new(),
            domain: String::new(),
            title: String::new(),
            description: String::new(),
            difficulty: Difficulty::Medium,
            time_estimate: "5min".to_string(),
            weight: 5,
            tags: Vec::new(),
            hints: Vec::new(),
            exam_tips: Vec::new(),
            solution_files: Vec::new(),
            setup_script: None,
            verify_script: None,
            verify_command: None,
            verify_expected: None,
            prerequisites: Vec::new(),
            solution: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "EASY"),
            Difficulty::Medium => write!(f, "MEDIUM"),
            Difficulty::Hard => write!(f, "HARD"),
        }
    }
}
