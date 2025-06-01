#!/usr/bin/env bash
# ABOUTME: Debug script to discover hidden event tags in Zwift API data
# Usage: ./debug_event_tags.sh

set -euo pipefail
IFS=$'\n\t'

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Error handling
die() { echo "ERROR: $*" >&2; exit 1; }

# Fetch raw events and look for potential hidden tags
fetch_and_analyze() {
    echo "Fetching raw event data to discover hidden tags..."
    
    # Get the raw JSON
    local json_file="${SCRIPT_DIR}/debug_event_tags.json"
    curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" > "$json_file" || die "Failed to fetch events"
    
    echo "Analyzing event structure for hidden fields..."
    echo
    
    # Look for all unique keys in the JSON
    echo "=== All unique keys in events ==="
    jq -r '.[0] | keys[]' "$json_file" 2>/dev/null | sort -u
    
    echo
    echo "=== Looking for tag-like fields ==="
    jq -r '.[] | select(.tags != null) | {name: .name, tags: .tags}' "$json_file" 2>/dev/null || echo "No 'tags' field found"
    
    echo
    echo "=== Checking for hidden fields (fields not commonly known) ==="
    # Check for fields that might contain categorization data
    local common_fields="id name description startTime durationInSeconds distanceInMeters sport eventType route_id"
    
    jq -r '.[0] | to_entries | .[] | select(.key as $k | ["id","name","description","startTime","durationInSeconds","distanceInMeters","sport","eventType","route_id","event_sub_groups","elevation_gain_in_meters","entrants","event_series_id","map_id","total_entrant_count"] | index($k) | not) | .key' "$json_file" 2>/dev/null | sort -u
    
    echo
    echo "=== Sample event with all fields ==="
    jq '.[0]' "$json_file" 2>/dev/null
    
    echo
    echo "=== Looking for series/category indicators ==="
    jq -r '.[] | select(.event_series_id != null) | {name: .name, series_id: .event_series_id, type: .eventType}' "$json_file" 2>/dev/null | head -10
    
    echo
    echo "Raw JSON saved to: $json_file"
}

# Main execution
main() {
    fetch_and_analyze
}

# Execute main
main "$@"