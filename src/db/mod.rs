mod pool;
mod schema;

use std::path::PathBuf;
use anyhow::{Context, Result};
use log::{info, error};
use std::fs;

pub use pool::{get_conn, init_pool};
pub use schema::{migrate, Command};

// Path to the database file
pub fn db_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to locate home directory")?;
    let data_dir = home_dir.join(".remindr");
    
    // Create data directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
    }
    
    Ok(data_dir.join("history.db"))
}

/// Initialize the database
pub fn init_database() -> Result<()> {
    let db_path = db_path()?;
    
    info!("Initializing database at: {:?}", db_path);
    
    // Initialize connection pool
    init_pool(&db_path.to_string_lossy())?;
    
    // Run migrations
    migrate().context("Failed to run database migrations")?;
    
    info!("Database initialization complete");
    Ok(())
}

/// Store a command in the database
pub fn store_command(raw_command: &str) -> Result<i64> {
    let command = Command::new(raw_command);
    command.save()
}

/// Get the last n commands
pub fn get_last_commands(limit: usize) -> Result<Vec<Command>> {
    Command::get_last(limit)
}

/// Search commands by keyword
pub fn search_commands(keyword: &str) -> Result<Vec<Command>> {
    Command::search(keyword)
}

/// Get daemon status
pub fn get_daemon_status() -> Result<bool> {
    schema::get_daemon_status()
}

/// Set daemon status
pub fn set_daemon_status(status: bool) -> Result<()> {
    schema::set_daemon_status(status)
}

// Additional exports for testing
// pub use schema::tests; 