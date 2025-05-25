#!/bin/bash
# Strava OAuth authentication setup
# This script helps you authenticate with Strava and get access tokens

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/strava_config.json"

echo "🚴 Strava API Authentication Setup"
echo "=================================="
echo ""
echo "First, you need to create a Strava API application:"
echo "1. Go to: https://www.strava.com/settings/api"
echo "2. Create a new application"
echo "3. Set Authorization Callback Domain to: localhost"
echo "4. Note your Client ID and Client Secret"
echo ""

# Check if config exists
if [[ -f "$CONFIG_FILE" ]]; then
    echo "Found existing config at $CONFIG_FILE"
    read -p "Do you want to reconfigure? [y/N] " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

# Get client credentials
read -p "Enter your Strava Client ID: " CLIENT_ID
read -s -p "Enter your Strava Client Secret: " CLIENT_SECRET
echo ""

# Save initial config
cat > "$CONFIG_FILE" << EOF
{
  "client_id": "$CLIENT_ID",
  "client_secret": "$CLIENT_SECRET",
  "redirect_uri": "http://localhost:8080/exchange_token",
  "scope": "read,activity:read_all"
}
EOF

echo ""
echo "✅ Config saved to $CONFIG_FILE"
echo ""
echo "Next, we'll open your browser to authorize the app."
echo "After you authorize, you'll be redirected to localhost."
echo "Copy the 'code' parameter from the URL and paste it here."
echo ""

# Generate authorization URL
AUTH_URL="https://www.strava.com/oauth/authorize?client_id=${CLIENT_ID}&response_type=code&redirect_uri=http://localhost:8080/exchange_token&approval_prompt=force&scope=read,activity:read_all"

echo "Opening browser to: $AUTH_URL"
echo ""

# Try to open browser
if command -v xdg-open &> /dev/null; then
    xdg-open "$AUTH_URL"
elif command -v open &> /dev/null; then
    open "$AUTH_URL"
else
    echo "Please open this URL in your browser:"
    echo "$AUTH_URL"
fi

echo ""
echo "After authorizing, you'll see a URL like:"
echo "http://localhost:8080/exchange_token?state=&code=YOUR_CODE_HERE&scope=read,activity:read_all"
echo ""
read -p "Enter the authorization code: " AUTH_CODE

# Exchange code for access token
echo ""
echo "Exchanging code for access token..."

RESPONSE=$(curl -s -X POST https://www.strava.com/oauth/token \
  -F client_id="${CLIENT_ID}" \
  -F client_secret="${CLIENT_SECRET}" \
  -F code="${AUTH_CODE}" \
  -F grant_type=authorization_code)

# Check if successful
if echo "$RESPONSE" | jq -e '.access_token' > /dev/null; then
    # Extract tokens
    ACCESS_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token')
    REFRESH_TOKEN=$(echo "$RESPONSE" | jq -r '.refresh_token')
    EXPIRES_AT=$(echo "$RESPONSE" | jq -r '.expires_at')
    ATHLETE_ID=$(echo "$RESPONSE" | jq -r '.athlete.id')
    
    # Update config with tokens
    jq ". + {
        access_token: \"$ACCESS_TOKEN\",
        refresh_token: \"$REFRESH_TOKEN\",
        expires_at: $EXPIRES_AT,
        athlete_id: $ATHLETE_ID
    }" "$CONFIG_FILE" > "${CONFIG_FILE}.tmp" && mv "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
    
    echo "✅ Authentication successful!"
    echo ""
    echo "Athlete ID: $ATHLETE_ID"
    echo "Token expires: $(date -d @$EXPIRES_AT)"
    echo ""
    echo "You can now run: ./strava_fetch_activities.sh"
else
    echo "❌ Authentication failed:"
    echo "$RESPONSE" | jq '.'
    exit 1
fi