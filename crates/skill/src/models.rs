use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentTarget {
    Claude,
    Codex,
    Antigravity,
    Pi,
    Junie,
    All,
}

impl AgentTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentTarget::Claude => "claude",
            AgentTarget::Codex => "codex",
            AgentTarget::Antigravity => "antigravity",
            AgentTarget::Pi => "pi",
            AgentTarget::Junie => "junie",
            AgentTarget::All => "all",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AgentTarget::Claude => "🤖 Claude Code",
            AgentTarget::Codex => "🤖 OpenAI Codex",
            AgentTarget::Antigravity => "🤖 Google Antigravity",
            AgentTarget::Pi => "🤖 Pi Coding Agent",
            AgentTarget::Junie => "🤖 JetBrains Junie",
            AgentTarget::All => "🌐 All Agents",
        }
    }

    pub fn all_supported() -> Vec<AgentTarget> {
        vec![
            AgentTarget::Claude,
            AgentTarget::Codex,
            AgentTarget::Antigravity,
            AgentTarget::Pi,
            AgentTarget::Junie,
        ]
    }
}

impl fmt::Display for AgentTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AgentTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "claude" | "claude-code" => Ok(AgentTarget::Claude),
            "codex" | "openai" => Ok(AgentTarget::Codex),
            "antigravity" | "agy" | "gemini" => Ok(AgentTarget::Antigravity),
            "pi" => Ok(AgentTarget::Pi),
            "junie" | "jetbrains" => Ok(AgentTarget::Junie),
            "all" => Ok(AgentTarget::All),
            _ => Err(format!(
                "Unknown agent target '{}'. Valid: claude, codex, antigravity, pi, junie, all",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillScope {
    Local,
    Global,
    Both,
}

impl fmt::Display for SkillScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillScope::Local => write!(f, "local"),
            SkillScope::Global => write!(f, "global"),
            SkillScope::Both => write!(f, "both"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSkill {
    pub manifest: SkillManifest,
    pub dir_path: PathBuf,
    pub skill_file: PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub name: String,
    pub target: AgentTarget,
    pub scope: SkillScope,
    pub path: PathBuf,
    pub is_symlink: bool,
    pub points_to: Option<PathBuf>,
}
