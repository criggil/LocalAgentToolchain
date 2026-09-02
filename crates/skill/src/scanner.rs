use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{AgentTarget, InstalledSkill, SkillScope};

pub struct AgentScanner;

impl AgentScanner {
    pub fn scan_all(project_root: &Path, filter_target: Option<AgentTarget>) -> Vec<InstalledSkill> {
        let targets = match filter_target {
            Some(AgentTarget::All) | None => AgentTarget::all_supported(),
            Some(t) => vec![t],
        };

        let mut installed = Vec::new();

        for target in targets {
            installed.extend(Self::scan_target(target, project_root));
        }

        installed.sort_by(|a, b| a.name.cmp(&b.name));
        installed
    }

    pub fn scan_target(target: AgentTarget, project_root: &Path) -> Vec<InstalledSkill> {
        let mut results = Vec::new();

        let home = std::env::var("HOME").map(PathBuf::from).ok();

        match target {
            AgentTarget::Claude => {
                // Global: ~/.claude/commands/*.md
                if let Some(ref h) = home {
                    Self::collect_files(&h.join(".claude").join("commands"), target, SkillScope::Global, &mut results);
                }
                // Local: ./.claude/commands/*.md
                Self::collect_files(&project_root.join(".claude").join("commands"), target, SkillScope::Local, &mut results);
            }

            AgentTarget::Codex => {
                // Global: ~/.codex/skills/*/SKILL.md
                if let Some(ref h) = home {
                    Self::collect_directories(&h.join(".codex").join("skills"), target, SkillScope::Global, &mut results);
                }
                // Local: ./.codex/skills/*/SKILL.md
                Self::collect_directories(&project_root.join(".codex").join("skills"), target, SkillScope::Local, &mut results);
            }

            AgentTarget::Antigravity => {
                // Global: ~/.gemini/config/skills/* and ~/.gemini/antigravity/skills/*
                if let Some(ref h) = home {
                    Self::collect_directories(&h.join(".gemini").join("config").join("skills"), target, SkillScope::Global, &mut results);
                    Self::collect_directories(&h.join(".gemini").join("antigravity").join("skills"), target, SkillScope::Global, &mut results);
                }
                // Local: ./.agents/skills/* and ./.agent/skills/*
                Self::collect_directories(&project_root.join(".agents").join("skills"), target, SkillScope::Local, &mut results);
                Self::collect_directories(&project_root.join(".agent").join("skills"), target, SkillScope::Local, &mut results);
            }

            AgentTarget::Pi => {
                // Global: ~/.pi/agent/skills/*
                if let Some(ref h) = home {
                    Self::collect_directories(&h.join(".pi").join("agent").join("skills"), target, SkillScope::Global, &mut results);
                }
                // Local: ./.pi/skills/*
                Self::collect_directories(&project_root.join(".pi").join("skills"), target, SkillScope::Local, &mut results);
            }

            AgentTarget::Junie => {
                // Global: ~/.junie/skills/*
                if let Some(ref h) = home {
                    Self::collect_directories(&h.join(".junie").join("skills"), target, SkillScope::Global, &mut results);
                }
                // Local: ./.junie/skills/*
                Self::collect_directories(&project_root.join(".junie").join("skills"), target, SkillScope::Local, &mut results);
            }

            AgentTarget::All => {}
        }

        results
    }

    fn collect_files(dir: &Path, target: AgentTarget, scope: SkillScope, out: &mut Vec<InstalledSkill>) {
        if !dir.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    let is_symlink = fs::symlink_metadata(&path).map(|m| m.file_type().is_symlink()).unwrap_or(false);
                    let points_to = if is_symlink { fs::read_link(&path).ok() } else { None };

                    out.push(InstalledSkill {
                        name,
                        target,
                        scope,
                        path,
                        is_symlink,
                        points_to,
                    });
                }
            }
        }
    }

    fn collect_directories(dir: &Path, target: AgentTarget, scope: SkillScope, out: &mut Vec<InstalledSkill>) {
        if !dir.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_symlink = fs::symlink_metadata(&path).map(|m| m.file_type().is_symlink()).unwrap_or(false);
                let points_to = if is_symlink { fs::read_link(&path).ok() } else { None };

                if path.is_dir() || is_symlink {
                    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    out.push(InstalledSkill {
                        name,
                        target,
                        scope,
                        path,
                        is_symlink,
                        points_to,
                    });
                }
            }
        }
    }
}
