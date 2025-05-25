#!/bin/bash
# Comprehensive setup for personal configuration with multiple options

set -e

echo "🚴 Zwift Race Finder - Personal Configuration Setup"
echo "=================================================="
echo ""
echo "Choose your preferred configuration method:"
echo ""
echo "1) Bitwarden (Recommended) - Secure, synced across devices"
echo "2) Local secure directory - Simple, no dependencies"
echo "3) Encrypted file - Maximum security, requires passphrase"
echo ""
read -p "Select option (1-3): " -n 1 -r
echo ""

case $REPLY in
    1)
        echo "Setting up Bitwarden integration..."
        ./setup_bitwarden_config.sh
        ;;
    2)
        echo "Setting up local secure directory..."
        
        # Create secure config directory
        CONFIG_DIR="${HOME}/.config/zwift-race-finder"
        mkdir -p "$CONFIG_DIR"
        chmod 700 "$CONFIG_DIR"
        
        # Create TOML config
        cat > "${CONFIG_DIR}/config.toml" << 'EOF'
# Zwift Race Finder Configuration
# This file is stored outside the git repository

[defaults]
zwift_score = 195
category = "D"

[import]
windows_username = "jackc"

[preferences]
default_duration = 120
default_tolerance = 30
default_days = 1

[display]
use_colors = true
debug = false

# Secrets - stored here for convenience
# For better security, use Bitwarden instead
[secrets]
zwiftpower_profile_id = "1106548"
zwiftpower_session_id = "05848fd47a65e93d504ee04ef04a459b"
EOF
        chmod 600 "${CONFIG_DIR}/config.toml"
        
        # Update config.rs to read secrets from TOML too
        cat > src/config_local.rs << 'EOF'
// Extension for local TOML with secrets
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SecretsConfig {
    pub zwiftpower_profile_id: Option<String>,
    pub zwiftpower_session_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigWithSecrets {
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub import: ImportConfig,
    #[serde(default)]
    pub preferences: Preferences,
    #[serde(default)]
    pub display: Display,
    #[serde(default)]
    pub secrets: SecretsConfig,
}
EOF
        
        echo "✅ Created secure config at ${CONFIG_DIR}/config.toml"
        ;;
    3)
        echo "Setting up encrypted configuration..."
        ./setup_encrypted_config.sh
        ;;
    *)
        echo "Invalid option"
        exit 1
        ;;
esac

# Create a personal wrapper script
cat > zwift-race-finder-personal << 'EOF'
#!/bin/bash
# Personal wrapper that handles all config methods

# Check for Bitwarden first
if command -v bw &>/dev/null && bw status 2>/dev/null | grep -q "unlocked"; then
    # Try to load from Bitwarden
    if source <(./bw_config.sh export 2>/dev/null); then
        exec zwift-race-finder "$@"
    fi
fi

# Check for local secure config
if [[ -f "${HOME}/.config/zwift-race-finder/config.toml" ]]; then
    # Config will be auto-loaded by the tool
    exec zwift-race-finder "$@"
fi

# Check for encrypted config
if [[ -f "${HOME}/.config/zwift-race-finder/personal_config.json.gpg" ]]; then
    ./zwift-race-finder-secure "$@"
else
    # No personal config found, run with defaults
    echo "Note: No personal configuration found. Using defaults."
    exec zwift-race-finder "$@"
fi
EOF
chmod +x zwift-race-finder-personal

echo ""
echo "✅ Personal configuration setup complete!"
echo ""
echo "📝 Quick reference:"
echo ""
echo "To use your personal config:"
echo "  ./zwift-race-finder-personal"
echo ""
echo "To install system-wide:"
echo "  cp zwift-race-finder-personal ~/.local/bin/"
echo "  zwift-race-finder-personal  # Use from anywhere"
echo ""
echo "Your configuration will survive sanitization and repo updates!"