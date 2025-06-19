# Zwift Race Finder

Find Zwift races that match your target duration and fitness level.

## Project Goals

This project serves two primary purposes:

1. **Learning AI/LLM Tools**: Exploring effective ways to use AI assistants (like Claude) for software development through practical implementation
2. **Improving Zwift Experience**: Creating tools that enhance training and racing on Zwift by solving real user problems

## What It Does

Zwift Race Finder predicts how long races will take based on your Zwift Racing Score. It solves a common problem: Zwift shows race distances but not expected durations. A 40km race might take 60 or 90 minutes depending on route profile and your fitness level.

**Key Features:**
- Fetches upcoming races from Zwift's public API
- Estimates completion time based on your Racing Score (0-999)
- Filters races by target duration (e.g., "show me 30-minute races")
- Achieves 20.4% prediction accuracy using real race data
- Supports both Traditional (A/B/C/D) and Racing Score events
- OCR capabilities for extracting real-time data from Zwift UI (experimental)

**Important**: Users are responsible for ensuring their use complies with Zwift's Terms of Service. Please review Zwift's ToS before using or modifying these tools.

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

## Documentation

### 🚴 For Racers - Optimize Your Performance
**[→ Racing Optimization Guides](docs/for-racers/)**

Learn how to race faster with guides on:
- **Power Optimization** - Managing the only variable you control
- **Draft Strategies** - Maximizing 24-33% power savings
- **Route Tactics** - Route-specific power distribution
- **Zwift as a Game** - Understanding and exploiting game mechanics
- **Category Racing** - Optimizing for your division

### 🛠️ For Developers
**[→ Developer Documentation](docs/for-developers/)**

Technical documentation for contributing:
- Architecture and design decisions
- Testing strategies and guidelines
- API research and integration patterns
- Active development plans

### 📚 Additional Resources
- **[Reference Docs](docs/reference/)** - Core algorithms, database schema, domain concepts
- **[Research](docs/research/)** - Zwift physics equations, performance analysis
- **[Guides](docs/guides/)** - Setup, data import, operations
- **[Project History](docs/project-history/)** - How we improved accuracy from 92.8% to 20.4%

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
│   ├── zwift_offline_client.rs # Integration with zwift-offline
│   └── bin/               # Utility programs
│       ├── analyze_descriptions.rs
│       ├── debug_tags.rs
│       ├── generate_filter_url.rs
│       └── import_zwift_offline_routes.rs
│
├── tools/                  # Development and maintenance tools
│   ├── import/            # Data import scripts
│   │   ├── zwiftpower/    # ZwiftPower race history import
│   │   ├── strava/        # Strava activity import
│   │   └── routes/        # Route data import
│   ├── debug/             # Debug and analysis tools
│   ├── ocr/               # OCR calibration and testing tools
│   ├── utils/             # Utility scripts
│   └── record-monitor2.ps1 # Windows PowerShell script for recording Zwift sessions
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
├── sessions/              # AI development session logs
├── scripts/               # Project-level scripts
│   └── import_from_zwift_offline.sh  # Import route data
└── zwift-offline/         # AGPL-licensed zwift-offline fork (gitignored)
```

## How It Works

### Duration Estimation

The tool estimates race duration using:

1. **Route Data**: Distance, elevation gain, and surface type from a database of 378 Zwift routes
2. **Racing Score**: Determines your speed category:
   - Cat D (0-199): ~30.9 km/h average
   - Cat C (200-299): ~33 km/h
   - Cat B (300-399): ~37 km/h
   - Cat A (400+): ~42 km/h
3. **Elevation Impact**: Hills add time based on climbing difficulty
4. **Lead-in Distance**: Accounts for neutral roll-out before the timed segment

### Accuracy

Current accuracy: **20.4% mean absolute error** (target was <20%, close but not quite achieved)

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

### Route Data Sources

The project maintains a comprehensive database of 378 Zwift routes from multiple sources:

```bash
# Import routes from third-party sources (ZwiftHacks, etc.)
./scripts/import_routes.sh

# Import from zwift-offline API (55 event-only routes)
./scripts/import_from_zwift_offline.sh --skip-ssl-verify

# Import into database
cargo run --bin import_zwift_offline_routes -- \
    --input-dir data/zwift_offline_export
```

See [docs/guides/ZWIFT_OFFLINE_INTEGRATION.md](docs/guides/ZWIFT_OFFLINE_INTEGRATION.md) for details on the zwift-offline integration.

### Recording Zwift Sessions

For capturing Zwift gameplay footage for analysis, OCR testing, or creating training datasets, use the included PowerShell recording script (Windows only):

#### Setup

```powershell
# Copy the script to a location in your PATH
# Example: C:\tools\ (adjust to your preference)
copy tools\record-monitor2.ps1 C:\tools\

# Or add the tools directory to your PATH environment variable
$env:Path += ";$PWD\tools"
```

#### Usage

```powershell
# Record 1-hour session with default settings (8fps video, 1fps PNG extraction)
record-monitor2.ps1 -duration 3600 -name "zwift_race"

# Record 30-minute race with higher framerate
record-monitor2.ps1 -fps 15 -duration 1800 -name "crit_city_race"

# Extract only PNG frames (no video) with smart filtering
record-monitor2.ps1 -pngOnly -smartFilter -duration 7200

# Custom resolution (useful for testing different screen sizes)
record-monitor2.ps1 -resolution "1920x1080" -duration 3600
```

**Features:**
- Records secondary monitor (where Zwift typically runs)
- Simultaneous video recording and PNG frame extraction
- Smart filtering to capture only frames with significant changes
- Configurable framerates for different use cases
- Output organized by timestamp for easy reference

**Output Location:** `%USERPROFILE%\Videos\Recordings\` (default)

**Requirements:**
- Windows with PowerShell
- FFmpeg installed and in PATH
- Secondary monitor running Zwift

**Use Cases:**
- Creating OCR training/testing datasets
- Analyzing race tactics and positioning
- Recording personal best efforts
- Debugging UI element detection

**OCR Integration:** The project includes experimental OCR capabilities for extracting real-time data from Zwift's UI (speed, power, distance, gradient, etc.). The recording script helps capture consistent footage for calibrating and testing OCR accuracy. See `tools/ocr/` for calibration tools and documentation.

## Essential Commands Reference

### Running Races
```bash
# Find races for your Racing Score
zwift-race-finder --zwift-score 195 --duration 120 --tolerance 30

# Record a race result for improving predictions
zwift-race-finder --record-result "route_id,minutes,event_name"
```

### Testing and Validation
```bash
# Run regression tests against 151 real races
cargo test regression

# Run specific test module
cargo test ocr

# Run mutation testing on a module
cargo mutants --file src/module.rs --timeout 30
```

### Personal Configuration
```bash
# Update rider stats for personalized predictions
./tools/utils/update_rider_stats.sh 86.0        # Weight only
./tools/utils/update_rider_stats.sh 86.0 250    # Weight and FTP
```

### Data Import
```bash
# Import from ZwiftPower (see docs/guides/DATA_IMPORT.md for details)
./tools/import/zwiftpower/import_zwiftpower_dev.sh

# Apply route mappings after import
./tools/utils/apply_route_mappings.sh
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

### Testing

The project includes comprehensive test coverage to ensure reliability and support safe refactoring:

#### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output for debugging
cargo test -- --nocapture

# Run specific test modules
cargo test integration    # Integration tests
cargo test regression    # Regression tests against real data
cargo test properties    # Property-based tests

# Run tests in release mode (faster)
cargo test --release

# Run with specific number of threads
cargo test -- --test-threads=1
```

#### Test Categories

**Unit Tests** (in `src/` files)
- Fast, focused tests for individual functions
- Run automatically with `cargo test`

**Integration Tests** (`tests/integration_tests.rs`)
- CLI command execution and parsing
- End-to-end workflows
- Database operations
- Output format verification

**Property-Based Tests** (`tests/property_tests.rs`)
- Mathematical invariants (monotonicity, boundaries)
- Randomized input generation
- Edge case discovery
```bash
# Run with more test cases
PROPTEST_CASES=1000 cargo test properties
```

**Snapshot Tests** (`tests/snapshot_tests.rs`)
- Behavioral verification for known routes
- Duration calculations across categories
- Ensures consistent output
```bash
# Update snapshots after intentional changes
cargo insta review
```

**Regression Tests** (`src/regression_test.rs`)
- Compare predictions against 151+ actual race results
- Target: <20% mean absolute error (currently 16.1%)
- Validates accuracy improvements

**API Tests** (`tests/api_tests.rs`)
- Mock-based API response handling
- Error scenarios and edge cases
- Racing Score event parsing

#### Testing During Development

```bash
# Run tests before committing
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check

# Run mutation testing to find weak tests
cargo install cargo-mutants
cargo mutants --file src/estimation.rs --timeout 120

# Generate test coverage report
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
# Open tarpaulin-report.html in browser
```

#### Golden Tests for Behavioral Preservation

The project uses golden tests to ensure refactoring doesn't change behavior:

```bash
# Generate behavioral baseline (run on main branch)
cargo test generate_golden_baseline -- --ignored

# Verify behavior matches baseline (run after changes)
cargo test golden_tests
```

#### Continuous Integration

Tests run automatically on:
- Every push to GitHub
- Pull request creation/update
- Pre-commit hooks (if configured)

To set up pre-commit hooks:
```bash
./setup_git_hooks.sh
```

### Adding New Routes

Routes can be added through multiple methods:

#### Method 1: Import from Data Sources
```bash
# Import from third-party sources
./scripts/import_routes.sh

# Import from zwift-offline (if running)
./scripts/import_from_zwift_offline.sh --skip-ssl-verify
```

#### Method 2: Manual Addition
1. Find the route on ZwiftHacks.com for the official route_id
2. Add to database:
   ```sql
   INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface)
   VALUES (route_id, distance, elevation, 'Route Name', 'World', 'road');
   ```
3. Update mappings in `sql/mappings/route_mappings.sql` if needed

For technical details on route data extraction, see [docs/guides/ROUTE_DATA_EXTRACTION.md](docs/guides/ROUTE_DATA_EXTRACTION.md).

### AI-Assisted Development Insights

This project demonstrates effective AI/LLM collaboration patterns:

**What Works Well:**
- **Clear Requirements**: Specific problems ("find 30-minute races") lead to focused solutions
- **Domain Expertise + AI**: Human knowledge of Zwift racing guided AI implementation
- **Iterative Testing**: Real race data validation exposed wrong assumptions quickly
- **Transparency**: AI explaining its reasoning caught errors early

**Key Learnings:**
- AI excels at implementation when given clear specifications
- Human domain knowledge is irreplaceable for validation
- Small, testable iterations work better than large changes
- Real data always beats theoretical models

**Development Process:**
1. Human identifies specific Zwift training need
2. AI implements solution with full code transparency
3. Human tests with actual race data
4. Iterate based on real-world results

The result is production-ready software achieving better than target accuracy while learning effective AI collaboration patterns.

## Security and Compliance

### OAuth Token Security
- OAuth tokens stored securely (environment vars, keyring, or encrypted files)
- Pre-commit hooks prevent accidental credential exposure
- All personal data excluded from repository

Run `tools/utils/check_secrets.sh` before committing.

### Zwift Terms of Service

**⚠️ Important**: Users are responsible for reviewing and complying with [Zwift's Terms of Service](https://zwift.com/terms). It is your responsibility to determine if your use of these tools is permitted.

**Our Approach:**
We aim to respect Zwift's ToS by:
- Using publicly available APIs
- Avoiding direct game client interaction
- Not providing automation or competitive advantages

**Techniques We Avoid** (based on community reports of bans):
- Network packet interception
- Memory reading or process manipulation  
- Protocol emulation or redirection
- Any form of gameplay automation

**Note**: Screen capture for analysis (like OBS streaming) appears to be accepted by the community, but users should verify this themselves.

Violations have reportedly resulted in 6-month racing bans. Always review current ToS before use.

## Credits

Built by Jack Chidley using Claude Code (Anthropic's AI assistant).

- Route data from multiple sources:
  - [zwift-data](https://github.com/andipaetzold/zwift-data) npm package
  - [WhatsOnZwift](https://whatsonzwift.com) route database
  - [ZwiftHacks](https://zwifthacks.com) comprehensive route info
  - [zwift-offline](https://github.com/zoffline/zwift-offline) API integration (55 event routes)
- Icons and visual assets from Zwift

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.