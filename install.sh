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
echo "Basic usage: zwift-race-finder"
echo ""
echo "Common options:"
echo "  -s, --zwift-score      Target Zwift score (auto-detected or specify)"
echo "  -d, --duration         Target duration in minutes (default: 120)"
echo "  -t, --tolerance        Duration tolerance in minutes (default: 30)"
echo "  -e, --event-type       Filter: all, race, fondo, group, workout, tt (default: race)"
echo "  -n, --days             Show next N days (default: 1)"
echo ""
echo "Advanced options:"
echo "  --show-unknown-routes  Show routes that need mapping"
echo "  --discover-routes      Discover unknown routes from web"
echo "  --debug                Show why events are filtered out"
echo ""
echo "For full options: zwift-race-finder --help"