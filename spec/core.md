# Specification: Shared Core Engine (`tracker-core`)

## 1. Overview & Core Principles

The `tracker-core` crate (`crates/core`) is the shared foundational library for the entire Local Task Tracker and Orchestrator workspace. It encapsulates project context resolution, multi-project storage modes, Git remote fingerprinting with self-healing path recovery, and central registry management.

All present and future CLI and service modules (`task`, `note`, `runner`, `secrets`, etc.) depend on `tracker-core` for unified path and project discovery.

### Core Principles
- **Single Source of Truth for Paths:** No module resolves paths manually. Every module queries `WorkspaceContext::discover()`.
- **Zero Configuration for Humans & AI Agents:** Commands automatically infer the active project and target directories from the current working directory (CWD).
- **Git-Agnostic & Clean-Git Ready:** Supports both in-repo storage (committed to Git) and detached external storage (zero files committed to Git).
- **Self-Healing on Directory Moves:** If a project directory is moved or renamed, the engine automatically matches its Git remote origin and self-heals the registered path without user intervention.
- **Sub-Millisecond Resolution:** Completely in-memory path checking and filesystem metadata traversal executed in <1ms.

---

## 2. Storage Modes

The engine provides three distinct storage modes for projects:

```mermaid
graph TD
    subgraph Mode1 ["1. Workspace Mode (Default)"]
        M1_Root["LocalAgentToolchain/"]
        M1_Tasks["workspace/tasks/"]
        M1_Notes["workspace/notes/"]
        M1_Root --> M1_Tasks
        M1_Root --> M1_Notes
    end

    subgraph Mode2 ["2. Embedded Mode (.tasks/ in Git)"]
        M2_Root["~/projects/my-app/"]
        M2_Git[".git/"]
        M2_Tasks[".tasks/ (Committed to Git)"]
        M2_Notes[".notes/ (Committed to Git)"]
        M2_Root --> M2_Git
        M2_Root --> M2_Tasks
        M2_Root --> M2_Notes
    end

    subgraph Mode3 ["3. Detached Mode (100% Clean Git)"]
        M3_Root["~/work/client-repo/ (100% clean, 0 files added)"]
        M3_Vault["~/.local-tracker/vaults/client-repo/"]
        M3_Tasks["tasks/"]
        M3_Notes["notes/"]
        M3_Vault --> M3_Tasks
        M3_Vault --> M3_Notes
        M3_Root -.->|Mapped in registry| M3_Vault
    end
```

| Mode | Target Layout | Use Case |
| :--- | :--- | :--- |
| **`Workspace`** | `./workspace/tasks`, `./workspace/notes` | Standalone local tracker or centralized orchestrator repository. |
| **`Embedded`** | `./.tasks/`, `./.notes/` inside the project root | Team projects where tasks and specs should travel with the code and be versioned in Git. |
| **`Detached`** | `~/.local-tracker/vaults/<project>/` | Client projects, NDA repositories, open-source projects where the repository must remain 100% clean. |

---

## 3. Cascading Resolution Algorithm

When `WorkspaceContext::discover(explicit_path)` is executed, it runs the following priority cascade:

```mermaid
flowchart TD
    Start["Call WorkspaceContext::discover(explicit_path)"] --> CheckExplicit{"Explicit path supplied?"}
    CheckExplicit -- YES --> UseExplicit["Use explicit directory<br><b>Mode: Workspace</b>"]
    CheckExplicit -- NO --> CheckRegistry{"Path matches registered project in<br>~/.local-tracker/projects.toml ?"}
    
    CheckRegistry -- YES --> MatchReg["Load registered project<br><i>(Longest matching path wins)</i><br><b>Mode: Embedded or Detached</b>"]
    CheckRegistry -- NO --> WalkUp{"Parent directory contains<br>workspace/tasks or .tasks/ ?"}
    
    WalkUp -- YES (workspace/tasks) --> FoundWS["Found workspace/tasks<br><b>Mode: Workspace</b>"]
    WalkUp -- YES (.tasks) --> FoundDot["Found .tasks/<br><b>Mode: Embedded</b>"]
    WalkUp -- NO --> CheckGit{"Inside a Git repository?"}
    
    CheckGit -- YES --> MatchGit{"Git remote or .git/tracker_id<br>matches a registered project?"}
    MatchGit -- YES --> SelfHeal["Auto-update project path in registry<br><b>Mode: Detached (Self-Healed)</b>"]
    MatchGit -- NO --> Fallback["Default to ./workspace in current directory<br><b>Mode: Workspace</b>"]
    CheckGit -- NO --> Fallback
```

---

## 4. Central Registry Specification

### Registry File: `~/.local-tracker/projects.toml`

The registry tracks all known projects on the machine. By default, it resides at `~/.local-tracker/projects.toml` (configurable via `TRACKER_HOME` environment variable):

```toml
[projects.local_agent_toolchain]
name = "local_agent_toolchain"
path = "/Users/developer/workspace/LocalAgentToolchain"
storage = "/Users/developer/workspace/LocalAgentToolchain"
git_remote = "github.com/developer/LocalAgentToolchain"
updated_at = "2026-09-03T00:00:00+00:00"

[projects.client_api]
name = "client_api"
path = "/Users/developer/work/client-api"
storage = "/Users/developer/.local-tracker/vaults/client_api"
git_remote = "github.com/client/client-api"
updated_at = "2026-09-03T00:50:00+00:00"
```

### Schema

| Field | Type | Description |
| :--- | :--- | :--- |
| `name` | `String` | Project unique key and identifier. |
| `path` | `PathBuf` | Absolute path to project root on the local filesystem. |
| `storage` | `PathBuf` | Absolute path to directory storing `tasks/`, `notes/`, `runs/`. |
| `git_remote` | `Option[String]` | Normalized Git remote URL (`github.com/owner/repo`). |
| `updated_at` | `Option[String]` | ISO 8601 timestamp of last registration or self-healing. |

---

## 5. Public Rust API (`tracker_core`)

Any crate in the Cargo Workspace adds `tracker-core = { path = "../core" }` to `Cargo.toml`.

### 1. `WorkspaceContext`

```rust
use tracker_core::{WorkspaceContext, StorageMode};

// Discover active context based on CWD
let ctx = WorkspaceContext::discover(None)?;

println!("Project Name: {}", ctx.project_name);
println!("Storage Mode: {}", ctx.mode);
println!("Tasks Directory: {:?}", ctx.tasks_dir);
println!("Notes Directory: {:?}", ctx.notes_dir);
println!("Runs Directory: {:?}", ctx.runs_dir);
```

### 2. Project Initialization (`init_project`)

```rust
use tracker_core::WorkspaceContext;

// Initialize embedded project (.tasks/ in current folder):
let embedded_ctx = WorkspaceContext::init_project(&current_dir, Some("my-app"), false)?;

// Initialize detached project (external vault, clean git):
let detached_ctx = WorkspaceContext::init_project(&current_dir, Some("client-app"), true)?;
```

### 3. Registry Operations (`Registry`)

```rust
use tracker_core::Registry;

// List all registered projects across the machine:
let projects = Registry::list();

// Register or update project:
Registry::register("proj_name", &project_path, &storage_path, Some(git_remote_url))?;

// Find project by current directory:
if let Some(project) = Registry::find_by_path(&current_dir) {
    println!("Found project: {}", project.name);
}
```

---

## 6. CLI Commands & Agent JSON Protocol

The core functionality is exposed through CLI commands in modules:

### 1. `task info` (Inspect Active Context)
```bash
task info
task info --json
```

Agent JSON Output:
```json
{
  "project": "client_api",
  "mode": "detached",
  "root_path": "/Users/developer/work/client-api",
  "tasks_dir": "/Users/developer/.local-tracker/vaults/client_api/tasks",
  "notes_dir": "/Users/developer/.local-tracker/vaults/client_api/notes"
}
```

### 2. `task init` (Initialize Project)
```bash
# Embedded (default):
task init

# Detached (external vault, clean git):
task init --detached --name my-client-project
```

### 3. `task projects` (List Registered Projects)
```bash
task projects
task projects --json
```

Agent JSON Output:
```json
[
  {
    "name": "client_api",
    "path": "/Users/developer/work/client-api",
    "storage": "/Users/developer/.local-tracker/vaults/client_api",
    "git_remote": "github.com/client/client-api",
    "updated_at": "2026-09-03T00:50:00+00:00"
  }
]
```
