use super::Task;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerificationResult {
    pub passed: bool,
    pub score: u8,
    pub feedback: String,
}

pub struct TaskVerifier;

impl TaskVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify(&self, task: &Task) -> VerificationResult {
        match &task.verify_command {
            Some(cmd) => self.run_command(cmd, task.verify_expected.as_deref()),
            None => VerificationResult {
                passed: false,
                score: 0,
                feedback: "No automated verification for this task. Mark complete manually with 'v'.".to_string(),
            },
        }
    }

    fn run_command(&self, cmd: &str, expected: Option<&str>) -> VerificationResult {
        let output = match std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                return VerificationResult {
                    passed: false,
                    score: 0,
                    feedback: format!("Failed to run command: {}", e),
                };
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return VerificationResult {
                passed: false,
                score: 0,
                feedback: format!("Command failed (exit code: {:?}):\n{}{}",
                    output.status.code(),
                    if stdout.is_empty() { String::new() } else { format!("stdout: {}\n", stdout) },
                    if stderr.is_empty() { String::new() } else { format!("stderr: {}", stderr) }),
            };
        }

        match expected {
            Some(exp) => {
                if stdout.trim().contains(exp.trim()) {
                    VerificationResult {
                        passed: true,
                        score: 100,
                        feedback: format!("PASS\n{}", stdout.trim()),
                    }
                } else {
                    VerificationResult {
                        passed: false,
                        score: 0,
                        feedback: format!("FAIL\nExpected to contain: {}\nGot: {}", exp.trim(), stdout.trim()),
                    }
                }
            }
            None => VerificationResult {
                passed: true,
                score: 100,
                feedback: format!("PASS\n{}", stdout.trim()),
            },
        }
    }
}
