use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use crate::models::{ProjectEntry, ProjectsConfig};

pub struct Registry;

impl Registry {
    pub fn base_dir() -> PathBuf {
        if let Ok(tracker_home) = std::env::var("TRACKER_HOME") {
            PathBuf::from(tracker_home)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".local-tracker")
        } else {
            PathBuf::from(".local-tracker")
        }
    }

    pub fn config_path() -> PathBuf {
        Self::base_dir().join("projects.toml")
    }

    pub fn vaults_dir() -> PathBuf {
        Self::base_dir().join("vaults")
    }

    pub fn load() -> ProjectsConfig {
        let path = Self::config_path();
        if !path.exists() {
            return ProjectsConfig::default();
        }

        if let Ok(content) = fs::read_to_string(&path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            ProjectsConfig::default()
        }
    }

    pub fn save(config: &ProjectsConfig) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create registry directory {:?}: {}", parent, e))?;
        }

        let toml_str = toml::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize projects config: {}", e))?;

        fs::write(&path, toml_str)
            .map_err(|e| format!("Failed to write projects config to {:?}: {}", path, e))?;

        Ok(())
    }

    pub fn register(
        name: &str,
        project_path: &Path,
        storage_path: &Path,
        git_remote: Option<String>,
    ) -> Result<ProjectEntry, String> {
        let mut config = Self::load();
        let now = Local::now().to_rfc3339();

        let entry = ProjectEntry {
            name: name.to_string(),
            path: project_path.to_path_buf(),
            storage: storage_path.to_path_buf(),
            git_remote,
            updated_at: Some(now),
        };

        config.projects.insert(name.to_string(), entry.clone());
        Self::save(&config)?;

        Ok(entry)
    }

    pub fn find_by_path(current_dir: &Path) -> Option<ProjectEntry> {
        let config = Self::load();
        let mut matching: Vec<ProjectEntry> = config
            .projects
            .into_values()
            .filter(|entry| current_dir.starts_with(&entry.path))
            .collect();

        // Longest path match wins (most specific sub-project)
        matching.sort_by(|a, b| b.path.as_os_str().len().cmp(&a.path.as_os_str().len()));
        matching.into_iter().next()
    }

    pub fn find_by_git_remote(remote: &str) -> Option<ProjectEntry> {
        let config = Self::load();
        for entry in config.projects.values() {
            if let Some(ref r) = entry.git_remote {
                if r.eq_ignore_ascii_case(remote) {
                    return Some(entry.clone());
                }
            }
        }
        None
    }

    pub fn update_path(name: &str, new_path: &Path) -> Result<(), String> {
        let mut config = Self::load();
        if let Some(entry) = config.projects.get_mut(name) {
            entry.path = new_path.to_path_buf();
            entry.updated_at = Some(Local::now().to_rfc3339());
            Self::save(&config)?;
            Ok(())
        } else {
            Err(format!("Project '{}' not found in registry", name))
        }
    }

    pub fn list() -> Vec<ProjectEntry> {
        let config = Self::load();
        let mut list: Vec<_> = config.projects.into_values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}
