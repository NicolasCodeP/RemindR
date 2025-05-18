use anyhow::Result;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Get command from environment variable
    let prev_cmd = match env::var("HIST_LAST_CMD") {
        Ok(cmd) => cmd,
        Err(_) => {
            eprintln!("No command provided via HIST_LAST_CMD environment variable");
            return Ok(());
        }
    };
    
    // Skip empty commands
    let command = prev_cmd.trim();
    if command.is_empty() {
        return Ok(());
    }
    
    // Skip common non-commands
    if command.chars().all(char::is_whitespace) || 
       command == "exit" || 
       command == "logout" || 
       command == "clear" || 
       command.starts_with('#') ||
       command.contains("shell_integration") { // Avoid recording this tool itself
        return Ok(());
    }
    
    // Check for duplicates
    if is_duplicate_command(command)? {
        return Ok(());
    }
    
    // Initialize the database silently (no logs)
    remindr::init()?;
    
    // Store the command in the database
    remindr::db::store_command(command)?;
    
    // Remember this command to avoid duplicates
    save_last_command(command)?;
    
    Ok(())
}

// Check if this command is a duplicate of the last saved command
fn is_duplicate_command(command: &str) -> Result<bool> {
    let last_command = get_last_command()?;
    Ok(last_command.as_deref() == Some(command))
}

// Get the last saved command
fn get_last_command() -> Result<Option<String>> {
    let last_command_path = get_last_command_path()?;
    
    if !last_command_path.exists() {
        return Ok(None);
    }
    
    let mut file = File::open(last_command_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.is_empty() {
        Ok(None)
    } else {
        Ok(Some(contents))
    }
}

// Save the command as the last executed command
fn save_last_command(command: &str) -> Result<()> {
    let last_command_path = get_last_command_path()?;
    
    // Ensure directory exists
    if let Some(parent) = last_command_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to file, overwriting previous content
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(last_command_path)?;
    
    file.write_all(command.as_bytes())?;
    
    Ok(())
}

// Get the path to the last command file
fn get_last_command_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?;
    Ok(home_dir.join(".remindr").join("last_command.txt"))
} 