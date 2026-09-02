---
name: knowledge-base
description: Search project specifications, read architecture docs via Wikilinks, and write execution reports using the note CLI tool.
version: 0.1.0
author: local-orchestrator
triggers:
  - note
  - wiki
  - documentation
  - spec
  - architecture
---

# Knowledge Base Standard Operating Procedure (SOP)

When researching project architecture, reading specifications, or documenting findings, use the local `note` CLI tool.

## Key CLI Commands

1. **Search Context & Documentation:**
   Perform full-text search across all notes and specs:
   `note search "<QUERY>" --json`

2. **Read Full Specification:**
   Output raw markdown content of a note without CLI formatting:
   `note show "<NOTE_TITLE>" --raw`

3. **Inspect Connections & Backlinks:**
   Discover outgoing links and incoming references across notes and tasks:
   `note links "<NOTE_TITLE>" --json`

4. **Append Notes / Findings:**
   Append a research note or benchmark result to the end of a document:
   `note append "<NOTE_PATH>" "<TEXT>"`

5. **Create Execution Report:**
   After completing a significant task, save an execution summary to the reports folder:
   `note new "<REPORT_TITLE>" -f 40_Reports -t report -c "<CONTENT>"`
