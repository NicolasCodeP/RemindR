use anyhow::{Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::sync::OnceLock;

// Global connection pool
static CONNECTION_POOL: OnceLock<Pool<SqliteConnectionManager>> = OnceLock::new();

/// Initialize the connection pool with the given database path
pub fn init_pool(db_path: &str) -> Result<()> {
    // Configure SQLite connection manager
    let manager = SqliteConnectionManager::file(db_path)
        .with_init(|conn| {
            // Enable foreign keys
            conn.execute_batch("PRAGMA foreign_keys = ON")?;
            // Use Write-Ahead Logging for better concurrency
            conn.execute_batch("PRAGMA journal_mode = WAL")?;
            // Set synchronous mode to NORMAL for better performance
            conn.execute_batch("PRAGMA synchronous = NORMAL")?;
            Ok(())
        });

    // Create and initialize the connection pool
    let pool = Pool::builder()
        .max_size(10)  // Maximum number of connections
        .build(manager)
        .context("Failed to build SQLite connection pool")?;

    // Set the global pool
    CONNECTION_POOL.set(pool).map_err(|_| {
        anyhow::anyhow!("Failed to set global connection pool - already initialized")
    })?;

    Ok(())
}

/// Get a connection from the pool
pub fn get_conn() -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
    let pool = CONNECTION_POOL.get()
        .context("Database connection pool not initialized")?;
    
    pool.get().context("Failed to get database connection from pool")
} 