use crate::core::Core;
use comfy_table::Table;

pub fn handle_show_command(target: crate::ShowTarget, core: &Core) -> anyhow::Result<()> {
    match target {
        crate::ShowTarget::Entries {
            date,
            search,
            person,
            project,
            tag,
        } => show_entries(core, date, search, person, project, tag),
        crate::ShowTarget::People => show_entities(core, "people"),
        crate::ShowTarget::Projects => show_entities(core, "projects"),
        crate::ShowTarget::Tags => show_entities(core, "tags"),
    }
}

fn show_entries(
    core: &Core,
    date_filter: Option<String>,
    search_term: Option<String>,
    person: Option<String>,
    project: Option<String>,
    tag: Option<String>,
) -> anyhow::Result<()> {
    // Get entries from database
    let entries = get_filtered_entries(core, date_filter, search_term, person, project, tag)?;

    // Display entries
    let mut table = Table::new();
    table.set_header(vec!["Date", "Entry"]);

    if entries.is_empty() {
        println!("No entries found matching your criteria.");
    } else {
        for entry in entries {
            table.add_row(vec![entry.date.to_string(), entry.content]);
        }
        println!("{table}");
    }

    Ok(())
}

fn get_filtered_entries(
    _core: &Core,
    _date_filter: Option<String>,
    _search_term: Option<String>,
    _person: Option<String>,
    _project: Option<String>,
    _tag: Option<String>,
) -> anyhow::Result<Vec<crate::core::Entry>> {
    // This is a placeholder - in a real implementation, we would:
    // 1. Build a SQL query based on the filters
    // 2. Execute the query against the database
    // 3. Return the results as Entry structs

    // For now, return a dummy entry to use the Entry struct
    let entry = crate::core::Entry {
        id: 1,
        content: "Example entry with @John and %Project".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
        created_at: chrono::Local::now(),
    };
    println!("Entry ID: {}, Created at: {}", entry.id, entry.created_at);

    Ok(vec![entry])
}

fn show_entities(core: &Core, entity_type: &str) -> anyhow::Result<()> {
    let mut table = Table::new();
    table.set_header(vec![entity_type]);

    let query = match entity_type {
        "people" => "SELECT name FROM people ORDER BY name",
        "projects" => "SELECT name FROM projects ORDER BY name",
        "tags" => "SELECT name FROM tags ORDER BY name",
        _ => return Err(anyhow::anyhow!("Invalid entity type")),
    };

    let mut stmt = core.conn.prepare(query)?;
    let entities = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for entity in entities {
        table.add_row(vec![entity?]);
    }

    if table.row_iter().count() == 0 {
        println!("No {} found.", entity_type);
    } else {
        println!("{table}");
    }

    Ok(())
}
