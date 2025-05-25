#!/bin/bash
# Install script for zwift-race-finder

set -e

echo "🚴 Installing Zwift Race Finder..."

# Build the release version
cargo build --release

# Copy to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/zwift-race-finder ~/.local/bin/

echo "✅ Installation complete!"
echo "Make sure ~/.local/bin is in your PATH"
echo ""
echo "Usage: zwift-race-finder [OPTIONS]"
echo "  -s, --zwift-score  Target Zwift score (default: 195)"
echo "  -d, --duration     Target duration in minutes (default: 120)"
echo "  -t, --tolerance    Duration tolerance in minutes (default: 30 for 1.5-2.5h range)"
echo "  -r, --races-only   Only show races (exclude group rides)"
echo "  -n, --days         Show next N days (default: 1)"