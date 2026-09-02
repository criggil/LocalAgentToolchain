use colored::*;
use tabled::{Table, Tabled, settings::{Style, Alignment, Modify, object::Columns}};
use crate::models::{Task, TaskStatus};

#[derive(Tabled)]
struct TaskRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "STATUS")]
    status: String,
    #[tabled(rename = "PRIORITY")]
    priority: String,
    #[tabled(rename = "TITLE")]
    title: String,
    #[tabled(rename = "CHECKLIST")]
    checklist: String,
    #[tabled(rename = "LABELS")]
    labels: String,
}

pub fn print_task_table(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "No tasks found. Use `task add \"Title\"` to create one.".bright_black());
        return;
    }

    let rows: Vec<TaskRow> = tasks
        .iter()
        .map(|t| {
            let total = t.checklists.len();
            let done = t.checklists.iter().filter(|c| c.completed).count();
            let checklist_str = if total > 0 {
                format!("{}/{}", done, total)
            } else {
                "-".to_string()
            };

            let labels_str = if t.frontmatter.labels.is_empty() {
                "-".to_string()
            } else {
                t.frontmatter.labels.join(", ")
            };

            TaskRow {
                id: t.frontmatter.id.bold().to_string(),
                status: format!("{}", t.frontmatter.status.display_colored()),
                priority: format!("{}", t.frontmatter.priority.display_colored()),
                title: t.frontmatter.title.clone(),
                checklist: checklist_str,
                labels: labels_str.cyan().to_string(),
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::modern());
    table.with(Modify::new(Columns::new(3..=3)).with(Alignment::left()));

    println!("{}", table);
    println!("{}", format!("Total tasks: {}", tasks.len()).bright_black());
}

pub fn print_kanban_board(tasks: &[Task]) {
    let todo: Vec<&Task> = tasks.iter().filter(|t| t.frontmatter.status == TaskStatus::Todo).collect();
    let in_prog: Vec<&Task> = tasks.iter().filter(|t| t.frontmatter.status == TaskStatus::InProgress).collect();
    let review: Vec<&Task> = tasks.iter().filter(|t| t.frontmatter.status == TaskStatus::Review).collect();
    let done: Vec<&Task> = tasks.iter().filter(|t| t.frontmatter.status == TaskStatus::Done).collect();

    println!("\n{}", "═══ KANBAN BOARD ═════════════════════════════════════════════════════".bright_blue().bold());

    let render_column = |_title: &str, color_title: ColoredString, col_tasks: &[&Task]| {
        println!("\n{} ({})", color_title, col_tasks.len());
        println!("{}", "─".repeat(50).bright_black());
        if col_tasks.is_empty() {
            println!("  {}", "(empty)".bright_black().italic());
        } else {
            for t in col_tasks {
                let total = t.checklists.len();
                let done = t.checklists.iter().filter(|c| c.completed).count();
                let check_badge = if total > 0 {
                    format!(" [{}/{}]", done, total).bright_green()
                } else {
                    "".clear()
                };

                let labels_badge = if !t.frontmatter.labels.is_empty() {
                    format!(" #{}", t.frontmatter.labels.join(" #")).cyan()
                } else {
                    "".clear()
                };

                println!(
                    "  {} {} {}{}{}",
                    format!("[{}]", t.frontmatter.id).bold(),
                    t.frontmatter.priority.display_colored(),
                    t.frontmatter.title,
                    check_badge,
                    labels_badge
                );
            }
        }
    };

    render_column("TODO", "📋 TO DO".bright_black().bold(), &todo);
    render_column("IN PROGRESS", "⚡ IN PROGRESS".yellow().bold(), &in_prog);
    render_column("REVIEW", "👀 REVIEW".blue().bold(), &review);
    render_column("DONE", "✅ DONE".green().bold(), &done);

    println!("\n{}", "──────────────────────────────────────────────────────────────────────".bright_black());
}

pub fn print_task_details(task: &Task) {
    println!("\n{}", format!("═══ {} ═══", task.frontmatter.id).bold().bright_blue());
    println!("{}: {}", "Title".bold(), task.frontmatter.title);
    println!("{}: {}", "Status".bold(), task.frontmatter.status.display_colored());
    println!("{}: {}", "Priority".bold(), task.frontmatter.priority.display_colored());
    
    if !task.frontmatter.labels.is_empty() {
        println!("{}: {}", "Labels".bold(), task.frontmatter.labels.join(", ").cyan());
    }

    if let Some(created) = &task.frontmatter.created_at {
        println!("{}: {}", "Created".bold(), task.frontmatter.created_at.as_deref().unwrap_or(created).bright_black());
    }

    println!("{}: {}", "File".bold(), task.path.display().to_string().bright_black());

    if !task.checklists.is_empty() {
        println!("\n{}", "── Checklist ──────────────────────────────".bold());
        for item in &task.checklists {
            let checkmark = if item.completed {
                "[x]".green().bold()
            } else {
                "[ ]".bright_black()
            };
            println!("  {} {}. {}", checkmark, item.index, item.text);
        }
    }

    if !task.content.trim().is_empty() {
        println!("\n{}", "── Description & Notes ─────────────────────".bold());
        println!("{}", task.content.trim());
    }
    println!();
}
