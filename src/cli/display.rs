use crate::db::Command;
use std::io::{self, Write};

// Terminal width for display (fallback if unable to detect)
const DEFAULT_TERM_WIDTH: usize = 80;

/// Get the terminal width
fn get_terminal_width() -> usize {
    // Try to get actual terminal size
    if let Some((width, _)) = term_size::dimensions() {
        width
    } else {
        DEFAULT_TERM_WIDTH
    }
}

/// Format a string to fit within terminal width
fn format_to_width(s: &str, width: usize) -> String {
    if s.len() <= width {
        s.to_string()
    } else {
        format!("{}...", &s[0..width - 3])
    }
}

/// Draw a horizontal line using specified character
fn draw_line(width: usize, ch: char) {
    println!("{}", ch.to_string().repeat(width));
}

/// Draw a bordered text line with padding
fn draw_bordered_text(text: &str, width: usize) {
    let inner_width = width - 4; // Account for borders and padding
    let formatted_text = format_to_width(text, inner_width);
    println!("│ {} {} │", formatted_text, " ".repeat(inner_width - formatted_text.len()));
}

/// Draw a centered header
fn draw_header(title: &str, width: usize) {
    let inner_width = width - 4;
    let padding = (inner_width - title.len()) / 2;
    let left_padding = padding;
    let right_padding = inner_width - title.len() - left_padding;
    
    println!("┌{}┐", "─".repeat(width - 2));
    println!("│ {}{}{} │", " ".repeat(left_padding), title, " ".repeat(right_padding));
    println!("├{}┤", "─".repeat(width - 2));
}

/// Draw a footer
fn draw_footer(width: usize) {
    println!("└{}┘", "─".repeat(width - 2));
}

/// Format a command for display
fn format_command(command: &Command, width: usize) -> String {
    let timestamp = command.timestamp.format("%Y-%m-%d %H:%M:%S");
    let max_cmd_width = width - 22; // Account for timestamp and formatting
    
    let cmd_display = format_to_width(&command.command, max_cmd_width);
    format!("{} | {}", timestamp, cmd_display)
}

/// Show the main status screen
pub fn show_status_screen(status: &str, commands: &[Command]) {
    let width = get_terminal_width();
    
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    
    // Draw header
    draw_header("Command Historizer Daemon", width);
    
    // Show status
    draw_bordered_text(&format!("Daemon Status: [{}]", status), width);
    draw_bordered_text("", width);
    
    // Show last commands section
    draw_bordered_text("Last Commands:", width);
    
    if commands.is_empty() {
        draw_bordered_text("No commands recorded yet", width);
    } else {
        for command in commands {
            draw_bordered_text(&format_command(command, width - 4), width);
        }
    }
    
    // Draw footer with options
    draw_bordered_text("", width);
    let options = if status == "ON" {
        "[OFF] Turn off daemon | [SEARCH] Search commands"
    } else {
        "[ON] Turn on daemon | [SEARCH] Search commands"
    };
    draw_bordered_text(options, width);
    
    draw_footer(width);
}

/// Show the search results screen
pub fn show_search_screen(status: &str, keyword: &str, commands: &[Command]) {
    let width = get_terminal_width();
    
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    
    // Draw header
    draw_header("Search Commands", width);
    
    // Show search info
    if keyword.is_empty() {
        draw_bordered_text("Showing most recent commands", width);
    } else {
        draw_bordered_text(&format!("Search results for: {}", keyword), width);
    }
    draw_bordered_text("", width);
    
    // Show results
    draw_bordered_text("Results:", width);
    
    if commands.is_empty() {
        draw_bordered_text("No matching commands found", width);
    } else {
        for command in commands {
            draw_bordered_text(&format_command(command, width - 4), width);
        }
    }
    
    // Draw footer
    draw_bordered_text("", width);
    draw_bordered_text("[STATUS] Return to status screen", width);
    
    draw_footer(width);
} 