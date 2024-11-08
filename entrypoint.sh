#!/bin/sh

# Use the environment variable SCRIPT_FILE, defaulting to "two_sum.rs" if not provided
SCRIPT_FILE="${SCRIPT_FILE:-two_sum.rs}"

# Print the script file for debugging
echo "Running file: $SCRIPT_FILE"

# Run cargo-watch with the specified or default file
exec cargo-watch -w /app/src/leet -x "script /app/src/leet/$SCRIPT_FILE"
