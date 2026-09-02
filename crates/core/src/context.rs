use std::fs;
use std::path::{Path, PathBuf};
use crate::git::{find_git_root, get_git_remote_url, get_git_tracker_id, set_git_tracker_id};
use crate::models::StorageMode;
use crate::registry::Registry;

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    pub project_name: String,
    pub root_path: PathBuf,
    pub tasks_dir: PathBuf,
    pub notes_dir: PathBuf,
    pub runs_dir: PathBuf,
    pub mode: StorageMode,
}

impl WorkspaceContext {
    pub fn discover(explicit_path: Option<&str>) -> Result<Self, String> {
        // 1. Explicit path override
        if let Some(p) = explicit_path {
            let path = PathBuf::from(p);
            let root = if path.ends_with("tasks") || path.ends_with("notes") {
                path.parent().unwrap_or(&path).to_path_buf()
            } else {
                path
            };

            let tasks_dir = root.join("tasks");
            let notes_dir = root.join("notes");
            let runs_dir = root.join("runs");

            fs::create_dir_all(&tasks_dir)
                .map_err(|e| format!("Failed to create tasks dir {:?}: {}", tasks_dir, e))?;
            fs::create_dir_all(&notes_dir).ok();
            fs::create_dir_all(&runs_dir).ok();

            let name = root.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "custom".to_string());

            return Ok(Self {
                project_name: name,
                root_path: root,
                tasks_dir,
                notes_dir,
                runs_dir,
                mode: StorageMode::Workspace,
            });
        }

        let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        // 2. Check registered projects (Registry match wins)
        if let Some(entry) = Registry::find_by_path(&current) {
            let mode = if entry.storage == entry.path {
                StorageMode::Embedded
            } else {
                StorageMode::Detached
            };
            return Ok(Self::from_storage(&entry.name, &entry.path, &entry.storage, mode));
        }

        // 3. Check local directories (walk-up)
        let mut dir = current.clone();
        loop {
            // Check workspace/tasks (standard project layout)
            let ws_tasks = dir.join("workspace").join("tasks");
            if ws_tasks.is_dir() {
                let ws = dir.join("workspace");
                let name = dir.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "workspace".to_string());

                return Ok(Self {
                    project_name: name,
                    root_path: dir,
                    tasks_dir: ws_tasks,
                    notes_dir: ws.join("notes"),
                    runs_dir: ws.join("runs"),
                    mode: StorageMode::Workspace,
                });
            }

            // Check embedded .tasks/ (in-repo embedded layout)
            let dot_tasks = dir.join(".tasks");
            if dot_tasks.is_dir() {
                let name = dir.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "project".to_string());

                return Ok(Self {
                    project_name: name,
                    root_path: dir.clone(),
                    tasks_dir: dot_tasks,
                    notes_dir: dir.join(".notes"),
                    runs_dir: dir.join(".runs"),
                    mode: StorageMode::Embedded,
                });
            }

            if !dir.pop() {
                break;
            }
        }

        // 3. Check Git Repository & Self-Healing Registry Lookup
        if let Some(git_root) = find_git_root(&current) {
            // Check .git/tracker_id
            if let Some(tracker_id) = get_git_tracker_id(&git_root) {
                let config = Registry::load();
                if let Some(entry) = config.projects.get(&tracker_id) {
                    if entry.path != git_root {
                        Registry::update_path(&tracker_id, &git_root).ok();
                    }
                    return Ok(Self::from_storage(&entry.name, &git_root, &entry.storage, StorageMode::Detached));
                }
            }

            // Check git remote URL
            if let Some(remote) = get_git_remote_url(&git_root) {
                if let Some(entry) = Registry::find_by_git_remote(&remote) {
                    if entry.path != git_root {
                        Registry::update_path(&entry.name, &git_root).ok();
                    }
                    return Ok(Self::from_storage(&entry.name, &git_root, &entry.storage, StorageMode::Detached));
                }
            }
        }

        // 4. Check registry path match
        if let Some(entry) = Registry::find_by_path(&current) {
            return Ok(Self::from_storage(&entry.name, &entry.path, &entry.storage, StorageMode::Detached));
        }

        // 5. Fallback: Default to ./workspace in current directory
        let fallback_ws = current.join("workspace");
        let tasks_dir = fallback_ws.join("tasks");
        let notes_dir = fallback_ws.join("notes");
        let runs_dir = fallback_ws.join("runs");

        fs::create_dir_all(&tasks_dir).ok();
        fs::create_dir_all(&notes_dir).ok();
        fs::create_dir_all(&runs_dir).ok();

        let name = current.file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "default".to_string());

        Ok(Self {
            project_name: name,
            root_path: current,
            tasks_dir,
            notes_dir,
            runs_dir,
            mode: StorageMode::Workspace,
        })
    }

    fn from_storage(project_name: &str, root_path: &Path, storage_path: &Path, mode: StorageMode) -> Self {
        let tasks_dir = storage_path.join("tasks");
        let notes_dir = storage_path.join("notes");
        let runs_dir = storage_path.join("runs");

        fs::create_dir_all(&tasks_dir).ok();
        fs::create_dir_all(&notes_dir).ok();
        fs::create_dir_all(&runs_dir).ok();

        Self {
            project_name: project_name.to_string(),
            root_path: root_path.to_path_buf(),
            tasks_dir,
            notes_dir,
            runs_dir,
            mode,
        }
    }

    pub fn init_project(target_dir: &Path, name: Option<&str>, detached: bool) -> Result<Self, String> {
        let proj_name = name
            .map(|s| s.to_string())
            .or_else(|| target_dir.file_name().map(|s| s.to_string_lossy().to_string()))
            .unwrap_or_else(|| "project".to_string());

        let git_root = find_git_root(target_dir);
        let git_remote = git_root.as_ref().and_then(|r| get_git_remote_url(r));

        if detached {
            let vault_dir = Registry::vaults_dir().join(&proj_name);
            fs::create_dir_all(vault_dir.join("tasks"))
                .map_err(|e| format!("Failed to create vault directory: {}", e))?;
            fs::create_dir_all(vault_dir.join("notes")).ok();
            fs::create_dir_all(vault_dir.join("runs")).ok();

            Registry::register(&proj_name, target_dir, &vault_dir, git_remote.clone())?;

            if let Some(ref gr) = git_root {
                set_git_tracker_id(gr, &proj_name).ok();
            }

            Ok(Self::from_storage(&proj_name, target_dir, &vault_dir, StorageMode::Detached))
        } else {
            let tasks_dir = target_dir.join(".tasks");
            let notes_dir = target_dir.join(".notes");
            let runs_dir = target_dir.join(".runs");

            fs::create_dir_all(&tasks_dir)
                .map_err(|e| format!("Failed to create .tasks directory: {}", e))?;
            fs::create_dir_all(&notes_dir).ok();
            fs::create_dir_all(&runs_dir).ok();

            Registry::register(&proj_name, target_dir, target_dir, git_remote)?;

            Ok(Self {
                project_name: proj_name,
                root_path: target_dir.to_path_buf(),
                tasks_dir,
                notes_dir,
                runs_dir,
                mode: StorageMode::Embedded,
            })
        }
    }
}
