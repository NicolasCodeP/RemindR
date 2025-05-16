use anyhow::{Context, Result};
use log::{info, error, debug};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

/// Get the path to the shell history file based on the current shell
fn get_shell_history_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to locate home directory")?;
    
    // Check which shell is being used
    let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash"));
    
    // Determine history file based on shell
    let history_path = if shell.contains("zsh") {
        home_dir.join(".zsh_history")
    } else if shell.contains("bash") {
        home_dir.join(".bash_history")
    } else if shell.contains("fish") {
        home_dir.join(".local/share/fish/fish_history")
    } else {
        // Default to bash history if unable to determine
        home_dir.join(".bash_history")
    };
    
    debug!("Using shell history file: {:?}", history_path);
    
    Ok(history_path)
}

/// Start the command watcher in a background task
pub async fn start_command_watcher() -> Result<()> {
    info!("Starting command watcher...");
    
    // Get history file path
    let history_path = get_shell_history_path()?;
    
    // Ensure file exists
    if !history_path.exists() {
        return Err(anyhow::anyhow!("Shell history file not found: {:?}", history_path));
    }
    
    // Watch history file directly without spawning a new task
    watch_history_file(history_path).await
}

/// Watch the shell history file for new commands
async fn watch_history_file(history_path: PathBuf) -> Result<()> {
    let mut file = File::open(&history_path)
        .context("Failed to open shell history file")?;
    
    // Start at the end of the file
    let file_size = file.metadata()
        .context("Failed to get history file metadata")?
        .len();
    file.seek(SeekFrom::End(0))
        .context("Failed to seek to end of history file")?;
    
    let mut last_size = file_size;
    
    // Continue watching until the program is terminated
    loop {
        // Sleep to avoid excessive CPU usage
        sleep(Duration::from_secs(1)).await;
        
        // Check if file size has changed
        match file.metadata() {
            Ok(metadata) => {
                let current_size = metadata.len();
                
                // If file has grown, read new content
                if current_size > last_size {
                    debug!("History file changed, reading new content");
                    
                    // Seek to where we left off
                    file.seek(SeekFrom::Start(last_size))
                        .context("Failed to seek in history file")?;
                    
                    // Read new content
                    let reader = BufReader::new(&file);
                    for line in reader.lines() {
                        match line {
                            Ok(command) => {
                                // Process and store the command
                                process_command(&command).await?;
                            },
                            Err(e) => {
                                error!("Error reading history line: {}", e);
                            }
                        }
                    }
                    
                    // Update last size
                    last_size = current_size;
                }
                // If file has shrunk (history cleared), start from the beginning
                else if current_size < last_size {
                    debug!("History file shrunk, restarting from beginning");
                    file.seek(SeekFrom::Start(0))
                        .context("Failed to seek to start of history file")?;
                    last_size = current_size;
                }
            },
            Err(e) => {
                error!("Failed to get history file metadata: {}", e);
                // Brief pause before retrying
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Process a command and store it in the database
async fn process_command(raw_command: &str) -> Result<()> {
    // Skip empty commands
    let command = raw_command.trim();
    if command.is_empty() {
        return Ok(());
    }
    
    // Skip commands with only whitespace or common non-commands
    if command.chars().all(char::is_whitespace) || 
       command == "exit" || 
       command == "logout" || 
       command == "clear" || 
       command.starts_with('#') {
        return Ok(());
    }
    
    debug!("Processing command: {}", command);
    
    // Store in database
    match crate::db::store_command(command) {
        Ok(id) => {
            debug!("Stored command with ID: {}", id);
            
            // TODO: Add command to AI analysis queue for categorization
            // This will be implemented later
            
            Ok(())
        },
        Err(e) => {
            error!("Failed to store command: {}", e);
            Err(e)
        }
    }
} 