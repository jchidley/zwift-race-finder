#!/bin/bash
# Bitwarden configuration helper for Zwift Race Finder

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Function to check if logged in to Bitwarden
check_bw_login() {
    if ! bw status | grep -q "unlocked"; then
        echo -e "${YELLOW}Bitwarden vault is locked or not logged in${NC}" >&2
        echo "Please login and unlock:" >&2
        echo "  bw login" >&2
        echo "  export BW_SESSION=\$(bw unlock --raw)" >&2
        return 1
    fi
    return 0
}

# Function to get secret from Bitwarden
get_bw_field() {
    local item_name="$1"
    local field_name="$2"
    
    bw get item "$item_name" 2>/dev/null | jq -r ".fields[] | select(.name == \"$field_name\") | .value"
}

# Function to create or update Bitwarden item
setup_bw_item() {
    echo "Setting up Bitwarden item..."
    
    # Check if item exists
    if bw get item "Zwift Race Finder" &>/dev/null; then
        echo -e "${GREEN}✓ Bitwarden item 'Zwift Race Finder' already exists${NC}"
        echo "Current values:"
        echo "  Profile ID: $(get_bw_field "Zwift Race Finder" "zwiftpower_profile_id")"
        echo "  Session ID: [hidden]"
    else
        echo "Creating new Bitwarden item..."
        # Use the proper setup script
        ./setup_bitwarden_proper.sh
    fi
}

# Function to export secrets as environment variables
export_secrets() {
    if ! check_bw_login; then
        return 1
    fi
    
    export ZWIFTPOWER_PROFILE_ID=$(get_bw_field "Zwift Race Finder" "zwiftpower_profile_id")
    export ZWIFTPOWER_SESSION_ID=$(get_bw_field "Zwift Race Finder" "zwiftpower_session_id")
    
    if [[ -n "$ZWIFTPOWER_PROFILE_ID" ]]; then
        echo -e "${GREEN}✓ Secrets loaded from Bitwarden${NC}" >&2
        return 0
    else
        echo -e "${RED}✗ Failed to load secrets from Bitwarden${NC}" >&2
        return 1
    fi
}

# Main command handling
case "${1:-help}" in
    setup)
        check_bw_login && setup_bw_item
        ;;
    export)
        if export_secrets; then
            # Output the export commands
            echo "export ZWIFTPOWER_PROFILE_ID=\"$ZWIFTPOWER_PROFILE_ID\""
            echo "export ZWIFTPOWER_SESSION_ID=\"$ZWIFTPOWER_SESSION_ID\""
        fi
        ;;
    get)
        if check_bw_login; then
            echo "ZWIFTPOWER_PROFILE_ID=$(get_bw_field "Zwift Race Finder" "zwiftpower_profile_id")"
            echo "ZWIFTPOWER_SESSION_ID=$(get_bw_field "Zwift Race Finder" "zwiftpower_session_id")"
        fi
        ;;
    test)
        if export_secrets; then
            echo "Profile ID: $ZWIFTPOWER_PROFILE_ID"
            echo "Session ID: ${ZWIFTPOWER_SESSION_ID:0:10}..."
        fi
        ;;
    *)
        echo "Zwift Race Finder - Bitwarden Integration"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  setup   - Create/verify Bitwarden item"
        echo "  export  - Export secrets as environment variables"
        echo "  get     - Print secrets (for debugging)"
        echo "  test    - Test Bitwarden connection"
        echo ""
        echo "Example workflow:"
        echo "  1. bw login"
        echo "  2. export BW_SESSION=\$(bw unlock --raw)"
        echo "  3. $0 setup"
        echo "  4. source <($0 export)"
        ;;
esac
