use std::process;
use clap::{Parser, Subcommand};
use colored::*;
use chrono::Local;

mod models;
mod parser;
mod storage;
mod formatters;

use models::{Priority, TaskFrontmatter, TaskStatus, TaskSummary};
use storage::Storage;
use formatters::human::*;
use formatters::json::print_json;

#[derive(Parser)]
#[command(
    name = "task",
    about = "⚡ Fast, agent-friendly Markdown CLI task manager",
    version = "0.1.0",
    arg_required_else_help = true
)]
struct Cli {
    #[arg(long, global = true, help = "Machine-readable JSON output (for Claude Code and AI agents)")]
    json: bool,

    #[arg(long, global = true, help = "Path to workspace directory with tasks")]
    workspace: Option<String>,

    #[arg(short, long, global = true, help = "Suppress informational messages")]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "list", aliases = ["ls"], about = "List tasks with optional filters")]
    List {
        #[arg(short, long, help = "Filter by status (todo, in_progress, review, done, blocked)")]
        status: Option<TaskStatus>,

        #[arg(short, long, help = "Filter by priority (low, medium, high, critical)")]
        priority: Option<Priority>,

        #[arg(short, long, help = "Filter by label/tag")]
        label: Option<String>,

        #[arg(long, help = "Display in Kanban board layout")]
        board: bool,
    },

    #[command(name = "add", about = "Create a new task in Markdown")]
    Add {
        #[arg(help = "Task title")]
        title: String,

        #[arg(short, long, default_value = "todo", help = "Initial task status")]
        status: TaskStatus,

        #[arg(short, long, default_value = "medium", help = "Task priority")]
        priority: Priority,

        #[arg(short, long, value_delimiter = ',', help = "Comma-separated labels")]
        labels: Vec<String>,

        #[arg(short, long, help = "Task description in Markdown")]
        desc: Option<String>,

        #[arg(short, long = "check", help = "Add a checklist item (repeatable)")]
        checklists: Vec<String>,
    },

    #[command(name = "show", about = "View task details")]
    Show {
        #[arg(help = "Task ID (e.g. task-001 or 001)")]
        id: String,

        #[arg(long, help = "Output raw markdown file content")]
        raw: bool,
    },

    #[command(name = "move", aliases = ["status"], about = "Move task to another status")]
    Move {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "New status (todo, in_progress, review, done, blocked)")]
        status: TaskStatus,
    },

    #[command(name = "start", about = "Quick shorthand to move task to 'in_progress'")]
    Start {
        #[arg(help = "Task ID")]
        id: String,
    },

    #[command(name = "review", about = "Quick shorthand to move task to 'review'")]
    Review {
        #[arg(help = "Task ID")]
        id: String,
    },

    #[command(name = "done", about = "Quick shorthand to move task to 'done'")]
    Done {
        #[arg(help = "Task ID")]
        id: String,
    },

    #[command(name = "check", about = "Toggle a checklist item")]
    Check {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "Item index (starting from 1)")]
        index: usize,

        #[arg(short, long, help = "Uncheck the item instead of checking")]
        uncheck: bool,
    },

    #[command(name = "log", aliases = ["comment"], about = "Append a timestamped log entry to task history")]
    Log {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "Message to log")]
        message: String,
    },

    #[command(name = "edit", about = "Open task file in text editor ($EDITOR)")]
    Edit {
        #[arg(help = "Task ID")]
        id: String,
    },

    #[command(name = "delete", aliases = ["rm"], about = "Delete task (.md file)")]
    Delete {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(short, long, help = "Confirm deletion without prompt")]
        yes: bool,
    },

    #[command(name = "init", about = "Initialize task tracking in current directory or external vault")]
    Init {
        #[arg(long, help = "Store tasks in external vault (~/.local-tracker/vaults/) without modifying repo")]
        detached: bool,

        #[arg(short, long, help = "Custom project name (defaults to current folder name)")]
        name: Option<String>,
    },

    #[command(name = "projects", about = "List all registered projects from central registry")]
    Projects,

    #[command(name = "info", about = "Show active project context, storage mode, and directories")]
    Info,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { detached, name } => {
            let cwd = match std::env::current_dir() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}: Failed to get current directory: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            match tracker_core::WorkspaceContext::init_project(&cwd, name.as_deref(), *detached) {
                Ok(ctx) => {
                    if cli.json {
                        println!(
                            "{{\"project\": \"{}\", \"mode\": \"{}\", \"tasks_dir\": {:?}}}",
                            ctx.project_name, ctx.mode, ctx.tasks_dir
                        );
                    } else {
                        println!(
                            "{} Initialized project '{}' (mode: {})",
                            "✓".green().bold(),
                            ctx.project_name.bold(),
                            ctx.mode
                        );
                        println!("  Tasks directory: {:?}", ctx.tasks_dir);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error initializing project".red().bold(), e);
                    process::exit(1);
                }
            }
            return;
        }

        Commands::Projects => {
            let projects = tracker_core::Registry::list();
            if cli.json {
                print_json(&projects);
            } else if projects.is_empty() {
                println!("{}", "No registered projects found. Use `task init` to register one.".bright_black());
            } else {
                println!("\n{}", "═══ Registered Projects ══════════════════════════════════════════════".bright_blue().bold());
                for p in projects {
                    println!("  📁 {} ({})", p.name.bold(), p.path.display().to_string().bright_black());
                    println!("     Storage: {:?}", p.storage);
                    if let Some(r) = p.git_remote {
                        println!("     Git: {}", r.cyan());
                    }
                }
                println!();
            }
            return;
        }

        _ => {}
    }

    let storage = match Storage::discover(cli.workspace.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            process::exit(1);
        }
    };

    match cli.command {
        Commands::Init { .. } | Commands::Projects => unreachable!(),
        Commands::Info => {
            let ctx = &storage.context;
            if cli.json {
                println!(
                    "{{\"project\": \"{}\", \"mode\": \"{}\", \"root_path\": {:?}, \"tasks_dir\": {:?}, \"notes_dir\": {:?}}}",
                    ctx.project_name, ctx.mode, ctx.root_path, ctx.tasks_dir, ctx.notes_dir
                );
            } else {
                println!("\n{}", format!("═══ Active Project: {} ═══", ctx.project_name).bold().bright_blue());
                println!("{}: {}", "Storage Mode".bold(), ctx.mode);
                println!("{}: {}", "Root Path".bold(), ctx.root_path.display().to_string().bright_black());
                println!("{}: {}", "Tasks Dir".bold(), ctx.tasks_dir.display().to_string().cyan());
                println!("{}: {}", "Notes Dir".bold(), ctx.notes_dir.display().to_string().cyan());
                println!();
            }
        }
        Commands::List { status, priority, label, board } => {
            let all_tasks = match storage.list_tasks() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}: {}", "Error reading tasks".red().bold(), e);
                    process::exit(1);
                }
            };

            // Filter tasks
            let filtered: Vec<_> = all_tasks
                .into_iter()
                .filter(|t| {
                    if let Some(s) = status {
                        if t.frontmatter.status != s {
                            return false;
                        }
                    }
                    if let Some(p) = priority {
                        if t.frontmatter.priority != p {
                            return false;
                        }
                    }
                    if let Some(ref l) = label {
                        let query = l.to_lowercase();
                        if !t.frontmatter.labels.iter().any(|label| label.to_lowercase().contains(&query)) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            if cli.json {
                let summaries: Vec<TaskSummary> = filtered.iter().map(TaskSummary::from).collect();
                print_json(&summaries);
            } else if board {
                print_kanban_board(&filtered);
            } else {
                print_task_table(&filtered);
            }
        }

        Commands::Add { title, status, priority, labels, desc, checklists } => {
            let id = storage.next_id();
            let now = Local::now().to_rfc3339();

            let fm = TaskFrontmatter {
                id: id.clone(),
                title: title.clone(),
                status,
                priority,
                labels,
                assignee: None,
                created_at: Some(now.clone()),
                updated_at: Some(now),
                due_date: None,
            };

            let mut body = String::new();
            if let Some(d) = desc {
                body.push_str(&format!("## Description\n{}\n\n", d.trim()));
            }

            if !checklists.is_empty() {
                body.push_str("## Checklist\n");
                for item in checklists {
                    body.push_str(&format!("- [ ] {}\n", item.trim()));
                }
                body.push('\n');
            }

            match storage.save_task(&fm, &body) {
                Ok(saved) => {
                    if cli.json {
                        print_json(&TaskSummary::from(&saved));
                    } else if !cli.quiet {
                        println!(
                            "{} Task {} ({}) created at {:?}",
                            "✓".green().bold(),
                            saved.frontmatter.id.bold(),
                            saved.frontmatter.title,
                            saved.path
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error saving task".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Show { id, raw } => {
            match storage.find_task(&id) {
                Ok(task) => {
                    if raw {
                        print!("{}", task.content);
                    } else if cli.json {
                        print_json(&task);
                    } else {
                        print_task_details(&task);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Move { id, status } => {
            handle_status_change(&storage, &id, status, cli.json, cli.quiet);
        }

        Commands::Start { id } => {
            handle_status_change(&storage, &id, TaskStatus::InProgress, cli.json, cli.quiet);
        }

        Commands::Review { id } => {
            handle_status_change(&storage, &id, TaskStatus::Review, cli.json, cli.quiet);
        }

        Commands::Done { id } => {
            handle_status_change(&storage, &id, TaskStatus::Done, cli.json, cli.quiet);
        }

        Commands::Check { id, index, uncheck } => {
            let task = match storage.find_task(&id) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let updated_content = match parser::toggle_checklist_item(&task.content, index, !uncheck) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}: {}", "Checklist error".red().bold(), e);
                    process::exit(1);
                }
            };

            let mut fm = task.frontmatter.clone();
            fm.updated_at = Some(Local::now().to_rfc3339());

            match storage.save_task(&fm, &updated_content) {
                Ok(saved) => {
                    if cli.json {
                        print_json(&TaskSummary::from(&saved));
                    } else if !cli.quiet {
                        let action = if uncheck { "unmarked" } else { "marked" };
                        println!(
                            "{} In task {}, {} item #{}",
                            "✓".green().bold(),
                            saved.frontmatter.id.bold(),
                            action,
                            index
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error saving task".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Log { id, message } => {
            let task = match storage.find_task(&id) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let updated_content = parser::append_log_entry(&task.content, &message);
            let mut fm = task.frontmatter.clone();
            fm.updated_at = Some(Local::now().to_rfc3339());

            match storage.save_task(&fm, &updated_content) {
                Ok(saved) => {
                    if cli.json {
                        print_json(&TaskSummary::from(&saved));
                    } else if !cli.quiet {
                        println!(
                            "{} Added log entry to task {}",
                            "✓".green().bold(),
                            saved.frontmatter.id.bold()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error saving task".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Edit { id } => {
            let task = match storage.find_task(&id) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| "vim".to_string());

            let status = process::Command::new(&editor)
                .arg(&task.path)
                .status();

            match status {
                Ok(s) if s.success() => {
                    if !cli.quiet {
                        println!("{} Task {} edited.", "✓".green().bold(), task.frontmatter.id);
                    }
                }
                _ => {
                    eprintln!("{}: Could not open editor '{}'", "Warning".yellow().bold(), editor);
                }
            }
        }

        Commands::Delete { id, yes } => {
            if !yes {
                eprintln!("{}: Please specify -y / --yes to confirm deletion", "Warning".yellow().bold());
                process::exit(1);
            }

            match storage.delete_task(&id) {
                Ok(deleted) => {
                    if cli.json {
                        print_json(&TaskSummary::from(&deleted));
                    } else if !cli.quiet {
                        println!(
                            "{} Task {} ({}) deleted",
                            "✓".green().bold(),
                            deleted.frontmatter.id.bold(),
                            deleted.frontmatter.title
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error deleting task".red().bold(), e);
                    process::exit(1);
                }
            }
        }
    }
}

fn handle_status_change(storage: &Storage, id: &str, new_status: TaskStatus, json_output: bool, quiet: bool) {
    let task = match storage.find_task(id) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            process::exit(1);
        }
    };

    let old_status = task.frontmatter.status;
    let mut fm = task.frontmatter.clone();
    fm.status = new_status;
    fm.updated_at = Some(Local::now().to_rfc3339());

    match storage.save_task(&fm, &task.content) {
        Ok(saved) => {
            if json_output {
                print_json(&TaskSummary::from(&saved));
            } else if !quiet {
                println!(
                    "{} Task {} status changed: {} ➔ {}",
                    "✓".green().bold(),
                    saved.frontmatter.id.bold(),
                    old_status.display_colored(),
                    new_status.display_colored()
                );
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error saving task status".red().bold(), e);
            process::exit(1);
        }
    }
}
