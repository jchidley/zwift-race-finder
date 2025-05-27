#!/bin/bash
# Fetch Zwift activities from Strava API using secure token storage
# Supports environment variables, keyring, and file-based storage

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_FILE="${SCRIPT_DIR}/strava_zwift_activities.json"

# Function to load tokens from environment
load_from_env() {
    if [[ -z "${STRAVA_ACCESS_TOKEN:-}" ]]; then
        return 1
    fi
    ACCESS_TOKEN="${STRAVA_ACCESS_TOKEN}"
    REFRESH_TOKEN="${STRAVA_REFRESH_TOKEN}"
    EXPIRES_AT="${STRAVA_EXPIRES_AT}"
    CLIENT_ID="${STRAVA_CLIENT_ID}"
    CLIENT_SECRET="${STRAVA_CLIENT_SECRET}"
    return 0
}

# Function to load tokens from keyring
load_from_keyring() {
    if ! command -v secret-tool &> /dev/null; then
        return 1
    fi
    
    # Try to load from keyring
    ACCESS_TOKEN=$(secret-tool lookup service zwift-race-finder username strava-access-token 2>/dev/null || true)
    if [[ -z "$ACCESS_TOKEN" ]]; then
        return 1
    fi
    
    REFRESH_TOKEN=$(secret-tool lookup service zwift-race-finder username strava-refresh-token 2>/dev/null || true)
    CLIENT_SECRET=$(secret-tool lookup service zwift-race-finder username strava-client-secret 2>/dev/null || true)
    
    # Load public data from config
    PUBLIC_CONFIG="${HOME}/.config/zwift-race-finder/strava_public.json"
    if [[ -f "$PUBLIC_CONFIG" ]]; then
        CLIENT_ID=$(jq -r '.client_id' "$PUBLIC_CONFIG")
        EXPIRES_AT=$(jq -r '.expires_at' "$PUBLIC_CONFIG")
    else
        return 1
    fi
    
    return 0
}

# Function to load tokens from file
load_from_file() {
    CONFIG_FILE="${SCRIPT_DIR}/strava_config.json"
    if [[ ! -f "$CONFIG_FILE" ]]; then
        return 1
    fi
    
    ACCESS_TOKEN=$(jq -r '.access_token' "$CONFIG_FILE")
    REFRESH_TOKEN=$(jq -r '.refresh_token' "$CONFIG_FILE")
    EXPIRES_AT=$(jq -r '.expires_at' "$CONFIG_FILE")
    CLIENT_ID=$(jq -r '.client_id' "$CONFIG_FILE")
    CLIENT_SECRET=$(jq -r '.client_secret' "$CONFIG_FILE")
    return 0
}

# Function to save updated tokens
save_tokens() {
    local new_access_token="$1"
    local new_refresh_token="$2"
    local new_expires_at="$3"
    
    # Save based on where we loaded from
    if [[ -n "${STRAVA_ACCESS_TOKEN:-}" ]]; then
        echo "⚠️  Token refreshed. Update your environment variables:"
        echo "export STRAVA_ACCESS_TOKEN=\"$new_access_token\""
        echo "export STRAVA_REFRESH_TOKEN=\"$new_refresh_token\""
        echo "export STRAVA_EXPIRES_AT=\"$new_expires_at\""
    elif command -v secret-tool &> /dev/null && secret-tool lookup service zwift-race-finder username strava-access-token &>/dev/null; then
        # Update keyring
        echo "$new_access_token" | secret-tool store --label="Strava Access Token" service zwift-race-finder username strava-access-token
        echo "$new_refresh_token" | secret-tool store --label="Strava Refresh Token" service zwift-race-finder username strava-refresh-token
        
        # Update public config
        PUBLIC_CONFIG="${HOME}/.config/zwift-race-finder/strava_public.json"
        jq ".expires_at = $new_expires_at" "$PUBLIC_CONFIG" > "${PUBLIC_CONFIG}.tmp" && \
            mv "${PUBLIC_CONFIG}.tmp" "$PUBLIC_CONFIG"
    else
        # Update file
        CONFIG_FILE="${SCRIPT_DIR}/strava_config.json"
        jq ".access_token = \"$new_access_token\" | 
            .refresh_token = \"$new_refresh_token\" | 
            .expires_at = $new_expires_at" "$CONFIG_FILE" > "${CONFIG_FILE}.tmp" && \
            mv "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
    fi
}

# Try to load tokens from various sources
echo "🔐 Loading Strava credentials..."
if load_from_env; then
    echo "✅ Using credentials from environment variables"
elif load_from_keyring; then
    echo "✅ Using credentials from system keyring"
elif load_from_file; then
    echo "✅ Using credentials from config file"
else
    echo "❌ No credentials found. Run ./strava_auth_secure.sh first"
    exit 1
fi

# Check if token expired
CURRENT_TIME=$(date +%s)
if [[ $CURRENT_TIME -gt $EXPIRES_AT ]]; then
    echo "🔄 Token expired, refreshing..."
    
    RESPONSE=$(curl -s -X POST https://www.strava.com/oauth/token \
        -F client_id="${CLIENT_ID}" \
        -F client_secret="${CLIENT_SECRET}" \
        -F grant_type=refresh_token \
        -F refresh_token="${REFRESH_TOKEN}")
    
    # Check if refresh was successful
    if echo "$RESPONSE" | jq -e '.access_token' > /dev/null; then
        # Update tokens
        ACCESS_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token')
        REFRESH_TOKEN=$(echo "$RESPONSE" | jq -r '.refresh_token')
        EXPIRES_AT=$(echo "$RESPONSE" | jq -r '.expires_at')
        
        # Save updated tokens
        save_tokens "$ACCESS_TOKEN" "$REFRESH_TOKEN" "$EXPIRES_AT"
        
        echo "✅ Token refreshed"
    else
        echo "❌ Token refresh failed:"
        echo "$RESPONSE" | jq '.'
        exit 1
    fi
fi

echo "🚴 Fetching Zwift activities from Strava"
echo "======================================="
echo ""

# Fetch activities (200 per page is max)
PER_PAGE=200
PAGE=1
ALL_ACTIVITIES=()

echo "Fetching activities..."
while true; do
    RESPONSE=$(curl -s -X GET "https://www.strava.com/api/v3/athlete/activities?page=${PAGE}&per_page=${PER_PAGE}" \
        -H "Authorization: Bearer ${ACCESS_TOKEN}")
    
    # Check if we got activities
    COUNT=$(echo "$RESPONSE" | jq '. | length')
    if [[ $COUNT -eq 0 ]]; then
        break
    fi
    
    echo "  Page $PAGE: Found $COUNT activities"
    
    # Filter for Zwift virtual rides
    ZWIFT_ACTIVITIES=$(echo "$RESPONSE" | jq '[
        .[] | 
        select(.type == "VirtualRide") |
        select(.name | contains("Zwift") or contains("Race") or contains("Group Ride")) |
        {
            id: .id,
            name: .name,
            start_date: .start_date,
            start_date_local: .start_date_local,
            distance: .distance,
            moving_time: .moving_time,
            elapsed_time: .elapsed_time,
            total_elevation_gain: .total_elevation_gain,
            average_speed: .average_speed,
            max_speed: .max_speed,
            average_watts: .average_watts,
            weighted_average_watts: .weighted_average_watts,
            kilojoules: .kilojoules,
            average_heartrate: .average_heartrate,
            max_heartrate: .max_heartrate,
            suffer_score: .suffer_score,
            description: .description,
            private: .private,
            flagged: .flagged,
            moving_time_minutes: (.moving_time / 60 | round),
            distance_km: (.distance / 1000 | round),
            average_speed_kmh: (.average_speed * 3.6 | round)
        }
    ]')
    
    # Add to collection
    ALL_ACTIVITIES+=("$ZWIFT_ACTIVITIES")
    
    # Check if we should continue
    if [[ $COUNT -lt $PER_PAGE ]]; then
        break
    fi
    
    PAGE=$((PAGE + 1))
done

# Combine all activities
echo ""
echo "Processing activities..."
COMBINED=$(echo "${ALL_ACTIVITIES[@]}" | jq -s 'add | sort_by(.start_date) | reverse')

# Filter for races (based on name patterns)
RACES=$(echo "$COMBINED" | jq '[
    .[] |
    select(
        (.name | test("Race|Racing|TT|Time Trial|Fondo"; "i")) and
        (.name | test("Group Ride|Easy|Recovery|Endurance|Zone 2|Meetup"; "i") | not)
    )
]')

# Save to file
echo "$RACES" > "$OUTPUT_FILE"

# Display summary
TOTAL_ZWIFT=$(echo "$COMBINED" | jq '. | length')
TOTAL_RACES=$(echo "$RACES" | jq '. | length')

echo ""
echo "✅ Activity fetch complete!"
echo ""
echo "📊 Summary:"
echo "  Total Zwift activities: $TOTAL_ZWIFT"
echo "  Likely races: $TOTAL_RACES"
echo ""
echo "📄 Race data saved to: $OUTPUT_FILE"
echo ""

# Show recent races
echo "🏁 Recent races:"
echo "$RACES" | jq -r '.[:5] | .[] | 
    "\(.start_date_local | split("T")[0]) - \(.name) - \(.moving_time_minutes)min - \(.distance_km)km"'

echo ""
echo "Next step: Run ./strava_import_to_db.sh to update the database"