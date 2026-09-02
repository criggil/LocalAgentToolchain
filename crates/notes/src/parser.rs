use std::path::Path;
use regex::Regex;
use crate::models::{Note, NoteFrontmatter};

pub fn parse_note_file(path: &Path, relative_path: &str, raw_content: &str) -> Result<Note, String> {
    let filename = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "note.md".to_string());

    if !raw_content.starts_with("---") {
        // Fallback for notes without frontmatter
        let title = filename.replace(".md", "").replace('_', " ");
        let fm = NoteFrontmatter {
            title,
            tags: vec![],
            aliases: vec![],
            created_at: None,
            updated_at: None,
        };
        let outgoing_links = extract_wikilinks(raw_content);
        return Ok(Note {
            frontmatter: fm,
            filename,
            relative_path: relative_path.to_string(),
            path: path.to_path_buf(),
            content: raw_content.to_string(),
            outgoing_links,
        });
    }

    let parts: Vec<&str> = raw_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(format!("File {} has corrupted YAML frontmatter", filename));
    }

    let yaml_str = parts[1];
    let body = parts[2].trim_start_matches("\r\n").trim_start_matches('\n');

    let frontmatter: NoteFrontmatter = match serde_yaml::from_str(yaml_str) {
        Ok(fm) => fm,
        Err(_) => {
            let title = filename.replace(".md", "").replace('_', " ");
            NoteFrontmatter {
                title,
                tags: vec![],
                aliases: vec![],
                created_at: None,
                updated_at: None,
            }
        }
    };

    let outgoing_links = extract_wikilinks(body);

    Ok(Note {
        frontmatter,
        filename,
        relative_path: relative_path.to_string(),
        path: path.to_path_buf(),
        content: body.to_string(),
        outgoing_links,
    })
}

pub fn extract_wikilinks(content: &str) -> Vec<String> {
    let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let mut links = Vec::new();

    for cap in re.captures_iter(content) {
        if let Some(matched) = cap.get(1) {
            let raw_link = matched.as_str().trim();
            // Handle aliases [[Target|Alias]]
            let target_no_alias = raw_link.split('|').next().unwrap_or(raw_link).trim();
            // Handle headers [[Target#Header]]
            let target = target_no_alias.split('#').next().unwrap_or(target_no_alias).trim();
            if !target.is_empty() && !links.contains(&target.to_string()) {
                links.push(target.to_string());
            }
        }
    }

    links
}

pub fn append_to_note(content: &str, text: &str) -> String {
    format!("{}\n\n{}", content.trim_end(), text.trim())
}

pub fn serialize_note_file(frontmatter: &NoteFrontmatter, body: &str) -> Result<String, String> {
    let yaml_str = serde_yaml::to_string(frontmatter)
        .map_err(|e| format!("YAML serialization error: {}", e))?;

    Ok(format!("---\n{}---\n\n{}", yaml_str, body.trim()))
}
