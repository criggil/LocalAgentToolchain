# Claude Code Extensibility Guide (Anthropic)

Claude Code is Anthropic's terminal-native agentic coding tool. It provides 5 core mechanisms to extend its capabilities, steer its behavior, and integrate with external systems.

---

## 1. `CLAUDE.md` — Persistent Project Memory & Rules

`CLAUDE.md` is automatically read by Claude Code at the start of every session. It serves as the primary way to provide persistent project architecture, build commands, and coding standards.

### Hierarchy & Locations
1. **Project Root:** `./CLAUDE.md` (checked into Git, shared across team).
2. **Global User Profile:** `~/.claude/CLAUDE.md` (applies to all projects run on the machine).

### Best Practices
* Generate a starting template using `/init`.
* Keep instructions concise, factual, and actionable.
* Document non-obvious commands (e.g. test flags, dev server ports, architectural boundaries).

### Auto-Memory System
In addition to `CLAUDE.md`, Claude Code maintains an internal **Auto-Memory** store (`~/.claude/memory/`). It automatically notes patterns, corrections you provide during chat sessions, and recurring workflows.

---

## 2. Skills (`SKILL.md` / `/skill-name`)

Skills are modular, reusable instruction packages. When you find yourself repeatedly pasting the same instructions, checklists, or procedures, Claude Code allows packaging them into a skill.

### Structure of a Skill
```text
my-skill/
├── SKILL.md       # Frontmatter + Markdown SOP
├── scripts/       # Optional helper scripts
└── examples/      # Optional reference examples
```

### Manifest Format (`SKILL.md`)
```yaml
---
name: code-review
description: Comprehensive security and quality code review procedure.
triggers:
  - review
  - audit
---

# Code Review Standard Operating Procedure (SOP)

1. Check for SQL injection vulnerabilities.
2. Verify all inputs are sanitized.
3. Ensure unit tests cover newly added branches.
```

### Invocation Modes
* **Explicit:** Users invoke directly via `/skill-name` in chat.
* **Autonomous:** Claude automatically activates the skill when the user prompt matches its triggers or description.
* **Built-in Bundled Skills:** Claude Code ships with `/debug`, `/code-review`, `/doctor`, and `/batch`.

---

## 3. Custom Slash Commands (`.claude/commands/`)

Slash commands provide quick terminal shortcuts for frequently executed tasks.

### Discovery Paths
* **Project Level:** `./.claude/commands/<command-name>.md`
* **User Level (Global):** `~/.claude/commands/<command-name>.md`

### Format
Each `.md` file represents a slash command:
```markdown
# /test-e2e Command

Run all Playwright end-to-end tests against the local staging environment:
1. Ensure docker container is running: `docker compose up -d db`
2. Run tests: `npx playwright test`
3. Summarize any failures.
```
When the user types `/test-e2e`, Claude executes this procedure.

---

## 4. Model Context Protocol (MCP)

Claude Code has native, first-class support for MCP (the open standard developed by Anthropic). MCP allows Claude to connect directly to external databases, APIs, and tools.

### Configuration
Managed via:
* CLI command: `claude mcp add <server-name> <command> [args...]`
* Configuration file: `.claude/mcp.json` (or `~/.claude/mcp.json`)

### Example `mcp.json`
```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres", "postgresql://localhost/mydb"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    }
  }
}
```
Claude discovers all tools exposed by the MCP server and can call them during reasoning.

---

## 5. Lifecycle Hooks & Approval Gates

Claude Code supports safety gates and hooks:
* **Permission Allowlist:** Configure approved terminal commands that Claude can run without asking for user confirmation (`.claude/config.json`).
* **Pre-tool / Post-tool Validation:** Shell filters to inspect or abort potentially destructive commands.
