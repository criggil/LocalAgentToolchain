# Specification: Task Manager CLI (`task`)

## 1. Overview & Core Principles

The Task Manager CLI is an autonomous, high-performance command-line tool built in Rust (`crates/task`). It manages tasks stored as plain Markdown files (`.md`) with YAML frontmatter.

### Core Principles
- **100% Plain Markdown Storage:** All tasks reside in `workspace/tasks/<id>.md`. Zero proprietary databases or locks.
- **Zero Runtime Dependencies:** Standalone compiled binary (`task`) with no external runtime requirements (no Node.js, Python, or JVM).
- **Sub-5ms Execution Time:** Fast CLI startup suitable for high-frequency execution in CI and AI agent loops.
- **Dual Output Interface:**
  - **Human Mode (default):** ANSI colors, formatted Unicode tables (`tabled`), and terminal Kanban board.
  - **Agent Mode (`--json`):** Strict, machine-readable JSON on `stdout` with clean `stderr` for LLM agents (Claude Code, Cursor, Aider).
- **Atomic Operations:** All file writes use temporary files (`tempfile`) and atomic renames to prevent partial write corruption.
- **Non-Interactive by Default:** Commands execute without blocking user prompts unless explicitly run in interactive mode.

---

## 2. File Format & Schema

Each task is saved as a distinct `.md` file in `workspace/tasks/<id>.md`:

```markdown
---
id: task-001
title: Design authorization schema
status: in_progress     # todo | in_progress | review | done | blocked
priority: high          # low | medium | high | critical
labels:
  - backend
  - auth
  - security
assignee: agent:coder
created_at: 2026-09-02T10:00:00Z
updated_at: 2026-09-02T12:30:00Z
due_date: null
---

## Description
Implement token-based authentication using Ed25519 signatures.

## Checklist
- [x] Select cryptography library
- [ ] Implement token verification middleware
- [ ] Add integration tests

## Logs
- [2026-09-02 12:00] Claude Code completed algorithm benchmarking.
```

### Frontmatter Fields

| Field | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `id` | `String` | Yes | Unique task identifier (e.g., `task-001`). |
| `title` | `String` | Yes | Brief task title. |
| `status` | `Enum` | Yes | `todo`, `in_progress`, `review`, `done`, `blocked`. |
| `priority` | `Enum` | Yes | `low`, `medium`, `high`, `critical`. |
| `labels` | `List[String]` | No | Categorization tags. |
| `assignee` | `Option[String]` | No | Person or agent identifier (`agent:coder`, `user:john`). |
| `created_at` | `String` | Yes | ISO 8601 creation timestamp. |
| `updated_at` | `Option[String]` | No | ISO 8601 update timestamp. |
| `due_date` | `Option[String]` | No | Optional deadline. |

---

## 3. CLI Commands

### Global Options

```text
--json                   Machine-readable JSON output
--workspace <PATH>       Custom path to tasks directory (auto-discovered if omitted)
-q, --quiet              Suppress informational output
-h, --help               Print help information
-V, --version            Print version information
```

### Command Reference

#### 1. `task list` (alias: `ls`)
List tasks with optional filters.

```bash
task list [OPTIONS]
```
- `-s, --status <STATUS>`: Filter by status (`todo`, `in_progress`, `review`, `done`, `blocked`).
- `-p, --priority <PRIORITY>`: Filter by priority (`low`, `medium`, `high`, `critical`).
- `-l, --label <LABEL>`: Filter by label string match.
- `--board`: Display visual terminal Kanban board.
- `--json`: Output full array of task summaries in JSON.

#### 2. `task add`
Create a new `.md` task file.

```bash
task add "<TITLE>" [OPTIONS]
```
- `-s, --status <STATUS>`: Initial status (default: `todo`).
- `-p, --priority <PRIORITY>`: Priority (default: `medium`).
- `-l, --labels <L1,L2>`: Comma-separated labels.
- `-d, --desc "<TEXT>"`: Markdown description body.
- `--check "<STEP>"`: Add checklist item (repeatable).

#### 3. `task show <ID>`
Inspect full task details.

```bash
task show <ID> [--raw] [--json]
```
- `--raw`: Print raw markdown file contents without formatting.
- `--json`: Emit full task object with parsed checklists.

#### 4. Status Transition Commands
Move tasks across lifecycle states:

```bash
task move <ID> <STATUS>   # Set explicit status
task start <ID>            # Shorthand for in_progress
task review <ID>           # Shorthand for review
task done <ID>             # Shorthand for done
```

#### 5. `task check <ID> <INDEX>`
Toggle checklist items in the Markdown body.

```bash
task check <ID> <INDEX> [--uncheck]
```
- `<INDEX>`: 1-based index of the target checklist item.
- `-u, --uncheck`: Mark as `- [ ]` instead of `- [x]`.

#### 6. `task log <ID> "<MESSAGE>"`
Append a timestamped event log entry under `## Logs`.

```bash
task log <ID> "Agent finished running test suite."
```

#### 7. `task edit <ID>`
Open task markdown file in `$EDITOR` (or `vim`/`nano`).

```bash
task edit <ID>
```

#### 8. `task delete <ID>` (alias: `rm`)
Delete task file.

```bash
task delete <ID> -y
```
- `-y, --yes`: Bypass confirmation in scripts or agents.

---

## 4. Agent Integration Protocol

### Exit Codes
- `0`: Success.
- `1`: Validation error, task not found, or file I/O error (details in `stderr`).
- `2`: Invalid CLI syntax or arguments.

### JSON Output Schema

`task list --json`:
```json
[
  {
    "id": "task-001",
    "title": "Design authorization schema",
    "status": "in_progress",
    "priority": "high",
    "labels": ["backend", "auth"],
    "created_at": "2026-09-02T10:00:00Z",
    "checklist_total": 3,
    "checklist_done": 1,
    "file_path": "/path/to/workspace/tasks/task-001.md"
  }
]
```

`task show <ID> --json`:
```json
{
  "id": "task-001",
  "title": "Design authorization schema",
  "status": "in_progress",
  "priority": "high",
  "labels": ["backend", "auth"],
  "assignee": "agent:coder",
  "created_at": "2026-09-02T10:00:00Z",
  "updated_at": "2026-09-02T12:30:00Z",
  "due_date": null,
  "filename": "task-001.md",
  "path": "/path/to/workspace/tasks/task-001.md",
  "content": "## Description\nImplement token-based authentication...",
  "checklists": [
    { "index": 1, "text": "Select cryptography library", "completed": true },
    { "index": 2, "text": "Implement token verification middleware", "completed": false },
    { "index": 3, "text": "Add integration tests", "completed": false }
  ]
}
```
