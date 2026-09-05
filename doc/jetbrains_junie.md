# JetBrains Junie CLI Extensibility Guide (JetBrains)

JetBrains Junie CLI is a terminal-native autonomous coding agent tailored for developers and IDE ecosystems. It offers a structured extension system centered around **Extensions**, **Skills**, **MCP**, and **Guidelines**.

---

## 1. Project Guidelines (`.junie/guidelines.md`)

Junie reads `.junie/guidelines.md` (and optionally `AGENTS.md`) at session initialization.

### Purpose
* Defines repository structure and conventions.
* Specifies default testing frameworks, formatting tools, and build commands.
* Sets strict constraints on what files Junie may or may not modify.

### Locations
* **Project Level:** `./.junie/guidelines.md`
* **Global Level:** `~/.junie/guidelines.md`

---

## 2. Skills (`.junie/skills/<name>/SKILL.md`)

Skills in Junie provide deep domain knowledge and procedural step-by-step checklists for specific technologies, architectures, or frameworks.

### Structure
```text
.junie/skills/spring-boot-upgrade/
├── SKILL.md       # Manifest and procedural instructions
├── patterns/      # Best practices and code patterns
└── antipatterns/  # What to avoid
```

### Loading
Junie inspects `.junie/skills/` at startup. Skills allow the agent to follow precise workflows (e.g. migrating from Spring Boot 2 to 3, adding Kotlin coroutines) without trial-and-error.

---

## 3. Extensions & Marketplace (`/extensions`)

Junie CLI features a modular **Extensions** architecture. Extensions are curated bundles that provide full-stack support for a technology without manual per-tool configuration.

### What an Extension Contains
* **Agent Skills:** Procedural workflows.
* **MCP Servers:** Tool and API integrations (e.g. database access, issue trackers).
* **Subagents:** Specialized agent configurations.
* **Custom Slash Commands:** Shortcuts for common tasks.
* **Lifecycle Hooks:** Pre/post execution verification.

### Management
* Slash command: `/extensions` (or `/plugin`) allows browsing, installing, and updating extensions directly within the terminal interface.

---

## 4. Live Prompting & Interactive Steering

Junie supports **Live Prompting**:
* Users can inject steering commands mid-execution without terminating the active turn or losing context.
* Slash commands like `/clear`, `/rollback`, and `/context` manage the conversation state.
