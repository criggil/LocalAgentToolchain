# Google Antigravity (AGY) Customization Guide (Google DeepMind)

Google Antigravity features a multi-tiered customization system designed for progressive disclosure, hierarchical rule discovery, and multi-agent coordination.

---

## 1. Customization Types Reference

| Type | Config File / Folder | Scope | Primary Purpose |
| :--- | :--- | :--- | :--- |
| **Rules** | `GEMINI.md`, `AGENTS.md`, `.agents/rules/*.md` | Hierarchical | Coding styles, architectural constraints, strict boundaries. |
| **Skills** | `.agents/skills/<name>/SKILL.md` | Progressive Disclosure | Procedural workflows, SOPs, helper scripts. |
| **Plugins** | `plugins/<name>/plugin.json` | Bundle | Packages related skills, rules, and MCP configs into a single unit. |
| **Hooks** | `hooks.json` | Lifecycle Event | Runs scripts before/after tool calls or agent turns. |
| **MCP Servers**| `mcp_config.json` | Tool Integration | Connects external APIs and tool providers via MCP. |
| **Subagents** | `define_subagent`, `invoke_subagent` | Process / Context Isolation | Spawns isolated specialist agents (e.g. Research, Refactor). |

---

## 2. Discovery Locations & Traversal

Antigravity traverses directories in the following order:

1. **Workspace Project Root:**
   * `.agents/` (or `.agent/`, `_agents/`, `_agent/`) at repository root.
   * Checked into Git to share with the team.
2. **Directory & Project Rules (Hierarchical):**
   * `GEMINI.md`, `AGENTS.md`, `.agents/rules/*.md`.
   * As files are accessed, Antigravity walks up from the file's folder to repository root, loading all rules.
3. **Global Configuration:**
   * `~/.gemini/config/` (machine-wide settings, global skills).
4. **Built-in Bundles:**
   * Mounted directly by the application runtime.

---

## 3. Loading Priority & Precedence

When multiple customizations conflict (e.g. two skills with the same name), Antigravity resolves them by strict priority:

1. **Workspace Project Customizations** (Highest — overrides all).
2. **Declared Configurations** (`skills.json`, `plugins.json`).
3. **Global Machine Discovery** (`~/.gemini/config/`).
4. **Built-in Bundled Customizations**.
5. **Global Declared Configurations** (Lowest).

---

## 4. Progressive Disclosure

To preserve the context window:
* **Skills:** Only `name` and `description` are initially mounted in the agent's system prompt. The full content of `SKILL.md` is loaded on-demand only when the agent or user triggers the skill.
* **Rules:** Rules with `trigger: model_decision` are loaded dynamically. Only `always_on` rules are loaded unconditionally.
* **Deduplication:** Rules are deduplicated by resolved canonical file paths, preventing duplicate token spend.

---

## 5. Lifecycle Hooks (`hooks.json`)

Antigravity allows executing deterministic scripts at specific lifecycle checkpoints:
```json
{
  "hooks": [
    {
      "event": "pre_tool_execution",
      "tool": "run_command",
      "command": "./scripts/audit_command.sh"
    },
    {
      "event": "post_tool_execution",
      "tool": "write_to_file",
      "command": "cargo fmt"
    }
  ]
}
```
* **Pre-tool execution:** Intercepts, validates, or blocks commands.
* **Post-tool execution:** Runs formatters or linters immediately after file writes.
