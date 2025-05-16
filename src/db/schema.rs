use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::info;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

use super::pool::get_conn;

// Database version for migrations
const DB_VERSION: i32 = 1;

/// Runs migrations to set up or update the database schema
pub fn migrate() -> Result<()> {
    let mut conn = get_conn()?;

    // Create user_version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_version (
            version INTEGER NOT NULL
        )",
        [],
    ).context("Failed to create user_version table")?;

    // Check current version
    let current_version: Option<i32> = conn
        .query_row("SELECT version FROM user_version LIMIT 1", [], |row| row.get(0))
        .optional()
        .context("Failed to get current database version")?;

    let current_version = current_version.unwrap_or(0);

    if current_version < DB_VERSION {
        info!("Migrating database from version {} to {}", current_version, DB_VERSION);

        // Start transaction for migrations
        let tx = conn.transaction().context("Failed to start migration transaction")?;

        // Run migrations based on current version
        if current_version < 1 {
            // Create commands table
            tx.execute(
                "CREATE TABLE IF NOT EXISTS commands (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    command TEXT NOT NULL,
                    categorization TEXT,
                    tags TEXT,
                    context TEXT
                )",
                [],
            ).context("Failed to create commands table")?;

            // Create status table
            tx.execute(
                "CREATE TABLE IF NOT EXISTS daemon_status (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    is_active BOOLEAN NOT NULL DEFAULT 0
                )",
                [],
            ).context("Failed to create daemon_status table")?;

            // Insert initial status
            tx.execute(
                "INSERT OR IGNORE INTO daemon_status (id, is_active) VALUES (1, 0)",
                [],
            ).context("Failed to insert initial daemon status")?;

            // Create index on timestamp for faster retrieval
            tx.execute(
                "CREATE INDEX IF NOT EXISTS idx_commands_timestamp ON commands(timestamp)",
                [],
            ).context("Failed to create timestamp index")?;

            // Create index on command for full-text search
            tx.execute(
                "CREATE INDEX IF NOT EXISTS idx_commands_command ON commands(command)",
                [],
            ).context("Failed to create command index")?;
        }

        // Update version
        if current_version == 0 {
            tx.execute(
                "INSERT INTO user_version (version) VALUES (?)",
                params![DB_VERSION],
            ).context("Failed to insert database version")?;
        } else {
            tx.execute(
                "UPDATE user_version SET version = ?",
                params![DB_VERSION],
            ).context("Failed to update database version")?;
        }

        // Commit changes
        tx.commit().context("Failed to commit migration changes")?;
        
        info!("Database migration completed successfully");
    }

    Ok(())
}

/// Command structure representing a historized command
#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub categorization: Option<String>,
    pub tags: Option<String>,
    pub context: Option<String>,
}

impl Command {
    /// Create a new command
    pub fn new(command: &str) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            command: command.to_string(),
            categorization: None, // Will be filled by AI later
            tags: None,
            context: None,
        }
    }

    /// Save command to database
    pub fn save(&self) -> Result<i64> {
        let conn = get_conn()?;
        
        let id = conn.execute(
            "INSERT INTO commands (timestamp, command, categorization, tags, context)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                self.timestamp.to_rfc3339(),
                self.command,
                self.categorization,
                self.tags,
                self.context
            ],
        ).context("Failed to insert command")?;

        Ok(conn.last_insert_rowid())
    }

    /// Get the last n commands
    pub fn get_last(limit: usize) -> Result<Vec<Self>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, command, categorization, tags, context 
             FROM commands 
             ORDER BY timestamp DESC 
             LIMIT ?",
        )?;

        let command_iter = stmt.query_map(params![limit as i64], |row| {
            let timestamp_str: String = row.get(1)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);
                
            Ok(Command {
                id: Some(row.get(0)?),
                timestamp,
                command: row.get(2)?,
                categorization: row.get(3)?,
                tags: row.get(4)?,
                context: row.get(5)?,
            })
        })?;

        let mut commands = Vec::new();
        for command in command_iter {
            commands.push(command?);
        }

        Ok(commands)
    }

    /// Search commands by keyword
    pub fn search(keyword: &str) -> Result<Vec<Self>> {
        let conn = get_conn()?;
        let query = format!("%{}%", keyword);
        
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, command, categorization, tags, context 
             FROM commands 
             WHERE command LIKE ?1 OR categorization LIKE ?1 OR tags LIKE ?1 OR context LIKE ?1
             ORDER BY timestamp DESC 
             LIMIT 50",
        )?;

        let command_iter = stmt.query_map(params![query], |row| {
            let timestamp_str: String = row.get(1)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);
                
            Ok(Command {
                id: Some(row.get(0)?),
                timestamp,
                command: row.get(2)?,
                categorization: row.get(3)?,
                tags: row.get(4)?,
                context: row.get(5)?,
            })
        })?;

        let mut commands = Vec::new();
        for command in command_iter {
            commands.push(command?);
        }

        Ok(commands)
    }
}

/// Get daemon status from database
pub fn get_daemon_status() -> Result<bool> {
    let conn = get_conn()?;
    
    let is_active: bool = conn
        .query_row(
            "SELECT is_active FROM daemon_status WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .context("Failed to get daemon status")?;
    
    Ok(is_active)
}

/// Set daemon status in database
pub fn set_daemon_status(status: bool) -> Result<()> {
    let conn = get_conn()?;
    
    conn.execute(
        "UPDATE daemon_status SET is_active = ? WHERE id = 1",
        params![status],
    ).context("Failed to update daemon status")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_command_creation() {
        let cmd = Command::new("git status");
        assert_eq!(cmd.command, "git status");
        assert!(cmd.id.is_none());
        assert!(cmd.categorization.is_none());
        assert!(cmd.tags.is_none());
        assert!(cmd.context.is_none());
    }
    
    #[test]
    fn test_command_serialization() {
        let cmd = Command::new("ls -la");
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("ls -la"));
        
        let deserialized: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.command, "ls -la");
    }
} 