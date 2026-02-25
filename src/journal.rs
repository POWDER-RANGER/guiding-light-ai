use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub created_at: DateTime<Utc>,
    pub decision: String,
    pub why: String,
}

pub fn init_db(db_path: &Path) -> Result<()> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).context("failed to create db dir")?;
    }
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        r#"CREATE TABLE IF NOT EXISTS journal (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TEXT NOT NULL,
            decision TEXT NOT NULL,
            why TEXT NOT NULL
        );"#,
    )?;
    Ok(())
}

pub fn add_entry(db_path: &Path, decision: &str, why: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO journal (created_at, decision, why) VALUES (?1, ?2, ?3)",
        params![now, decision, why],
    )?;
    Ok(())
}

pub fn list_entries(db_path: &Path) -> Result<Vec<JournalEntry>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "SELECT created_at, decision, why FROM journal ORDER BY id DESC LIMIT 50",
    )?;
    
    let rows = stmt.query_map([], |row| {
        let created_at: String = row.get(0)?;
        let dt = created_at.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now());
        Ok(JournalEntry {
            created_at: dt,
            decision: row.get(1)?,
            why: row.get(2)?,
        })
    })?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
