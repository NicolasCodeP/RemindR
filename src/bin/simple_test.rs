use anyhow::Result;
use remindr::db;
use remindr::db::Command;

fn main() -> Result<()> {
    println!("RemindR Database Test");
    println!("====================");
    
    // Initialize the database
    remindr::init()?;
    
    // Insert some test commands
    println!("\nInserting test commands...");
    db::store_command("ls -la")?;
    db::store_command("cargo build")?;
    db::store_command("git commit -m 'Add new feature'")?;
    db::store_command("echo 'Hello, world!'")?;
    db::store_command("cat ~/.zshrc")?;
    
    // Wait a moment for commands to be stored
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Retrieve and display the commands
    println!("\nRetrieving last 5 commands:");
    let commands = db::get_last_commands(5)?;
    
    for cmd in &commands {
        println!("ID: {}, Command: {}", cmd.id.unwrap_or(0), cmd.command);
    }
    
    // Search for a command
    println!("\nSearching for 'git' commands:");
    let search_results = db::search_commands("git")?;
    
    for cmd in &search_results {
        println!("ID: {}, Command: {}", cmd.id.unwrap_or(0), cmd.command);
    }
    
    println!("\nDatabase test completed successfully!");
    Ok(())
} 