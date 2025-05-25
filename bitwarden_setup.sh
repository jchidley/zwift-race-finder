#!/bin/bash
# Simplified Bitwarden setup that works with the CLI

set -e

echo "🔐 Setting up Bitwarden item for Zwift Race Finder..."

# Check if logged in
if ! bw status | grep -q "unlocked"; then
    echo "❌ Bitwarden is locked. Please unlock first:"
    echo "  export BW_SESSION=\$(bw unlock --raw)"
    exit 1
fi

# Check if item already exists
if bw get item "Zwift Race Finder" &>/dev/null; then
    echo "✅ Item already exists in Bitwarden"
    exit 0
fi

# Create the item using the simpler CLI format
echo "Creating Bitwarden item..."

# Create a secure note with custom fields
ITEM_JSON=$(cat <<EOF
{
  "type": 2,
  "name": "Zwift Race Finder",
  "notes": "Secrets for zwift-race-finder tool",
  "secureNote": {
    "type": 0
  },
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
  ]
}
EOF
)

# Create the item
echo "$ITEM_JSON" | bw encode | bw create item

if [ $? -eq 0 ]; then
    echo "✅ Successfully created 'Zwift Race Finder' in Bitwarden!"
    echo ""
    echo "Test it with:"
    echo "  ./bw_config.sh test"
else
    echo "❌ Failed to create item"
    echo ""
    echo "Alternative: Create manually in Bitwarden with:"
    echo "  Name: Zwift Race Finder"
    echo "  Type: Secure Note"
    echo "  Custom Fields:"
    echo "    - zwiftpower_profile_id = 1106548"
    echo "    - zwiftpower_session_id = 05848fd47a65e93d504ee04ef04a459b"
fi