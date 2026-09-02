use std::fs;
use std::path::{Path, PathBuf};

pub fn find_git_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

pub fn get_git_remote_url(git_root: &Path) -> Option<String> {
    let config_path = git_root.join(".git").join("config");
    if !config_path.is_file() {
        return None;
    }

    let content = fs::read_to_string(config_path).ok()?;
    let mut in_origin_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("[remote \"origin\"]") {
            in_origin_section = true;
            continue;
        } else if trimmed.starts_with('[') {
            in_origin_section = false;
        }

        if in_origin_section && trimmed.starts_with("url =") {
            let url = trimmed.trim_start_matches("url =").trim().to_string();
            // Normalize: remove trailing .git and convert ssh to standard slug
            let clean_url = url.trim_end_matches(".git").to_string();
            return Some(clean_url);
        }
    }

    None
}

pub fn get_git_tracker_id(git_root: &Path) -> Option<String> {
    let id_file = git_root.join(".git").join("tracker_id");
    if id_file.is_file() {
        fs::read_to_string(id_file).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

pub fn set_git_tracker_id(git_root: &Path, id: &str) -> std::io::Result<()> {
    let git_dir = git_root.join(".git");
    if git_dir.is_dir() {
        let id_file = git_dir.join("tracker_id");
        fs::write(id_file, id.trim())?;
    }
    Ok(())
}
