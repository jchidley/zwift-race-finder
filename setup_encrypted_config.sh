#!/bin/bash
# Setup encrypted configuration storage using GPG

set -e

echo "🔐 Setting up encrypted configuration storage..."

# Check if GPG is available
if ! command -v gpg &> /dev/null; then
    echo "❌ GPG not found. Install with: sudo apt install gnupg"
    exit 1
fi

# Secure config location
SECURE_CONFIG_DIR="${HOME}/.config/zwift-race-finder"
CONFIG_FILE="personal_config.json"
ENCRYPTED_FILE="${SECURE_CONFIG_DIR}/${CONFIG_FILE}.gpg"

# Create secure directory
mkdir -p "$SECURE_CONFIG_DIR"
chmod 700 "$SECURE_CONFIG_DIR"

# Create temporary config file
TEMP_CONFIG=$(mktemp)
cat > "$TEMP_CONFIG" << 'EOF'
{
  "zwiftpower_profile_id": "1106548",
  "zwiftpower_session_id": "05848fd47a65e93d504ee04ef04a459b",
  "default_zwift_score": 195,
  "default_category": "D",
  "windows_username": "jackc"
}
EOF

# Encrypt the config file
echo "Encrypting configuration..."
echo "You'll be prompted for a passphrase. Remember it!"
gpg --symmetric --cipher-algo AES256 --armor --output "$ENCRYPTED_FILE" "$TEMP_CONFIG"

# Remove temporary file
rm -f "$TEMP_CONFIG"

# Set restrictive permissions
chmod 600 "$ENCRYPTED_FILE"

# Create decrypt helper script
cat > decrypt_config.sh << 'EOF'
#!/bin/bash
# Decrypt configuration for current session

SECURE_CONFIG_DIR="${HOME}/.config/zwift-race-finder"
ENCRYPTED_FILE="${SECURE_CONFIG_DIR}/personal_config.json.gpg"

if [[ ! -f "$ENCRYPTED_FILE" ]]; then
    echo "❌ No encrypted config found at $ENCRYPTED_FILE"
    echo "Run ./setup_encrypted_config.sh first"
    exit 1
fi

echo "🔓 Decrypting configuration..."
gpg --decrypt "$ENCRYPTED_FILE" > config.json 2>/dev/null

if [[ $? -eq 0 ]]; then
    echo "✅ Configuration decrypted to config.json"
    echo "⚠️  Remember to delete config.json before committing!"
else
    echo "❌ Decryption failed"
    exit 1
fi
EOF
chmod +x decrypt_config.sh

# Create auto-decrypt wrapper
cat > zwift-race-finder-secure << 'EOF'
#!/bin/bash
# Secure wrapper that decrypts config, runs the tool, then cleans up

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if config.json exists
if [[ ! -f "config.json" ]]; then
    # Try to decrypt
    if [[ -f "${HOME}/.config/zwift-race-finder/personal_config.json.gpg" ]]; then
        echo "🔓 Decrypting configuration..."
        gpg --decrypt "${HOME}/.config/zwift-race-finder/personal_config.json.gpg" > config.json 2>/dev/null
        if [[ $? -ne 0 ]]; then
            echo "❌ Failed to decrypt configuration"
            exit 1
        fi
        CLEANUP_CONFIG=1
    fi
fi

# Run the actual tool
zwift-race-finder "$@"
EXIT_CODE=$?

# Clean up decrypted config if we created it
if [[ "${CLEANUP_CONFIG}" == "1" ]]; then
    rm -f config.json
fi

exit $EXIT_CODE
EOF
chmod +x zwift-race-finder-secure

# Update .gitignore to ensure encrypted file isn't committed
if ! grep -q "personal_config.json.gpg" .gitignore 2>/dev/null; then
    echo "personal_config.json.gpg" >> .gitignore
fi

echo ""
echo "✅ Encrypted configuration setup complete!"
echo ""
echo "📁 Encrypted config stored at:"
echo "   $ENCRYPTED_FILE"
echo ""
echo "🔧 Usage options:"
echo ""
echo "1. Manual decrypt before use:"
echo "   ./decrypt_config.sh"
echo "   zwift-race-finder"
echo "   rm config.json  # Clean up after"
echo ""
echo "2. Use secure wrapper (auto decrypt/cleanup):"
echo "   ./zwift-race-finder-secure"
echo ""
echo "3. Install secure wrapper to PATH:"
echo "   cp zwift-race-finder-secure ~/.local/bin/"
echo "   zwift-race-finder-secure  # Use from anywhere"
echo ""
echo "⚠️  Important: Your passphrase is required to decrypt"