use anyhow::{Context, Result};
use log::{info, error};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

// Get the path to the PID file
fn pid_file_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to locate home directory")?;
    let pid_path = home_dir.join(".remindr").join("remindr.pid");
    
    Ok(pid_path)
}

/// Check if daemon process is running
pub fn is_daemon_running() -> Result<bool> {
    let pid_path = pid_file_path()?;
    
    // Check if PID file exists
    if !pid_path.exists() {
        return Ok(false);
    }
    
    // Read PID from file
    let pid = match get_daemon_pid() {
        Ok(pid) => pid,
        Err(_) => return Ok(false),
    };
    
    // Check if process is running (platform-specific)
    #[cfg(target_family = "unix")]
    {
        // On Unix-like systems, check if process exists
        let output = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
            .context("Failed to execute ps command")?;
        
        // If exit status is 0, process is running
        Ok(output.status.success())
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, use tasklist
        let output = Command::new("tasklist")
            .arg("/FI")
            .arg(format!("PID eq {}", pid))
            .arg("/NH")
            .output()
            .context("Failed to execute tasklist command")?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // If output contains the PID, process is running
        Ok(output_str.contains(&pid.to_string()))
    }
    
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        // For other platforms, assume running if PID file exists
        Ok(true)
    }
}

/// Get the daemon process ID from the PID file
pub fn get_daemon_pid() -> Result<u32> {
    let pid_path = pid_file_path()?;
    
    // Check if PID file exists
    if !pid_path.exists() {
        return Err(anyhow::anyhow!("Daemon PID file not found"));
    }
    
    // Read PID from file
    let mut file = fs::File::open(pid_path)
        .context("Failed to open PID file")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read PID file")?;
    
    // Parse PID
    let pid = contents.trim().parse::<u32>()
        .context("Failed to parse PID from file")?;
    
    Ok(pid)
}

/// Kill the daemon process
pub fn kill_daemon() -> Result<()> {
    let pid = get_daemon_pid()?;
    
    info!("Killing daemon process with PID: {}", pid);
    
    // Send SIGTERM signal to process (platform-specific)
    #[cfg(target_family = "unix")]
    {
        // On Unix-like systems
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        
        kill(Pid::from_raw(pid as i32), Signal::SIGTERM)
            .context("Failed to send SIGTERM to daemon process")?;
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows
        let status = Command::new("taskkill")
            .arg("/PID")
            .arg(pid.to_string())
            .arg("/F")
            .status()
            .context("Failed to execute taskkill command")?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to kill daemon process"));
        }
    }
    
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        return Err(anyhow::anyhow!("Killing daemon not supported on this platform"));
    }
    
    // Remove PID file
    let pid_path = pid_file_path()?;
    if pid_path.exists() {
        fs::remove_file(pid_path)
            .context("Failed to remove PID file")?;
    }
    
    Ok(())
} 