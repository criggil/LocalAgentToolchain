use std::fs;
use std::path::{Path, PathBuf};
use tracker_core::WorkspaceContext;
use crate::models::{SkillManifest, WorkspaceSkill};

pub struct SkillStorage {
    pub skills_dir: PathBuf,
    pub context: WorkspaceContext,
}

impl SkillStorage {
    pub fn discover(explicit_path: Option<&str>) -> Result<Self, String> {
        let ctx = WorkspaceContext::discover(explicit_path)?;
        let skills_dir = ctx.root_path.join("workspace").join("skills");
        fs::create_dir_all(&skills_dir)
            .map_err(|e| format!("Failed to create skills directory {:?}: {}", skills_dir, e))?;

        Ok(Self {
            skills_dir,
            context: ctx,
        })
    }

    pub fn list_skills(&self) -> Vec<WorkspaceSkill> {
        let mut skills = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let skill_file = path.join("SKILL.md");
                    if skill_file.is_file() {
                        if let Ok(skill) = Self::parse_skill_file(&path, &skill_file) {
                            skills.push(skill);
                        }
                    }
                }
            }
        }

        skills.sort_by(|a, b| a.manifest.name.cmp(&b.manifest.name));
        skills
    }

    pub fn find_skill(&self, query: &str) -> Result<WorkspaceSkill, String> {
        let skills = self.list_skills();
        let clean_q = query.trim().to_lowercase();

        // 1. Exact match
        if let Some(s) = skills.iter().find(|s| s.manifest.name.to_lowercase() == clean_q) {
            return Ok(s.clone());
        }

        // 2. Substring match
        let matches: Vec<&WorkspaceSkill> = skills
            .iter()
            .filter(|s| s.manifest.name.to_lowercase().contains(&clean_q))
            .collect();

        if matches.len() == 1 {
            return Ok((*matches[0]).clone());
        } else if matches.len() > 1 {
            return Err(format!("Multiple skills matched '{}'. Please specify full name.", query));
        }

        Err(format!("Skill '{}' not found in {:?}", query, self.skills_dir))
    }

    pub fn create_skill(&self, name: &str, description: Option<&str>) -> Result<WorkspaceSkill, String> {
        let clean_name = name.trim().replace(' ', "-").to_lowercase();
        let target_dir = self.skills_dir.join(&clean_name);

        if target_dir.exists() {
            return Err(format!("Skill '{}' already exists at {:?}", clean_name, target_dir));
        }

        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create skill folder: {}", e))?;
        fs::create_dir_all(target_dir.join("scripts")).ok();
        fs::create_dir_all(target_dir.join("examples")).ok();

        let desc = description.unwrap_or("Specialized procedural skill for autonomous coding agents.");
        let template_content = format!(
            "---\nname: {}\ndescription: {}\nversion: 0.1.0\nauthor: user\ntriggers:\n  - {}\n---\n\n# {} Standard Operating Procedure (SOP)\n\nWhen this skill is invoked:\n1. Step 1 description...\n2. Step 2 description...\n",
            clean_name, desc, clean_name, clean_name
        );

        let skill_file = target_dir.join("SKILL.md");
        fs::write(&skill_file, template_content)
            .map_err(|e| format!("Failed to write SKILL.md: {}", e))?;

        Self::parse_skill_file(&target_dir, &skill_file)
    }

    fn parse_skill_file(dir_path: &Path, skill_file: &Path) -> Result<WorkspaceSkill, String> {
        let content = fs::read_to_string(skill_file)
            .map_err(|e| format!("Failed to read {:?}: {}", skill_file, e))?;

        let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy().to_string();

        if !content.starts_with("---") {
            let manifest = SkillManifest {
                name: dir_name,
                description: "Skill instructions".to_string(),
                version: None,
                author: None,
                triggers: vec![],
            };
            return Ok(WorkspaceSkill {
                manifest,
                dir_path: dir_path.to_path_buf(),
                skill_file: skill_file.to_path_buf(),
                body: content,
            });
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(format!("Malformed YAML frontmatter in {:?}", skill_file));
        }

        let yaml_str = parts[1];
        let body = parts[2].trim().to_string();

        let manifest: SkillManifest = serde_yaml::from_str(yaml_str)
            .map_err(|e| format!("YAML parsing error in {:?}: {}", skill_file, e))?;

        Ok(WorkspaceSkill {
            manifest,
            dir_path: dir_path.to_path_buf(),
            skill_file: skill_file.to_path_buf(),
            body,
        })
    }
}
