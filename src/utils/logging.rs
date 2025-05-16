use anyhow::Result;
use log::LevelFilter;
use std::fs;
use std::path::PathBuf;

/// Get the path to the log file
fn log_file_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?;
    let log_dir = home_dir.join(".remindr").join("logs");
    
    // Create logs directory if it doesn't exist
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }
    
    Ok(log_dir.join("remindr.log"))
}

/// Initialize the logger
pub fn init_logger() -> Result<()> {
    // Check if environment variable is set
    if std::env::var("RUST_LOG").is_err() {
        // Set default log level if not specified
        std::env::set_var("RUST_LOG", "info");
    }
    
    // Get log file path
    let log_path = log_file_path()?;
    
    // Configure logger
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format_module_path(true)
        .format_target(false)
        .filter(None, LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Never)
        .init();
    
    Ok(())
} 