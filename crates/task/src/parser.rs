use std::path::Path;
use crate::models::{ChecklistItem, Priority, Task, TaskFrontmatter, TaskStatus};
use chrono::Local;

pub fn parse_task_file(path: &Path, raw_content: &str) -> Result<Task, String> {
    let filename = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "task.md".to_string());

    if !raw_content.starts_with("---") {
        // Fallback for markdown files without YAML frontmatter
        let title = filename.replace(".md", "");
        let fm = TaskFrontmatter {
            id: title.clone(),
            title,
            status: TaskStatus::Todo,
            priority: Priority::Medium,
            labels: vec![],
            assignee: None,
            created_at: None,
            updated_at: None,
            due_date: None,
        };
        let checklists = extract_checklists(raw_content);
        return Ok(Task {
            frontmatter: fm,
            filename,
            path: path.to_path_buf(),
            content: raw_content.to_string(),
            checklists,
        });
    }

    let parts: Vec<&str> = raw_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(format!("File {} has corrupted frontmatter", filename));
    }

    let yaml_str = parts[1];
    let body = parts[2].trim_start_matches("\r\n").trim_start_matches('\n');

    let frontmatter: TaskFrontmatter = serde_yaml::from_str(yaml_str)
        .map_err(|e| format!("YAML parsing error in {}: {}", filename, e))?;

    let checklists = extract_checklists(body);

    Ok(Task {
        frontmatter,
        filename,
        path: path.to_path_buf(),
        content: body.to_string(),
        checklists,
    })
}

pub fn extract_checklists(content: &str) -> Vec<ChecklistItem> {
    let mut items = Vec::new();
    let mut idx = 1;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [ ]") {
            let text = trimmed[5..].trim().to_string();
            items.push(ChecklistItem {
                index: idx,
                text,
                completed: false,
            });
            idx += 1;
        } else if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            let text = trimmed[5..].trim().to_string();
            items.push(ChecklistItem {
                index: idx,
                text,
                completed: true,
            });
            idx += 1;
        }
    }

    items
}

pub fn toggle_checklist_item(content: &str, target_idx: usize, set_completed: bool) -> Result<String, String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut current_idx = 1;
    let mut found = false;

    for line in lines.iter_mut() {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];

        if trimmed.starts_with("- [ ]") {
            if current_idx == target_idx {
                let rest = &trimmed[5..];
                *line = if set_completed {
                    format!("{}- [x]{}", indent, rest)
                } else {
                    format!("{}- [ ]{}", indent, rest)
                };
                found = true;
                break;
            }
            current_idx += 1;
        } else if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            if current_idx == target_idx {
                let rest = &trimmed[5..];
                *line = if set_completed {
                    format!("{}- [x]{}", indent, rest)
                } else {
                    format!("{}- [ ]{}", indent, rest)
                };
                found = true;
                break;
            }
            current_idx += 1;
        }
    }

    if !found {
        return Err(format!(
            "Checklist item #{} not found (total items: {})",
            target_idx,
            current_idx - 1
        ));
    }

    Ok(lines.join("\n"))
}

pub fn append_log_entry(content: &str, message: &str) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let log_line = format!("- [{}] {}", timestamp, message);

    if content.contains("## Logs") {
        content.replace("## Logs", &format!("## Logs\n{}", log_line))
    } else if content.contains("## Логи и заметки") {
        content.replace("## Логи и заметки", &format!("## Логи и заметки\n{}", log_line))
    } else {
        format!("{}\n\n## Logs\n{}", content.trim_end(), log_line)
    }
}

pub fn serialize_task_file(frontmatter: &TaskFrontmatter, body: &str) -> Result<String, String> {
    let yaml_str = serde_yaml::to_string(frontmatter)
        .map_err(|e| format!("YAML serialization error: {}", e))?;

    Ok(format!("---\n{}---\n\n{}", yaml_str, body.trim()))
}
