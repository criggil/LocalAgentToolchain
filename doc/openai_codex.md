# OpenAI Codex CLI & Agents SDK Extensibility Guide (OpenAI)

OpenAI's autonomous agent ecosystem is built around the **Codex CLI** and the **OpenAI Agents SDK**. It standardizes on repository-level context, modular skills, and the industry-wide **Agent Plugins** standard.

---

## 1. `AGENTS.md` — The Universal Agent Specification

`AGENTS.md` is an open-format Markdown file that serves as the single source of truth for agent behavior, project conventions, architecture, and coding rules.

### Discovery & Traversal
* **Hierarchical Traversal:** Codex CLI walks up from the current working directory to the repository root, loading all `AGENTS.md` files in the path.
* **Nested Directives:** Subdirectories can contain their own `AGENTS.md` to define localized rules (e.g. `packages/frontend/AGENTS.md` vs `packages/backend/AGENTS.md`).
* **Global Overrides:** `~/.codex/AGENTS.md` defines user-wide preferences across all projects.

### Typical Content
```markdown
# Repository Architecture & Guidelines

## Tech Stack
- Rust 2021 edition (cargo)
- PostgreSQL 16
- Tokio asynchronous runtime

## Testing Rules
- Always run `cargo test --workspace` before submitting code.
- New public APIs must include docstrings and doctests.
```

---

## 2. Agent Skills (`SKILL.md`)

In the OpenAI Agents ecosystem, **Agent Skills** are folder-based packages that provide specialized task instructions and executable scripts.

### Locations
* **Project Level:** `./.codex/skills/<skill-name>/`
* **Global Level:** `~/.codex/skills/<skill-name>/`

### Anatomy of a Skill
```text
.codex/skills/database-migrator/
├── SKILL.md       # Manifest with frontmatter and step-by-step SOP
├── scripts/       # Automation scripts (Python, Bash, Node)
└── templates/     # Code templates
```

### Loading Behavior
Codex uses an index of available skills. When a prompt requires a specific capability (e.g. database migration, benchmark execution), Codex reads the corresponding `SKILL.md` and executes the procedural steps or helper scripts.

---

## 3. Model Context Protocol (MCP) Integration

OpenAI Codex CLI and the OpenAI Agents SDK natively support **MCP**.

### Configuration
Defined in `mcp_config.json` or `.codex/mcp.json`:
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "uvx",
      "args": ["mcp-server-sqlite", "--db-path", "test.db"]
    }
  }
}
```
Exposes external tools and resources as callable agent functions.

---

## 4. Agent Plugins (Industry Standard 2026)

In August 2026, OpenAI, AWS, Cursor, GitHub, Vercel, and partners announced the **Agent Plugins** open specification.

### Purpose
Agent Plugins solve the fragmentation between instructions and tools:
* **The Problem:** Skills provide instructions (`SKILL.md`), while MCP provides tools. Developers had to configure both separately.
* **The Solution:** An **Agent Plugin** bundles:
  1. MCP server configuration (executable tools).
  2. Agent Skills (SOPs and prompt guidelines).
  3. Security / permission declarations.
* **Portability:** An Agent Plugin built for OpenAI Codex works interchangeably in Claude, Antigravity, and Cursor.

### Plugin Manifest Structure (`plugin.json`)
```json
{
  "name": "docker-tools",
  "version": "1.0.0",
  "description": "Docker container management and optimization",
  "skills": ["skills/docker-deploy", "skills/docker-audit"],
  "mcp": {
    "command": "docker-mcp-server"
  }
}
```
