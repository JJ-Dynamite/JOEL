#!/bin/bash

# Serve script for JOEL documentation

set -e

echo "📚 Serving JOEL Documentation"
echo "============================"
echo ""

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null; then
    echo "❌ mdbook not found"
    echo ""
    echo "Install mdbook with:"
    echo "  cargo install mdbook"
    echo ""
    exit 1
fi

# Serve documentation
echo "🚀 Starting documentation server..."
echo ""
echo "📖 Documentation will be available at:"
echo "   http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop"
echo ""

cd "$(dirname "$0")"
mdbook serve --open

