use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use crate::models::{Task, TaskFrontmatter};
use crate::parser::{parse_task_file, serialize_task_file};

pub struct Storage {
    pub context: tracker_core::WorkspaceContext,
    pub tasks_dir: PathBuf,
}

impl Storage {
    pub fn discover(explicit_path: Option<&str>) -> Result<Self, String> {
        let ctx = tracker_core::WorkspaceContext::discover(explicit_path)?;
        let tasks_dir = ctx.tasks_dir.clone();
        Ok(Self {
            context: ctx,
            tasks_dir,
        })
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>, String> {
        let mut tasks = Vec::new();

        let entries = fs::read_dir(&self.tasks_dir)
            .map_err(|e| format!("Error reading directory {:?}: {}", self.tasks_dir, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(task) = parse_task_file(&path, &content) {
                        tasks.push(task);
                    }
                }
            }
        }

        // Sort by ID
        tasks.sort_by(|a, b| a.frontmatter.id.cmp(&b.frontmatter.id));
        Ok(tasks)
    }

    pub fn find_task(&self, id_query: &str) -> Result<Task, String> {
        let tasks = self.list_tasks()?;

        // Exact match
        if let Some(task) = tasks.iter().find(|t| t.frontmatter.id.eq_ignore_ascii_case(id_query)) {
            return Ok(task.clone());
        }

        // Match by numeric suffix (e.g. user typed "1" or "001" for "task-001")
        let clean_query = id_query.trim_start_matches("task-").trim_start_matches('#');
        let matched: Vec<&Task> = tasks
            .iter()
            .filter(|t| {
                let task_num = t.frontmatter.id.trim_start_matches("task-");
                task_num == clean_query || task_num.trim_start_matches('0') == clean_query.trim_start_matches('0')
            })
            .collect();

        if matched.len() == 1 {
            return Ok((*matched[0]).clone());
        } else if matched.len() > 1 {
            return Err(format!("Multiple tasks matched '{}'. Please specify the full ID.", id_query));
        }

        Err(format!("Task with ID '{}' not found in {:?}", id_query, self.tasks_dir))
    }

    pub fn save_task(&self, frontmatter: &TaskFrontmatter, body: &str) -> Result<Task, String> {
        let filename = format!("{}.md", frontmatter.id);
        let target_path = self.tasks_dir.join(&filename);

        let full_text = serialize_task_file(frontmatter, body)?;

        // Atomic write via tempfile in the same directory
        let mut temp_file = NamedTempFile::new_in(&self.tasks_dir)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        temp_file
            .write_all(full_text.as_bytes())
            .map_err(|e| format!("Failed to write to temporary file: {}", e))?;

        temp_file
            .persist(&target_path)
            .map_err(|e| format!("Failed to save task file {:?}: {}", target_path, e))?;

        parse_task_file(&target_path, &full_text)
    }

    pub fn delete_task(&self, id_query: &str) -> Result<Task, String> {
        let task = self.find_task(id_query)?;
        fs::remove_file(&task.path)
            .map_err(|e| format!("Failed to delete task file {:?}: {}", task.path, e))?;
        Ok(task)
    }

    pub fn next_id(&self) -> String {
        let tasks = self.list_tasks().unwrap_or_default();
        let mut max_num = 0;

        for task in &tasks {
            let id = &task.frontmatter.id;
            if let Some(num_str) = id.strip_prefix("task-") {
                if let Ok(num) = num_str.parse::<u32>() {
                    if num > max_num {
                        max_num = num;
                    }
                }
            }
        }

        format!("task-{:03}", max_num + 1)
    }
}
