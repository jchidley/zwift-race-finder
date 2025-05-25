# Zwift Race Finder 🚴

A command-line tool to find Zwift races that match your target duration and racing score. Designed for cyclists who want to find races that fit their schedule and fitness level.

## Features

- 🎯 Filters Zwift events by estimated duration based on your racing score
- 📊 Uses historical race data for accurate time predictions
- 🗺️ Route-aware duration estimation considering elevation and surface type
- 🔄 Imports your race history from ZwiftPower for regression testing
- 📈 Continuously improves predictions using your actual race results

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder

# Build and install
cargo build --release
./install.sh
```

Make sure `~/.local/bin` is in your PATH.

## Quick Start

### For New Users

1. **Clone and build the project:**
   ```bash
   git clone https://github.com/jchidley/zwift-race-finder.git
   cd zwift-race-finder
   cargo build --release
   ./install.sh
   ```

2. **Basic usage (no configuration needed):**
   ```bash
   # Find races using default settings (195 Zwift Score)
   zwift-race-finder
   
   # Specify your Zwift Score
   zwift-race-finder --zwift-score 250
   ```

3. **Optional: Set up personal configuration:**
   ```bash
   # Copy the example config
   cp config.example.json config.json
   
   # Edit config.json with your details:
   # - zwiftpower_profile_id: Find in your ZwiftPower profile URL
   # - default_zwift_score: Your current racing score
   # - windows_username: For WSL users with Downloads in Windows
   ```

4. **Optional: Load sample route data:**
   ```bash
   # Load common Zwift routes for better duration estimates
   sqlite3 ~/.local/share/zwift-race-finder/races.db < sample_routes.sql
   ```

### For Jack (After Sanitization)

**One-Time Setup** - Choose your preferred method:

```bash
# Interactive setup wizard
./setup_personal_config.sh
```

This offers three options:

1. **Bitwarden Integration (Recommended):**
   - Secrets stored in your password manager
   - Syncs across all devices
   - Most secure option
   - Config uses TOML format for readability

2. **Local Secure Directory:**
   - Simple, no dependencies
   - Stored at `~/.config/zwift-race-finder/config.toml`
   - Auto-loaded by the tool

3. **Encrypted File:**
   - GPG encrypted with passphrase
   - Maximum local security

**After setup**, just use:
```bash
./zwift-race-finder-personal
# or install system-wide:
cp zwift-race-finder-personal ~/.local/bin/
```

## Usage

### Basic Usage

```bash
# Find races for the next 24 hours (uses your cached Zwift score)
zwift-race-finder

# Specify your Zwift Racing Score
zwift-race-finder --zwift-score 195

# Look for 90-minute races (±20 minutes)
zwift-race-finder --duration 90 --tolerance 20

# Show races for the next 3 days
zwift-race-finder --days 3

# Show only races (exclude group rides and fondos)
zwift-race-finder --event-type race
```

### Advanced Features

```bash
# Show unknown routes that need mapping
zwift-race-finder --show-unknown-routes

# Record a race result for improving predictions
zwift-race-finder --record-result "route_id,minutes,event_name"

# Enable debug mode to see filtering details
zwift-race-finder --debug
```

## How It Works

The tool estimates race duration using:

1. **Zwift Racing Score**: Determines your category and expected speed
   - Cat D: 0-199 (25-27 km/h base speed)
   - Cat C: 200-299 (30 km/h base speed)
   - Cat B: 300-399 (35 km/h base speed)
   - Cat A: 400+ (40 km/h base speed)

2. **Route Data**: Distance, elevation gain, and surface type
   - Elevation gain affects speed (flat routes are faster)
   - Gravel/mixed surfaces apply speed penalties

3. **Historical Calibration**: Your actual race times improve predictions

## Importing Race History from ZwiftPower

To calibrate predictions with your actual performance:

1. Visit your ZwiftPower profile page
2. Open browser console (F12)
3. Copy and run the extraction script:
   ```bash
   cat extract_zwiftpower_v2.js | xclip -selection clipboard
   ```
4. Paste in browser console - a file will download
5. Import the results:
   ```bash
   ./dev_import_results.sh
   ```

## Development

### Building

```bash
# Development build
cargo build

# Run tests including regression tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Adding Route Data

Routes are stored in SQLite. To add a new route:

```sql
INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES (route_id, distance, elevation, 'Route Name', 'World', 'road');
```

Route IDs can be found on [ZwiftHacks.com](https://zwifthacks.com/).

### Project Structure

```
├── src/
│   ├── main.rs          # CLI and event filtering logic
│   ├── database.rs      # SQLite integration
│   └── regression_test.rs # Accuracy testing
├── extract_zwiftpower_v2.js # Browser script for data extraction
├── dev_import_results.sh    # Import race history
└── route_mappings.sql       # Route data mappings
```

## Security

This project includes security tools to prevent accidental exposure of personal data:

- **`./check_secrets.sh`** - Scans for API keys, tokens, and personal information
- **`./sanitize_personal_data.sh`** - Removes all personal data before making public
- **`./setup_git_hooks.sh`** - Installs pre-commit hooks to prevent committing secrets

Always run `./check_secrets.sh` before committing or making the repository public.

## Contributing

Contributions are welcome! Areas that need help:

- Adding more route data (check `--show-unknown-routes`)
- Improving duration estimation algorithms
- Adding support for more event types
- Creating a web interface

Before contributing:
1. Run `./setup_git_hooks.sh` to install security checks
2. Never commit personal ZwiftPower data or session IDs
3. Use the config templates for any personal settings

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgments

- Route data from [ZwiftHacks](https://zwifthacks.com/) and [Zwift Insider](https://zwiftinsider.com/)
- Zwift and ZwiftPower are trademarks of Zwift, Inc.
## Configuration

For personal settings, copy `config.example.json` to `config.json` and update with your values:
- `zwiftpower_profile_id`: Your ZwiftPower profile ID (found in your profile URL)
- `zwiftpower_session_id`: Session ID from ZwiftPower cookies (optional)
- `default_zwift_score`: Your current Zwift Racing Score
- `windows_username`: Your Windows username (for WSL paths)

## Security Notice

- **Never commit** your ZwiftPower session IDs or profile IDs to version control
- Keep your `config.json` and `.env` files private (they're in .gitignore)
- Browser extraction scripts should only be used on your own ZwiftPower profile
- This tool only uses public Zwift APIs and does not store credentials

## Privacy

This tool stores all data locally on your machine. No data is sent to external servers.
