# Configuration Management

The Zwift Race Finder supports flexible configuration management to personalize your experience and save your preferences across updates.

## Configuration Priority

Settings are loaded in the following priority order (highest to lowest):

1. **Environment Variables** - Override any config file settings
2. **Local config.toml** - Project-specific configuration
3. **~/.config/zwift-race-finder/config.toml** - User configuration directory
4. **~/.local/share/zwift-race-finder/config.toml** - Data directory (survives updates)
5. **Built-in defaults** - Fallback values

## Configuration File Format

Create a `config.toml` file with your personal settings:

```toml
[defaults]
zwift_score = 195         # Your Zwift Racing Score
category = "D"            # Your racing category (A/B/C/D/E)
height_m = 1.82          # Height in meters (for aerodynamics)
weight_kg = 86.0         # Weight in kilograms
ftp_watts = 250          # Functional Threshold Power (optional)

[preferences]
default_duration = 120    # Default race duration to search (minutes)
default_tolerance = 30    # Duration tolerance (+/- minutes)
default_days = 1         # Days ahead to search

[display]
use_colors = true        # Enable colored output
debug = false           # Show debug information

[import]
# For WSL users - set your Windows username
windows_username = "YOUR_USERNAME"
```

## Environment Variable Overrides

You can override any configuration setting using environment variables:

- `ZWIFT_SCORE` - Override Zwift Racing Score
- `ZWIFT_CATEGORY` - Override racing category
- `ZWIFT_WEIGHT_KG` - Override weight
- `ZWIFT_HEIGHT_M` - Override height
- `ZWIFT_FTP_WATTS` - Override FTP
- `ZWIFT_DEFAULT_DURATION` - Override default duration
- `ZWIFT_DEFAULT_TOLERANCE` - Override default tolerance  
- `ZWIFT_DEFAULT_DAYS` - Override default days

Example:
```bash
# Run with custom duration preference
ZWIFT_DEFAULT_DURATION=90 zwift-race-finder

# Run with different racing score
ZWIFT_SCORE=250 zwift-race-finder
```

## Recommended Setup

### 1. User Configuration (Survives Updates)

Save your personal configuration to the data directory:

```bash
mkdir -p ~/.local/share/zwift-race-finder
cat > ~/.local/share/zwift-race-finder/config.toml << 'EOF'
[defaults]
zwift_score = 195
category = "D"
height_m = 1.82
weight_kg = 86.0
ftp_watts = 250

[preferences]
default_duration = 120
default_tolerance = 30
EOF
```

### 2. Personal Wrapper Script

Create a personal wrapper that loads your configuration:

```bash
#!/bin/bash
# ~/bin/zrf - Personal Zwift Race Finder wrapper

# Load personal defaults
export ZWIFT_SCORE=195
export ZWIFT_WEIGHT_KG=86.0
export ZWIFT_HEIGHT_M=1.82

# Run the tool with any additional arguments
exec zwift-race-finder "$@"
```

Make it executable: `chmod +x ~/bin/zrf`

### 3. Project-Specific Configuration

For testing different scenarios, create a local `config.toml`:

```toml
# Testing lighter rider performance
[defaults]
zwift_score = 195
weight_kg = 70.0  # Test with lighter weight
height_m = 1.75

[preferences]
default_duration = 30  # Look for shorter races
default_tolerance = 10
```

## Security Notes

- **Never** put secrets (API tokens, passwords) in config files
- Use the secure storage options for OAuth tokens:
  - Environment variables from secure sources
  - System keyring (if enabled)
  - Encrypted files with proper permissions
- See `SECURE_TOKEN_MIGRATION.md` for secure token storage

## Command Line vs Configuration

Command line arguments always override configuration settings:

```bash
# Uses config duration of 120, but overrides tolerance to 60
zwift-race-finder -t 60

# Completely ignores config, uses all command line values
zwift-race-finder -s 250 -d 90 -t 20
```

## Future Enhancements

The configuration system is designed to support future features:
- Power curve data for better predictions
- Preferred routes and blacklists
- Time zone preferences
- Notification settings
- API rate limit configuration