use std::process;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod models;
mod scanner;
mod storage;
mod adapters;
mod formatters;

use models::{AgentTarget, SkillScope};
use scanner::AgentScanner;
use storage::SkillStorage;
use adapters::AdapterRegistry;
use formatters::human::{print_installed_skills_tree, print_skill_details, print_workspace_skills};
use formatters::json::print_json;

#[derive(Parser)]
#[command(
    name = "skill",
    version,
    about = "Universal AI Agent Skill Manager for Claude, Codex, Antigravity, Pi, and Junie",
    long_about = "Manage, package, audit, and deploy procedural skills across Claude Code, OpenAI Codex, Google Antigravity, Pi, and JetBrains Junie."
)]
struct Cli {
    #[arg(long, help = "Output in machine-readable JSON format", global = true)]
    json: bool,

    #[arg(short, long, help = "Explicit project workspace path", global = true)]
    workspace: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "list", aliases = ["ls"], about = "List workspace skills or audit installed agent skills")]
    List {
        #[arg(short, long, help = "Deep scan all installed skills across agent directories on machine")]
        installed: bool,

        #[arg(short, long, help = "Filter by agent target (claude, codex, antigravity, pi, junie, all)")]
        target: Option<AgentTarget>,
    },

    #[command(name = "install", aliases = ["add"], about = "Deploy a skill to AI agent discovery directories")]
    Install {
        #[arg(help = "Name of the skill to install (from workspace/skills/)")]
        name: String,

        #[arg(short, long, help = "Target agent (claude, codex, antigravity, pi, junie, all) [default: all]")]
        target: Option<AgentTarget>,

        #[arg(short, long, help = "Install globally into user home directories")]
        global: bool,

        #[arg(short, long, help = "Install locally into current project repository")]
        local: bool,

        #[arg(long, help = "Create live symbolic link instead of copying file [default: true]")]
        copy: bool,
    },

    #[command(name = "remove", aliases = ["uninstall", "rm"], about = "Remove or unlink an installed skill from agent directories")]
    Remove {
        #[arg(help = "Name of the skill to remove")]
        name: String,

        #[arg(short, long, help = "Target agent (claude, codex, antigravity, pi, junie, all) [default: all]")]
        target: Option<AgentTarget>,

        #[arg(short, long, help = "Remove from global user home directories")]
        global: bool,

        #[arg(short, long, help = "Remove from local project repository")]
        local: bool,

        #[arg(short, long, help = "Confirm deletion without prompt")]
        yes: bool,
    },

    #[command(name = "show", about = "Inspect full skill SOP and configuration")]
    Show {
        #[arg(help = "Skill name")]
        name: String,

        #[arg(long, help = "Print raw markdown without headers")]
        raw: bool,
    },

    #[command(name = "new", about = "Scaffold a new skill package template")]
    New {
        #[arg(help = "Skill name (e.g. database-migrator)")]
        name: String,

        #[arg(short, long, help = "Description of what this skill does and when to invoke it")]
        desc: Option<String>,
    },

    #[command(name = "sync", about = "Synchronize all workspace skills to detected agents")]
    Sync {
        #[arg(short, long, help = "Sync globally")]
        global: bool,

        #[arg(short, long, help = "Sync locally")]
        local: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let storage = match SkillStorage::discover(cli.workspace.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            process::exit(1);
        }
    };

    let project_root = storage.context.root_path.clone();

    match cli.command {
        Commands::List { installed, target } => {
            let installed_skills = AgentScanner::scan_all(&project_root, target);

            if installed {
                if cli.json {
                    print_json(&installed_skills);
                } else {
                    print_installed_skills_tree(&installed_skills);
                }
            } else {
                let workspace_skills = storage.list_skills();
                if cli.json {
                    print_json(&workspace_skills);
                } else {
                    print_workspace_skills(&workspace_skills, &installed_skills);
                }
            }
        }

        Commands::Install { name, target, global, local, copy } => {
            let skill = match storage.find_skill(&name) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            let target = target.unwrap_or(AgentTarget::All);
            let scope = determine_scope(global, local);
            let use_symlink = !copy;

            match AdapterRegistry::deploy(target, &skill, &project_root, scope, use_symlink) {
                Ok(messages) => {
                    if cli.json {
                        println!("{{\"skill\": \"{}\", \"installed\": true, \"targets\": {:?}}}", skill.manifest.name, messages);
                    } else {
                        println!(
                            "{} Skill '{}' successfully deployed!",
                            "✓".green().bold(),
                            skill.manifest.name.bold()
                        );
                        for msg in messages {
                            println!("  • {}", msg);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error installing skill".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Remove { name, target, global, local, yes } => {
            let target = target.unwrap_or(AgentTarget::All);
            let scope = determine_scope(global, local);

            if !yes && !cli.json {
                print!("Are you sure you want to remove skill '{}' from {}? (y/N): ", name.bold(), target.display_name());
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok();
                if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
                    println!("Aborted.");
                    return;
                }
            }

            match AdapterRegistry::remove(target, &name, &project_root, scope) {
                Ok(messages) => {
                    if cli.json {
                        println!("{{\"skill\": \"{}\", \"removed\": true, \"actions\": {:?}}}", name, messages);
                    } else if messages.is_empty() {
                        println!("{} No matching skill files found for '{}' to remove.", "ℹ".blue(), name);
                    } else {
                        println!("{} Successfully removed skill '{}':", "✓".green().bold(), name.bold());
                        for msg in messages {
                            println!("  • {}", msg);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error removing skill".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Show { name, raw } => {
            let skill = match storage.find_skill(&name) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };

            if raw {
                println!("{}", skill.body);
            } else if cli.json {
                print_json(&skill);
            } else {
                let installed = AgentScanner::scan_all(&project_root, None);
                print_skill_details(&skill, &installed);
            }
        }

        Commands::New { name, desc } => {
            match storage.create_skill(&name, desc.as_deref()) {
                Ok(skill) => {
                    if cli.json {
                        print_json(&skill);
                    } else {
                        println!(
                            "{} Created skill package '{}' at {:?}",
                            "✓".green().bold(),
                            skill.manifest.name.bold(),
                            skill.dir_path
                        );
                        println!("  File: {:?}", skill.skill_file);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error creating skill".red().bold(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Sync { global, local } => {
            let skills = storage.list_skills();
            if skills.is_empty() {
                println!("{}", "No workspace skills found to sync.".bright_black());
                return;
            }

            let scope = determine_scope(global, local);
            let mut total_deployed = 0;

            println!("Syncing {} workspace skill(s)...", skills.len());
            for s in &skills {
                if let Ok(msgs) = AdapterRegistry::deploy(AgentTarget::All, s, &project_root, scope, true) {
                    total_deployed += msgs.len();
                }
            }

            println!("{} Synchronized {} skill bindings across detected agents.", "✓".green().bold(), total_deployed);
        }
    }
}

fn determine_scope(global: bool, local: bool) -> SkillScope {
    if global && local {
        SkillScope::Both
    } else if global {
        SkillScope::Global
    } else {
        SkillScope::Local
    }
}
