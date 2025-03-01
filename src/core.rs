use rusqlite::{Connection, params};
use chrono::{Local, NaiveDate};
use thiserror::Error;

use crate::config;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum Error {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),
}

pub struct Core {
    pub conn: Connection,
}

#[derive(Debug)]
pub struct Entry {
    pub id: i64,
    pub content: String,
    pub date: NaiveDate,
    pub created_at: chrono::DateTime<Local>,
}

impl Core {
    pub fn init() -> Result<Self, Error> {
        let config = config::Config::load()?;
        let conn = Connection::open(&config.diary_db_path)?;
        
        Self::init_tables(&conn)?;
        
        Ok(Self { conn })
    }

    fn init_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                date DATE NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS people (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS entry_people (
                entry_id INTEGER NOT NULL REFERENCES entries(id),
                person_id INTEGER NOT NULL REFERENCES people(id),
                PRIMARY KEY (entry_id, person_id)
            );

            CREATE TABLE IF NOT EXISTS entry_projects (
                entry_id INTEGER NOT NULL REFERENCES entries(id),
                project_id INTEGER NOT NULL REFERENCES projects(id),
                PRIMARY KEY (entry_id, project_id)
            );

            CREATE TABLE IF NOT EXISTS entry_tags (
                entry_id INTEGER NOT NULL REFERENCES entries(id),
                tag_id INTEGER NOT NULL REFERENCES tags(id),
                PRIMARY KEY (entry_id, tag_id)
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts 
                USING fts5(content, tokenize = 'porter unicode61');
            "#
        )
    }

    pub fn add_entry(&mut self, content: &str, date: Option<&str>) -> Result<(), Error> {
        let date = date.map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d"))
            .transpose()?
            .unwrap_or_else(|| Local::now().date_naive());

        let tx = self.conn.transaction()?;
        
        // Insert main entry
        tx.execute(
            "INSERT INTO entries (content, date) VALUES (?, ?)",
            params![content, date.to_string()]
        )?;
        
        let entry_id = tx.last_insert_rowid();
        
        // Extract and process entities
        // Using a separate method to avoid borrow issues
        Core::process_entities(&tx, entry_id, content)?;
        
        // Insert into FTS table
        tx.execute(
            "INSERT INTO entries_fts (rowid, content) VALUES (?, ?)",
            params![entry_id, content]
        )?;

        tx.commit()?;
        Ok(())
    }

    fn process_entities(
        tx: &rusqlite::Transaction,
        entry_id: i64,
        content: &str
    ) -> Result<(), rusqlite::Error> {
        // Extract people (@name), projects (%name), and tags (#name)
        let people_regex = regex::Regex::new(r"@([\w-]+)").unwrap();
        let projects_regex = regex::Regex::new(r"%([\w-]+)").unwrap();
        let tags_regex = regex::Regex::new(r"#([\w-]+)").unwrap();
        
        // Process people
        for cap in people_regex.captures_iter(content) {
            let name = &cap[1];
            
            // Insert or get person
            tx.execute(
                "INSERT OR IGNORE INTO people (name) VALUES (?)",
                params![name]
            )?;
            
            let person_id = tx.query_row(
                "SELECT id FROM people WHERE name = ?",
                params![name],
                |row| row.get::<_, i64>(0)
            )?;
            
            // Create relationship
            tx.execute(
                "INSERT OR IGNORE INTO entry_people (entry_id, person_id) VALUES (?, ?)",
                params![entry_id, person_id]
            )?;
        }
        
        // Process projects
        for cap in projects_regex.captures_iter(content) {
            let name = &cap[1];
            
            // Insert or get project
            tx.execute(
                "INSERT OR IGNORE INTO projects (name) VALUES (?)",
                params![name]
            )?;
            
            let project_id = tx.query_row(
                "SELECT id FROM projects WHERE name = ?",
                params![name],
                |row| row.get::<_, i64>(0)
            )?;
            
            // Create relationship
            tx.execute(
                "INSERT OR IGNORE INTO entry_projects (entry_id, project_id) VALUES (?, ?)",
                params![entry_id, project_id]
            )?;
        }
        
        // Process tags
        for cap in tags_regex.captures_iter(content) {
            let name = &cap[1];
            
            // Insert or get tag
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?)",
                params![name]
            )?;
            
            let tag_id = tx.query_row(
                "SELECT id FROM tags WHERE name = ?",
                params![name],
                |row| row.get::<_, i64>(0)
            )?;
            
            // Create relationship
            tx.execute(
                "INSERT OR IGNORE INTO entry_tags (entry_id, tag_id) VALUES (?, ?)",
                params![entry_id, tag_id]
            )?;
        }
        
        Ok(())
    }
}
