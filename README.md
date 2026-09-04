# ⚡ Local Agent Toolchain (LAT)

> A high-performance, Unix-philosophy workstation toolchain for developers and autonomous AI coding agents, built in pure Rust.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Startup Time](https://img.shields.io/badge/startup_%E2%8F%B1%EF%B8%8F-%3C10ms-brightgreen.svg)]()
[![Multi--Agent](https://img.shields.io/badge/agents-Claude%20%7C%20Codex%20%7C%20AGY%20%7C%20Pi%20%7C%20Junie-purple.svg)]()

---

## 💡 Philosophy

Modern AI coding agents (Claude Code, OpenAI Codex, Antigravity, Pi, Junie) need structured state, verifiable checklists, external project memory, and reusable procedural skills. 

Traditional agent frameworks attempt this with heavy Python runtimes, bloated vector databases, and opaque abstractions. **Local Agent Toolchain** takes a different approach based on the **Unix Philosophy**:

- **Lightning-Fast (<10ms):** Compiled native Rust binaries that return instant machine-readable JSON or beautiful ANSI terminal outputs.
- **LLM-Native Plain Markdown:** Tasks, notes, and skills are stored as human-readable Markdown files with YAML frontmatter. Zero binary lock-in. Git-friendly.
- **Clean-Git Ready (Detached Vaults):** Supports both in-repo storage (`.tasks/`) and completely isolated personal vaults (`~/.local-tracker/vaults/`) leaving client or open-source repositories with **zero modified files**.
- **Self-Healing on Directory Moves:** Uses Git remote fingerprints to automatically heal and reconnect projects when folders are renamed or moved.
- **Cross-Agent Interoperability:** Author a skill or procedural runbook once, and deploy or symlink it across all 5 major AI coding agents.

---

## 🏗️ Architecture

```text
               ┌─────────────────────────────────────────────────────────┐
               │              AI CODING AGENTS                           │
               │   (Claude Code, OpenAI Codex, Antigravity, Pi, Junie)   │
               └──────────▲──────────────────────────────────▲───────────┘
                          │                                  │
    ┌─────────────────────┴──────────────┐    ┌──────────────┴─────────────────────┐
    │  🤹 PROTOCOL & SKILL LAYER (`skill`)│    │  ⚙️ EXECUTION ENGINE (`runner`)     │
    │  - Cross-agent skill deployment    │    │  - Process supervisor & sandbox    │
    │  - Native directory auto-discovery │    │  - Timeout watchdog & PID tracking │
    │  - Progressive disclosure & SOP    │    │  - Stdout/stderr log streaming     │
    └─────────────────────▲──────────────┘    └──────────────▲─────────────────────┘
                          │                                  │
    ┌─────────────────────┴──────────────┐    ┌──────────────┴─────────────────────┐
    │  📋 STATE & OBJECTIVES (`task`)     │    │  🧠 CONTEXT & MEMORY (`note`)       │
    │  - Deterministic state machine     │    │  - Long-term memory & wiki         │
    │  - Checklist step verification     │    │  - Bidirectional Wikilinks [[...]] │
    │  - Timestamped execution audit log │    │  - Architecture specs & reports    │
    └─────────────────────▲──────────────┘    └──────────────▲─────────────────────┘
                          │                                  │
                          └──────────────────┬───────────────┘
                                             │
                               ┌─────────────┴──────────────┐
                               │  🧠 ORCHESTRATION (`core`)  │
                               │  - Embedded vs Detached    │
                               │  - Self-healing Git remote │
                               │  - Projects registry       │
                               └────────────────────────────┘
```

---

## 📦 Toolchain Suite

| Module | Binary | Role | Description |
| :--- | :--- | :--- | :--- |
| **`core`** | `tracker-core` | Shared Foundation | Workspace discovery, self-healing Git remote resolution, and central registry (`~/.local-tracker/projects.toml`). |
| **`task`** | `task` | State & Objectives | Deterministic task state machine (`todo`, `in_progress`, `done`), interactive Kanban board, checklist verification, and timestamped logs. |
| **`notes`** | `note` | Context & Memory | Project knowledge base, Obsidian-style bidirectional Wikilinks (`[[...]]`), automatic backlinks, and fast full-text search. |
| **`skill`** | `skill` | Capability & Protocol | AI Agent Skill Manager. Inspects, audits, packages, symlinks, and removes skills across Claude, Codex, Antigravity, Pi, and Junie. |
| **`runner`** | `run` *(Next)* | Process Supervisor | Execution engine, sandboxed background agent runner, log streaming, and timeout watchdog. |

---

## 📥 Installation

### Method 1: Homebrew (macOS & Linux)
Install directly via Homebrew:

```bash
brew install criggil/LocalAgentToolchain/local-agent-toolchain
```

To update to the latest release at any time:
```bash
brew upgrade local-agent-toolchain
```

---

### Method 2: One-Line Installer (macOS & Linux)
Install or update the latest pre-compiled release without Homebrew (Apple Silicon M1/M2/M3/M4, Intel, or Linux):

```bash
curl -fsSL https://raw.githubusercontent.com/criggil/LocalAgentToolchain/main/install.sh | bash
```

This installs `task`, `note`, and `skill` into `~/.local/bin/`.

Ensure `~/.local/bin` is in your `$PATH` (add to your `~/.zshrc` or `~/.bashrc` if needed):
```bash
export PATH="$HOME/.local/bin:$PATH"
```

---

### Method 3: Build from Source
If you have the Rust toolchain installed:

```bash
git clone https://github.com/criggil/LocalAgentToolchain.git
cd LocalAgentToolchain
cargo build --release
```
Compiled binaries will be available in `./target/release/` (`task`, `note`, `skill`).

---

## 🚀 Quick Start

### 1. Task Manager (`task`)

```bash
# Initialize task tracking (embedded in repo):
task init

# Initialize in detached mode (100% clean Git, external vault):
task init --detached --name my-client-project

# Add a task with checklist:
task add "Implement JWT authentication" -p critical -l auth,security -c "Design schema,Add middleware,Write tests"

# View interactive terminal Kanban board:
task list --board

# Machine-readable JSON output for AI agents:
task list --json

# Start working and check off steps:
task start task-001
task check task-001 1
task log task-001 "Completed schema design and test coverage"
task done task-001
```

---

### 2. Knowledge Base & Wiki (`note`)

```bash
# Create an architecture note with Wikilinks:
note new "Auth Architecture" -f 20_Wiki -t auth,security -c "See [[Database Schema]] and relates to [[task-001]]."

# Search across all notes and tasks:
note search "JWT"

# Inspect outgoing links and automatic incoming backlinks:
note links "Auth Architecture"

# Machine-readable raw output:
note show "Auth Architecture" --raw
```

---

### 3. AI Agent Skill Manager (`skill`)

```bash
# List workspace skills:
skill list

# Deep-scan all installed skills across agents on the machine:
skill list --installed

# Deploy a skill to Claude Code (creates slash command in ~/.claude/commands/):
skill install task-manager --target claude --global

# Deploy to Google Antigravity locally (creates live symlink in .agents/skills/):
skill install task-manager --target antigravity --local

# Deploy to all detected agents at once:
skill install task-manager

# Safely remove or unlink an installed skill:
skill remove task-manager --target codex --local -y

# Scaffold a new skill package:
skill new db-migrator --desc "Safe PostgreSQL migration procedures"
```

---

## 🤖 Supported AI Coding Agents

| Agent | Project / Local Discovery | User / Global Discovery | Native Format |
| :--- | :--- | :--- | :--- |
| **Claude Code** (Anthropic) | `./CLAUDE.md`, `./.claude/commands/` | `~/.claude/CLAUDE.md`, `~/.claude/commands/` | Slash commands / Prompts |
| **OpenAI Codex CLI** | `./AGENTS.md`, `./.codex/skills/` | `~/.codex/skills/` | `SKILL.md` (YAML frontmatter) |
| **Google Antigravity** | `./AGENTS.md`, `./.agents/skills/` | `~/.gemini/config/skills/` | `SKILL.md` (Progressive disclosure) |
| **Pi Coding Agent** | `./AGENTS.md`, `./.pi/skills/` | `~/.pi/agent/skills/` | Modular skill packages |
| **JetBrains Junie CLI** | `./AGENTS.md`, `./.junie/skills/` | `~/.junie/skills/` | `Skill.md` (SOP procedures) |

---

## 📁 Repository Layout

```text
LocalAgentToolchain/
├── Cargo.toml                  # Cargo Workspace configuration
├── crates/
│   ├── core/                   # tracker-core (shared context & self-healing)
│   ├── task/                   # task CLI (tasks, kanban, checklists)
│   ├── notes/                  # note CLI (wiki, backlinks, search)
│   └── skill/                  # skill CLI (multi-agent skill manager)
├── workspace/                  # Project data storage (Workspace Mode)
│   ├── tasks/                  # Task markdown files (*.md)
│   ├── notes/                  # Notes by category (00_Inbox, 10_Projects, 20_Wiki...)
│   └── skills/                 # Universal skill packages (task-manager, knowledge-base...)
└── spec/                       # Official architectural specifications
    ├── core.md                 # Workspace resolution & registry spec
    ├── task_manager.md         # Task CLI spec
    ├── knowledge_base.md       # Notes CLI spec
    └── skill_manager.md        # Skill Manager spec
```

---

## 📜 Specifications
- [Shared Core Engine Spec](spec/core.md)
- [Task Manager Spec](spec/task_manager.md)
- [Knowledge Base Spec](spec/knowledge_base.md)
- [AI Agent Skill Manager Spec](spec/skill_manager.md)

---

## 📄 License
MIT License. Free and open source for human developers and autonomous agents alike.
