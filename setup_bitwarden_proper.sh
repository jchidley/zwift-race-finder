#!/bin/bash
# Proper Bitwarden setup using official template format

set -e

echo "🔐 Setting up Bitwarden item for Zwift Race Finder..."

# Create the item JSON using the official template structure
ITEM_JSON=$(cat <<'EOF'
{
  "organizationId": null,
  "collectionIds": null,
  "folderId": null,
  "type": 2,
  "name": "Zwift Race Finder",
  "notes": "Secrets for zwift-race-finder tool\n\nThis item stores your ZwiftPower credentials securely.",
  "favorite": false,
  "fields": [
    {
      "name": "zwiftpower_profile_id",
      "value": "1106548",
      "type": 0
    },
    {
      "name": "zwiftpower_session_id",
      "value": "05848fd47a65e93d504ee04ef04a459b",
      "type": 0
    }
  ],
  "secureNote": {
    "type": 0
  },
  "reprompt": 0
}
EOF
)

# Encode the JSON (Bitwarden requires base64 encoding)
ENCODED_JSON=$(echo "$ITEM_JSON" | bw encode)

echo "Creating Bitwarden item..."
echo ""

# Create the item
if bw create item "$ENCODED_JSON" > /dev/null; then
    echo "✅ Successfully created 'Zwift Race Finder' in Bitwarden!"
    echo ""
    echo "Verifying creation..."
    
    # Verify it was created and show the fields
    if ITEM=$(bw get item "Zwift Race Finder" 2>/dev/null); then
        echo "✅ Item verified! Found fields:"
        echo "$ITEM" | jq -r '.fields[] | "  - " + .name + ": " + (.value | if length > 10 then .[0:10] + "..." else . end)'
    fi
    
    echo ""
    echo "Test the integration with:"
    echo "  ./bw_config.sh test"
else
    echo "❌ Failed to create item"
    exit 1
fi