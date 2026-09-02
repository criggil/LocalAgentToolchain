use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageMode {
    /// Local files directly inside repository (e.g. .tasks/, .notes/)
    Embedded,
    /// Detached personal vault outside project (e.g. ~/.local-tracker/vaults/<project>/)
    Detached,
    /// Default workspace directory (e.g. ./workspace/)
    Workspace,
}

impl fmt::Display for StorageMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageMode::Embedded => write!(f, "embedded"),
            StorageMode::Detached => write!(f, "detached"),
            StorageMode::Workspace => write!(f, "workspace"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub name: String,
    pub path: PathBuf,
    pub storage: PathBuf,
    #[serde(default)]
    pub git_remote: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectsConfig {
    #[serde(default)]
    pub projects: HashMap<String, ProjectEntry>,
}
