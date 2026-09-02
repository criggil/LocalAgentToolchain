use std::path::{Path, PathBuf};
use crate::models::{SkillScope, WorkspaceSkill};
use super::{deploy_folder_or_symlink, remove_entry_safely};

pub fn deploy(
    skill: &WorkspaceSkill,
    project_root: &Path,
    scope: SkillScope,
    symlink: bool,
) -> Result<String, String> {
    let home = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

    let dest_dir = match scope {
        SkillScope::Global => home.join(".codex").join("skills").join(&skill.manifest.name),
        SkillScope::Local | SkillScope::Both => project_root.join(".codex").join("skills").join(&skill.manifest.name),
    };

    deploy_folder_or_symlink(&skill.dir_path, &dest_dir, symlink)?;
    Ok(format!("OpenAI Codex ({}): Installed to {:?}", scope, dest_dir))
}

pub fn remove(skill_name: &str, project_root: &Path, scope: SkillScope) -> Result<Vec<String>, String> {
    let mut msgs = Vec::new();
    let home = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

    if scope == SkillScope::Global || scope == SkillScope::Both {
        let global_dir = home.join(".codex").join("skills").join(skill_name);
        if remove_entry_safely(&global_dir) {
            msgs.push(format!("OpenAI Codex: Removed global skill {:?}", global_dir));
        }
    }

    if scope == SkillScope::Local || scope == SkillScope::Both {
        let local_dir = project_root.join(".codex").join("skills").join(skill_name);
        if remove_entry_safely(&local_dir) {
            msgs.push(format!("OpenAI Codex: Removed local skill {:?}", local_dir));
        }
    }

    Ok(msgs)
}
