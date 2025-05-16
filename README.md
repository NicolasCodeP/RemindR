# RemindR - Command History and Categorization Tool

RemindR is a Rust-based command history and categorization tool that records, categorizes, and makes your shell command history searchable.

## Current Status

This is an initial working version with the following features:

- Command storage in a SQLite database
- Command retrieval and display with a formatted terminal UI
- Command search functionality
- Status display

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

3. The binary will be available at `target/release/hist`

## Usage

### Basic Commands

- Show current status and last commands:
  ```
  hist status
  ```

- Turn on daemon (still in development):
  ```
  hist on
  ```

- Turn off daemon:
  ```
  hist off
  ```

- Search commands:
  ```
  hist search <keyword>
  ```

### Testing Functionality

While the daemon mode is being developed, you can test the basic functionality using the test program:

```
cargo run --bin simple_test
```

This will insert some test commands into the database and demonstrate basic retrieval and search functionality.

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

## Future Improvements

1. Robust shell history monitoring
2. Command categorization using AI/NLP
3. Command tagging and organization
4. Suggestions based on command history
5. Web interface for viewing and searching commands

## License

This project is licensed under the MIT License - see the LICENSE file for details. 