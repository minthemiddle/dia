// src/db.rs
use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use dirs::home_dir;

pub fn get_db_path() -> PathBuf {
    let config_dir = home_dir()
        .expect("Could not find home directory")
        .join(".config/dia");
    
    config_dir.join("diary.db")
}

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    // Create entries table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            content TEXT NOT NULL,
            last_reviewed TEXT
        )",
        [],
    )?;
    
    // Create entity tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    // Create relation tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entry_projects (
            entry_id INTEGER,
            project_id INTEGER,
            PRIMARY KEY (entry_id, project_id),
            FOREIGN KEY (entry_id) REFERENCES entries (id) ON DELETE CASCADE,
            FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entry_tags (
            entry_id INTEGER,
            tag_id INTEGER,
            PRIMARY KEY (entry_id, tag_id),
            FOREIGN KEY (entry_id) REFERENCES entries (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entry_people (
            entry_id INTEGER,
            person_id INTEGER,
            PRIMARY KEY (entry_id, person_id),
            FOREIGN KEY (entry_id) REFERENCES entries (id) ON DELETE CASCADE,
            FOREIGN KEY (person_id) REFERENCES people (id) ON DELETE CASCADE
        )",
        [],
    )?;
    
    // Create FTS virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(
            content,
            content='entries',
            content_rowid='id'
        )",
        [],
    )?;
    
    // Create trigger to keep FTS table updated
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
            INSERT INTO entries_fts(rowid, content) VALUES (new.id, new.content);
        END",
        [],
    )?;
    
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entries_ad AFTER DELETE ON entries BEGIN
            INSERT INTO entries_fts(entries_fts, rowid, content) VALUES('delete', old.id, old.content);
        END",
        [],
    )?;
    
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entries_au AFTER UPDATE ON entries BEGIN
            INSERT INTO entries_fts(entries_fts, rowid, content) VALUES('delete', old.id, old.content);
            INSERT INTO entries_fts(rowid, content) VALUES (new.id, new.content);
        END",
        [],
    )?;
    
    // Create indices
    conn.execute("CREATE INDEX IF NOT EXISTS idx_entries_date ON entries (date)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_projects_name ON projects (name)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_people_name ON people (name)", [])?;
    
    Ok(())
}
