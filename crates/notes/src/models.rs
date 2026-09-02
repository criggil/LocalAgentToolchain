use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteFrontmatter {
    pub title: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    #[serde(flatten)]
    pub frontmatter: NoteFrontmatter,
    pub filename: String,
    pub relative_path: String,
    pub path: PathBuf,
    pub content: String,
    pub outgoing_links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteSummary {
    pub path: String,
    pub title: String,
    pub folder: String,
    pub tags: Vec<String>,
    pub updated_at: Option<String>,
    pub links_count: usize,
}

impl From<&Note> for NoteSummary {
    fn from(n: &Note) -> Self {
        let folder = std::path::Path::new(&n.relative_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        Self {
            path: n.relative_path.clone(),
            title: n.frontmatter.title.clone(),
            folder: if folder.is_empty() { ".".to_string() } else { folder },
            tags: n.frontmatter.tags.clone(),
            updated_at: n.frontmatter.updated_at.clone().or_else(|| n.frontmatter.created_at.clone()),
            links_count: n.outgoing_links.len(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchMatch {
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub tags: Vec<String>,
    pub matches: Vec<SearchMatch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkGraph {
    pub note_path: String,
    pub note_title: String,
    pub outgoing_links: Vec<String>,
    pub backlinks: Vec<String>,
}
