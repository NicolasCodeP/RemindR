use anyhow::Result;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    // Initialize the database
    remindr::init()?;
    
    // Get command from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return Ok(());
    }
    
    // Combine all arguments after the program name
    let command_str = args[1..].join(" ");
    
    // Execute the command
    println!("Executing: {}", command_str);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command_str])
            .output()?
    } else {
        Command::new("sh")
            .args(["-c", &command_str])
            .output()?
    };
    
    // Print command output
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Store the command
    remindr::db::store_command(&command_str)?;
    println!("Command saved to history database");
    
    Ok(())
} 