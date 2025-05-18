mod process;
mod watcher;

use anyhow::{Context, Result};
use daemonize::Daemonize;
use log::{info, error};
use std::fs::File;
use std::path::PathBuf;

pub use process::{is_daemon_running, get_daemon_pid, kill_daemon};
pub use watcher::start_command_watcher;

// Daemon-related file paths
fn daemon_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to locate home directory")?;
    let daemon_dir = home_dir.join(".remindr");
    
    std::fs::create_dir_all(&daemon_dir).context("Failed to create daemon directory")?;
    
    Ok(daemon_dir)
}

fn pid_file_path() -> Result<PathBuf> {
    Ok(daemon_dir()?.join("remindr.pid"))
}

fn stdout_file_path() -> Result<PathBuf> {
    Ok(daemon_dir()?.join("remindr.out"))
}

fn stderr_file_path() -> Result<PathBuf> {
    Ok(daemon_dir()?.join("remindr.err"))
}

/// Start the daemon process
pub fn start_daemon() -> Result<()> {
    // Check if daemon is already running
    if is_daemon_running()? {
        info!("Daemon is already running");
        return Ok(());
    }
    
    info!("Starting daemon...");
    
    // Set up daemon files
    let pid_path = pid_file_path()?;
    let stdout_path = stdout_file_path()?;
    let stderr_path = stderr_file_path()?;
    
    // Prepare file handles
    let stdout = File::create(&stdout_path)
        .context(format!("Failed to create stdout file at {:?}", stdout_path))?;
    let stderr = File::create(&stderr_path)
        .context(format!("Failed to create stderr file at {:?}", stderr_path))?;
    
    // Configure daemonize
    let daemonize = Daemonize::new()
        .pid_file(pid_path)
        .chown_pid_file(false)
        .working_directory(daemon_dir()?)
        .stdout(stdout)
        .stderr(stderr);
    
    // Start daemon process
    match daemonize.start() {
        Ok(_) => {
            // We're now in the daemon process
            info!("Daemon started successfully");
            
            // Update daemon status in DB
            if let Err(e) = crate::db::set_daemon_status(true) {
                error!("Failed to update daemon status: {}", e);
            }
            
            // Create a multi-threaded runtime for async operations
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .context("Failed to create Tokio runtime")?;
            
            // Run the command watcher in the runtime, blocking indefinitely
            rt.block_on(async {
                info!("Starting command watcher in runtime");
                if let Err(e) = start_command_watcher().await {
                    error!("Command watcher error: {}", e);
                }
                
                // Keep the daemon running indefinitely
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                }
            });
            
            Ok(())
        },
        Err(e) => {
            error!("Failed to start daemon: {}", e);
            Err(anyhow::anyhow!("Failed to start daemon: {}", e))
        }
    }
}

/// Stop the daemon process
pub fn stop_daemon() -> Result<()> {
    // Check if daemon is running
    if !is_daemon_running()? {
        info!("Daemon is not running");
        return Ok(());
    }
    
    info!("Stopping daemon...");
    
    // Kill the daemon process
    kill_daemon().context("Failed to kill daemon process")?;
    
    // Update daemon status in DB
    crate::db::set_daemon_status(false).context("Failed to update daemon status")?;
    
    info!("Daemon stopped successfully");
    Ok(())
} 