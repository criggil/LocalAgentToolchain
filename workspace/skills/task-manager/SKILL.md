---
name: task-manager
description: Manage project tasks, view Kanban boards, check off steps, and log progress using the local task CLI tool.
version: 0.1.0
author: local-orchestrator
triggers:
  - task
  - todo
  - kanban
  - checklist
---

# Task Manager Standard Operating Procedure (SOP)

When working on project tasks, tracking development progress, or reporting execution status, use the local `task` CLI tool.

## Key CLI Commands

1. **List Tasks:**
   Query current tasks in machine-readable JSON:
   `task list --json`
   Filter by status:
   `task list -s todo --json`
   `task list -s in_progress --json`

2. **Start a Task:**
   Move task to in-progress status before beginning work:
   `task start <TASK_ID>`

3. **Check Off Checklist Items:**
   Mark individual checklist items complete:
   `task check <TASK_ID> <ITEM_INDEX>`
   Uncheck an item if reverted:
   `task check <TASK_ID> <ITEM_INDEX> --uncheck`

4. **Log Progress / Key Findings:**
   Record benchmark results, test completions, or notes into task history:
   `task log <TASK_ID> "<MESSAGE>"`

5. **Complete a Task:**
   Mark the task as done once all checklist items are verified:
   `task done <TASK_ID>`

6. **Inspect Full Task Details:**
   View all checklists, description, and logs:
   `task show <TASK_ID> --json`
