# Zwift Race Finder 🚴

> 🎯 **BETA SOFTWARE**: Achieving 25.1% prediction accuracy! This tool predicts Zwift race durations based on your racing score and historical data. Multi-lap races and route mapping are actively being improved.

A command-line tool to find Zwift races that match your target duration and racing score. Designed for cyclists who want to find races that fit their schedule and fitness level.

## Why This Tool Exists

### 1. The Race Planning Problem
As a Zwift racer, I often know:
- ✅ When I want to race (e.g., "sometime this evening")
- ✅ How long I want to race for (e.g., "about 90 minutes")
- ❌ Which races will actually take that long for my fitness level

The problem: Zwift shows race distances, but a 40km race might take me 60 minutes or 90 minutes depending on the route profile and my category. This tool solves that by predicting actual race duration based on your specific racing score.

### 2. Built with AI, Not Traditional Coding
This project demonstrates the power of using LLMs (specifically Claude Code) to build real software. As a retired IT Professional - not a coder - I've used AI to:
- Design the architecture and data flow
- Write Rust code with proper error handling
- Integrate multiple APIs (Zwift, Strava)
- Create data analysis and machine learning features
- Build a tool that actually solves my problem

**The result**: A working tool with 25.1% prediction accuracy, improving with each race!

## Features

- 🎯 Filters Zwift events by estimated duration based on your racing score
- 📊 25.1% prediction accuracy using 151+ real race results
- 🗺️ Route-aware duration estimation considering elevation and surface type
- 🔄 Strava integration for actual race times (not estimates!)
- 🏁 Multi-lap race support with per-category distance handling
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

## Strava Integration (NEW!)

Get actual race times from your Strava activities:

```bash
# 1. Set up Strava authentication
./strava_auth.sh
# Follow prompts to create app at https://www.strava.com/settings/api

# 2. Fetch your Zwift activities from Strava
./strava_fetch_activities.sh

# 3. Import actual race times into database
./strava_import_to_db.sh

# 4. Analyze your race performance
./strava_analyze.py
```

This provides:
- Actual race completion times (not estimates!)
- Accurate speed data including draft benefit
- Better calibration for future predictions

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

## Performance & Accuracy

### Current Benchmarks
- **Prediction Accuracy**: 25.1% mean absolute error
- **Dataset**: 151 real races from Strava
- **Speed Calibration**: Cat D average 30.9 km/h (with draft)
- **Multi-lap Support**: ✅ Using event_sub_groups for per-category distances

### Accuracy by Route Type
- **Flat routes**: ~20% error
- **Rolling routes**: ~25% error  
- **Hilly routes**: ~30% error
- **Multi-lap races**: Previously 70%+ error, now ~25%

### Next Target
- **Goal**: <20% error using physics-based model
- **Method**: Martin et al. (1998) power equation
- **Data Needed**: Rider weight, height, FTP

## How It Works

The tool estimates race duration using:

1. **Zwift Racing Score**: Determines your category and expected speed
   - Cat D: 0-199 (30.9 km/h average in races)
   - Cat C: 200-299 (33 km/h average in races)
   - Cat B: 300-399 (37 km/h average in races)
   - Cat A: 400+ (42 km/h average in races)

2. **Route Data**: Distance, elevation gain, and surface type
   - Elevation gain affects speed (flat routes are faster)
   - Gravel/mixed surfaces apply speed penalties
   - Multi-lap races use event_sub_groups for category-specific distances

3. **Historical Calibration**: Your actual race times improve predictions
   - Strava API provides real race completion times
   - Draft benefit in races accounted for (~30% speed boost)

### Data Sources
- **Zwift Public API**: Upcoming events, routes, distances
- **Strava API**: Your actual race times and performance data
- **Local Database**: Stores route information and race history

## Importing Race History from ZwiftPower

To calibrate predictions with your actual performance:

1. Visit your ZwiftPower profile page
2. Open browser console (F12)
3. Copy and run the extraction script:
   ```bash
   cat zwiftpower_profile_extractor.js | xclip -selection clipboard
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

## Built with Claude Code

This entire project was built using Claude Code (claude.ai/code) without traditional programming knowledge. The development process:

1. **Problem Definition**: Explained the race planning challenge to Claude
2. **Iterative Development**: 
   - Started with basic Zwift API integration
   - Added database storage when hardcoded routes became unwieldy
   - Integrated Strava when we discovered ZwiftPower limitations
   - Fixed multi-lap races using Claude's debugging assistance
3. **Transparency in Development**: Claude shows me:
   - What it's doing and WHY it's making each decision
   - Where my instructions might be ambiguous (leading to valid but different assumptions)
   - When the data tells a different story than expected (event names vs actual distances)
   - How to spot and fix misunderstandings before they become bugs

### Key Milestones
- **Initial version**: 92.8% error (using fake data)
- **Strava integration**: Dropped to 31.2% error
- **Multi-lap fix**: Achieved 25.1% error
- **Next goal**: Sub-20% with physics modeling

### Lessons for Building with AI
This is like managing a very willing and enthusiastic employee. Success requires:
- **Clear Problem Definition**: Know exactly what you're trying to achieve
- **Domain Knowledge**: The more you know about your problem space (racing physics, training data), the better
- **Technical Foundation**: 40 years on the command line means I can guide implementation choices
- **Understanding Limitations**: Both yours and the LLM's (it makes assumptions, you catch them)
- **Iterative Refinement**: Test with real data, spot discrepancies, adjust approach
- **Good Documentation**: Track decisions and discoveries (see ZWIFT_API_LOG.md)

The sweet spot: You don't need to code, but your technical experience helps you:
- Spot when something doesn't look right
- Suggest better approaches (like using SQLite over JSON files)
- Understand system constraints and possibilities
- Debug issues by asking the right questions

**For a detailed guide on this development approach, see [AI_DEVELOPMENT.md](AI_DEVELOPMENT.md)**

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
