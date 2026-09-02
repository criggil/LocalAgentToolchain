use std::process;
use clap::{Parser, Subcommand};
use colored::*;
use chrono::Local;

mod models;
mod parser;
mod storage;
mod formatters;

use models::{NoteFrontmatter, NoteSummary};
use storage::NoteStorage;
use formatters::human::*;
use formatters::json::print_json;

#[derive(Parser)]
#[command(
    name = "note",
    about = "🧠 Fast, agent-friendly Markdown knowledge base and wiki manager",
    version = "0.1.0",
    arg_required_else_help = true
)]
struct Cli {
    #[arg(long, global = true, help = "Machine-readable JSON output (for Claude Code and AI agents)")]
    json: bool,

    #[arg(long, global = true, help = "Path to workspace directory with notes")]
    workspace: Option<String>,

    #[arg(short, long, global = true, help = "Suppress informational messages")]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "list", aliases = ["ls"], about = "List notes in workspace or folder")]
    List {
        #[arg(help = "Optional folder filter (e.g. 20_Wiki, 00_Inbox)")]
        folder: Option<String>,

        #[arg(short, long, help = "Filter by tag")]
        tag: Option<String>,
    },

    #[command(name = "new", aliases = ["add"], about = "Create a new note with YAML frontmatter")]
    New {
        #[arg(help = "Note title")]
        title: String,

        #[arg(short, long, default_value = "00_Inbox", help = "Target folder")]
        folder: String,

        #[arg(short, long, value_delimiter = ',', help = "Comma-separated tags")]
        tags: Vec<String>,

        #[arg(short, long, value_delimiter = ',', help = "Comma-separated aliases")]
        aliases: Vec<String>,

        #[arg(short, long, help = "Initial markdown content body")]
        content: Option<String>,
    },

    #[command(name = "show", aliases = ["view"], about = "Display note content and metadata")]
    Show {
        #[arg(help = "Note title, filename, or relative path")]
        query: String,

        #[arg(long, help = "Print raw markdown file content")]
        raw: bool,
    },

    #[command(name = "search", aliases = ["find"], about = "Full-text search across notes")]
    Search {
        #[arg(help = "Search query string")]
        query: String,

        #[arg(short, long, help = "Filter search results by tag")]
        tag: Option<String>,
    },

    #[command(name = "links", aliases = ["graph", "backlinks"], about = "Show outgoing Wikilinks and backlinks")]
    Links {
        #[arg(help = "Note title, filename, or relative path")]
        query: String,
    },

    #[command(name = "append", about = "Append text to the end of a note")]
    Append {
        #[arg(help = "Note title, filename, or relative path")]
        query: String,

        #[arg(help = "Text to append")]
        text: String,
    },

    #[command(name = "edit", about = "Open note in text editor ($EDITOR)")]
    Edit {
        #[arg(help = "Note title, filename, or relative path")]
        query: String,
    },

    #[command(name = "delete", aliases = ["rm"], about = "Delete a note file")]
    Delete {
        #[arg(help = "Note title, filename, or relative path")]
        query: String,

        #[arg(short, long, help = "Confirm deletion without prompt")]
        yes: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let storage = match NoteStorage::discover(cli.workspace.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            process::exit(1);
        }
    };

    match cli.command {
        Commands::List { folder, tag } => {
            let all_notes = match storage.list_notes(folder.as_deref()) {
                Ok(notes) => notes,
                Err(e) => {
                    eprintln!("{}: {}", "Error reading notes".red().bold(), e);
                    process::exit(1);
                }
            };

            let filtered: Vec<_> = all_notes
                .into_iter()
                .filter(|n| {
                    if let Some(ref t) = tag {
                        let q = t.to_lowercase();
                        if !n.frontmatter.tags.iter().any(|tag| tag.to_lowercase() == q) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            if cli.json {
                let summaries: Vec<NoteSummary> = filtered.iter().map(NoteSummary::from).collect();
                print_json(&summaries);
            } else {
                print_notes_table(&filtered);
            }
        }

        Commands::New { title, folder, tags, aliases, content } => {
            let today = Local::now().format("%Y-%m-%d").to_string();

            let fm = NoteFrontmatter {
                title: title.clone(),
                tags,
                aliases,
                created_at: Some(today.clone()),
                updated_at: Some(today),
            };

            let body = content.unwrap_or_else(|| format!("# {}\n\n", title));
            let filename = title.replace(' ', "_");

            match storage.save_note(Some(&folder), &filename, &fm, &body) {
                Ok(saved) => {
                    if cli.json {
                        print_json(&NoteSummary::from(&saved));
                    } else if !cli.quiet {
                        println!(
                            "{} Note created: {} ({:?})",
                            "✓".green().bold(),
                            saved.frontmatter.title.bold(),
                            saved.path
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error saving note".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Show { query, raw } => {
            match storage.find_note(&query) {
                Ok(note) => {
                    if raw {
                        print!("{}", note.content);
                    } else if cli.json {
                        print_json(&note);
                    } else {
                        print_note_details(&note);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Search { query, tag } => {
            match storage.search(&query, tag.as_deref()) {
                Ok(results) => {
                    if cli.json {
                        print_json(&results);
                    } else {
                        print_search_results(&query, &results);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Search error".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Links { query } => {
            match storage.get_link_graph(&query) {
                Ok(graph) => {
                    if cli.json {
                        print_json(&graph);
                    } else {
                        print_link_graph(&graph);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Append { query, text } => {
            let note = match storage.find_note(&query) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let updated_body = parser::append_to_note(&note.content, &text);
            let mut fm = note.frontmatter.clone();
            fm.updated_at = Some(Local::now().format("%Y-%m-%d").to_string());

            let folder = std::path::Path::new(&note.relative_path)
                .parent()
                .map(|p| p.to_string_lossy().to_string());

            match storage.save_note(folder.as_deref(), &note.filename, &fm, &updated_body) {
                Ok(saved) => {
                    if cli.json {
                        print_json(&NoteSummary::from(&saved));
                    } else if !cli.quiet {
                        println!(
                            "{} Appended text to {}",
                            "✓".green().bold(),
                            saved.relative_path.bold()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error updating note".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Edit { query } => {
            let note = match storage.find_note(&query) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| "vim".to_string());

            let status = process::Command::new(&editor)
                .arg(&note.path)
                .status();

            match status {
                Ok(s) if s.success() => {
                    if !cli.quiet {
                        println!("{} Note {} edited.", "✓".green().bold(), note.relative_path);
                    }
                }
                _ => {
                    eprintln!("{}: Could not open editor '{}'", "Warning".yellow().bold(), editor);
                }
            }
        }

        Commands::Delete { query, yes } => {
            if !yes {
                eprintln!("{}: Please specify -y / --yes to confirm deletion", "Warning".yellow().bold());
                process::exit(1);
            }

            match storage.delete_note(&query) {
                Ok(deleted) => {
                    if cli.json {
                        print_json(&NoteSummary::from(&deleted));
                    } else if !cli.quiet {
                        println!(
                            "{} Note {} ({}) deleted",
                            "✓".green().bold(),
                            deleted.relative_path.bold(),
                            deleted.frontmatter.title
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error deleting note".red().bold(), e);
                    process::exit(1);
                }
            }
        }
    }
}
