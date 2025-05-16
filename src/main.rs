use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{error, info};
use remindr::cli::commands;

#[derive(Parser)]
#[command(author, version, about = "Command Historization and Categorization App")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the daemon to begin recording commands
    On,
    
    /// Stop the daemon and cease recording commands
    Off,
    
    /// Display the current status and last commands
    Status,
    
    /// Search command history with optional keyword
    Search {
        /// Search keyword or phrase
        #[arg(default_value = "")]
        keyword: String,
    },
    
    /// Run a basic test to validate functionality (for development)
    Test,
}

fn main() -> Result<()> {
    // Initialize the application
    remindr::init().context("Failed to initialize application")?;
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Process commands
    match cli.command {
        Some(Commands::On) => {
            info!("Starting daemon...");
            commands::start_daemon().context("Failed to start daemon")?;
        }
        Some(Commands::Off) => {
            info!("Stopping daemon...");
            commands::stop_daemon().context("Failed to stop daemon")?;
        }
        Some(Commands::Status) => {
            commands::show_status().context("Failed to show status")?;
        }
        Some(Commands::Search { keyword }) => {
            commands::search_commands(&keyword).context("Failed to search commands")?;
        }
        Some(Commands::Test) => {
            run_basic_test()?;
        }
        None => {
            // Default behavior: show status if no command is provided
            commands::show_status().context("Failed to show status")?;
        }
    }

    Ok(())
}

/// Run a basic test to validate core functionality
fn run_basic_test() -> Result<()> {
    println!("RemindR Test");
    println!("===========");
    
    // Create a new command
    let cmd = remindr::db::Command::new("test command");
    println!("Command: {}", cmd.command);
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&cmd).unwrap();
    println!("\nJSON representation:");
    println!("{}", json);
    
    println!("\nCommand structure test passed!");
    Ok(())
} 