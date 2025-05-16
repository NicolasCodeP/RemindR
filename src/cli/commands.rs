use anyhow::{Context, Result};
use log::{info, error};

use crate::daemon;
use crate::db;
use super::display;

/// Start the daemon process
pub fn start_daemon() -> Result<()> {
    // Check if daemon is already running
    if db::get_daemon_status()? {
        println!("RemindR is already running");
        return Ok(());
    }
    
    // Start the daemon
    daemon::start_daemon().context("Failed to start daemon")?;
    
    println!("RemindR daemon started successfully");
    Ok(())
}

/// Stop the daemon process
pub fn stop_daemon() -> Result<()> {
    // Check if daemon is running
    if !db::get_daemon_status()? {
        println!("RemindR is not running");
        return Ok(());
    }
    
    // Stop the daemon
    daemon::stop_daemon().context("Failed to stop daemon")?;
    
    println!("RemindR daemon stopped successfully");
    Ok(())
}

/// Show daemon status and last few commands
pub fn show_status() -> Result<()> {
    // Get daemon status
    let is_active = db::get_daemon_status()?;
    let status_text = if is_active { "ON" } else { "OFF" };
    
    // Get last 5 commands
    let commands = db::get_last_commands(5)?;
    
    // Display status and commands
    display::show_status_screen(status_text, &commands);
    
    Ok(())
}

/// Search commands by keyword
pub fn search_commands(keyword: &str) -> Result<()> {
    // Get daemon status
    let is_active = db::get_daemon_status()?;
    let status_text = if is_active { "ON" } else { "OFF" };
    
    // Search for commands
    let commands = if keyword.trim().is_empty() {
        // If no keyword, show most recent commands
        db::get_last_commands(10)?
    } else {
        // Search by keyword
        db::search_commands(keyword)?
    };
    
    // Display search results
    display::show_search_screen(status_text, keyword, &commands);
    
    Ok(())
} 