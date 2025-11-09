#!/bin/bash

# Build script for JOEL documentation

set -e

echo "📚 Building JOEL Documentation"
echo "================================"
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

# Build documentation
echo "🔨 Building documentation..."
cd "$(dirname "$0")"
mdbook build

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Documentation built successfully!"
    echo ""
    echo "📖 Output: docs/book/"
    echo ""
    echo "To serve locally:"
    echo "  cd docs && mdbook serve"
    echo ""
else
    echo ""
    echo "❌ Build failed"
    exit 1
fi

