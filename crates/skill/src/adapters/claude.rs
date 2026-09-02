use std::path::{Path, PathBuf};
use crate::models::{SkillScope, WorkspaceSkill};
use super::{deploy_file_or_symlink, remove_entry_safely};

pub fn deploy(
    skill: &WorkspaceSkill,
    project_root: &Path,
    scope: SkillScope,
    symlink: bool,
) -> Result<String, String> {
    let home = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

    let dest_file = match scope {
        SkillScope::Global => home.join(".claude").join("commands").join(format!("{}.md", skill.manifest.name)),
        SkillScope::Local | SkillScope::Both => project_root.join(".claude").join("commands").join(format!("{}.md", skill.manifest.name)),
    };

    deploy_file_or_symlink(&skill.skill_file, &dest_file, symlink)?;
    Ok(format!("Claude Code ({}): Installed to {:?}", scope, dest_file))
}

pub fn remove(skill_name: &str, project_root: &Path, scope: SkillScope) -> Result<Vec<String>, String> {
    let mut msgs = Vec::new();
    let home = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

    let filename = if skill_name.ends_with(".md") { skill_name.to_string() } else { format!("{}.md", skill_name) };

    if scope == SkillScope::Global || scope == SkillScope::Both {
        let global_file = home.join(".claude").join("commands").join(&filename);
        if remove_entry_safely(&global_file) {
            msgs.push(format!("Claude Code: Removed global command {:?}", global_file));
        }
    }

    if scope == SkillScope::Local || scope == SkillScope::Both {
        let local_file = project_root.join(".claude").join("commands").join(&filename);
        if remove_entry_safely(&local_file) {
            msgs.push(format!("Claude Code: Removed local command {:?}", local_file));
        }
    }

    Ok(msgs)
}
