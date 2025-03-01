// src/completion.rs
use rusqlite::{Connection, params, Result as SqliteResult, Error as SqliteError};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use std::borrow::Cow::{self, Borrowed, Owned};

pub struct DiaCompleter {
    conn: Connection,
}

impl DiaCompleter {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
    
    fn complete_project(&self, word: &str) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM projects WHERE name LIKE ? || '%' ORDER BY name LIMIT 10"
        )?;
        
        let rows = stmt.query_map(params![word], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        
        let mut completions = Vec::new();
        for row in rows {
            completions.push(row?);
        }
        
        Ok(completions)
    }
    
    fn complete_tag(&self, word: &str) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM tags WHERE name LIKE ? || '%' ORDER BY name LIMIT 10"
        )?;
        
        let rows = stmt.query_map(params![word], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        
        let mut completions = Vec::new();
        for row in rows {
            completions.push(row?);
        }
        
        Ok(completions)
    }
    
    fn complete_person(&self, word: &str) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM people WHERE name LIKE ? || '%' ORDER BY name LIMIT 10"
        )?;
        
        let rows = stmt.query_map(params![word], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        
        let mut completions = Vec::new();
        for row in rows {
            completions.push(row?);
        }
        
        Ok(completions)
    }
}

impl Completer for DiaCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) 
        -> Result<(usize, Vec<Pair>), ReadlineError> 
    {
        // Find the word we're completing
        let (start, word) = find_word_at_pos(line, pos);
        
        // If empty, return empty completions
        if word.is_empty() {
            return Ok((start, vec![]));
        }
        
        // Different completion based on prefix
        let completions = match word.chars().next() {
            Some('%') => {
                // Project completion
                let word_without_prefix = &word[1..];
                match self.complete_project(word_without_prefix) {
                    Ok(projects) => projects.into_iter()
                        .map(|p| Pair {
                            display: p.clone(),
                            replacement: format!("%{}", p),
                        })
                        .collect(),
                    Err(_) => vec![],
                }
            },
            Some('#') => {
                // Tag completion
                let word_without_prefix = &word[1..];
                match self.complete_tag(word_without_prefix) {
                    Ok(tags) => tags.into_iter()
                        .map(|t| Pair {
                            display: t.clone(),
                            replacement: format!("#{}", t),
                        })
                        .collect(),
                    Err(_) => vec![],
                }
            },
            Some('@') => {
                // Person completion
                let word_without_prefix = &word[1..];
                match self.complete_person(word_without_prefix) {
                    Ok(people) => people.into_iter()
                        .map(|p| Pair {
                            display: p.clone(),
                            replacement: format!("@{}", p),
                        })
                        .collect(),
                    Err(_) => vec![],
                }
            },
            _ => vec![],
        };
        
        Ok((start, completions))
    }
}

fn find_word_at_pos(line: &str, pos: usize) -> (usize, &str) {
    let line_prefix = &line[..pos];
    
    // Find the start of the current word
    let start = line_prefix.rfind(|c: char| c.is_whitespace() || c == ',')
        .map(|i| i + 1)
        .unwrap_or(0);
    
    // Find the end of the current word
    let end = line[pos..].find(|c: char| c.is_whitespace() || c == ',')
        .map(|i| i + pos)
        .unwrap_or(line.len());
    
    (start, &line[start..end.min(pos)])
}

// Interactive entry input with autocompletion
pub fn input_with_completion(conn: &Connection, prompt: &str) -> Result<String, ReadlineError> {
    let mut rl = rustyline::Editor::<DiaCompleter>::new()?;
    let completer = DiaCompleter::new(conn.clone());
    rl.set_completer(Some(completer));
    
    rl.readline(prompt)
}
