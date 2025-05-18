# RemindR - Command History and Categorization Tool

RemindR is a Rust-based command history and categorization tool that records, categorizes, and makes your shell command history searchable.

## Current Status

This is an initial working version with the following features:

- Command storage in a SQLite database
- Command retrieval and display with a formatted terminal UI
- Command search functionality
- Status display
- Multiple ways to capture and save commands
- Shell integration for automatic command capture

The daemon functionality for automatic command tracking is still in development.

## Installation

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/RemindR.git
   cd RemindR
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binaries will be available in `target/release/`

### Quick Setup (Recommended)

The easiest way to set up RemindR with shell integration is to use the provided setup script:

```
./setup.sh
```

This script will:
1. Build all binaries in release mode
2. Make the shell integration script executable
3. Add the shell integration to your `.zshrc` or `.bashrc`
4. Provide instructions for completing the setup

After running the setup script, either restart your terminal or run:
```
source ~/.zshrc  # or ~/.bashrc for Bash
```

## Usage

### Command Reference

RemindR provides several commands for interacting with your command history:

#### Main Commands (`hist`)

- Show current status and last commands:
  ```
  hist status
  ```

- Search commands:
  ```
  hist search <keyword>
  ```
  This command searches your command history for the specified keyword and displays matching results.
  
- Search without a keyword to see all commands:
  ```
  hist search
  ```
  This displays your entire command history.

- Turn daemon on/off (legacy feature, not needed with shell integration):
  ```
  hist on    # Start recording commands via daemon
  hist off   # Stop recording commands via daemon
  ```

#### Shell Integration Management (`integration_manager`)

These commands control the behavior of the shell integration:

- Check integration status:
  ```
  ./target/release/integration_manager status
  ```
  Shows whether integration is enabled, loaded in current shell, and displays basic database statistics.

- Enable command capturing:
  ```
  ./target/release/integration_manager enable
  ```
  Turns on command capturing for all future shells.

- Disable command capturing:
  ```
  ./target/release/integration_manager disable
  ```
  Turns off command capturing for all future shells.

- Toggle integration state:
  ```
  ./target/release/integration_manager toggle
  ```
  Switches between enabled and disabled states.

- List all RemindR processes:
  ```
  ./target/release/integration_manager processes
  ```
  Shows all currently running RemindR-related processes with their PIDs.

- Kill all RemindR processes:
  ```
  ./target/release/integration_manager kill
  ```
  Terminates all running RemindR processes.

- Kill a specific process:
  ```
  ./target/release/integration_manager kill <pid>
  ```
  Terminates a specific RemindR process by PID.

#### Command Execution Wrapper (`save_command`)

- Run and save a command:
  ```
  ./target/release/save_command "your command here"
  ```
  Executes the command and simultaneously saves it to the RemindR database.

- Example:
  ```
  ./target/release/save_command "ls -la | grep '.rs'"
  ```
  This will list all Rust files in the current directory and save the command to history.

#### Shell Integration Direct Usage

- Save a specific command manually:
  ```
  HIST_LAST_CMD="command to save" ./target/release/shell_integration
  ```
  This method can be useful for scripts or for manually adding commands that weren't captured automatically.

### Creating Aliases for Convenience

Add these aliases to your `.zshrc` or `.bashrc` file for easier access:

```bash
# Main commands
alias hist-status="~/path/to/RemindR/target/release/hist status"
alias hist-search="~/path/to/RemindR/target/release/hist search"

# Integration management
alias remindr-status="~/path/to/RemindR/target/release/integration_manager status"
alias remindr-on="~/path/to/RemindR/target/release/integration_manager enable"
alias remindr-off="~/path/to/RemindR/target/release/integration_manager disable"
alias remindr-toggle="~/path/to/RemindR/target/release/integration_manager toggle"
alias remindr-processes="~/path/to/RemindR/target/release/integration_manager processes"
alias remindr-kill="~/path/to/RemindR/target/release/integration_manager kill"

# Command wrapper
alias save-cmd="~/path/to/RemindR/target/release/save_command"
```

After adding these aliases and reloading your shell configuration, you can use simplified commands like:
- `hist-status` - Check daemon status and recent commands
- `hist-search git` - Search for git commands in your history
- `remindr-on` - Enable command capturing
- `remindr-off` - Disable command capturing
- `save-cmd "complex command"` - Run and save a specific command

## Command Saving Mechanisms

RemindR offers multiple ways to save commands to the database:

### 1. Shell Integration (Recommended)

The shell integration automatically captures all commands you type in your terminal. This is the most seamless way to use RemindR.

#### How Shell Integration Works

The shell integration:
- Hooks into your shell's command execution cycle
- Captures each command after you run it
- Filters out duplicates and system commands
- Saves commands to the RemindR database

#### Manual Setup

If you prefer to set up the shell integration manually:

1. Build the shell integration binary:
   ```
   cargo build --bin shell_integration
   ```

2. Make the script executable:
   ```
   chmod +x shell_integration.sh
   ```

3. Add to your shell configuration:
   ```
   # For Zsh
   echo "source $(pwd)/shell_integration.sh" >> ~/.zshrc
   
   # For Bash
   echo "source $(pwd)/shell_integration.sh" >> ~/.bashrc
   ```

4. Reload your configuration:
   ```
   source ~/.zshrc  # or ~/.bashrc
   ```

### 2. Command Wrapper

The `save_command` utility lets you run commands while saving them to the database:

```
./target/release/save_command "your command here"
```

Example:
```
./target/release/save_command "find . -name '*.rs'"
```

This executes the command and saves it to the database in one step.

### 3. Direct Storage

For scripting or programmatic use, you can directly store commands:

```bash
# Set the environment variable and run the shell integration binary
HIST_LAST_CMD="your command" ./target/release/shell_integration
```

### 4. Legacy Daemon Mode

The original daemon approach (using `hist on` and `hist off`) is still available but not recommended. The shell integration provides a more reliable method for capturing commands.

## Shell Integration Management

RemindR includes a dedicated tool to manage the shell integration:

```
# Show current integration status
./target/debug/integration_manager status

# Enable integration
./target/debug/integration_manager enable

# Disable integration
./target/debug/integration_manager disable

# Toggle integration on/off
./target/debug/integration_manager toggle

# List all RemindR processes
./target/debug/integration_manager processes

# Kill all RemindR processes
./target/debug/integration_manager kill

# Kill a specific process by PID
./target/debug/integration_manager kill <pid>
```

When the integration is disabled, commands will no longer be saved to the database, even if the integration script is loaded in your shell.

You can also add an alias to your shell configuration for easier management:

```bash
# Add to .zshrc or .bashrc
alias remindr_status="~/path/to/RemindR/target/debug/integration_manager status"
alias remindr_on="~/path/to/RemindR/target/debug/integration_manager enable"
alias remindr_off="~/path/to/RemindR/target/debug/integration_manager disable"
alias remindr_toggle="~/path/to/RemindR/target/debug/integration_manager toggle"
```

After adding these aliases, you can use:
- `remindr_status` - Check if integration is enabled
- `remindr_on` - Turn on command saving 
- `remindr_off` - Turn off command saving
- `remindr_toggle` - Toggle between on and off

## Troubleshooting

### Shell Integration Not Working

If commands aren't being saved:

1. Verify the shell integration is loaded:
   ```
   echo $SHELL_INTEGRATION_BIN
   ```
   Should display the path to the binary

2. Check the shell integration binary exists:
   ```
   ls -l $SHELL_INTEGRATION_BIN
   ```

3. Manually run the shell integration with a test command:
   ```
   HIST_LAST_CMD="test command" ./target/release/shell_integration
   ```

4. Check the database for the test command:
   ```
   cargo run --bin hist status
   ```

### Duplicate Commands

If you see duplicate commands, make sure you're using the latest version with duplicate prevention. You can reset your integration by:

```
source shell_integration.sh
```

You should see: "RemindR shell integration loaded (with duplicate prevention)"

## Database

RemindR uses a SQLite database stored at `~/.remindr/history.db`. The database structure includes:

- `commands` table: Stores command history with timestamps
- `daemon_status` table: Tracks whether the daemon is active

## Project Structure

- `src/`
  - `main.rs`: CLI entry point
  - `lib.rs`: Library exports
  - `cli/`: Command-line interface functionality
  - `daemon/`: Daemon and background process management
  - `db/`: Database operations and models
  - `utils/`: Utility functions
  - `bin/`: Additional binaries
    - `simple_test.rs`: Test program
    - `save_command.rs`: Command execution wrapper
    - `shell_integration.rs`: Shell history tracking
- `shell_integration.sh`: Script for hooking into your shell
- `setup.sh`: One-click setup script

## Future Improvements

1. Enhanced shell history monitoring
2. Command categorization using AI/NLP
3. Command tagging and organization
4. Suggestions based on command history
5. Web interface for viewing and searching commands

## License

This project is licensed under the MIT License - see the LICENSE file for details. 