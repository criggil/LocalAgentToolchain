use colored::*;
use tabled::{Table, Tabled, settings::{Style, Alignment, Modify, object::Columns}};
use crate::models::{LinkGraph, Note, SearchResult};

#[derive(Tabled)]
struct NoteRow {
    #[tabled(rename = "FOLDER")]
    folder: String,
    #[tabled(rename = "TITLE")]
    title: String,
    #[tabled(rename = "TAGS")]
    tags: String,
    #[tabled(rename = "LINKS")]
    links: String,
    #[tabled(rename = "UPDATED")]
    updated: String,
}

pub fn print_notes_table(notes: &[Note]) {
    if notes.is_empty() {
        println!("{}", "No notes found. Use `note new \"Title\"` to create one.".bright_black());
        return;
    }

    let rows: Vec<NoteRow> = notes
        .iter()
        .map(|n| {
            let folder = std::path::Path::new(&n.relative_path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let tags_str = if n.frontmatter.tags.is_empty() {
                "-".to_string()
            } else {
                n.frontmatter.tags.join(", ")
            };

            let updated_str = n
                .frontmatter
                .updated_at
                .as_deref()
                .or(n.frontmatter.created_at.as_deref())
                .unwrap_or("-");

            NoteRow {
                folder: if folder.is_empty() { ".".to_string() } else { folder.bold().to_string() },
                title: n.frontmatter.title.clone(),
                tags: tags_str.cyan().to_string(),
                links: format!("{}", n.outgoing_links.len()),
                updated: updated_str.bright_black().to_string(),
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::modern());
    table.with(Modify::new(Columns::new(1..=1)).with(Alignment::left()));

    println!("{}", table);
    println!("{}", format!("Total notes: {}", notes.len()).bright_black());
}

pub fn print_note_details(note: &Note) {
    println!("\n{}", format!("═══ {} ═══", note.frontmatter.title).bold().bright_blue());
    println!("{}: {}", "Path".bold(), note.relative_path.bright_black());

    if !note.frontmatter.tags.is_empty() {
        println!("{}: {}", "Tags".bold(), note.frontmatter.tags.join(", ").cyan());
    }

    if !note.frontmatter.aliases.is_empty() {
        println!("{}: {}", "Aliases".bold(), note.frontmatter.aliases.join(", ").yellow());
    }

    if let Some(ref created) = note.frontmatter.created_at {
        println!("{}: {}", "Created".bold(), created.bright_black());
    }

    if !note.outgoing_links.is_empty() {
        println!("{}: {}", "Outgoing Links".bold(), note.outgoing_links.join(", ").green());
    }

    println!("\n{}", "── Content ──────────────────────────────".bold());
    println!("{}", note.content.trim());
    println!();
}

pub fn print_search_results(query: &str, results: &[SearchResult]) {
    println!("\n{}", format!("═══ Search Results for '{}' ═══", query).bold().bright_blue());

    if results.is_empty() {
        println!("{}", "No matching notes found.".bright_black());
        return;
    }

    for res in results {
        let tags_str = if res.tags.is_empty() {
            "".to_string()
        } else {
            format!(" [{}]", res.tags.join(", ")).cyan().to_string()
        };

        println!("\n📄 {} ({}){}", res.title.bold(), res.path.bright_black(), tags_str);

        for m in &res.matches {
            let highlighted = m.line.replace(query, &query.yellow().bold().to_string());
            println!("  {:>4}: {}", m.line_number.to_string().bright_black(), highlighted);
        }
    }

    println!("\n{}", format!("Matched {} note(s)", results.len()).bright_black());
}

pub fn print_link_graph(graph: &LinkGraph) {
    println!("\n{}", format!("═══ Links for '{}' ═══", graph.note_title).bold().bright_blue());
    println!("{}: {}", "File".bold(), graph.note_path.bright_black());

    println!("\n{}", "Outgoing Links (notes and tasks referenced):".bold().cyan());
    if graph.outgoing_links.is_empty() {
        println!("  {}", "(none)".bright_black().italic());
    } else {
        for link in &graph.outgoing_links {
            if link.starts_with("task-") {
                println!("  → 📋 {}", link.yellow().bold());
            } else {
                println!("  → 📄 [[{}]]", link.blue().bold());
            }
        }
    }

    println!("\n{}", "Backlinks (notes and tasks referencing this note):".bold().green());
    if graph.backlinks.is_empty() {
        println!("  {}", "(none)".bright_black().italic());
    } else {
        for bl in &graph.backlinks {
            if bl.contains("workspace/tasks/") {
                println!("  ← 📋 {}", bl.yellow());
            } else {
                println!("  ← 📄 {}", bl.blue());
            }
        }
    }
    println!();
}
