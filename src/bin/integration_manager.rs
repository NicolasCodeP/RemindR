use anyhow::Result;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_usage();
        return Ok(());
    }
    
    match args[1].as_str() {
        "status" => show_status()?,
        "enable" => enable_integration()?,
        "disable" => disable_integration()?,
        "toggle" => toggle_integration()?,
        "processes" => list_processes()?,
        "kill" => {
            if args.len() > 2 {
                kill_process(&args[2])?
            } else {
                kill_all_processes()?
            }
        },
        _ => show_usage(),
    }
    
    Ok(())
}

fn show_usage() {
    println!("RemindR Shell Integration Manager");
    println!("Usage:");
    println!("  integration_manager status   - Show integration status");
    println!("  integration_manager enable   - Enable shell integration");
    println!("  integration_manager disable  - Disable shell integration");
    println!("  integration_manager toggle   - Toggle integration on/off");
    println!("  integration_manager processes - List all RemindR processes");
    println!("  integration_manager kill     - Kill all RemindR processes");
    println!("  integration_manager kill <pid> - Kill specific process by PID");
}

fn get_integration_path() -> Result<PathBuf> {
    // Get current executable path
    let current_exe = env::current_exe()?;
    let bin_dir = current_exe.parent().unwrap();
    let project_root = bin_dir.parent().unwrap();
    
    Ok(project_root.join("shell_integration.sh"))
}

fn get_status_file_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?;
    Ok(home_dir.join(".remindr").join("integration_status"))
}

fn is_integration_enabled() -> Result<bool> {
    let status_file = get_status_file_path()?;
    
    if !status_file.exists() {
        return Ok(true); // Default to enabled if file doesn't exist
    }
    
    let mut file = File::open(status_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents.trim() == "enabled")
}

fn set_integration_status(enabled: bool) -> Result<()> {
    let status_file = get_status_file_path()?;
    
    // Ensure directory exists
    if let Some(parent) = status_file.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write status to file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(status_file)?;
    
    let status = if enabled { "enabled" } else { "disabled" };
    file.write_all(status.as_bytes())?;
    
    Ok(())
}

fn show_status() -> Result<()> {
    let enabled = is_integration_enabled()?;
    
    println!("RemindR Shell Integration Status");
    println!("-------------------------------");
    println!("Integration is currently: {}", if enabled { "ENABLED ✅" } else { "DISABLED ❌" });
    
    // Check environment variables to see if integration is loaded in current shell
    let shell_int_bin = env::var("SHELL_INTEGRATION_BIN").ok();
    
    if let Some(bin) = shell_int_bin {
        println!("Integration is loaded in current shell: ✅");
        println!("Using binary: {}", bin);
    } else {
        println!("Integration is NOT loaded in current shell: ❌");
        println!("Run 'source shell_integration.sh' to load it");
    }
    
    // Show database status
    remindr::init()?;
    let is_daemon_on = remindr::db::get_daemon_status()?;
    let last_commands = remindr::db::get_last_commands(3)?;
    
    println!("\nDatabase Status");
    println!("--------------");
    println!("Daemon status: {}", if is_daemon_on { "ON" } else { "OFF" });
    println!("Commands in database: {}", last_commands.len());
    println!("Last command: {}", if !last_commands.is_empty() {
        &last_commands[0].command
    } else {
        "None"
    });
    
    Ok(())
}

fn enable_integration() -> Result<()> {
    set_integration_status(true)?;
    
    // Get shell integration script path
    let script_path = get_integration_path()?;
    
    println!("Shell integration enabled");
    println!("To load it in the current shell, run:");
    println!("source {}", script_path.display());
    
    Ok(())
}

fn disable_integration() -> Result<()> {
    set_integration_status(false)?;
    
    println!("Shell integration disabled");
    println!("Note: It's still loaded in the current shell.");
    println!("To completely unload it, please start a new terminal.");
    
    Ok(())
}

fn toggle_integration() -> Result<()> {
    let currently_enabled = is_integration_enabled()?;
    
    if currently_enabled {
        disable_integration()
    } else {
        enable_integration()
    }
}

fn list_processes() -> Result<()> {
    println!("RemindR Processes");
    println!("-----------------");
    
    // Get list of running processes
    let output = if cfg!(target_os = "windows") {
        Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq *remindr*", "/NH"])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("ps -ef | grep -i '[r]emindr\\|[h]ist\\|[s]hell_integration' | grep -v grep")
            .output()?
    };
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    if output_str.trim().is_empty() {
        println!("No RemindR processes found");
        return Ok(());
    }
    
    println!("{}", output_str);
    
    // If on macOS/Linux, parse the output to extract PIDs
    if !cfg!(target_os = "windows") {
        println!("\nTo kill a process: integration_manager kill <PID>");
        println!("To kill all processes: integration_manager kill");
    }
    
    Ok(())
}

fn kill_process(pid: &str) -> Result<()> {
    println!("Killing process with PID: {}", pid);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("taskkill")
            .args(["/PID", pid, "/F"])
            .output()?
    } else {
        Command::new("kill")
            .arg("-9")
            .arg(pid)
            .output()?
    };
    
    if output.status.success() {
        println!("Process killed successfully");
    } else {
        println!("Failed to kill process:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

fn kill_all_processes() -> Result<()> {
    println!("Killing all RemindR processes...");
    
    if cfg!(target_os = "windows") {
        // Kill all remindr processes on Windows
        let output = Command::new("taskkill")
            .args(["/FI", "IMAGENAME eq *remindr*", "/F"])
            .output()?;
            
        if output.status.success() {
            println!("All processes killed successfully");
        } else {
            println!("Failed to kill processes:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        // On Unix-like systems, get PIDs and kill them
        let output = Command::new("sh")
            .arg("-c")
            .arg("ps -ef | grep -i '[r]emindr\\|[h]ist\\|[s]hell_integration' | grep -v grep | awk '{print $2}'")
            .output()?;
            
        let pids = String::from_utf8_lossy(&output.stdout);
        
        if pids.trim().is_empty() {
            println!("No RemindR processes found");
            return Ok(());
        }
        
        for pid in pids.split_whitespace() {
            kill_process(pid)?;
        }
        
        println!("All RemindR processes killed");
    }
    
    Ok(())
} 