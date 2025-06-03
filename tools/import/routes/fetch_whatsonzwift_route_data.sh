#!/usr/bin/env bash
# ABOUTME: Fetches route data from WhatsOnZwift and updates database

set -euo pipefail
IFS=$'\n\t'

# Database path
DB_PATH="$HOME/.local/share/zwift-race-finder/races.db"

# Function to fetch and parse route data
fetch_route_data() {
    local world="$1"
    local slug="$2"
    local route_id="$3"
    local url="https://whatsonzwift.com/world/${world}/route/${slug}"
    
    # Transform slug to match WhatsOnZwift format
    local woz_slug="$slug"
    
    # Convert abbreviated reverse suffix to full
    if [[ "$slug" =~ -rev$ ]]; then
        woz_slug="${slug%-rev}-reverse"
    fi
    
    # Update URL with transformed slug
    url="https://whatsonzwift.com/world/${world}/route/${woz_slug}"
    
    echo "Fetching: $url"
    
    # Fetch the page
    local html=$(curl -s "$url" || echo "")
    
    if [ -z "$html" ]; then
        echo "  ✗ Failed to fetch page"
        return 1
    fi
    
    # Extract route stats from table format
    # Look for patterns like: <td><strong>Distance</strong></td><td>9.19 km / 5.71 mi</td>
    local distance_km=$(echo "$html" | grep '<strong>Distance</strong>' | sed -E 's/.*<td>([0-9.]+) km.*/\1/' | grep -E '^[0-9.]+$' | head -1)
    local elevation_m=$(echo "$html" | grep '<strong>Elevation gain</strong>' | sed -E 's/.*<td>([0-9.]+) m.*/\1/' | grep -E '^[0-9.]+$' | head -1)
    local lead_in_km=$(echo "$html" | grep '<strong>Lead-in distance</strong>' | sed -E 's/.*<td>([0-9.]+) km.*/\1/' | grep -E '^[0-9.]+$' | head -1)
    local lead_in_m=$(echo "$html" | grep '<strong>Lead-in elevation gain</strong>' | sed -E 's/.*<td>([0-9.]+) m.*/\1/' | grep -E '^[0-9.]+$' | head -1)
    
    echo "  Distance: ${distance_km:-N/A} km"
    echo "  Elevation: ${elevation_m:-N/A} m"
    echo "  Lead-in distance: ${lead_in_km:-N/A} km"
    echo "  Lead-in elevation: ${lead_in_m:-N/A} m"
    
    # Update database if we have data
    local updates=""
    if [ -n "$distance_km" ]; then
        updates="${updates}distance_km = $distance_km, "
    fi
    if [ -n "$elevation_m" ]; then
        updates="${updates}elevation_m = $elevation_m, "
    fi
    if [ -n "$lead_in_km" ]; then
        updates="${updates}lead_in_distance_km = $lead_in_km, "
    fi
    if [ -n "$lead_in_m" ]; then
        updates="${updates}lead_in_elevation_m = $lead_in_m, "
    fi
    
    # Remove trailing comma
    updates="${updates%, }"
    
    if [ -n "$updates" ]; then
        sqlite3 "$DB_PATH" "UPDATE routes SET $updates WHERE route_id = $route_id;"
        echo "  ✓ Updated route $route_id"
    else
        echo "  - No data to update"
    fi
}

# Main script
main() {
    local limit="${1:-10}"
    local route_id="${2:-}"
    
    echo "WhatsOnZwift Route Data Fetcher"
    echo "==============================="
    
    # Get routes to process
    local query
    if [ -n "$route_id" ]; then
        query="SELECT route_id, name, world, slug FROM routes WHERE route_id = $route_id AND slug IS NOT NULL;"
    else
        query="SELECT route_id, name, world, slug FROM routes WHERE slug IS NOT NULL AND (lead_in_distance_km = 0 OR lead_in_distance_km IS NULL) ORDER BY name LIMIT $limit;"
    fi
    
    # Process each route
    sqlite3 "$DB_PATH" "$query" | while IFS='|' read -r route_id name world slug; do
        echo
        echo "Processing: $name ($world/$slug)"
        fetch_route_data "$world" "$slug" "$route_id"
        
        # Be polite to the server
        sleep 1
    done
}

# Show usage if --help
if [[ "${1:-}" == "--help" ]]; then
    echo "Usage: $0 [limit] [route_id]"
    echo "  limit: Number of routes to process (default: 10)"
    echo "  route_id: Process specific route ID only"
    exit 0
fi

main "$@"