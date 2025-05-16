use remindr::db::Command;

fn main() {
    println!("RemindR Basic Usage Example");
    println!("==========================");
    
    // Create a new command
    let cmd = Command::new("cargo run --example basic_usage");
    println!("Created command: {}", cmd.command);
    println!("Timestamp: {}", cmd.timestamp);
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&cmd).unwrap();
    println!("\nJSON representation:");
    println!("{}", json);
    
    println!("\nThis is just a simple example that doesn't interact with the database.");
    println!("To use the full functionality, run the main application with 'cargo run'.");
} 