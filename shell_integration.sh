#!/bin/bash
# RemindR Shell Integration
# Add to your .bashrc or .zshrc by adding: source /path/to/shell_integration.sh

# Get the absolute path to the shell_integration binary
REMINDR_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHELL_INTEGRATION_BIN="${REMINDR_PATH}/target/debug/shell_integration"

# If debug build doesn't exist, try release build
if [ ! -f "$SHELL_INTEGRATION_BIN" ]; then
    SHELL_INTEGRATION_BIN="${REMINDR_PATH}/target/release/shell_integration"
fi

# Variable to store the last command
LAST_SAVED_COMMAND=""

# Status file path
INTEGRATION_STATUS_FILE="$HOME/.remindr/integration_status"

# Function to check if integration is enabled
is_integration_enabled() {
    # Default to enabled if file doesn't exist
    if [ ! -f "$INTEGRATION_STATUS_FILE" ]; then
        return 0  # True in bash
    fi
    
    # Check file content
    local status=$(cat "$INTEGRATION_STATUS_FILE")
    if [ "$status" = "enabled" ]; then
        return 0  # True
    else
        return 1  # False
    fi
}

# Make sure the binary exists
if [ ! -f "$SHELL_INTEGRATION_BIN" ]; then
    echo "RemindR shell_integration binary not found at $SHELL_INTEGRATION_BIN"
    echo "Please build it first with: cargo build --bin shell_integration"
    return 1
fi

# Echo the full path of the binary we're using
echo "Using RemindR shell_integration binary: $SHELL_INTEGRATION_BIN"

# Function to save the last command to the database
function save_command_to_remindr() {
    # Skip if integration is disabled
    is_integration_enabled || return
    
    # Get the last command from history
    local last_cmd=$(fc -ln -1)
    
    # Skip if it's the same as the last saved command (avoid duplicates)
    if [ "$LAST_SAVED_COMMAND" = "$last_cmd" ]; then
        return
    fi
    
    # Skip the 'save_command_to_remindr' command itself
    if [[ "$last_cmd" == *"save_command_to_remindr"* ]]; then
        return
    fi
    
    # Save this command as the last saved command
    LAST_SAVED_COMMAND="$last_cmd"
    
    # Export the command for the shell_integration tool
    export HIST_LAST_CMD="$last_cmd"
    
    # Run the shell_integration tool to save the command
    "$SHELL_INTEGRATION_BIN" > /dev/null 2>&1
}

# For bash
if [ -n "$BASH_VERSION" ]; then
    # Set up trap to run after each command
    if [[ -z "$PROMPT_COMMAND" ]]; then
        PROMPT_COMMAND="save_command_to_remindr"
    else
        PROMPT_COMMAND="$PROMPT_COMMAND;save_command_to_remindr"
    fi
fi

# For zsh
if [ -n "$ZSH_VERSION" ]; then
    # Add precmd hook
    autoload -Uz add-zsh-hook
    add-zsh-hook precmd save_command_to_remindr
fi

# Show integration status
if is_integration_enabled; then
    echo "RemindR shell integration loaded and ENABLED ✅"
    echo "Use 'integration_manager disable' to turn it off"
else
    echo "RemindR shell integration loaded but DISABLED ❌"
    echo "Use 'integration_manager enable' to turn it on"
fi 