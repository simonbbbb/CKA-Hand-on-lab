use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::{Difficulty, Domain, Task};

/// Map from filesystem folder prefix to domain ID string (as used in YAML)
const DOMAIN_MAP: &[(&str, &str, &str, u8)] = &[
    ("01_storage", "storage", "Storage", 10),
    ("02_workloads", "workloads-scheduling", "Workloads & Scheduling", 15),
    ("03_networking", "services-networking", "Services & Networking", 20),
    ("04_troubleshooting", "troubleshooting", "Troubleshooting", 30),
    ("05_cluster_arch", "cluster-architecture", "Cluster Architecture", 25),
];

/// Intermediate struct for deserialising YAML task definitions.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct YamlTask {
    id: String,
    domain: String,
    title: String,
    description: String,
    difficulty: String,
    time_estimate: String,
    weight: u8,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    hints: Vec<String>,
    #[serde(default)]
    exam_tips: Vec<String>,
    #[serde(default)]
    solution_files: Vec<String>,
    setup_script: Option<String>,
    verify_script: Option<String>,
    verify_command: Option<String>,
    verify_expected: Option<String>,
    #[serde(default)]
    prerequisites: Vec<String>,
}

pub struct TaskLoader {
    tasks_path: String,
    solutions_path: String,
}

impl TaskLoader {
    pub fn new(base_path: &str) -> Self {
        // Detect the repo root: the base_path points to the working directory;
        // tasks/ and solutions/ are siblings at the top level.
        Self {
            tasks_path: Path::new(base_path).join("tasks").to_string_lossy().to_string(),
            solutions_path: Path::new(base_path).join("solutions").to_string_lossy().to_string(),
        }
    }

    pub fn load_domains(&self) -> Result<Vec<Domain>> {
        let mut domains = Vec::new();

        for (folder, domain_id, name, weight) in DOMAIN_MAP {
            let domain_path = Path::new(&self.tasks_path).join(folder);
            if !domain_path.exists() {
                eprintln!("Warning: Domain directory not found: {}", domain_path.display());
                domains.push(Domain {
                    id: domain_id.to_string(),
                    name: name.to_string(),
                    description: String::new(),
                    weight: *weight,
                    tasks: Vec::new(),
                });
                continue;
            }

            // Read domain README for description (optional)
            let readme_path = domain_path.join("README.md");
            let description = if readme_path.exists() {
                fs::read_to_string(&readme_path)
                    .context(format!("Failed to read {}", readme_path.display()))?
            } else {
                String::new()
            };

            // Load YAML task files
            let tasks = self.load_yaml_tasks(folder, domain_id)?;

            domains.push(Domain {
                id: domain_id.to_string(),
                name: name.to_string(),
                description,
                weight: *weight,
                tasks,
            });
        }

        Ok(domains)
    }

    fn load_yaml_tasks(&self, folder: &str, domain_id: &str) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        let domain_path = Path::new(&self.tasks_path).join(folder);
        if !domain_path.is_dir() {
            return Ok(tasks);
        }

        // Collect YAML files and sort them so task order is stable
        let mut yaml_files: Vec<_> = fs::read_dir(&domain_path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                matches!(path.extension().and_then(|s| s.to_str()), Some("yaml" | "yml"))
            })
            .collect();
        yaml_files.sort_by_key(|e| e.file_name());

        for entry in &yaml_files {
            let path = entry.path();
            let content = fs::read_to_string(&path)
                .context(format!("Failed to read {}", path.display()))?;

            let yaml_task: YamlTask = serde_yaml::from_str(&content)
                .context(format!("Failed to parse {}", path.display()))?;

            // Resolve solution content from solutions/ directory
            let solution = self.resolve_solution(folder, &yaml_task.solution_files);

            tasks.push(Task {
                id: yaml_task.id,
                domain: domain_id.to_string(),
                title: yaml_task.title,
                description: yaml_task.description,
                difficulty: match yaml_task.difficulty.to_lowercase().as_str() {
                    "easy" => Difficulty::Easy,
                    "hard" => Difficulty::Hard,
                    _ => Difficulty::Medium,
                },
                time_estimate: yaml_task.time_estimate,
                weight: yaml_task.weight,
                tags: yaml_task.tags,
                hints: yaml_task.hints,
                exam_tips: yaml_task.exam_tips,
                solution_files: yaml_task.solution_files,
                setup_script: yaml_task.setup_script,
                verify_script: yaml_task.verify_script,
                verify_command: yaml_task.verify_command,
                verify_expected: yaml_task.verify_expected,
                prerequisites: yaml_task.prerequisites,
                solution,
            });
        }

        Ok(tasks)
    }

    fn resolve_solution(&self, folder: &str, solution_files: &[String]) -> String {
        if solution_files.is_empty() {
            return "No solution file listed.".to_string();
        }
        let mut parts = Vec::new();
        for sol_file in solution_files {
            match self.try_read_solution(sol_file, folder) {
                Some(content) => parts.push(content),
                None => parts.push(format!("# Solution file not found: {}", sol_file)),
            }
        }
        if parts.is_empty() {
            "No solution file found.".to_string()
        } else {
            parts.join("\n---\n")
        }
    }

    fn try_read_solution(&self, sol_file: &str, folder: &str) -> Option<String> {
        // Strip "solutions/" prefix if present
        let stripped = sol_file.strip_prefix("solutions/").unwrap_or(sol_file);

        // Strategy 1: direct path from solutions/ root
        let p1 = Path::new(&self.solutions_path).join(stripped);
        if p1.exists() {
            return fs::read_to_string(&p1).ok();
        }

        // Strategy 2: solutions/<folder>/<filename>
        let fname = Path::new(stripped).file_name()?;
        let p2 = Path::new(&self.solutions_path).join(folder).join(fname);
        if p2.exists() {
            return fs::read_to_string(&p2).ok();
        }

        // Strategy 3: try treating the whole path as under solutions/<folder>/
        let p3 = Path::new(&self.solutions_path).join(folder).join(stripped);
        if p3.exists() {
            return fs::read_to_string(&p3).ok();
        }

        None
    }
}
