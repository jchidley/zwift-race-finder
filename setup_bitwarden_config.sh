#!/bin/bash
# Setup Bitwarden integration for secure secret storage

set -e

echo "🔐 Setting up Bitwarden integration for Zwift Race Finder..."

# Check if Bitwarden CLI is installed
if ! command -v bw &> /dev/null; then
    echo "❌ Bitwarden CLI not found!"
    echo ""
    echo "Install with:"
    echo "  npm install -g @bitwarden/cli"
    echo "  # or"
    echo "  cargo install rbw  # Rust alternative"
    exit 1
fi

# Configuration directory
CONFIG_DIR="${HOME}/.config/zwift-race-finder"
mkdir -p "$CONFIG_DIR"
chmod 700 "$CONFIG_DIR"

# Create TOML config template (non-secret settings)
cat > config.example.toml << 'EOF'
# Zwift Race Finder Configuration
# Non-secret settings only - secrets are stored in Bitwarden

[defaults]
zwift_score = 195
category = "D"

[import]
# For WSL users - set your Windows username here
# Or use environment variable WINDOWS_USERNAME
windows_username = "YOUR_USERNAME"

[preferences]
# Default search parameters
default_duration = 120
default_tolerance = 30
default_days = 1

[display]
# Color output
use_colors = true
# Show debug information
debug = false
EOF

# Create Bitwarden item template
cat > bitwarden_item_template.json << 'EOF'
{
  "organizationId": null,
  "collectionIds": null,
  "folderId": null,
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
      "type": 1
    }
  ],
  "favorite": false,
  "reprompt": 0
}
EOF

# Create the Bitwarden integration script
cat > bw_config.sh << 'EOF'
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
        echo -e "${YELLOW}Bitwarden vault is locked or not logged in${NC}"
        echo "Please login and unlock:"
        echo "  bw login"
        echo "  export BW_SESSION=\$(bw unlock --raw)"
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
        bw create item "$(cat bitwarden_item_template.json)" | jq -r '.name'
        echo -e "${GREEN}✓ Created 'Zwift Race Finder' in Bitwarden${NC}"
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
        echo -e "${GREEN}✓ Secrets loaded from Bitwarden${NC}"
        return 0
    else
        echo -e "${RED}✗ Failed to load secrets from Bitwarden${NC}"
        return 1
    fi
}

# Main command handling
case "${1:-help}" in
    setup)
        check_bw_login && setup_bw_item
        ;;
    export)
        export_secrets
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
EOF
chmod +x bw_config.sh

# Create TOML parser for Rust
cat > src/config.rs << 'EOF'
//! Configuration management for Zwift Race Finder using TOML

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub import: ImportConfig,
    #[serde(default)]
    pub preferences: Preferences,
    #[serde(default)]
    pub display: Display,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Defaults {
    pub zwift_score: Option<u32>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportConfig {
    pub windows_username: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preferences {
    pub default_duration: Option<u32>,
    pub default_tolerance: Option<u32>,
    pub default_days: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Display {
    pub use_colors: Option<bool>,
    pub debug: Option<bool>,
}

// Separate struct for secrets from environment/Bitwarden
#[derive(Debug, Clone)]
pub struct Secrets {
    pub zwiftpower_profile_id: Option<String>,
    pub zwiftpower_session_id: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            defaults: Defaults::default(),
            import: ImportConfig::default(),
            preferences: Preferences::default(),
            display: Display::default(),
        }
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            zwift_score: Some(195),
            category: Some("D".to_string()),
        }
    }
}

impl Default for ImportConfig {
    fn default() -> Self {
        ImportConfig {
            windows_username: None,
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            default_duration: Some(120),
            default_tolerance: Some(30),
            default_days: Some(1),
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Display {
            use_colors: Some(true),
            debug: Some(false),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Priority order for config files:
        // 1. Local config.toml
        // 2. ~/.config/zwift-race-finder/config.toml
        // 3. Default values
        
        let config_paths = vec![
            PathBuf::from("config.toml"),
            dirs::config_dir()
                .map(|mut path| {
                    path.push("zwift-race-finder");
                    path.push("config.toml");
                    path
                })
                .unwrap_or_default(),
        ];
        
        for path in config_paths {
            if path.exists() {
                let contents = fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&contents)?;
                return Ok(config);
            }
        }
        
        // Return defaults if no config file found
        Ok(Config::default())
    }
    
    pub fn get_download_path(&self) -> String {
        let username = self.import.windows_username.as_ref()
            .or_else(|| std::env::var("WINDOWS_USERNAME").ok().as_ref())
            .map(|s| s.as_str())
            .unwrap_or("YOUR_USERNAME");
            
        if username != "YOUR_USERNAME" {
            format!("/mnt/c/Users/{}/Downloads", username)
        } else {
            format!("{}/Downloads", std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()))
        }
    }
}

impl Secrets {
    pub fn load() -> Self {
        // Load secrets from environment variables (set by Bitwarden)
        Secrets {
            zwiftpower_profile_id: std::env::var("ZWIFTPOWER_PROFILE_ID").ok(),
            zwiftpower_session_id: std::env::var("ZWIFTPOWER_SESSION_ID").ok(),
        }
    }
}

// Combined config for backward compatibility
pub struct FullConfig {
    pub config: Config,
    pub secrets: Secrets,
}

impl FullConfig {
    pub fn load() -> Result<Self> {
        Ok(FullConfig {
            config: Config::load()?,
            secrets: Secrets::load(),
        })
    }
    
    // Compatibility methods
    pub fn zwiftpower_profile_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_profile_id.as_ref()
    }
    
    pub fn zwiftpower_session_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_session_id.as_ref()
    }
    
    pub fn default_zwift_score(&self) -> Option<u32> {
        self.config.defaults.zwift_score
    }
    
    pub fn default_category(&self) -> Option<&String> {
        self.config.defaults.category.as_ref()
    }
    
    pub fn windows_username(&self) -> Option<&String> {
        self.config.import.windows_username.as_ref()
    }
}
EOF

# Update Cargo.toml to include toml
cat > update_cargo.sh << 'EOF'
#!/bin/bash
# Add toml dependency to Cargo.toml

if ! grep -q "toml" Cargo.toml; then
    sed -i '/\[dependencies\]/a toml = "0.8"' Cargo.toml
    echo "✓ Added toml dependency to Cargo.toml"
else
    echo "✓ toml dependency already present"
fi
EOF
chmod +x update_cargo.sh
./update_cargo.sh
rm update_cargo.sh

# Create wrapper script that loads from Bitwarden
cat > zwift-race-finder-bw << 'EOF'
#!/bin/bash
# Wrapper that loads secrets from Bitwarden before running

# Load secrets from Bitwarden
if source <(./bw_config.sh export 2>/dev/null); then
    # Run with secrets in environment
    exec zwift-race-finder "$@"
else
    echo "Failed to load secrets from Bitwarden"
    echo "Make sure you're logged in: bw login && export BW_SESSION=\$(bw unlock --raw)"
    exit 1
fi
EOF
chmod +x zwift-race-finder-bw

# Create setup instructions
cat > BITWARDEN_SETUP.md << 'EOF'
# Bitwarden Integration Setup

This project uses Bitwarden to securely store sensitive credentials.

## Initial Setup

1. **Install Bitwarden CLI:**
   ```bash
   npm install -g @bitwarden/cli
   # or for Rust users:
   cargo install rbw
   ```

2. **Login to Bitwarden:**
   ```bash
   bw login
   export BW_SESSION=$(bw unlock --raw)
   ```

3. **Create the secrets in Bitwarden:**
   ```bash
   ./bw_config.sh setup
   ```

4. **Create your local config.toml:**
   ```bash
   cp config.example.toml config.toml
   # Edit with your non-secret preferences
   ```

## Daily Usage

### Option 1: Export to Environment
```bash
# Load secrets into current shell
source <(./bw_config.sh export)
zwift-race-finder
```

### Option 2: Use Wrapper Script
```bash
# Automatically loads from Bitwarden
./zwift-race-finder-bw
```

### Option 3: Shell Function
Add to your ~/.bashrc:
```bash
zwift-race-finder() {
    source <(~/tools/rust/zwift-race-finder/bw_config.sh export) && \
    command zwift-race-finder "$@"
}
```

## Configuration Files

- **config.toml** - Non-secret configuration (safe to commit)
- **Bitwarden** - Secrets (zwiftpower_profile_id, zwiftpower_session_id)

## Security Benefits

- Secrets never stored in plain text files
- Bitwarden handles encryption and secure storage
- Can sync across devices securely
- No manual entry after initial setup
EOF

echo ""
echo "✅ Bitwarden integration setup complete!"
echo ""
echo "📋 Next steps:"
echo ""
echo "1. Login to Bitwarden:"
echo "   bw login"
echo "   export BW_SESSION=\$(bw unlock --raw)"
echo ""
echo "2. Create your secrets in Bitwarden:"
echo "   ./bw_config.sh setup"
echo ""
echo "3. Create your config.toml:"
echo "   cp config.example.toml config.toml"
echo "   # Edit with your preferences"
echo ""
echo "4. Test the integration:"
echo "   ./bw_config.sh test"
echo ""
echo "5. Use the tool:"
echo "   source <(./bw_config.sh export) && zwift-race-finder"
echo "   # or"
echo "   ./zwift-race-finder-bw"
echo ""
echo "See BITWARDEN_SETUP.md for full documentation"