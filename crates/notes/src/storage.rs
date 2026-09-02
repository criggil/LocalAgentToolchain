use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use walkdir::WalkDir;

use crate::models::{LinkGraph, Note, NoteFrontmatter, SearchMatch, SearchResult};
use crate::parser::{extract_wikilinks, parse_note_file, serialize_note_file};

pub struct NoteStorage {
    pub notes_dir: PathBuf,
    pub tasks_dir: Option<PathBuf>,
}

impl NoteStorage {
    pub fn discover(explicit_path: Option<&str>) -> Result<Self, String> {
        let ctx = tracker_core::WorkspaceContext::discover(explicit_path)?;
        let notes_dir = ctx.notes_dir.clone();
        let tasks_dir = if ctx.tasks_dir.is_dir() {
            Some(ctx.tasks_dir)
        } else {
            None
        };

        // Ensure default subfolders exist in notes directory
        fs::create_dir_all(notes_dir.join("00_Inbox")).ok();
        fs::create_dir_all(notes_dir.join("10_Projects")).ok();
        fs::create_dir_all(notes_dir.join("20_Wiki")).ok();
        fs::create_dir_all(notes_dir.join("30_Prompts")).ok();
        fs::create_dir_all(notes_dir.join("40_Reports")).ok();

        Ok(Self {
            notes_dir,
            tasks_dir,
        })
    }

    pub fn list_notes(&self, folder_filter: Option<&str>) -> Result<Vec<Note>, String> {
        let mut notes = Vec::new();

        let root_scan = if let Some(folder) = folder_filter {
            let sub = self.notes_dir.join(folder.trim_start_matches('/'));
            if !sub.exists() {
                return Err(format!("Folder '{}' does not exist in notes directory", folder));
            }
            sub
        } else {
            self.notes_dir.clone()
        };

        for entry in WalkDir::new(&root_scan).min_depth(1).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                let rel_path = path
                    .strip_prefix(&self.notes_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();

                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(note) = parse_note_file(path, &rel_path, &content) {
                        notes.push(note);
                    }
                }
            }
        }

        notes.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(notes)
    }

    pub fn find_note(&self, query: &str) -> Result<Note, String> {
        let clean_q = query.trim_end_matches(".md").to_lowercase();
        let notes = self.list_notes(None)?;

        // 1. Exact relative path match
        if let Some(n) = notes.iter().find(|n| {
            n.relative_path.to_lowercase() == clean_q || n.relative_path.to_lowercase() == format!("{}.md", clean_q)
        }) {
            return Ok(n.clone());
        }

        // 2. Exact filename match
        if let Some(n) = notes.iter().find(|n| {
            let stem = n.filename.replace(".md", "").to_lowercase();
            stem == clean_q
        }) {
            return Ok(n.clone());
        }

        // 3. Exact title match
        if let Some(n) = notes.iter().find(|n| {
            n.frontmatter.title.to_lowercase() == clean_q
        }) {
            return Ok(n.clone());
        }

        // 4. Alias match
        if let Some(n) = notes.iter().find(|n| {
            n.frontmatter.aliases.iter().any(|a| a.to_lowercase() == clean_q)
        }) {
            return Ok(n.clone());
        }

        // 5. Partial substring match on filename or title
        let matches: Vec<&Note> = notes
            .iter()
            .filter(|n| {
                n.filename.to_lowercase().contains(&clean_q)
                    || n.frontmatter.title.to_lowercase().contains(&clean_q)
            })
            .collect();

        if matches.len() == 1 {
            return Ok((*matches[0]).clone());
        } else if matches.len() > 1 {
            return Err(format!(
                "Multiple notes matched '{}'. Please provide a more specific path.",
                query
            ));
        }

        Err(format!("Note '{}' not found in {:?}", query, self.notes_dir))
    }

    pub fn search(&self, query: &str, tag_filter: Option<&str>) -> Result<Vec<SearchResult>, String> {
        let notes = self.list_notes(None)?;
        let q = query.to_lowercase();
        let tag_q = tag_filter.map(|t| t.to_lowercase());

        let mut results = Vec::new();

        for note in notes {
            if let Some(ref t) = tag_q {
                if !note.frontmatter.tags.iter().any(|tag| tag.to_lowercase() == *t) {
                    continue;
                }
            }

            let mut matches = Vec::new();

            for (idx, line) in note.content.lines().enumerate() {
                if line.to_lowercase().contains(&q) {
                    matches.push(SearchMatch {
                        line_number: idx + 1,
                        line: line.trim().to_string(),
                    });
                }
            }

            if !matches.is_empty() || note.frontmatter.title.to_lowercase().contains(&q) {
                results.push(SearchResult {
                    path: note.relative_path,
                    title: note.frontmatter.title,
                    tags: note.frontmatter.tags,
                    matches,
                });
            }
        }

        Ok(results)
    }

    pub fn get_link_graph(&self, note_query: &str) -> Result<LinkGraph, String> {
        let target_note = self.find_note(note_query)?;
        let target_stem = target_note.filename.replace(".md", "").to_lowercase();
        let target_title = target_note.frontmatter.title.to_lowercase();

        let all_notes = self.list_notes(None)?;
        let mut backlinks = Vec::new();

        // 1. Scan other notes for backlinks
        for other in &all_notes {
            if other.relative_path == target_note.relative_path {
                continue;
            }

            let has_ref = other.outgoing_links.iter().any(|link| {
                let l = link.to_lowercase();
                l == target_stem
                    || l == target_title
                    || target_note.frontmatter.aliases.iter().any(|a| a.to_lowercase() == l)
                    || target_note.relative_path.to_lowercase().contains(&l)
            });

            if has_ref {
                backlinks.push(other.relative_path.clone());
            }
        }

        // 2. Scan tasks directory for backlinks if tasks exist
        if let Some(ref tasks_dir) = self.tasks_dir {
            if let Ok(entries) = fs::read_dir(tasks_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let task_links = extract_wikilinks(&content);
                            let has_ref = task_links.iter().any(|link| {
                                let l = link.to_lowercase();
                                l == target_stem
                                    || l == target_title
                                    || target_note.frontmatter.aliases.iter().any(|a| a.to_lowercase() == l)
                            });

                            if has_ref {
                                let task_file = path.file_name().unwrap_or_default().to_string_lossy();
                                backlinks.push(format!("workspace/tasks/{}", task_file));
                            }
                        }
                    }
                }
            }
        }

        Ok(LinkGraph {
            note_path: target_note.relative_path,
            note_title: target_note.frontmatter.title,
            outgoing_links: target_note.outgoing_links,
            backlinks,
        })
    }

    pub fn save_note(
        &self,
        folder: Option<&str>,
        filename: &str,
        frontmatter: &NoteFrontmatter,
        body: &str,
    ) -> Result<Note, String> {
        let folder_path = match folder {
            Some(f) if !f.trim().is_empty() => self.notes_dir.join(f.trim_start_matches('/')),
            _ => self.notes_dir.join("00_Inbox"),
        };

        fs::create_dir_all(&folder_path)
            .map_err(|e| format!("Failed to create folder {:?}: {}", folder_path, e))?;

        let clean_filename = if filename.ends_with(".md") {
            filename.to_string()
        } else {
            format!("{}.md", filename.replace(' ', "_"))
        };

        let target_path = folder_path.join(&clean_filename);
        let full_text = serialize_note_file(frontmatter, body)?;

        // Atomic write via tempfile
        let mut temp_file = NamedTempFile::new_in(&folder_path)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        temp_file
            .write_all(full_text.as_bytes())
            .map_err(|e| format!("Failed to write to temporary file: {}", e))?;

        temp_file
            .persist(&target_path)
            .map_err(|e| format!("Failed to save note file {:?}: {}", target_path, e))?;

        let rel_path = target_path
            .strip_prefix(&self.notes_dir)
            .unwrap_or(&target_path)
            .to_string_lossy()
            .to_string();

        parse_note_file(&target_path, &rel_path, &full_text)
    }

    pub fn delete_note(&self, query: &str) -> Result<Note, String> {
        let note = self.find_note(query)?;
        fs::remove_file(&note.path)
            .map_err(|e| format!("Failed to delete note file {:?}: {}", note.path, e))?;
        Ok(note)
    }
}
