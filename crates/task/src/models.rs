use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use colored::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Review => "review",
            TaskStatus::Done => "done",
            TaskStatus::Blocked => "blocked",
        }
    }

    pub fn display_colored(&self) -> ColoredString {
        match self {
            TaskStatus::Todo => "TODO".bright_black(),
            TaskStatus::InProgress => "IN_PROGRESS".yellow().bold(),
            TaskStatus::Review => "REVIEW".blue().bold(),
            TaskStatus::Done => "DONE".green().bold(),
            TaskStatus::Blocked => "BLOCKED".red().bold(),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "todo" | "to_do" => Ok(TaskStatus::Todo),
            "in_progress" | "inprogress" | "doing" | "start" => Ok(TaskStatus::InProgress),
            "review" => Ok(TaskStatus::Review),
            "done" | "complete" | "completed" => Ok(TaskStatus::Done),
            "blocked" => Ok(TaskStatus::Blocked),
            _ => Err(format!(
                "Unknown status '{}'. Valid values: todo, in_progress, review, done, blocked",
                s
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }

    pub fn display_colored(&self) -> ColoredString {
        match self {
            Priority::Low => "low".bright_black(),
            Priority::Medium => "medium".blue(),
            Priority::High => "high".yellow().bold(),
            Priority::Critical => "CRITICAL".red().bold(),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" | "med" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" | "crit" => Ok(Priority::Critical),
            _ => Err(format!(
                "Unknown priority '{}'. Valid values: low, medium, high, critical",
                s
            )),
        }
    }
}

fn default_priority() -> Priority {
    Priority::Medium
}

fn default_status() -> TaskStatus {
    TaskStatus::Todo
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskFrontmatter {
    pub id: String,
    pub title: String,
    #[serde(default = "default_status")]
    pub status: TaskStatus,
    #[serde(default = "default_priority")]
    pub priority: Priority,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChecklistItem {
    pub index: usize,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(flatten)]
    pub frontmatter: TaskFrontmatter,
    pub filename: String,
    pub path: PathBuf,
    pub content: String,
    pub checklists: Vec<ChecklistItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskSummary {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub labels: Vec<String>,
    pub created_at: Option<String>,
    pub checklist_total: usize,
    pub checklist_done: usize,
    pub file_path: String,
}

impl From<&Task> for TaskSummary {
    fn from(t: &Task) -> Self {
        let total = t.checklists.len();
        let done = t.checklists.iter().filter(|c| c.completed).count();
        Self {
            id: t.frontmatter.id.clone(),
            title: t.frontmatter.title.clone(),
            status: t.frontmatter.status,
            priority: t.frontmatter.priority,
            labels: t.frontmatter.labels.clone(),
            created_at: t.frontmatter.created_at.clone(),
            checklist_total: total,
            checklist_done: done,
            file_path: t.path.to_string_lossy().to_string(),
        }
    }
}
