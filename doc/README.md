# AI Agent Extensibility Documentation

Comprehensive guides and architectural specifications for extending all major autonomous coding agents (2026).

---

## 📑 Table of Contents

| Document | Agent Ecosystem | Creator | Core Extension Mechanisms |
| :--- | :--- | :--- | :--- |
| **[Grand Overview](overview.md)** | **All Agents** | Universal Taxonomy | 6-Layer Architecture, Comparative Matrix, Portability. |
| **[Claude Code](claude_code.md)** | Claude Code CLI | Anthropic | `CLAUDE.md`, Skills (`SKILL.md`), Slash Commands, MCP, Auto-memory. |
| **[OpenAI Codex](openai_codex.md)** | Codex CLI & Agents SDK | OpenAI | `AGENTS.md`, Agent Skills, MCP, Agent Plugins (Open Standard 2026). |
| **[Google Antigravity](google_antigravity.md)** | Antigravity (AGY) | Google DeepMind | Rules (`GEMINI.md`, `AGENTS.md`), Skills (Progressive Disclosure), Plugins, Hooks, MCP, Subagents. |
| **[JetBrains Junie](jetbrains_junie.md)** | Junie CLI | JetBrains | `guidelines.md`, Skills (`.junie/skills/`), Extensions (`/extensions`), MCP. |
| **[Pi Coding Agent](pi_agent.md)** | Pi Agent | Pi Architecture | `SYSTEM.md`, `AGENTS.md`, Skills (`.pi/skills/`), Minimalist Tool Extensions. |

---

## 🌉 How Local Agent Toolchain (LAT) Bridges Them

The **Local Agent Toolchain (`skill`)** acts as the universal adapter across all these ecosystems. 

When you create or install a skill:
```bash
skill install task-manager --target all --link
```
LAT automatically maps and symlinks the skill into the native directory formats expected by **Claude Code, Codex, Antigravity, Junie, and Pi**, enabling **100% interoperability** without vendor lock-in.
