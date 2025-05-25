#!/bin/bash
# Setup secure configuration storage outside of git repository

set -e

# Secure config location in your home directory (outside git)
SECURE_CONFIG_DIR="${HOME}/.config/zwift-race-finder"
SECURE_CONFIG_FILE="${SECURE_CONFIG_DIR}/personal.json"
SECURE_ENV_FILE="${SECURE_CONFIG_DIR}/.env"

echo "🔒 Setting up secure configuration storage..."

# Create secure directory with restricted permissions
mkdir -p "$SECURE_CONFIG_DIR"
chmod 700 "$SECURE_CONFIG_DIR"

# Create personal config file if it doesn't exist
if [[ ! -f "$SECURE_CONFIG_FILE" ]]; then
    echo "Creating secure personal configuration..."
    cat > "$SECURE_CONFIG_FILE" << 'EOF'
{
  "zwiftpower_profile_id": "1106548",
  "zwiftpower_session_id": "05848fd47a65e93d504ee04ef04a459b",
  "default_zwift_score": 195,
  "default_category": "D",
  "windows_username": "jackc"
}
EOF
    chmod 600 "$SECURE_CONFIG_FILE"
    echo "✅ Created $SECURE_CONFIG_FILE"
else
    echo "✅ Secure config already exists at $SECURE_CONFIG_FILE"
fi

# Create secure .env file
if [[ ! -f "$SECURE_ENV_FILE" ]]; then
    cat > "$SECURE_ENV_FILE" << 'EOF'
# ZwiftPower Configuration
ZWIFTPOWER_PROFILE_ID=1106548
ZWIFTPOWER_SESSION_ID=05848fd47a65e93d504ee04ef04a459b
DEFAULT_ZWIFT_SCORE=195
DEFAULT_CATEGORY=D
WINDOWS_USERNAME=jackc
EOF
    chmod 600 "$SECURE_ENV_FILE"
    echo "✅ Created $SECURE_ENV_FILE"
fi

# Create auto-restore script
cat > restore_personal_config.sh << 'EOF'
#!/bin/bash
# Restore personal configuration from secure storage

SECURE_CONFIG_DIR="${HOME}/.config/zwift-race-finder"
SECURE_CONFIG_FILE="${SECURE_CONFIG_DIR}/personal.json"

if [[ -f "$SECURE_CONFIG_FILE" ]]; then
    echo "🔄 Restoring personal configuration..."
    cp "$SECURE_CONFIG_FILE" config.json
    echo "✅ Configuration restored!"
else
    echo "❌ No secure config found at $SECURE_CONFIG_FILE"
    echo "Run ./setup_secure_config.sh first"
    exit 1
fi
EOF
chmod +x restore_personal_config.sh

# Create shell alias/function for automatic config loading
SHELL_RC="${HOME}/.bashrc"
if [[ -f "${HOME}/.zshrc" ]]; then
    SHELL_RC="${HOME}/.zshrc"
fi

# Add function to shell RC if not already present
if ! grep -q "zwift-race-finder-config" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# Auto-load Zwift Race Finder secure config" >> "$SHELL_RC"
    echo "zwift-race-finder-config() {" >> "$SHELL_RC"
    echo "    if [[ -f \"\${HOME}/.config/zwift-race-finder/.env\" ]]; then" >> "$SHELL_RC"
    echo "        export \$(grep -v '^#' \"\${HOME}/.config/zwift-race-finder/.env\" | xargs)" >> "$SHELL_RC"
    echo "    fi" >> "$SHELL_RC"
    echo "}" >> "$SHELL_RC"
fi

# Update config.rs to check secure location
echo ""
echo "📝 Updating config loader to check secure location..."

# Create updated config.rs
cat > src/config.rs << 'EOF'
//! Configuration management for Zwift Race Finder

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub zwiftpower_profile_id: Option<String>,
    pub zwiftpower_session_id: Option<String>,
    pub default_zwift_score: Option<u32>,
    pub default_category: Option<String>,
    pub windows_username: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            zwiftpower_profile_id: None,
            zwiftpower_session_id: None,
            default_zwift_score: Some(195),
            default_category: Some("D".to_string()),
            windows_username: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Priority order:
        // 1. Local config.json (in git repo)
        // 2. Secure config at ~/.config/zwift-race-finder/personal.json
        // 3. Environment variables
        // 4. Default values
        
        // Try local config first
        let local_config_path = PathBuf::from("config.json");
        if local_config_path.exists() {
            let contents = fs::read_to_string(local_config_path)?;
            return Ok(serde_json::from_str(&contents)?);
        }
        
        // Try secure config location
        let secure_config_path = dirs::config_dir()
            .map(|mut path| {
                path.push("zwift-race-finder");
                path.push("personal.json");
                path
            })
            .unwrap_or_else(|| PathBuf::from("~/.config/zwift-race-finder/personal.json"));
            
        if secure_config_path.exists() {
            let contents = fs::read_to_string(secure_config_path)?;
            return Ok(serde_json::from_str(&contents)?);
        }
        
        // Fall back to environment variables
        Ok(Config {
            zwiftpower_profile_id: std::env::var("ZWIFTPOWER_PROFILE_ID").ok(),
            zwiftpower_session_id: std::env::var("ZWIFTPOWER_SESSION_ID").ok(),
            default_zwift_score: std::env::var("DEFAULT_ZWIFT_SCORE")
                .ok()
                .and_then(|s| s.parse().ok()),
            default_category: std::env::var("DEFAULT_CATEGORY").ok(),
            windows_username: std::env::var("WINDOWS_USERNAME").ok(),
        })
    }
    
    pub fn get_download_path(&self) -> String {
        if let Some(username) = &self.windows_username {
            format!("/mnt/c/Users/{}/Downloads", username)
        } else {
            // Fallback to Linux home directory
            format!("{}/Downloads", std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()))
        }
    }
}
EOF

echo "✅ Configuration system updated!"
echo ""
echo "🎉 Setup complete!"
echo ""
echo "Your personal configuration is now stored securely at:"
echo "  📁 $SECURE_CONFIG_DIR/"
echo "  📄 $SECURE_CONFIG_FILE (mode 600)"
echo "  🔐 $SECURE_ENV_FILE (mode 600)"
echo ""
echo "This location is:"
echo "  ✅ Outside the git repository"
echo "  ✅ Protected with restrictive permissions"
echo "  ✅ Automatically loaded by the tool"
echo ""
echo "After running sanitize_personal_data.sh, just run:"
echo "  ./restore_personal_config.sh"
echo ""
echo "Or the tool will automatically find your config at ~/.config/zwift-race-finder/"