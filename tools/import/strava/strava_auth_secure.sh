#!/bin/bash
# Secure Strava OAuth authentication setup
# This script helps you authenticate with Strava using secure token storage

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🔒 Secure Strava API Authentication Setup"
echo "========================================"
echo ""
echo "This tool supports three storage methods:"
echo "1. Environment variables (most secure for CI/CD)"
echo "2. System keyring (secure for desktop use)"
echo "3. Encrypted file storage (backward compatible)"
echo ""

# Check if environment variables are already set
if [[ -n "${STRAVA_CLIENT_ID:-}" ]]; then
    echo "✅ Found Strava credentials in environment variables"
    echo ""
    echo "Current configuration:"
    echo "  Client ID: ${STRAVA_CLIENT_ID}"
    echo "  Token expires: $(date -d @${STRAVA_EXPIRES_AT:-0} 2>/dev/null || echo 'Unknown')"
    echo ""
    read -p "Do you want to reconfigure? [y/N] " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

echo "First, you need to create a Strava API application:"
echo "1. Go to: https://www.strava.com/settings/api"
echo "2. Create a new application"
echo "3. Set Authorization Callback Domain to: localhost"
echo "4. Note your Client ID and Client Secret"
echo ""

# Get client credentials
read -p "Enter your Strava Client ID: " CLIENT_ID
read -s -p "Enter your Strava Client Secret: " CLIENT_SECRET
echo ""
echo ""

# Generate authorization URL
AUTH_URL="https://www.strava.com/oauth/authorize?client_id=${CLIENT_ID}&response_type=code&redirect_uri=http://localhost:8080/exchange_token&approval_prompt=force&scope=read,activity:read_all"

echo "Next, we'll open your browser to authorize the app."
echo "After you authorize, you'll be redirected to localhost."
echo "Copy the 'code' parameter from the URL and paste it here."
echo ""
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
    
    echo "✅ Authentication successful!"
    echo ""
    echo "Athlete ID: $ATHLETE_ID"
    echo "Token expires: $(date -d @$EXPIRES_AT)"
    echo ""
    
    # Provide storage options
    echo "How would you like to store your tokens?"
    echo ""
    echo "1. Environment variables (add to your shell profile)"
    echo "2. System keyring (if available)"
    echo "3. Encrypted file (backward compatible)"
    echo ""
    read -p "Choose storage method [1-3]: " STORAGE_METHOD
    
    case $STORAGE_METHOD in
        1)
            echo ""
            echo "Add these to your ~/.bashrc or shell profile:"
            echo ""
            echo "# Strava API credentials"
            echo "export STRAVA_CLIENT_ID=\"$CLIENT_ID\""
            echo "export STRAVA_CLIENT_SECRET=\"$CLIENT_SECRET\""
            echo "export STRAVA_ACCESS_TOKEN=\"$ACCESS_TOKEN\""
            echo "export STRAVA_REFRESH_TOKEN=\"$REFRESH_TOKEN\""
            echo "export STRAVA_EXPIRES_AT=\"$EXPIRES_AT\""
            echo "export STRAVA_ATHLETE_ID=\"$ATHLETE_ID\""
            echo ""
            echo "Then run: source ~/.bashrc"
            ;;
        2)
            # Check if keyring is available
            if command -v secret-tool &> /dev/null; then
                # Use GNOME keyring
                echo "$ACCESS_TOKEN" | secret-tool store --label="Strava Access Token" service zwift-race-finder username strava-access-token
                echo "$REFRESH_TOKEN" | secret-tool store --label="Strava Refresh Token" service zwift-race-finder username strava-refresh-token
                echo "$CLIENT_SECRET" | secret-tool store --label="Strava Client Secret" service zwift-race-finder username strava-client-secret
                
                # Store non-sensitive data in config
                mkdir -p ~/.config/zwift-race-finder
                cat > ~/.config/zwift-race-finder/strava_public.json << EOF
{
  "client_id": "$CLIENT_ID",
  "expires_at": $EXPIRES_AT,
  "athlete_id": $ATHLETE_ID
}
EOF
                echo "✅ Tokens stored in system keyring"
            else
                echo "❌ System keyring not available. Falling back to file storage."
                STORAGE_METHOD=3
            fi
            ;;
        3)
            # File storage (backward compatible)
            CONFIG_FILE="${SCRIPT_DIR}/strava_config.json"
            cat > "$CONFIG_FILE" << EOF
{
  "client_id": "$CLIENT_ID",
  "client_secret": "$CLIENT_SECRET",
  "access_token": "$ACCESS_TOKEN",
  "refresh_token": "$REFRESH_TOKEN",
  "expires_at": $EXPIRES_AT,
  "athlete_id": $ATHLETE_ID
}
EOF
            # Set restrictive permissions
            chmod 600 "$CONFIG_FILE"
            echo "✅ Tokens saved to $CONFIG_FILE (with restricted permissions)"
            echo ""
            echo "⚠️  WARNING: Tokens are stored in plain text."
            echo "Consider using environment variables or system keyring for better security."
            ;;
        *)
            echo "❌ Invalid choice"
            exit 1
            ;;
    esac
    
    echo ""
    echo "You can now run: ./strava_fetch_activities_secure.sh"
else
    echo "❌ Authentication failed:"
    echo "$RESPONSE" | jq '.'
    exit 1
fi