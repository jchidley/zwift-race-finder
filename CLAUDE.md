# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Philosophy

This project demonstrates AI-assisted development where:
- **The Human** (Jack) provides: domain expertise (Zwift racing), technical knowledge (40 years IT), problem definition, and quality control
- **The AI** (Claude) provides: code implementation, API integration, debugging assistance, and transparent reasoning
- **Success comes from**: Clear communication, iterative refinement, and testing against real data

Key principles:
1. **Transparency**: Always explain what you're doing and why
2. **Assumptions**: Flag when making assumptions based on ambiguous requirements
3. **Data-Driven**: When data contradicts descriptions, investigate and clarify
4. **Pragmatic**: Simple solutions first, optimize based on real-world performance

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

# Update rider stats for personalized predictions
./update_rider_stats.sh 86.0        # Weight only
./update_rider_stats.sh 86.0 250    # Weight and FTP
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
2. Check event_sub_groups for category-specific distances (multi-lap races)
3. Use dual-speed model (if rider stats available):
   - Calculate drop probability based on elevation, weight, category
   - Pack speed: Category-based (Cat D: 30.9 km/h)
   - Solo speed: 77% of pack speed (no draft)
   - Weighted average based on drop probability
4. Apply elevation penalties (no bonuses for flat routes)
5. Fallback: Category-based estimation without drop dynamics

### Pack Dynamics Insights
- **Draft is King**: 33% power savings in Zwift vs 25% in real world (Source: Zwift Insider research)
- **Pack > Physics**: Pure physics model overestimated by 127% (68+ min for 30 min races)
- **Zwift != Real**: Martin et al. (1998) model accurate for real cycling but fails in Zwift
- **Context Matters**: Bigger races = more consistent draft = better predictions (empirical observation from 151 races)

### Drop Dynamics Discovery (Session 2025-05-25)
- **Root Cause of Variance**: Getting dropped on hills explains 82.6% variance (Bell Lap: 32-86 min)
- **Binary State**: Either with pack (30.9 km/h) or solo (23.8 km/h) - no middle ground
- **Weight Penalty**: Jack at 86kg vs typical 70-75kg = major disadvantage on climbs
- **Cascade Effect**: Drop early → race mostly solo → much longer total time
- **Model Status**: Dual-speed model implemented with 36.9% accuracy
- **Key Insight**: High variance is inherent to racing, not a prediction failure

### Route ID System
- Zwift uses internal route IDs that are stable across event name changes
- Route 9999 is a placeholder for unmapped routes
- The system automatically logs unknown routes for future mapping

### Event Types (Discovered & Fixed 2025-05-26)
Zwift has two mutually exclusive event categorization systems:
1. **Traditional Categories**: A/B/C/D/E with `distanceInMeters` populated
2. **Racing Score Events**: Score ranges (0-650) with `distanceInMeters: 0`
   - Identified by `rangeAccessLabel` field in event subgroups
   - Distance only available in description text
   - Tool now handles both types seamlessly with `is_racing_score_event()` and `parse_distance_from_description()`
   - Pro tip from Jack: Use `site:https://whatsonzwift.com` search for accurate route data

### UX Enhancements (Implemented 2025-05-26)
The tool now provides better guidance when users search for events:
1. **Event Type Summary**: Shows counts after fetching (e.g., "Found: 91 group rides, 52 races, 33 group workouts, 5 time trials")
2. **Smart No Results Messages**: Context-aware suggestions based on what was searched
   - For races: Explains most are 20-30 minutes, provides working examples
   - For time trials: Notes they're less common, suggests alternatives
   - General tips: Wider tolerance, different event types, typical durations
3. **Working Command Examples**: Shows exact commands like `cargo run -- -d 30 -t 30`

## Key Files and Modules

### Rust Code
- `src/main.rs`: CLI interface, event filtering, duration estimation
- `src/database.rs`: SQLite integration for routes and race results
- `src/regression_test.rs`: Tests comparing estimates vs actual race times

### Shell Scripts
- `import_zwiftpower_dev.sh`: Development import with schema matching
- `import_zwiftpower.sh`: Production import from browser extraction
- `zwiftpower_profile_extractor.js`: Browser script to extract race history
- `strava_auth.sh`: OAuth authentication for Strava
- `strava_fetch_activities.sh`: Fetch Zwift activities from Strava
- `strava_import_to_db.sh`: Import real race times to database

### Data Files
- `route_mappings.sql`: Maps event names to route IDs
- `zwiftpower_results.json`: Extracted race history (git-ignored)

## Regression Testing Strategy

The project uses Jack's actual race history to calibrate duration estimates:
1. Import historical races from Strava (151 races with real times)
2. Map events to route IDs using patterns and manual research
3. Compare predicted vs actual times
4. Adjust difficulty multipliers based on error analysis

Current accuracy: 23.6% mean absolute error (down from 92.8%)
Status: Production ready - exceeded <30% target

## Database Management

### Backup Strategy
```bash
# Backup database before major changes
cp ~/.local/share/zwift-race-finder/races.db ~/.local/share/zwift-race-finder/races.db.backup

# Create timestamped backup
cp ~/.local/share/zwift-race-finder/races.db ~/.local/share/zwift-race-finder/races_$(date +%Y%m%d).db
```

### Database Location
- Primary: `~/.local/share/zwift-race-finder/races.db`
- Contains: routes, race_results, strava_activities, unknown_routes

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

### Debugging Event Filtering Issues
When races aren't being found:
1. Run with `--debug` to see raw event data
2. Check if distance is 0.0 (Racing Score events always have 0)
3. Look for `rangeAccessLabel` field to identify Racing Score events
4. Verify route_id exists in database
5. Check if event description contains distance info
6. See plan.md "Solution Plan" for Racing Score event handling

## Log Management

This project uses hierarchical log management to keep LLM context efficient:
- **ZWIFT_API_LOG.md**: Index file pointing to Summary/Recent/Archives
- **ZWIFT_API_LOG_SUMMARY.md**: Executive summary (<3KB)
- **ZWIFT_API_LOG_RECENT.md**: Latest sessions (<2KB)
- **sessions/**: Archived complete logs by date

When adding log entries, append to ZWIFT_API_LOG_RECENT.md for current work.