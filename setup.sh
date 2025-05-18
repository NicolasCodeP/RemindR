#!/bin/bash
# RemindR Setup Script

# Build all binaries
echo "Building RemindR binaries..."
cargo build --release || { echo "Build failed"; exit 1; }

# Make shell integration script executable
chmod +x shell_integration.sh

# Check if integration is already in shell config
ZSHRC_PATH="$HOME/.zshrc"
BASHRC_PATH="$HOME/.bashrc"

CURRENT_PATH="$(pwd)"
INTEGRATION_LINE="source $CURRENT_PATH/shell_integration.sh"

# Function to add integration to shell config
add_to_shell_config() {
    local config_file="$1"
    
    if [ -f "$config_file" ]; then
        if grep -q "shell_integration.sh" "$config_file"; then
            echo "Integration already exists in $config_file"
        else
            echo "Adding integration to $config_file"
            echo "" >> "$config_file"
            echo "# RemindR Shell Integration" >> "$config_file"
            echo "$INTEGRATION_LINE" >> "$config_file"
        fi
    else
        echo "Creating $config_file with RemindR integration"
        echo "# RemindR Shell Integration" > "$config_file"
        echo "$INTEGRATION_LINE" >> "$config_file"
    fi
}

# Detect shell and add integration
if [ -n "$ZSH_VERSION" ]; then
    echo "Detected Zsh shell"
    add_to_shell_config "$ZSHRC_PATH"
elif [ -n "$BASH_VERSION" ]; then
    echo "Detected Bash shell"
    add_to_shell_config "$BASHRC_PATH"
else
    echo "Neither Zsh nor Bash detected"
    echo "Please add the following line to your shell configuration file:"
    echo "$INTEGRATION_LINE"
fi

echo ""
echo "RemindR setup complete!"
echo "Please restart your terminal or run: source ~/.zshrc (or ~/.bashrc)"
echo ""
echo "To verify installation, run: cargo run --bin hist status" 