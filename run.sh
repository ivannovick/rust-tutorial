#!/usr/bin/env bash
# Compile and run a lesson inside the rust:latest docker container.
#
# Usage:
#   ./run.sh 01-hello-world
#
# Works on Linux/macOS and on Windows Git Bash / WSL.
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <lesson-folder>"
    echo "Example: $0 01-hello-world"
    exit 1
fi

LESSON_DIR="$1"

if [ ! -d "$LESSON_DIR" ]; then
    echo "Error: directory '$LESSON_DIR' does not exist."
    exit 1
fi

if [ ! -f "$LESSON_DIR/main.rs" ]; then
    echo "Error: '$LESSON_DIR/main.rs' not found."
    exit 1
fi

echo "=== Building and running: $LESSON_DIR ==="

# Absolute path to the lesson directory (portable on Git Bash/WSL too)
ABS_DIR="$(cd "$LESSON_DIR" && pwd)"

docker run --rm \
    -v "$ABS_DIR":/app \
    -w /app \
    rust-tutorial:latest \
    sh -c "rustc main.rs -o /tmp/program 2>&1 && echo '--- program output ---' && /tmp/program"
