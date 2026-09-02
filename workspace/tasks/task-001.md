---
id: task-001
title: Спроектировать архитектуру оркестратора
status: done
priority: high
labels:
  - architecture
  - tauri
  - rust
created_at: 2026-09-01
---

## Описание задачи
Определить ключевые модули:
- TaskManager (Trello-like Kanban)
- Knowledge Base (Obsidian-like Markdown)
- Execution Engine & Terminal
- Secrets Manager & GitHub Hub

## Чек-лист
- [x] Выбрать стек (Tauri v2 + React + Vite + Tailwind)
- [x] Отказаться от SQLite в пользу 100% Plain Markdown
- [x] Спроектировать IDE-подобный лейаут с тайлами и 3-мя Activity Bars
