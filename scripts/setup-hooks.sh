#!/bin/sh
# Setup git hooks for music-tools

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "📦 Setting up git hooks..."

# Configure git to use our hooks directory
git config core.hooksPath .githooks

echo "✅ Git hooks configured!"
echo ""
echo "Pre-commit hook will now run 'cargo fmt --check' before each commit."
echo "To bypass (not recommended): git commit --no-verify"

