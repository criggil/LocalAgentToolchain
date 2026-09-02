use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use crate::models::{AgentTarget, SkillScope, WorkspaceSkill};

pub mod claude;
pub mod codex;
pub mod antigravity;
pub mod pi;
pub mod junie;

pub struct AdapterRegistry;

impl AdapterRegistry {
    pub fn deploy(
        target: AgentTarget,
        skill: &WorkspaceSkill,
        project_root: &Path,
        scope: SkillScope,
        create_symlink: bool,
    ) -> Result<Vec<String>, String> {
        let targets = match target {
            AgentTarget::All => AgentTarget::all_supported(),
            t => vec![t],
        };

        let mut messages = Vec::new();

        for t in targets {
            let res = match t {
                AgentTarget::Claude => claude::deploy(skill, project_root, scope, create_symlink),
                AgentTarget::Codex => codex::deploy(skill, project_root, scope, create_symlink),
                AgentTarget::Antigravity => antigravity::deploy(skill, project_root, scope, create_symlink),
                AgentTarget::Pi => pi::deploy(skill, project_root, scope, create_symlink),
                AgentTarget::Junie => junie::deploy(skill, project_root, scope, create_symlink),
                AgentTarget::All => unreachable!(),
            };

            match res {
                Ok(msg) => messages.push(msg),
                Err(e) => eprintln!("Warning deploying to {}: {}", t.display_name(), e),
            }
        }

        Ok(messages)
    }

    pub fn remove(
        target: AgentTarget,
        skill_name: &str,
        project_root: &Path,
        scope: SkillScope,
    ) -> Result<Vec<String>, String> {
        let targets = match target {
            AgentTarget::All => AgentTarget::all_supported(),
            t => vec![t],
        };

        let mut messages = Vec::new();

        for t in targets {
            let res = match t {
                AgentTarget::Claude => claude::remove(skill_name, project_root, scope),
                AgentTarget::Codex => codex::remove(skill_name, project_root, scope),
                AgentTarget::Antigravity => antigravity::remove(skill_name, project_root, scope),
                AgentTarget::Pi => pi::remove(skill_name, project_root, scope),
                AgentTarget::Junie => junie::remove(skill_name, project_root, scope),
                AgentTarget::All => unreachable!(),
            };

            match res {
                Ok(msg) => messages.extend(msg),
                Err(e) => eprintln!("Warning removing from {}: {}", t.display_name(), e),
            }
        }

        Ok(messages)
    }
}

pub(crate) fn deploy_folder_or_symlink(
    src_dir: &Path,
    dest_dir: &Path,
    create_symlink: bool,
) -> Result<(), String> {
    if let Some(parent) = dest_dir.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {:?}: {}", parent, e))?;
    }

    // Remove existing if any
    if dest_dir.exists() || dest_dir.is_symlink() {
        if dest_dir.is_dir() && !dest_dir.is_symlink() {
            fs::remove_dir_all(dest_dir).ok();
        } else {
            fs::remove_file(dest_dir).ok();
        }
    }

    if create_symlink {
        symlink(src_dir, dest_dir)
            .map_err(|e| format!("Failed to create symlink from {:?} to {:?}: {}", src_dir, dest_dir, e))?;
    } else {
        copy_dir_all(src_dir, dest_dir)
            .map_err(|e| format!("Failed to copy directory from {:?} to {:?}: {}", src_dir, dest_dir, e))?;
    }

    Ok(())
}

pub(crate) fn deploy_file_or_symlink(
    src_file: &Path,
    dest_file: &Path,
    create_symlink: bool,
) -> Result<(), String> {
    if let Some(parent) = dest_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {:?}: {}", parent, e))?;
    }

    if dest_file.exists() || dest_file.is_symlink() {
        fs::remove_file(dest_file).ok();
    }

    if create_symlink {
        symlink(src_file, dest_file)
            .map_err(|e| format!("Failed to create symlink: {}", e))?;
    } else {
        fs::copy(src_file, dest_file)
            .map_err(|e| format!("Failed to copy file: {}", e))?;
    }

    Ok(())
}

pub(crate) fn remove_entry_safely(path: &Path) -> bool {
    if path.is_symlink() || path.is_file() {
        fs::remove_file(path).is_ok()
    } else if path.is_dir() {
        fs::remove_dir_all(path).is_ok()
    } else {
        false
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if ft.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
