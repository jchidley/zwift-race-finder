# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Build and Run
```bash
# Build and run with default settings (shows races for next 24h)
cargo run

# Build release version and install
./install.sh

# Run with specific parameters
cargo run -- --zwift-score 195 --duration 120 --tolerance 30

# Show unknown routes that need mapping
cargo run -- --show-unknown-routes

# Record a race result
cargo run -- --record-result "route_id,minutes,event_name"

# Run tests (includes regression tests with actual race data)
cargo test

# Run specific test module
cargo test regression
```

### Data Import from ZwiftPower
```bash
# 1. First extract data from ZwiftPower in browser:
cat zwiftpower_profile_extractor.js | xclip -selection clipboard
# Then paste in browser console on ZwiftPower profile page

# 2. Import the downloaded results:
./import_zwiftpower_dev.sh  # For development/testing
./import_zwiftpower.sh      # For production

# 3. Apply route mappings to imported data:
./apply_route_mappings.sh
# Or manually: sqlite3 ~/.local/share/zwift-race-finder/races.db < route_mappings.sql
```

## Architecture Overview

### Core Concepts
The project estimates Zwift race durations based on:
1. **Route Data**: Distance, elevation, and surface type affect estimated completion time
2. **Zwift Racing Score**: Determines rider speed category (D: 0-199, C: 200-299, B: 300-399, A: 400+)
3. **Historical Data**: Jack's actual race results for regression testing and calibration

### Data Flow
```
Zwift API → Filter Events → Estimate Duration → Display Results
     ↓                            ↑
ZwiftPower → Import → SQLite → Route Mappings
```

### Database Schema
- **routes**: Known Zwift routes with distance, elevation, surface type
- **race_results**: Jack's historical race data for regression testing
- **unknown_routes**: Tracks events that need route mapping

### Duration Estimation Algorithm
1. Primary method: Use route_id to lookup known route data
2. Calculate base speed from Zwift Racing Score (Cat D: 25-27 km/h)
3. Apply difficulty multiplier based on elevation gain per km
4. Apply surface penalty for gravel/mixed routes
5. Fallback: Estimate from event name patterns or provided distance

### Route ID System
- Zwift uses internal route IDs that are stable across event name changes
- Route 9999 is a placeholder for unmapped routes
- The system automatically logs unknown routes for future mapping

## Key Files and Modules

### Rust Code
- `src/main.rs`: CLI interface, event filtering, duration estimation
- `src/database.rs`: SQLite integration for routes and race results
- `src/regression_test.rs`: Tests comparing estimates vs actual race times

### Shell Scripts
- `dev_import_results.sh`: Development import with schema matching
- `import_zwiftpower_results.sh`: Production import from browser extraction
- `extract_zwiftpower_v2.js`: Browser script to extract race history

### Data Files
- `route_mappings.sql`: Maps event names to route IDs
- `zwiftpower_results.json`: Extracted race history (git-ignored)

## Regression Testing Strategy

The project uses Jack's actual race history to calibrate duration estimates:
1. Import historical races from ZwiftPower
2. Map events to route IDs using patterns and manual research
3. Compare predicted vs actual times
4. Adjust difficulty multipliers based on error analysis

Current accuracy target: < 20% mean absolute error

## Common Development Tasks

### Adding New Routes
1. Find route on ZwiftHacks.com for official route_id
2. Get distance/elevation from Zwift Insider
3. Add to database: `INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface)`
4. Update `route_mappings.sql` if mapping from event names

### Improving Duration Estimates
1. Run regression tests: `cargo test regression`
2. Analyze prediction errors by route
3. Adjust elevation-based multipliers in `get_route_difficulty_multiplier_from_elevation()`
4. Consider draft benefit differences between races and time trials

### Debugging Unknown Routes
1. Run `cargo run -- --show-unknown-routes`
2. Research route details on ZwiftHacks/Zwift Insider
3. Add to routes table with proper data
4. Update import scripts to map event names