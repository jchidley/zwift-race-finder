# Zwift Race Finder

Find Zwift races that match your target duration and fitness level.

## What It Does

Zwift Race Finder predicts how long races will take based on your Zwift Racing Score. It solves a common problem: Zwift shows race distances but not expected durations. A 40km race might take 60 or 90 minutes depending on route profile and your fitness level.

**Key Features:**
- Fetches upcoming races from Zwift's public API
- Estimates completion time based on your Racing Score (0-999)
- Filters races by target duration (e.g., "show me 30-minute races")
- Achieves 16.1% prediction accuracy using real race data
- Supports both Traditional (A/B/C/D) and Racing Score events

## Quick Start

### Installation

```bash
# Build and install to ~/.local/bin
./install.sh

# Or build manually
cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/
```

### Basic Usage

```bash
# Find races around 30 minutes for a Cat D rider (score 195)
zwift-race-finder --zwift-score 195 --duration 30

# Allow 15-minute tolerance window (15-45 minutes)
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15

# Show next 3 days of races
zwift-race-finder --zwift-score 195 --duration 30 --days 3

# Show only group rides instead of races
zwift-race-finder --zwift-score 195 --duration 60 --event-type group_ride
```

### Configuration

Create a config file at `~/.config/zwift-race-finder/config.toml`:

```toml
[defaults]
zwift_score = 195
duration = 30
tolerance = 15

[preferences]
colored_output = true
```

See `config.example.toml` for all options.

## Project Structure

```
zwift-race-finder/
├── src/                    # Core Rust application
│   ├── main.rs            # CLI interface and event filtering
│   ├── lib.rs             # Library root
│   ├── config.rs          # Configuration management
│   ├── database.rs        # SQLite operations
│   ├── secure_storage.rs  # OAuth token storage
│   ├── regression_test.rs # Accuracy testing
│   └── bin/               # Utility programs
│       ├── analyze_descriptions.rs
│       ├── debug_tags.rs
│       └── generate_filter_url.rs
│
├── tools/                  # Development and maintenance tools
│   ├── import/            # Data import scripts
│   │   ├── zwiftpower/    # ZwiftPower race history import
│   │   ├── strava/        # Strava activity import
│   │   └── routes/        # Route data import
│   ├── debug/             # Debug and analysis tools
│   └── utils/             # Utility scripts
│
├── sql/                   # Database scripts
│   ├── migrations/        # Schema updates
│   ├── mappings/          # Route mapping data
│   └── analysis/          # Data analysis queries
│
├── docs/                  # Documentation
│   ├── development/       # Development logs and guides
│   ├── guides/            # User guides
│   ├── research/          # Technical research
│   └── screenshots/       # Visual documentation
│
└── sessions/              # AI development session logs
```

## How It Works

### Duration Estimation

The tool estimates race duration using:

1. **Route Data**: Distance, elevation gain, and surface type from a database of 264 Zwift routes
2. **Racing Score**: Determines your speed category:
   - Cat D (0-199): ~30.9 km/h average
   - Cat C (200-299): ~33 km/h
   - Cat B (300-399): ~37 km/h
   - Cat A (400+): ~42 km/h
3. **Elevation Impact**: Hills add time based on climbing difficulty
4. **Lead-in Distance**: Accounts for neutral roll-out before the timed segment

### Accuracy

Current accuracy: **16.1% mean absolute error** (target was <20%)

This was achieved by:
- Importing 151 actual race results from Strava
- Calibrating predictions against real data
- Accounting for pack dynamics and draft benefits

## Advanced Usage

### Update Rider Stats

For personalized predictions based on your weight and FTP:

```bash
tools/utils/update_rider_stats.sh 86.0        # Weight only
tools/utils/update_rider_stats.sh 86.0 250    # Weight and FTP
```

### Import Historical Data

#### From Strava (Recommended)

```bash
# 1. Set up authentication
tools/import/strava/strava_auth.sh

# 2. Fetch activities
tools/import/strava/strava_fetch_activities.sh

# 3. Import to database
tools/import/strava/strava_import_to_db.sh
```

#### From ZwiftPower

```bash
# 1. Extract data using browser console
cat tools/import/zwiftpower/zwiftpower_profile_extractor.js | xclip -selection clipboard
# Paste in browser console on your ZwiftPower profile page

# 2. Import the downloaded file
tools/import/zwiftpower/import_zwiftpower.sh
```

### Debug and Analysis

```bash
# Show unknown routes that need mapping
zwift-race-finder --show-unknown-routes

# Apply route mappings
tools/utils/apply_route_mappings.sh

# Check for exposed secrets
tools/utils/check_secrets.sh
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder

# Build debug version
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

### Adding New Routes

1. Find the route on ZwiftHacks.com for the official route_id
2. Add to database:
   ```sql
   INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface)
   VALUES (route_id, distance, elevation, 'Route Name', 'World', 'road');
   ```
3. Update mappings in `sql/mappings/route_mappings.sql` if needed

### Project Philosophy

This project was built using AI-assisted development (Claude Code) demonstrating:
- **Human expertise**: Domain knowledge (Zwift racing) and technical experience
- **AI capabilities**: Code implementation, API integration, debugging
- **Iterative refinement**: Real data validation and continuous improvement

The result is production-ready software achieving better than target accuracy.

## Security

- OAuth tokens stored securely (environment vars, keyring, or encrypted files)
- Pre-commit hooks prevent accidental credential exposure
- All personal data excluded from repository

Run `tools/utils/check_secrets.sh` before committing.

## Credits

Built by Jack Chidley using Claude Code (Anthropic's AI assistant).

- Route data from [zwift-data](https://github.com/andipaetzold/zwift-data) npm package
- Additional route info from [WhatsOnZwift](https://whatsonzwift.com)
- Icons and visual assets from Zwift

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.