use std::collections::HashMap;
use colored::Colorize;
use tabled::{Table, Tabled};
use tabled::settings::Style;
use crate::models::{AgentTarget, InstalledSkill, SkillScope, WorkspaceSkill};

#[derive(Tabled)]
struct SkillTableRow {
    #[tabled(rename = "SKILL")]
    name: String,

    #[tabled(rename = "VERSION")]
    version: String,

    #[tabled(rename = "DESCRIPTION")]
    description: String,

    #[tabled(rename = "INSTALLED TARGETS")]
    installed: String,
}

pub fn print_workspace_skills(skills: &[WorkspaceSkill], installed: &[InstalledSkill]) {
    if skills.is_empty() {
        println!("{}", "No workspace skills found in workspace/skills/".bright_black());
        println!("Run `skill new <name>` to create your first skill package.");
        return;
    }

    let rows: Vec<SkillTableRow> = skills
        .iter()
        .map(|s| {
            let matches: Vec<String> = installed
                .iter()
                .filter(|i| i.name == s.manifest.name)
                .map(|i| format!("{} ({})", i.target.as_str(), i.scope))
                .collect();

            let installed_str = if matches.is_empty() {
                "-".bright_black().to_string()
            } else {
                matches.join(", ").cyan().to_string()
            };

            SkillTableRow {
                name: s.manifest.name.bold().to_string(),
                version: s.manifest.version.clone().unwrap_or_else(|| "0.1.0".to_string()),
                description: truncate(&s.manifest.description, 50),
                installed: installed_str,
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{}", table);
    println!("Total workspace skills: {}\n", skills.len());
}

pub fn print_installed_skills_tree(installed: &[InstalledSkill]) {
    if installed.is_empty() {
        println!("{}", "No skills currently installed across any agent directories.".bright_black());
        return;
    }

    println!("\n{}", "═══ Installed Agent Skills ══════════════════════════════════════════════".bright_blue().bold());

    let mut by_target: HashMap<AgentTarget, Vec<&InstalledSkill>> = HashMap::new();
    for item in installed {
        by_target.entry(item.target).or_default().push(item);
    }

    for target in AgentTarget::all_supported() {
        if let Some(items) = by_target.get(&target) {
            println!("\n{}", target.display_name().bold());

            // Group by scope
            let mut global_items = Vec::new();
            let mut local_items = Vec::new();

            for item in items {
                match item.scope {
                    SkillScope::Global => global_items.push(*item),
                    SkillScope::Local | SkillScope::Both => local_items.push(*item),
                }
            }

            if !global_items.is_empty() {
                println!("  {}", "[Global Scope]".bright_black());
                for it in global_items {
                    print_skill_item(it);
                }
            }

            if !local_items.is_empty() {
                println!("  {}", "[Project Scope]".bright_black());
                for it in local_items {
                    print_skill_item(it);
                }
            }
        }
    }
    println!();
}

fn print_skill_item(it: &InstalledSkill) {
    let symlink_badge = if it.is_symlink {
        if let Some(ref p) = it.points_to {
            format!("(symlink -> {})", p.display().to_string().bright_black()).cyan()
        } else {
            "(symlink)".cyan()
        }
    } else {
        "(file/dir)".bright_black()
    };

    println!("    • {} {}", it.name.bold(), symlink_badge);
}

pub fn print_skill_details(skill: &WorkspaceSkill, installed: &[InstalledSkill]) {
    println!("\n{}", format!("═══ Skill: {} ═══", skill.manifest.name).bold().bright_blue());
    println!("{}: {}", "Description".bold(), skill.manifest.description);
    if let Some(ref v) = skill.manifest.version {
        println!("{}: {}", "Version".bold(), v);
    }
    if let Some(ref a) = skill.manifest.author {
        println!("{}: {}", "Author".bold(), a);
    }
    if !skill.manifest.triggers.is_empty() {
        println!("{}: {}", "Triggers".bold(), skill.manifest.triggers.join(", ").yellow());
    }

    let matches: Vec<String> = installed
        .iter()
        .filter(|i| i.name == skill.manifest.name)
        .map(|i| format!("{} ({})", i.target.as_str(), i.scope))
        .collect();

    if !matches.is_empty() {
        println!("{}: {}", "Installed Targets".bold(), matches.join(", ").green());
    }

    println!("{}: {:?}", "Directory".bold(), skill.dir_path);

    println!("\n{}", "─── Standard Operating Procedure (SOP) ───────────────────────────────".bright_black());
    println!("{}", skill.body);
    println!();
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        format!("{}...", s.chars().take(max_chars - 3).collect::<String>())
    } else {
        s.to_string()
    }
}
