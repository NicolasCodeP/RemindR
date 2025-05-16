pub mod ai;
pub mod cli;
pub mod daemon;
pub mod db;
pub mod models;
pub mod utils;

use anyhow::Result;
use log::{debug, info};

/// Initialize the application, including setting up the database 
/// and other core components.
pub fn init() -> Result<()> {
    // Initialize logging
    utils::logging::init_logger()?;
    
    info!("Initializing RemindR v{}", version());
    debug!("Debug logging enabled");
    
    // Initialize database
    db::init_database()?;
    
    info!("Initialization complete");
    Ok(())
}

/// Get the application version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get the application name
pub fn name() -> &'static str {
    env!("CARGO_PKG_NAME")
} 