#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
DEBUG="${DEBUG:-0}"

die() { echo "ERROR: $*" >&2; exit 1; }
[[ "${BASH_VERSION%%.*}" -ge 4 ]] || die "Bash 4+ required"

# Script to import route data from zwift-offline fork
# This maintains license separation by using HTTP API

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ZWIFT_OFFLINE_URL="${ZWIFT_OFFLINE_URL:-https://localhost:8443}"
OUTPUT_DIR="${OUTPUT_DIR:-$PROJECT_ROOT/data/zwift_offline_export}"

# Help text
usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Import route data from zwift-offline server via API

Options:
  -u, --url URL          zwift-offline server URL (default: https://localhost:8443)
  -o, --output DIR       Output directory for JSON files (default: data/zwift_offline_export)
  -s, --skip-ssl-verify  Skip SSL certificate verification (for self-signed certs)
  -h, --help            Show this help message

Examples:
  # Import from local zwift-offline (default - uses self-signed cert)
  $0 --skip-ssl-verify

  # Import from remote server
  $0 --url https://zwift-offline.example.com --skip-ssl-verify

Note: zwift-offline server must be running with route_export module enabled
EOF
}

# Parse command line arguments
SKIP_SSL=""
while [[ $# -gt 0 ]]; do
    case $1 in
        -u|--url)
            ZWIFT_OFFLINE_URL="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -s|--skip-ssl-verify)
            SKIP_SSL="-k"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            die "Unknown option: $1"
            ;;
    esac
done

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Importing route data from zwift-offline at $ZWIFT_OFFLINE_URL"
echo "Output directory: $OUTPUT_DIR"

# Function to fetch and save data
fetch_endpoint() {
    local endpoint="$1"
    local output_file="$2"
    local url="${ZWIFT_OFFLINE_URL}/api/export/${endpoint}"
    
    echo -n "Fetching $endpoint... "
    
    if curl $SKIP_SSL -s -f -o "$output_file" "$url"; then
        # Pretty-print the JSON
        if command -v jq >/dev/null 2>&1; then
            jq . "$output_file" > "${output_file}.tmp" && mv "${output_file}.tmp" "$output_file"
        fi
        echo "✓ $(jq -r '.count // 0' "$output_file" 2>/dev/null || echo "unknown") items"
    else
        echo "✗ Failed"
        return 1
    fi
}

# Test connection with summary endpoint
echo -n "Testing connection... "
if ! curl $SKIP_SSL -s -f "${ZWIFT_OFFLINE_URL}/api/export/summary" >/dev/null; then
    echo "✗"
    echo ""
    echo "Cannot connect to zwift-offline at $ZWIFT_OFFLINE_URL"
    echo ""
    echo "To start the server:"
    echo "  cd zwift-offline"
    echo "  ./run_server.sh"
    echo ""
    echo "Or if you haven't set it up yet:"
    echo "  cd zwift-offline"
    echo "  ./setup_venv.sh"
    echo "  ./run_server.sh"
    exit 1
fi
echo "✓"

# Fetch all endpoints
fetch_endpoint "summary" "$OUTPUT_DIR/summary.json"
fetch_endpoint "routes" "$OUTPUT_DIR/routes.json"
fetch_endpoint "start_lines" "$OUTPUT_DIR/start_lines.json"
fetch_endpoint "events" "$OUTPUT_DIR/events.json"

echo ""
echo "Import complete! Files saved to: $OUTPUT_DIR"
echo ""
echo "Next steps:"
echo "1. Review the exported data in $OUTPUT_DIR"
echo "2. Run the Rust import tool to update the database:"
echo "   cargo run --bin import_zwift_offline_routes -- --input-dir $OUTPUT_DIR"