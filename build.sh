#!/bin/bash

# Swiss Army Suite Build Script
# Advanced Security Toolkit v2.0

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║         🔥 LEGEND DORKER - Build Script 🔥                    ║"
echo "║                  Version 3.0.0                                ║"
echo "║              Made by @LEGEND_BL                               ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if Rust is installed
echo -e "${CYAN}[1/5] Checking Rust installation...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found!${NC}"
    echo ""
    echo "Please install Rust from: https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo -e "${GREEN}✓ Found: $RUST_VERSION${NC}"
echo ""

# Clean previous builds
echo -e "${CYAN}[2/5] Cleaning previous builds...${NC}"
cargo clean 2>/dev/null || true
echo -e "${GREEN}✓ Cleanup complete${NC}"
echo ""

# Update dependencies
echo -e "${CYAN}[3/5] Updating dependencies...${NC}"
cargo update
echo -e "${GREEN}✓ Dependencies updated${NC}"
echo ""

# Build in release mode
echo -e "${CYAN}[4/5] Building in release mode (this may take a while)...${NC}"
echo -e "${YELLOW}⚙  Compiling with optimizations...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi
echo ""

# Copy binary to current directory
echo -e "${CYAN}[5/5] Creating executable...${NC}"

if [ -f "target/release/legend-dorker" ]; then
    cp target/release/legend-dorker ./LegendDorker
    chmod +x LegendDorker
    echo -e "${GREEN}✓ Executable created: LegendDorker${NC}"
elif [ -f "target/release/legend-dorker.exe" ]; then
    cp target/release/legend-dorker.exe ./LegendDorker.exe
    echo -e "${GREEN}✓ Executable created: LegendDorker.exe${NC}"
else
    echo -e "${RED}❌ Could not find compiled binary${NC}"
    exit 1
fi
echo ""

# Get file size
if [ -f "./LegendDorker" ]; then
    SIZE=$(ls -lh LegendDorker | awk '{print $5}')
    echo -e "${GREEN}✓ Binary size: $SIZE${NC}"
fi
echo ""

# Success message
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║           🔥 BUILD SUCCESSFUL! 🔥                             ║"
echo "║          LEGEND DORKER v3.0 Ready                             ║"
echo "║          Made by @LEGEND_BL                                   ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}To run the application:${NC}"
echo -e "  ${CYAN}./LegendDorker${NC}"
echo ""
echo -e "${YELLOW}⚠  For Educational and Authorized Testing Only!${NC}"
echo -e "${GREEN}📧 Contact: sarthakgrid1@gmail.com${NC}"
echo -e "${GREEN}📱 Instagram: @sar_thak106${NC}"
echo ""
