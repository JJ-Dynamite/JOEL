#!/bin/bash

# JOEL Examples Test Script
# This script validates example files and shows expected outputs

set -e

echo "🧪 Testing JOEL Examples"
echo "========================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if joel binary exists
if [ -f "target/release/joel" ]; then
    JOEL_CMD="./target/release/joel"
elif command -v joel &> /dev/null; then
    JOEL_CMD="joel"
else
    echo -e "${YELLOW}⚠️  joel binary not found${NC}"
    echo "Building joel..."
    cargo build --release
    JOEL_CMD="./target/release/joel"
fi

# Test files
EXAMPLES=(
    "examples/hello.joel"
    "examples/arithmetic.joel"
    "examples/control_flow.joel"
)

# Test each example
for example in "${EXAMPLES[@]}"; do
    if [ ! -f "$example" ]; then
        echo -e "${RED}❌ File not found: $example${NC}"
        continue
    fi
    
    echo -e "${GREEN}Testing: $example${NC}"
    echo "----------------------------------------"
    
    # Check if file has proper header
    if grep -q "^\[Interpreted\]" "$example" || grep -q "^\[Compiled\]" "$example"; then
        echo "✅ Has proper header"
    else
        echo -e "${RED}❌ Missing [Interpreted] or [Compiled] header${NC}"
    fi
    
    # Try to run it
    if $JOEL_CMD run "$example" 2>&1; then
        echo -e "${GREEN}✅ $example executed successfully${NC}"
    else
        echo -e "${RED}❌ $example failed to execute${NC}"
    fi
    
    echo ""
done

echo "========================="
echo -e "${GREEN}Tests complete!${NC}"

