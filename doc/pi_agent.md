# Pi Coding Agent Extensibility Guide

Pi is a minimalist, terminal-native AI coding harness designed for maximum hackability and modularity. Rather than packing everything into the core runtime, Pi keeps its core minimal and relies on **Extensions**, **Skills**, and **System Prompt Hierarchies**.

---

## 1. Minimalist Philosophy

Pi's core runtime ships with only four foundational tools:
* `read` — Read file contents.
* `write` — Create or overwrite files.
* `edit` — In-place line replacements.
* `bash` — Shell execution.

Everything else (subagents, browser automation, task tracking, custom approval gates) is implemented as **Extensions** and **Skills**.

---

## 2. Configuration Hierarchy: `SYSTEM.md` & `AGENTS.md`

Pi controls its behavior through hierarchical context files:

### `AGENTS.md` (Project Guidelines)
* Contains project-specific conventions, architecture documentation, and test commands.
* **Traversal:** Pi checks the current directory, walks up to the repository root, and falls back to global `~/.pi/agent/AGENTS.md`.
* **Override Support:** If `AGENTS.override.md` exists in a folder, it takes precedence over standard `AGENTS.md`.

### `SYSTEM.md` (System Prompt Replacement & Appends)
* `.pi/SYSTEM.md` — Completely replaces the agent's default system prompt.
* `.pi/APPEND_SYSTEM.md` — Appends custom rules to the existing system prompt without replacing it.
* `~/.pi/agent/SYSTEM.md` — Global system prompt customization.

---

## 3. Skills (`.pi/skills/<name>/SKILL.md`)

Skills in Pi are procedural guides that teach the model *when* and *how* to use its basic tools to accomplish complex goals.

### Locations
* **Local:** `./.pi/skills/<name>/SKILL.md`
* **Global:** `~/.pi/agent/skills/<name>/SKILL.md`

### Usage
Pi indexes installed skills. When a task matches the skill's triggers, the agent loads the skill into its working memory.

---

## 4. Extensions & Packages (NPM & Git)

To add executable capabilities to the Pi runtime:
* Extensions can add new tools to the toolchain.
* Extensions can register event gates on tool calls (e.g. asking confirmation before running `rm -rf` or modifying protected files).
* Users install extensions via package managers or Git submodules into `~/.pi/extensions/`.
