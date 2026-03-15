# Zwift Race Finder — Complete Documentation

Generated: 2026-03-15

This file contains every active documentation file in the project, concatenated
for single-file review. Files are grouped by category.

---

## Table of Contents

1. [./AGENTS.md] — AGENTS.md
2. [./CLAUDE.md] — CLAUDE.md
3. [./README.md] — Zwift Race Finder
4. [./PROJECT_WISDOM.md] — PROJECT_WISDOM.md - Zwift Race Finder
5. [./tests/README.md] — Test Suite
6. [./docs/README.md] — Documentation
7. [./docs/tutorial/getting-started.md] — Tutorial: Find Your First Zwift Race
8. [./docs/howto/CONFIG_MANAGEMENT.md] — Configuration Management
9. [./docs/howto/DATA_IMPORT.md] — Data Import Guide
10. [./docs/howto/DEPLOYMENT.md] — Zwift Race Finder - Deployment Guide
11. [./docs/howto/SECRETS_SETUP.md] — Secrets Setup (direnv + ak)
12. [./docs/howto/SECURE_TOKEN_MIGRATION.md] — Secure Token Storage Migration Guide
13. [./docs/howto/TESTING_GUIDE.md] — Testing Guide
14. [./docs/howto/ZWIFT_OFFLINE_INTEGRATION.md] — Zwift-Offline Integration Guide
15. [./docs/howto/ZWIFTPOWER_EXPORT_STEPS.md] — ZwiftPower Export Steps
16. [./docs/reference/ALGORITHMS.md] — Duration Estimation Algorithms
17. [./docs/reference/ARCHITECTURE.md] — Zwift Race Finder Architecture
18. [./docs/reference/DATABASE.md] — Database Reference
19. [./docs/reference/INTEGRATION_TEST_COVERAGE.md] — Integration Test Coverage Analysis
20. [./docs/reference/PHYSICAL_STATS.md] — Physical Stats and Their Impact on Zwift Performance
21. [./docs/reference/REFACTORING_RULES.md] — Refactoring Rules for Claude
22. [./docs/reference/REQUIREMENTS.md] — REQUIREMENTS.md
23. [./docs/reference/ROUTE_DATA_EXTRACTION.md] — Zwift Route Data Extraction Documentation
24. ~~REMOVED: ./docs/reference/RUST_REFACTORING.md — merged into REFACTORING_RULES.md~~
25. [./docs/reference/SECURITY_AUDIT.md] — Security Audit Report - Zwift Race Finder
26. [./docs/reference/SIMULATION_TOOLS.md] — Zwift Simulation Tools for Automated Testing
27. [./docs/reference/TEST_SUITE_SUMMARY.md] — Zwift Race Finder Test Suite Summary
28. [./docs/reference/ZWIFT_DOMAIN.md] — Zwift Domain Knowledge
29. [./docs/reference/ZWIFT_QUICK_REFERENCE.md] — Zwift Racing Quick Reference
30. [./docs/explanation/AI_DEVELOPMENT.md] — AI-Assisted Development: Building Zwift Race Finder with Claude Code
31. [./docs/explanation/REFACTORING_EXPLAINED.md] — Understanding Refactoring: A Human's Guide
32. [./docs/explanation/TESTING_PHILOSOPHY.md] — Testing Philosophy
33. [./docs/explanation/ZWIFT_PHYSICS_EQUATIONS.md] — Zwift Physics Equations and Sources
34. [./docs/explanation/ZWIFT_PHYSICS.md] — Zwift Physics
35. [./docs/explanation/ZWIFT_RACING_TACTICS.md] — Zwift Racing Tactics
36. [./docs/project-history/ACCURACY_TIMELINE.md] — Zwift Race Finder Accuracy Timeline
37. [./docs/project-history/FEEDBACK.md] — Zwift Race Finder - User Feedback
38. [./docs/project-history/HISTORICAL_DISCOVERIES.md] — Historical Discoveries
39. [./docs/screenshots/README.md] — Zwift Event Screenshots

---

<!-- ============================================================ -->
<!-- FILE: ./AGENTS.md -->
<!-- ============================================================ -->

# AGENTS.md

## Overview
Zwift Race Finder predicts race durations from Zwift event data and a rider's Racing Score, then filters events by target duration. Rust CLI, SQLite database, Zwift public API.

## Commands
| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Run | `cargo run --release --bin zwift-race-finder -- --zwift-score 195 --duration 30 --tolerance 15` |
| All tests | `cargo test` |
| Regression (accuracy) | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| API tests | `cargo test --test api_tests` |
| Integration tests | `cargo test --test integration_tests` |
| Property tests | `cargo test --test property_tests` |
| Show unknown routes | `cargo run --release --bin zwift-race-finder -- --show-unknown-routes` |
| Import routes | `./scripts/import_from_zwift_offline.sh` |

## Build prerequisites
Debian/Ubuntu: `sudo apt-get install -y libssl-dev pkg-config`

## Project structure
```
src/main.rs                  # CLI entry point (26 unit tests)
src/duration_estimation.rs   # Core prediction: distance / (category_speed * difficulty_multiplier)
src/event_filtering.rs       # Event matching and filtering
src/event_display.rs         # Output formatting
src/estimation.rs            # Route lookup + duration calculation bridge
src/category.rs              # Score→category mapping, category speeds
src/database.rs              # SQLite (path: ~/.local/share/zwift-race-finder/races.db)
src/config.rs                # Config loading (./config.toml → ~/.config/zwift-race-finder/config.toml → defaults)
src/bin/                     # 7 secondary binaries (OCR, debug, import)
data/zwift_offline_export/   # Route data (JSON)
sql/                         # Schema migrations and mappings
docs/                        # Diátaxis: tutorial/ howto/ reference/ explanation/
archive/                     # Historical docs — do not modify
```

## Multiple binaries
This crate has 8 binaries. Always use `--bin zwift-race-finder` with `cargo run`. OCR binaries require `--features ocr` and additional system deps (leptonica, tesseract).

## Boundaries
- Never change duration formula behavior without running regression test and comparing MAE.
- Never use route names as identifiers. Always use `route_id` (u32; values can exceed i32 range).
- Never commit credentials or personal data (Strava tokens, ZwiftPower IDs).
- Do not modify files under `archive/` — they are historical records.

## Duration model (what actually runs)
The production algorithm is `distance_km / (category_speed * difficulty_multiplier)`. Category speeds: E=28, D=30.9, C=33, B=37, A=42, A++=45 km/h. These are empirical from real races and already include draft benefit.

The difficulty multiplier uses piecewise linear interpolation on elevation gain per km (m/km), with category-aware penalties on climbs >15 m/km. Lower categories (D, E) are disproportionately slower on climbs due to lower w/kg. The `estimate_duration_with_distance` and `estimate_duration_from_route_id` functions both use the elevation-based multiplier when elevation data is available. The name-based fallback (`estimate_duration_for_category`) is only used when elevation data is unavailable.

Route aliases map alternative Zwift API route IDs (event-only variants) to canonical DB route IDs via the `route_aliases` table, checked transparently by `Database::get_route()`.

Lead-in distance is added only for multi-lap races in `event_filtering.rs`. Single-lap estimation does not add lead-in (potential accuracy gap).

## Current metrics
- MAE: 16.6% on 125 matched races (target: <20%)
- Routes in DB: 126 (+ 11 route aliases for event-only variants)
- Total tests: 170 passing (across lib + 7 integration test files)

## Troubleshooting
| Symptom | Fix |
|---------|-----|
| Route not found | Run `--show-unknown-routes`, find `route_id` on ZwiftHacks, update route data |
| Prediction looks wrong | Check lead-in distance and Racing Score input. Run regression test. |
| Build fails (OpenSSL) | Install `libssl-dev pkg-config` |
| `cargo run` ambiguous binary | Add `--bin zwift-race-finder` |
| OCR build fails | Needs `--features ocr` plus leptonica/tesseract system libs |
| `--record-result` FK error | Route must exist in routes table first; add it before recording results |

## Known issues
- Only one API base URL in code (`us-or-rly101`) — no failover to other regions.
- `rider_stats` table stores weight/FTP but values are not used directly in the estimation algorithm. The w/kg effect on climbs is captured through the category × elevation interaction in the difficulty multiplier.
- Time trials use same draft-inclusive category speeds as races (known inaccuracy — TTs have no draft).
- Route data export has 309 routes but only 126 are imported to DB — gap may contain usable routes.
- Single-lap estimation does not add lead-in distance (potential accuracy gap).

## Unimplemented improvements (from archive review 2026-03-15)
Verified against current codebase — these are still valid and not yet done:
- **TT draft removal**: Time trials should use solo speeds, not pack speeds.
- **Race field size factor**: Bigger fields = more consistent draft, smaller fields = more variance.
- **Elevation profile database**: Currently only uses elevation/km ratio, not gradient profile.
- **Route mapping gaps**: Event series like EVO CC, Sydkysten, Tofu Tornado still use placeholder or missing route IDs. Research in `archive/consolidated-originals/ROUTE_MAPPING_RESEARCH.md` has specific route_id mappings not yet imported.
- **Lap detection from descriptions**: Could automate multi-lap detection from event description text.
- **Route completion tracking**: DB schema exists (`route_completion` table) but no CLI integration. Design spec in `archive/consolidated-originals/ROUTE_TRACKING_IDEAS.md`.
- **Hidden event tags**: Zwift API contains undocumented tags useful for filtering (research in `archive/consolidated-originals/ZWIFTHACKS_TECHNIQUES.md`).

## Documentation
Human-readable docs follow Diátaxis in `docs/`:
- `docs/tutorial/` — getting started
- `docs/howto/` — task guides (deployment, data import, config, secrets)
- `docs/reference/` — algorithms, architecture, database, requirements
- `docs/explanation/` — Zwift physics, racing tactics, testing philosophy

<!-- ============================================================ -->
<!-- FILE: ./CLAUDE.md -->
<!-- ============================================================ -->

# CLAUDE.md

Read and follow **AGENTS.md** for all project instructions.

<!-- ============================================================ -->
<!-- FILE: ./README.md -->
<!-- ============================================================ -->

# Zwift Race Finder

Predict Zwift race durations from your Racing Score and find events that fit your schedule.

## Quick Start

```bash
cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15
```

Optionally save defaults to `~/.config/zwift-race-finder/config.toml` (see `config.example.toml`).

## Documentation

| Need | Go to |
|------|-------|
| Learn to use it | [Tutorial: Find your first race](docs/tutorial/getting-started.md) |
| Do a specific task | [How-to guides](docs/howto/) — deployment, data import, config, secrets |
| Look something up | [Reference](docs/reference/) — algorithms, architecture, database, CLI |
| Understand why | [Explanation](docs/explanation/) — Zwift physics, racing tactics, testing philosophy |

Other resources:
- [Project history](docs/project-history/) — accuracy timeline, discoveries
- [PROJECT_WISDOM.md](PROJECT_WISDOM.md) — learning log

## Notes

- Follow Zwift's Terms of Service when using or modifying this tool.
- Run `cargo test regression_test -- --nocapture` before claiming accuracy changes.

## About This Code

Almost all of this code is AI/LLM-generated. It's best used as a source of
inspiration for your own AI/LLM efforts rather than as a traditional library.

**This is personal alpha software.** If you want to use it:

- **Pin to a specific commit** — don't track `main`, it changes without warning
- **Use AI/LLM to adapt** — without AI assistance, this project is hard to use
- **Treat as inspiration** — build your own version rather than depending on mine

Suggestions welcome as inspiration for future improvements.

## License

MIT OR Apache-2.0

<!-- ============================================================ -->
<!-- FILE: ./PROJECT_WISDOM.md -->
<!-- ============================================================ -->

# PROJECT_WISDOM.md - Zwift Race Finder

Learning log for project-specific insights and solutions.

*Note: Older entries archived to PROJECT_WISDOM_ARCHIVE_20250614.md*

## 2025-06-13: Gymnasticon UDP Control - Correct netcat Syntax
**Insight**: OpenBSD netcat (on Raspberry Pi) requires specific syntax for UDP: `echo '{"power":250}' | nc -u -w1 127.0.0.1 3000`. The `-w1` flag and explicit IP `127.0.0.1` (not `localhost`) are critical.
**Impact**: UDP messages won't work without proper syntax. Always use the exact format shown above.

## 2025-06-13: Gymnasticon Bluetooth/Noble - Socket Binding Issue
**Insight**: The gymnasticon bot mode had a duplicate `bind()` call in the compiled code causing "Socket already bound" errors. Check for duplicate binds when debugging UDP server issues.
**Impact**: If you see "ERR_SOCKET_ALREADY_BOUND", look for duplicate bind() calls in the code, not just port conflicts.

## 2025-06-13: Zwift Connection - Both Power AND Cadence Required
**Insight**: When connecting gymnasticon to Zwift, you MUST connect both Power Source AND Cadence sensors (same device). Connecting only Power often fails. Connection reliability varies between Bluetooth and ANT+.
**Impact**: Always connect both sensors, have both protocols available, and be prepared to try different connection orders.

## 2025-06-14: License Integration - API Boundary for AGPL/MIT Compatibility
**Insight**: Successfully integrated AGPL-licensed zwift-offline with MIT/Apache zwift-race-finder using API boundary pattern. Created fork with export endpoints, no code copying required.
**Impact**: When integrating copyleft (AGPL/GPL) code with permissive licenses, use HTTP APIs or service boundaries. Never copy code directly.

## 2025-06-14: Python Environment - Use uv for zwift-offline Setup
**Insight**: zwift-offline Python environment setup is fast and reliable with `uv`: create setup_venv.sh script, use `uv venv` and `uv pip install -r requirements.txt`.
**Impact**: Avoid pip/virtualenv issues. Always use uv for Python dependency management in this project.

## 2025-06-14: Port Privileges - zwift-offline Environment Variables
**Insight**: zwift-offline defaults to privileged ports (80/443) causing "Permission denied". Use environment variables: ZOFFLINE_CDN_PORT=8080 ZOFFLINE_API_PORT=8443.
**Impact**: Never run as root. Always set port environment variables for non-privileged operation.

## 2025-06-14: Database Path - races.db not zwift_routes.db
**Insight**: zwift-race-finder uses `~/.local/share/zwift-race-finder/races.db` (from get_database_path()), not zwift_routes.db. Always check actual implementation.
**Impact**: Database operations fail with wrong filename. Verify paths in database.rs before assuming filenames.

## 2025-06-14: Route Data Location - WAD Archives Contain Everything
**Insight**: All Zwift route data exists in WAD archives at `C:\Program Files (x86)\Zwift\assets\Worlds\world*\data_1.wad`. Complete extraction requires wad_unpack.exe (no longer publicly available from referenced repos).
**Impact**: Can't extract complete route data without the decompression tool. Third-party sources and zwift-offline API provide sufficient coverage.

## 2025-06-14: zwift-offline Route Filtering - Event-Only by Design
**Insight**: zwift-offline only exports routes with `eventOnly='1'` attribute (55 routes). Free-ride routes are intentionally filtered out in the codebase.
**Impact**: This is a design choice, not a bug. For complete route coverage, use third-party sources that include all route types.

## 2025-06-14: Zwift Event Structure - Routes + Modifiers
**Insight**: Events don't create new routes - they use existing base routes with modifiers: laps (repeat loop N times), distance (fixed total), or duration (time-based). Free-ride routes have lead-in + lap structure.
**Impact**: When analyzing events, look for the base route_id and modifier, not a unique "event route". All base data is in the route definition.

## 2025-06-14: Route ID Handling - Use IDs Not Names
**Insight**: Route IDs can be negative (signed integers). Always use route_id as the identifier, never route name, as names change frequently but IDs are permanent.
**Impact**: Database and import tools must handle signed route IDs. Name-based lookups will fail when Zwift updates route names.

## 2025-06-19: Duration Model Simplification - Draft Already in Category Speeds
**Insight**: Claude simplified the original modeling approach for unknown reasons. Original intent was to model solo riding accurately then apply drafting factors, assuming detailed route profiling beyond just elevation/distance. Current system uses category speeds (30.9 km/h for Cat D) that already include average draft benefits from 151 real races.
**Impact**: The dual-speed model was removed (2026-03-15) since it was never called in production. Current 16.6% accuracy achieved with simpler category speed model because empirical speeds inherently include draft. More sophisticated modeling still possible with better route profiling.

## 2025-06-19: Power Simulation Tools - vpower and gymnasticon
**Insight**: Tools like vpower (https://github.com/oldnapalm/vpower) and gymnasticon can simulate power output for Zwift, enabling controlled testing of race duration algorithms with repeatable power profiles.
**Impact**: Can validate duration predictions by simulating consistent power outputs across different routes and conditions. Useful for understanding power/speed relationships and testing edge cases without needing real race data.

## 2025-06-22: Garmin Connect API - FIT Files Wrapped in ZIP
**Insight**: Garmin Connect API returns FIT files wrapped in ZIP containers (identified by PK\x03\x04 header). Virtual cycling activities from Zwift use "virtual_ride" or "virtual_cycling" activity type keys, not the standard cycling types.
**Impact**: When downloading FIT files, check for ZIP header and extract. Must include virtual_* activity types to capture Zwift rides.
## 2026-03-15: Zwift Route ID Aliasing - Same Route, Different IDs
**Insight**: Zwift uses different internal route IDs for the same physical route depending on whether it's accessed in a free-ride or event-only context. The zwift-offline export lists 309 routes with their IDs, and cross-referencing with the `unknown_routes` table revealed 11 cases where the API sends one route_id but the DB stores the same route under a different ID. Example: "Scotland - Loch Loop" is 742057576 in event context but 3019598975 in the DB.
**Impact**: Added `route_aliases` table and transparent alias resolution in `Database::get_route()`. Resolved ~2,640 previously-unresolvable event sightings. Always check aliases when a route_id lookup fails. The alias SQL is in `sql/mappings/route_aliases.sql`.

## 2026-03-15: Climb Speed Modeling - Category × Elevation Interaction
**Insight**: The question "should we use rider weight/height for better predictions?" was a misframing. Analysis of 125 races showed: on flat terrain, all categories ride near their empirical speed (ratio ≈ 1.0), but on climbs >20 m/km, Cat D achieves only 48% of its flat speed while Cat C achieves 54%. This is the w/kg effect, but it's better modeled as a category × elevation interaction than raw weight input. Two bugs were the real culprit: (a) `estimate_duration_with_distance()` used name-based multiplier, ignoring elevation data for routes like "Road to Sky" (got 1.0 instead of ~0.35); (b) the elevation multiplier capped at 0.7, far too high for steep climbs.
**Impact**: Replaced step-function with 9-breakpoint piecewise linear interpolation + category climbing penalty. MAE: 17.9% → 16.6%. Climbing MAE: 43.8% → <20%. No flat route regression. Weight/height remain stored but unused — the effect is fully captured through category-aware elevation multipliers.

<!-- ============================================================ -->
<!-- FILE: ./tests/README.md -->
<!-- ============================================================ -->

# Test Suite

## Running tests

| Purpose | Command |
|---------|---------|
| All tests | `cargo test` |
| Regression accuracy | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| API tests | `cargo test --test api_tests` |
| Integration tests | `cargo test --test integration_tests` |
| Config tests | `cargo test --test config_tests` |
| Property tests | `cargo test --test property_tests` |
| Snapshot tests | `cargo test --test snapshot_tests` |
| Benchmarks | `cargo bench` |
| Unit tests only | `cargo test --lib` |

## Current state (2026-03-15)

- **170 tests passing** (91 lib + 79 integration/property/snapshot)
- **Regression accuracy**: 16.6% MAE on 125 matched races
- **6 tests ignored** (golden tests, full regression on 7500+ races)

## Test locations

| Location | Tests | Purpose |
|----------|-------|---------|
| `src/main.rs` | 26 | CLI logic, event filtering, URL parsing |
| `src/duration_estimation.rs` | 12 | Duration math, difficulty multipliers, category-aware climbing |
| `src/event_filtering.rs` | 25 | Filter logic, category matching |
| `src/event_display.rs` | 18 | Output formatting |
| `src/regression_test.rs` | 4 | Accuracy vs real race data |
| `src/category.rs` | 4 | Score→category, speeds |
| `tests/api_tests.rs` | 6 | API interaction with mocks |
| `tests/config_tests.rs` | 6 | Config loading, precedence |
| `tests/integration_tests.rs` | 12 | CLI end-to-end workflows |
| `tests/property_tests.rs` | 7 | Property-based edge cases |
| `tests/properties_tests.rs` | 9 | Additional properties |
| `tests/snapshot_tests.rs` | 4 | Output format stability |

<!-- ============================================================ -->
<!-- FILE: ./docs/README.md -->
<!-- ============================================================ -->

# Documentation

## Start here

New to the tool? Follow the **[Tutorial: Find your first race](tutorial/getting-started.md)**.

## How-to guides

Task-oriented instructions for specific goals.

- [How to deploy and update](howto/DEPLOYMENT.md)
- [How to configure settings](howto/CONFIG_MANAGEMENT.md)
- [How to set up secrets](howto/SECRETS_SETUP.md)
- [How to migrate to secure token storage](howto/SECURE_TOKEN_MIGRATION.md)
- [How to import data from Strava and ZwiftPower](howto/DATA_IMPORT.md)
- [How to export from ZwiftPower](howto/ZWIFTPOWER_EXPORT_STEPS.md)
- [How to integrate with zwift-offline](howto/ZWIFT_OFFLINE_INTEGRATION.md)
- [How to test (mutation testing, golden tests, validation)](howto/TESTING_GUIDE.md)

## Reference

Facts, specs, and lookup tables. No step-by-step instructions.

- [Algorithms](reference/ALGORITHMS.md) — duration estimation model
- [Architecture](reference/ARCHITECTURE.md) — system components and data flow
- [Database](reference/DATABASE.md) — schema, tables, queries
- [Requirements](reference/REQUIREMENTS.md) — functional and non-functional requirements
- [Zwift domain](reference/ZWIFT_DOMAIN.md) — event types, categories, route IDs
- [Test suite](reference/TEST_SUITE_SUMMARY.md) — what's tested and how
- [Integration tests](reference/INTEGRATION_TEST_COVERAGE.md) — API and DB test coverage
- [Security audit](reference/SECURITY_AUDIT.md) — credential handling review
- [Route data extraction](reference/ROUTE_DATA_EXTRACTION.md) — how route data is sourced
- [Simulation tools](reference/SIMULATION_TOOLS.md) — power simulation and testing
- [Refactoring rules](reference/REFACTORING_RULES.md) — contract, mechanics, and Rust patterns for behaviour-preserving changes
- [Physical stats](reference/PHYSICAL_STATS.md) — height, weight, aerodynamics
- [Zwift quick reference](reference/ZWIFT_QUICK_REFERENCE.md) — group size, power, gap decision tables

## Explanation

Background, rationale, and deep dives. Read away from the keyboard.

### Zwift racing
- [Zwift racing tactics](explanation/ZWIFT_RACING_TACTICS.md) — pack dynamics, breakaways, positioning, timing
- [Zwift physics](explanation/ZWIFT_PHYSICS.md) — equations, draft, aerodynamics, real vs virtual
- [Zwift physics equations](explanation/ZWIFT_PHYSICS_EQUATIONS.md) — CdA formula, Crr values, Martin equation

### Development
- [AI-assisted development](explanation/AI_DEVELOPMENT.md) — building software with Claude Code
- [Testing philosophy](explanation/TESTING_PHILOSOPHY.md) — why we test this way, the 0% mutation lesson
- [Refactoring explained](explanation/REFACTORING_EXPLAINED.md) — LLM refactoring research, competence boundaries, and prompting strategies

## Project history

- [Accuracy timeline](project-history/ACCURACY_TIMELINE.md)
- [Historical discoveries](project-history/HISTORICAL_DISCOVERIES.md)
- [User feedback](project-history/FEEDBACK.md)

<!-- ============================================================ -->
<!-- FILE: ./docs/tutorial/getting-started.md -->
<!-- ============================================================ -->

# Tutorial: Find Your First Zwift Race

Find a Zwift race that fits your schedule by predicting how long it will take you to finish.

## Before you start

You need:
- Linux or WSL2 (Windows Subsystem for Linux)
- Rust toolchain (`rustup` installed)
- Build dependencies: `sudo apt-get install -y libssl-dev pkg-config`
- Your Zwift Racing Score (find it on [ZwiftPower](https://zwiftpower.com) or in the Zwift Companion app)

## Step 1: Install

Clone and build the tool:

```bash
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder
cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/
```

You should see:

```
Finished `release` profile [optimized] target(s) in ...
```

## Step 2: Find races around 30 minutes

Run the tool with your Racing Score. We'll use 195 as an example (Category D):

```bash
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15
```

You should see a table of upcoming Zwift races estimated to last 15–45 minutes for a rider with your score. Each row shows the event name, start time, distance, and predicted duration.

If you see "No events found", try widening the search:

```bash
zwift-race-finder --zwift-score 195 --duration 60 --tolerance 30
```

## Step 3: Try different event types

By default, only races are shown. See all event types:

```bash
zwift-race-finder --zwift-score 195 --duration 60 --tolerance 30 --event-type all
```

Notice the event type summary at the top — it tells you how many races, group rides, fondos, and workouts were found.

## Step 4: Look further ahead

Search the next 3 days instead of just today:

```bash
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15 --days 3
```

Note: Zwift's API returns a maximum of 200 events (~12 hours). Multi-day searches may not cover the full range.

## Step 5: Save your defaults

Create a config file so you don't have to type your score every time:

```bash
mkdir -p ~/.config/zwift-race-finder
cp config.example.toml ~/.config/zwift-race-finder/config.toml
```

Edit `~/.config/zwift-race-finder/config.toml` and set your Racing Score, weight, and preferred duration.

Now you can run with just:

```bash
zwift-race-finder
```

## What next

- [How to import race data from Strava](../howto/DATA_IMPORT.md) — improve prediction accuracy with your actual race times
- [How to deploy and update](../howto/DEPLOYMENT.md) — install the tool permanently
- [Algorithm reference](../reference/ALGORITHMS.md) — understand how duration predictions work
- [About Zwift racing tactics](../explanation/ZWIFT_RACING_TACTICS.md) — pack dynamics, positioning, attack timing

<!-- ============================================================ -->
<!-- FILE: ./docs/howto/CONFIG_MANAGEMENT.md -->
<!-- ============================================================ -->

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
<!-- ============================================================ -->
<!-- FILE: ./docs/howto/DATA_IMPORT.md -->
<!-- ============================================================ -->

# Data Import Guide

## Overview

Zwift Race Finder can import data from multiple sources to improve predictions and discover new routes.

## ZwiftPower Import

### Prerequisites
- Active ZwiftPower account linked to your Zwift account
- Web browser with developer console access

### Export Process

1. **Navigate to Your Profile**:
   - Log into ZwiftPower.com
   - Go to your profile page

2. **Extract Race Data**:
   ```bash
   # Copy the extraction script to clipboard
   cat tools/zwiftpower/zwiftpower_profile_extractor.js | xclip -selection clipboard
   
   # Or on macOS:
   cat tools/zwiftpower/zwiftpower_profile_extractor.js | pbcopy
   ```

3. **Run in Browser Console**:
   - Open browser developer tools (F12)
   - Go to Console tab
   - Paste and run the script
   - Save the downloaded `zwiftpower_results.json`

4. **Import to Database**:
   ```bash
   # For development/testing
   ./tools/zwiftpower/import_zwiftpower_dev.sh
   
   # For production
   ./tools/zwiftpower/import_zwiftpower.sh
   ```

### What Gets Imported
- Race finish times
- Route information
- Category/score data
- Event dates
- Power/weight data (if available)

## Strava Import

### Prerequisites
- Strava account with Zwift activities
- Strava API application (create at https://www.strava.com/settings/api)

### Setup

1. **Configure Strava App**:
   ```bash
   # Copy example config
   cp tools/import/strava/strava_config.json.example strava_config.json
   
   # Edit with your app details
   vim strava_config.json
   ```

2. **Authenticate**:
   ```bash
   # Run authentication flow
   ./tools/import/strava/strava_auth.sh
   
   # Or with secure storage
   ./tools/import/strava/strava_auth_secure.sh
   ```

3. **Fetch Activities**:
   ```bash
   # Fetch recent Zwift activities
   ./tools/import/strava/strava_fetch_activities.sh
   
   # Import to database
   ./tools/import/strava/strava_import_to_db.sh
   ```

### What Gets Imported
- Actual race completion times
- Route names from activity titles
- Distance and elevation
- Average speed/power
- Date and time

## Route Data Import

### WhatsOnZwift Data

1. **Fetch Route Data**:
   ```bash
   cd tools/import/routes
   
   # Fetch all routes
   ./fetch_whatsonzwift_route_data.sh
   
   # Parse and import
   python3 fetch_whatsonzwift_route_data_parser.py
   ```

2. **Import to Database**:
   ```bash
   # Import parsed routes
   python3 import_zwift_data_routes.py
   ```

### Manual Route Addition

For routes not in automated sources:

```sql
-- Connect to database
sqlite3 ~/.local/share/zwift-race-finder/races.db

-- Add new route
INSERT INTO routes (route_id, distance_km, elevation_m, lead_in_km, 
                   lead_in_elevation_m, name, world, surface, slug)
VALUES (
    12345,           -- route_id from ZwiftHacks
    25.3,            -- distance in km
    341,             -- elevation in meters
    2.1,             -- lead-in distance
    15,              -- lead-in elevation
    'Epic Route',    -- route name
    'Watopia',       -- world
    'road',          -- surface type
    'epic-route'     -- URL slug
);
```

## Route Mapping

### Apply Standard Mappings

```bash
# Apply all route mappings
./tools/utils/apply_route_mappings.sh

# Or manually
sqlite3 ~/.local/share/zwift-race-finder/races.db < sql/mappings/route_mappings.sql
```

### Fix Unmapped Routes

```bash
# Show routes needing mapping
cargo run -- --show-unknown-routes

# Apply fixes
./tools/utils/fix_unmapped_routes.sh
```

### Multi-Lap Race Mappings

```sql
-- Special handling for multi-lap races
sqlite3 ~/.local/share/zwift-race-finder/races.db < sql/mappings/fix_multi_lap_mappings.sql
```

## Event Description Import

For Racing Score events that need description parsing:

```bash
# Fetch event descriptions
cd tools/import/routes
./fetch_event_descriptions.sh

# This helps identify:
# - Distance information in descriptions
# - Multi-stage event details
# - Special event formats
```

## Data Validation

### Verify Import Success

```sql
-- Check imported race results
SELECT COUNT(*) as total_races FROM race_results;

-- Check route coverage
SELECT COUNT(DISTINCT route_id) as unique_routes FROM routes;

-- Find missing data
SELECT event_title, COUNT(*) as occurrences
FROM unknown_routes
WHERE route_id = 9999
GROUP BY event_title
ORDER BY occurrences DESC
LIMIT 10;
```

### Test Predictions

```bash
# Run regression tests with imported data
cargo test regression

# Check accuracy
cargo run -- --test-predictions
```

## Troubleshooting

### Common Issues

1. **Duplicate Imports**:
   ```sql
   -- Remove duplicates (keeping first occurrence)
   DELETE FROM race_results
   WHERE rowid NOT IN (
       SELECT MIN(rowid)
       FROM race_results
       GROUP BY event_name, event_date, category
   );
   ```

2. **Missing Route Mappings**:
   - Check event title variations
   - Look for route_id on ZwiftHacks
   - Add to `route_mappings.sql`

3. **Import Script Fails**:
   - Check file permissions
   - Verify JSON format
   - Check database path

### Data Cleanup

```bash
# Backup before cleanup
cp ~/.local/share/zwift-race-finder/races.db ~/.local/share/zwift-race-finder/races.db.pre-cleanup

# Remove old/test data
sqlite3 ~/.local/share/zwift-race-finder/races.db "DELETE FROM race_results WHERE event_date < date('now', '-2 years');"

# Vacuum database
sqlite3 ~/.local/share/zwift-race-finder/races.db "VACUUM;"
```

## Best Practices

1. **Regular Imports**: Weekly for active racers
2. **Backup First**: Always backup before bulk imports
3. **Verify Data**: Check a few records manually
4. **Map Routes**: Investigate unknown routes promptly
5. **Share Mappings**: Contribute route discoveries back
<!-- ============================================================ -->
<!-- FILE: ./docs/howto/DEPLOYMENT.md -->
<!-- ============================================================ -->

# Zwift Race Finder - Deployment Guide

This guide covers deployment and production usage of Zwift Race Finder.

## Production Installation

### Quick Install (Recommended)
```bash
# Clone, build, and install in one step
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder
./install.sh
```

This will:
1. Build the release version with optimizations
2. Install to `~/.local/bin/zwift-race-finder`
3. Show basic usage examples

### Manual Installation
```bash
# Build release version
cargo build --release

# Copy to local bin
cp target/release/zwift-race-finder ~/.local/bin/

# Ensure ~/.local/bin is in PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## First Run

### Basic Usage (No Config Needed)
```bash
# Find races for next 24 hours with default settings
zwift-race-finder

# The tool will show what it found:
# "Found: 91 group rides, 52 races, 33 group workouts, 5 time trials"

# Common searches:
zwift-race-finder -d 30 -t 30    # 20-60 minute races
zwift-race-finder -d 90 -t 30    # 60-120 minute races
zwift-race-finder -e tt           # Time trials only
```

### With Personal Config (Optional)
```bash
# Create config from template
cp config.example.toml ~/.config/zwift-race-finder/config.toml

# Edit with your Zwift Racing Score
editor ~/.config/zwift-race-finder/config.toml

# Tool will auto-load config from this location
zwift-race-finder
```

## Production Features

### Performance
- **API Calls**: Cached for 5 minutes to reduce load
- **Database**: SQLite with indexes for fast queries
- **Binary Size**: ~10MB standalone executable
- **Memory Usage**: <50MB typical
- **Response Time**: <2 seconds for most queries

### Reliability
- **Error Handling**: Graceful degradation on API failures
- **Offline Mode**: Works with cached data when API unavailable
- **Data Persistence**: All data stored in `~/.local/share/zwift-race-finder/`

### Accuracy (16.6% Mean Error on 125 races)
- **Flat Routes**: ~12% error
- **Rolling Routes**: ~16% error
- **Hilly Routes**: ~20% error
- **Multi-lap Races**: Fixed from 533% to ~16% error
- **Note**: Accuracy depends on having race results imported via Strava

## Monitoring & Maintenance

### Check Database Health
```bash
# Database location
sqlite3 ~/.local/share/zwift-race-finder/races.db "PRAGMA integrity_check;"

# Database size
du -h ~/.local/share/zwift-race-finder/races.db

# Route count
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT COUNT(*) FROM routes;"
```

### Update Route Data
```bash
# Apply latest route mappings
cd zwift-race-finder
git pull
./apply_route_mappings.sh

# Check for unknown routes
zwift-race-finder --show-unknown-routes
```

### Backup Data
```bash
# Backup database
cp ~/.local/share/zwift-race-finder/races.db ~/zwift-races-backup-$(date +%Y%m%d).db

# Backup config
cp ~/.config/zwift-race-finder/config.toml ~/zwift-config-backup-$(date +%Y%m%d).toml
```

## Troubleshooting

### No Events Found
1. Check API is accessible: `curl -s https://us-or-rly101.zwift.com/api/public/events | head`
2. Try wider tolerance: `-t 60` for ±60 minutes
3. Try all event types: `-e all`
4. Remember API limit: only ~12 hours of events available

### Wrong Predictions
1. Record actual times: `--record-result "route_id,minutes,event_name"`
2. Update rider stats: `./update_rider_stats.sh 86.0`
3. Check if multi-lap race in `multi_lap_events` table
4. Report via GitHub if consistently wrong

### Performance Issues
1. Check disk space: `df -h ~/.local/share/`
2. Vacuum database: `sqlite3 ~/.local/share/zwift-race-finder/races.db "VACUUM;"`
3. Clear old logs if any exist
4. Rebuild with latest version

## Integration

### Shell Aliases
Add to `~/.bashrc` or `~/.zshrc`:
```bash
# Quick race searches
alias zr='zwift-race-finder'
alias zr30='zwift-race-finder -d 30 -t 30'
alias zr60='zwift-race-finder -d 60 -t 30'
alias zr90='zwift-race-finder -d 90 -t 30'
alias zrtt='zwift-race-finder -e tt'
```

### Cron Jobs
Schedule regular updates:
```bash
# Update route mappings weekly
0 3 * * 0 cd ~/zwift-race-finder && git pull && ./apply_route_mappings.sh

# Backup database monthly
0 2 1 * * cp ~/.local/share/zwift-race-finder/races.db ~/backups/zwift-races-$(date +\%Y\%m).db
```

## Security Best Practices

1. **Never share** your config.toml if it contains personal data
2. **Use environment variables** for any API tokens:
   ```bash
   export ZWIFT_SCORE=195
   zwift-race-finder -s $ZWIFT_SCORE
   ```
3. **Regular updates**: `git pull` for security fixes
4. **Check permissions**: `ls -la ~/.config/zwift-race-finder/`

## Support

- **Issues**: https://github.com/jchidley/zwift-race-finder/issues
- **Feedback**: See FEEDBACK.md
- **Updates**: Watch the GitHub repo for new releases
<!-- ============================================================ -->
<!-- FILE: ./docs/howto/SECRETS_SETUP.md -->
<!-- ============================================================ -->

# Secrets Setup (direnv + ak)

This project expects a profile ID in the environment:

- `ZWIFTPOWER_PROFILE_ID`

## Prerequisites

1. `direnv` installed and hooked into your shell.
2. `ak` installed from `~/tools/api-keys` with GPG set up.

## One-Time Setup

1. Create `ak` service metadata for proper env var names:
   ```bash
   ak edit zwiftpower-profile-id
   ```
   Set `env_var` to `ZWIFTPOWER_PROFILE_ID`.

2. Store the profile ID:
   ```bash
   ak set zwiftpower-profile-id
   ```

3. Allow direnv in this repo:
   ```bash
   direnv allow
   ```

## Daily Usage

With direnv allowed, the environment variables are loaded automatically when you `cd` into the repo.

## Refresh ZwiftPower Stats (No Stored Session Cookie)

To avoid storing a session cookie, refresh stats through a browser session state file.

```bash
scripts/refresh_zwiftpower_stats.sh
# Or pass profile ID directly:
scripts/refresh_zwiftpower_stats.sh 1106548
```

This will:
- Open a browser to your ZwiftPower profile
- Let you log in once
- Save a Playwright storage state file under your cache directory
- Refresh `~/.cache/zwift-race-finder/user_stats.json` for 24 hours

Delete the storage state file anytime to require a fresh login.

## Bitwarden Migration (One-Off)

If you previously stored the profile ID in Bitwarden, convert once:
```bash
ak set zwiftpower-profile-id "$(bw get item 'Zwift Race Finder' | jq -r '.fields[] | select(.name=="zwiftpower_profile_id") | .value')"
```

<!-- ============================================================ -->
<!-- FILE: ./docs/howto/SECURE_TOKEN_MIGRATION.md -->
<!-- ============================================================ -->

# Secure Token Storage Migration Guide

## Overview

The Zwift Race Finder now supports secure storage for OAuth tokens, addressing the security concern of storing sensitive credentials in plain text files.

## Storage Options

### 1. Environment Variables (Recommended for CI/CD)
Most secure for automated environments and server deployments.

**Setup:**
```bash
# Add to your ~/.bashrc or shell profile
export STRAVA_CLIENT_ID="your_client_id"
export STRAVA_CLIENT_SECRET="your_client_secret"
export STRAVA_ACCESS_TOKEN="your_access_token"
export STRAVA_REFRESH_TOKEN="your_refresh_token"
export STRAVA_EXPIRES_AT="1234567890"
export STRAVA_ATHLETE_ID="your_athlete_id"
```

**Benefits:**
- No files on disk
- Easy to manage in CI/CD pipelines
- Can be injected by secret managers

### 2. System Keyring (Recommended for Desktop)
Uses your operating system's secure credential storage.

**Supported Systems:**
- Linux: GNOME Keyring, KWallet
- macOS: Keychain
- Windows: Credential Manager

**Benefits:**
- Encrypted at rest
- OS-level protection
- No plain text files

### 3. File Storage (Backward Compatible)
Original method, now with improved permissions.

**Location:** `strava_config.json` in project directory

**Security Improvements:**
- File permissions set to 600 (owner read/write only)
- Clear warnings about plain text storage
- Migration path to more secure options

## Migration Steps

### From Existing strava_config.json

1. **Check current setup:**
   ```bash
   ls -la strava_config.json
   ```

2. **Choose your storage method:**
   - For development machines: Use system keyring
   - For servers/CI: Use environment variables
   - For testing: Keep file storage

3. **Run secure authentication:**
   ```bash
   ./strava_auth_secure.sh
   ```
   This will:
   - Detect existing credentials
   - Offer migration options
   - Set up your chosen storage method

4. **Update your scripts:**
   ```bash
   # Replace old scripts with secure versions
   ./strava_fetch_activities_secure.sh  # Instead of strava_fetch_activities.sh
   ```

### For New Installations

1. **Run secure setup directly:**
   ```bash
   ./strava_auth_secure.sh
   ```

2. **Choose your preferred storage method**

3. **Follow the prompts to authenticate**

## Security Best Practices

### DO:
- Use environment variables for CI/CD
- Use system keyring for desktop development
- Rotate tokens regularly
- Keep client secret truly secret
- Use `.gitignore` to exclude token files

### DON'T:
- Commit tokens to git (even encrypted ones)
- Share tokens between environments
- Log token values
- Store tokens in world-readable locations
- Use the same tokens for production and development

## Troubleshooting

### "No keyring available"
- Install keyring support: `sudo apt install gnome-keyring` (Debian/Ubuntu)
- Or fall back to environment variables

### "Token refresh failed"
- Check if your app is still authorized in Strava settings
- Verify client ID and secret are correct
- Re-run authentication if needed

### "Environment variables not found"
- Ensure you've sourced your shell profile: `source ~/.bashrc`
- Check variable names are exactly as specified
- Use `printenv | grep STRAVA` to verify

## Future Enhancements

1. **Rust Integration**: The secure storage module is ready for integration into the main Rust application
2. **Token Rotation**: Automatic token refresh before expiration
3. **Multi-Account Support**: Store tokens for multiple Strava accounts
4. **Audit Logging**: Track token usage and access

## Questions?

If you encounter issues with token migration or storage, please:
1. Check this guide first
2. Review error messages carefully
3. Submit an issue with details (excluding any actual token values)
<!-- ============================================================ -->
<!-- FILE: ./docs/howto/TESTING_GUIDE.md -->
<!-- ============================================================ -->

# Testing Guide

How to run tests, add tests, and use mutation testing in this project.

## Running Tests

```bash
# All tests
cargo test

# Specific suites
cargo test --lib                    # Unit tests only
cargo test --test api_tests         # API tests (mocked)
cargo test --test integration_tests # CLI end-to-end
cargo test --test property_tests    # Property-based
cargo test --test snapshot_tests    # Output stability

# Regression accuracy (requires race_results in DB)
cargo test --lib test_race_predictions_accuracy -- --nocapture

# Benchmarks
cargo bench
```

## Adding Tests

### Unit Test for a Pure Function

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(90), "01:30");
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(61), "01:01");
    }
}
```

**Then immediately mutate**:
```bash
cargo mutants --file src/duration_estimation.rs --function format_duration --timeout 30
```

Fix any surviving mutations before moving on.

### Property Test for Invariants

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn duration_always_positive(distance in 1.0..200.0, elevation in 0.0..2000.0) {
        let duration = estimate_duration(distance, elevation);
        prop_assert!(duration > 0.0);
    }

    #[test]
    fn longer_distance_takes_longer(d1 in 10.0..100.0, d2 in 100.0..200.0) {
        let t1 = estimate_duration(d1, 0.0);
        let t2 = estimate_duration(d2, 0.0);
        prop_assert!(t2 > t1, "Longer route should take more time");
    }
}
```

### Integration Test for CLI

```rust
use assert_cmd::Command;

#[test]
fn test_help_shows_options() {
    Command::cargo_bin("zwift-race-finder")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("duration"));
}
```

## Mutation Testing

### Quick Check (Single Function)
```bash
cargo mutants --file src/estimation.rs --function estimate_duration --timeout 30
```

### Module Check (Before Commit)
```bash
cargo mutants --file src/duration_estimation.rs --timeout 180
```

### Full Codebase (Background)
```bash
nohup cargo mutants --jobs 8 --timeout 180 > mutation_$(date +%Y%m%d).log 2>&1 &
tail -f mutation_*.log | grep -E "MISSED|CAUGHT|tested"
```

### Interpreting Results

- **Killed**: Test caught the mutation ✅
- **Survived**: Tests didn't catch it — add/improve tests
- **Timeout**: Mutation caused infinite loop (often fine to ignore)

Target: >75% kill rate for core logic. Don't chase 100%.

### Code Movement Problem

If you refactored between starting mutation tests and analysing results, mutation reports reference old line numbers. Solution:

```bash
# Tag before mutation testing
git tag pre-mutation-$(date +%Y%m%d)
cargo mutants
# Later, compare: git diff pre-mutation-$(date +%Y%m%d)
```

## Golden Tests

Golden tests capture expected output for known inputs, preventing regressions.

### Generate Baseline
```bash
cargo test generate_golden_baseline_improved -- --ignored
```

### Test Data Selection
Focused on 11 representative routes × 11 distances × 12 scores = 1,452 cases (reduced from 9,414 by picking representatives of each terrain type and category boundary).

### No Database Dependency
Golden tests use `estimate_duration_for_category` (pure function), not the database. Tests are reproducible across environments.

### Tests That Need Database Access
For functions that require a real database, use isolated test databases:

```rust
use zwift_race_finder::test_db::TestDatabase;

#[test]
fn test_with_database() {
    let test_db = TestDatabase::new().unwrap();
    test_db.seed_test_routes().unwrap();
    // Run tests — database auto-deleted when test_db drops
}
```

## Test Data Validation

After changing routes in the database or adjusting the algorithm:

```bash
# Validate test routes against production DB
cargo test validate_test_routes -- --ignored --nocapture

# Validate against race history
cargo test validate_against_race_history -- --ignored --nocapture
```

Good results: <10% difference in mean/std dev between test routes and all routes.

## Tools

```bash
# Install
cargo install cargo-mutants    # Mutation testing
cargo install cargo-nextest    # Faster test runner
cargo install cargo-tarpaulin  # Coverage reports

# Usage
cargo mutants --file src/mod.rs --timeout 30
cargo nextest run
cargo tarpaulin --out Html --ignore-tests
```

## When to Write Tests

| Situation | Action |
|-----------|--------|
| Fixing a bug | Write regression test first |
| New calculation | Unit test + property test + mutate |
| New CLI flag | Integration test |
| Refactoring | Run existing tests, never modify them |
| Before release | Full mutation test run |

<!-- ============================================================ -->
<!-- FILE: ./docs/howto/ZWIFT_OFFLINE_INTEGRATION.md -->
<!-- ============================================================ -->

# Zwift-Offline Integration Guide

This document explains how to use the zwift-offline fork to import route data into zwift-race-finder while maintaining license compatibility.

## License Compatibility

- **zwift-race-finder**: MIT/Apache-2.0 (permissive)
- **zwift-offline**: AGPL v3 (copyleft)

The integration uses an **API boundary** to prevent license contamination. We never copy AGPL code into our MIT/Apache project. Instead, we make HTTP API calls to zwift-offline running as a separate service.

## Setup

### 1. Fork Setup (Already Complete)

You've already created and cloned the fork:
- Fork: https://github.com/jchidley/zwift-offline
- Local path: `zwift-offline/` (gitignored)

### 2. Enable Route Export in zwift-offline

The route export module has been added to your fork:
- `zwift_offline/route_export.py` - Export endpoints
- Automatically registered when zwift-offline starts

### 3. Setup Python Environment

```bash
cd zwift-offline
./setup_venv.sh  # One-time setup
```

### 4. Run zwift-offline

```bash
cd zwift-offline
./run_server.sh  # Uses uv to run in virtual environment
```

The server will start on:
- CDN server: `http://localhost:8080`
- API server: `https://localhost:8443` (with self-signed certificate)

No root privileges needed.

## Import Process

### Option A: Direct API Import

```bash
# Import directly from running zwift-offline server
./scripts/import_from_zwift_offline.sh --skip-ssl-verify

# This creates JSON files in data/zwift_offline_export/
```

### Option B: Import to Database

```bash
# First export from zwift-offline (if not done already)
./scripts/import_from_zwift_offline.sh --skip-ssl-verify

# Then import into SQLite database
cargo run --bin import_zwift_offline_routes -- \
    --input-dir data/zwift_offline_export \
    --database zwift_routes.db
```

### Option C: Dry Run First

```bash
# See what would be imported without making changes
cargo run --bin import_zwift_offline_routes -- \
    --input-dir data/zwift_offline_export \
    --database zwift_routes.db \
    --dry-run
```

## Available Endpoints

Your zwift-offline fork now provides:

- `/api/export/summary` - Overview of available data
- `/api/export/routes` - All routes with IDs, distances, worlds
- `/api/export/start_lines` - Start line positions and timing
- `/api/export/events` - Event definitions with route mappings

## Data Mapping

The integration maps zwift-offline data to zwift-race-finder's schema:

| zwift-offline | zwift-race-finder |
|---------------|-------------------|
| route (ID) | route_id |
| name | name |
| distance | distance_km |
| course | world_id (via mapping) |
| sport | sport (0=Cycling, 1=Running) |

## Current Limitations

The zwift-offline integration provides **55 event-only routes**. This is a subset of all routes because:
- zwift-offline filters routes with `eventOnly='1'` attribute
- Free-ride routes are not included in the export
- Complete route extraction requires WAD file decompression tools that are not currently available

For comprehensive route coverage, import routes from third-party sources (ZwiftHacks, WhatsOnZwift) or run the zwift-offline import script.

## Contributing Back

If the route export functionality proves useful, consider submitting a PR to the original zwift-offline project. The export module is self-contained and doesn't modify core functionality.

## Troubleshooting

### SSL Certificate Errors

zwift-offline uses self-signed certificates. Use `--skip-ssl-verify` flag when importing.

### Missing Data Files

Ensure zwift-offline has the latest game data:
```bash
cd zwift-offline
python get_events.py
python get_start_lines.py
```

### Route ID Mismatches

zwift-offline uses game-internal route IDs which may differ from ZwiftPower. The integration preserves zwift-offline's IDs as authoritative.
<!-- ============================================================ -->
<!-- FILE: ./docs/howto/ZWIFTPOWER_EXPORT_STEPS.md -->
<!-- ============================================================ -->

# ZwiftPower Export Steps

## Quick Commands (run from anywhere)

1. **Copy JavaScript to clipboard** (requires xclip):
   ```bash
   cat ~/tools/rust/zwift-race-finder/extract_zwiftpower.js | xclip -selection clipboard
   ```

2. **After running JavaScript in browser, copy the downloaded file**:
   ```bash
   ~/tools/rust/zwift-race-finder/copy_results.sh
   ```

3. **Import the results to database**:
   ```bash
   ~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import
   ```

## Full Process

1. Log into ZwiftPower: https://zwiftpower.com/profile.php?z=YOUR_PROFILE_ID
2. Open Developer Tools (F12) → Console tab
3. Copy and paste the JavaScript from `~/tools/rust/zwift-race-finder/extract_zwiftpower.js`
4. File downloads to `/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json`
5. Run the copy script: `~/tools/rust/zwift-race-finder/copy_results.sh`
6. Import to database: `~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import`

## What This Does

- Extracts all your race results from ZwiftPower
- Saves them to the SQLite database
- Allows the race finder to use your actual race times for accurate predictions
- Enables regression testing based on your real performance data
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/ALGORITHMS.md -->
<!-- ============================================================ -->

# Duration Estimation Algorithms

## Overview

Estimates how long a Zwift race will take for a specific rider based on route characteristics and Racing Score category.

## Production Algorithm: Category Speed Model

```
duration_minutes = (distance_km / (category_speed × difficulty_multiplier)) × 60
```

### Category Speeds (km/h)

Empirical averages from real races. Already include draft benefit.

| Category | Score Range | Speed (km/h) | Source |
|----------|-------------|--------------|--------|
| E | 0–99 | 28.0 | Estimated ~10% slower than Cat D |
| D | 100–199 | 30.9 | Calibrated from 151 real races |
| C | 200–299 | 33.0 | Scaled from Cat D |
| B | 300–399 | 37.0 | Scaled from Cat D |
| A | 400–599 | 42.0 | Scaled from Cat D |
| A++ | 600+ | 45.0 | Estimated ~7% faster than Cat A |

Score 600+ maps to "A++" (45.0 km/h). The detailed category function splits this further: 590–649 = "A+", 650+ = "A++".

### Route Difficulty Multiplier

Multiplied with category speed. Values >1.0 mean faster (flat), <1.0 mean slower (hilly).

**Elevation-based** (`get_route_difficulty_multiplier_from_elevation_and_category`):

Uses piecewise linear interpolation across 9 breakpoints for smooth transitions, with a category-aware penalty on steep climbs.

| Elevation/km | Base Multiplier | Terrain Example |
|-------------|-----------------|-----------------|
| 0 m/km | 1.10 | Tempus Fugit (~1 m/km) |
| 5 m/km | 1.05 | Most Watopia routes |
| 10 m/km | 1.00 | Rolling (transition zone) |
| 15 m/km | 0.93 | Rolling hills |
| 20 m/km | 0.85 | Mountain 8 (~21 m/km) |
| 30 m/km | 0.70 | Very hilly |
| 40 m/km | 0.55 | Mountain climbs |
| 60 m/km | 0.45 | Road to Sky (~60 m/km) |
| 100 m/km | 0.30 | Theoretical maximum |

Values between breakpoints are linearly interpolated.

**Category climbing penalty** (applied above 15 m/km):

On flat terrain, all categories ride near their empirical category speed (ratio ≈ 1.0). On steep climbs, lower categories are disproportionately slower because w/kg matters more than raw watts. This was measured from race data: Cat D achieves 48% of its flat speed on >20 m/km climbs, while Cat C achieves 54%.

| Category | Climbing Factor | Effect |
|----------|----------------|--------|
| A / A++ | 1.00 | No penalty |
| B | 0.97 | Minor penalty |
| C | 0.93 | Moderate penalty |
| D | 0.85 | Significant penalty |
| E | 0.75 | Large penalty |

The final multiplier on a climb is `base_multiplier × category_factor`. For example, Road to Sky (60 m/km) for Cat D: `0.45 × 0.85 = 0.38`, giving an effective speed of `30.9 × 0.38 = 11.7 km/h` — close to the observed 9.6 km/h from actual race data.

**Name-based fallback** (`get_route_difficulty_multiplier`) — used only when no elevation data is available:

| Route name contains | Multiplier |
|---------------------|------------|
| "alpe", "ventoux" | 0.7 |
| "epic", "mountain" | 0.8 |
| "flat", "tempus" | 1.1 |
| (anything else) | 1.0 |

### Which Multiplier Is Used Where

| Function | Multiplier Used | When Called |
|----------|----------------|------------|
| `estimate_duration_from_route_id` | Elevation + category | Route known, no distance override |
| `estimate_duration_with_distance` | Elevation + category | Route known, distance provided (multi-lap) |
| `estimate_duration_for_category` | Name-based only | No route lookup possible (fallback) |
| `event_display` (verbose) | Elevation + category | Displaying detailed predictions |

### Estimation Priority

`event_filtering.rs` tries these methods in order, returning on first match:

1. **Fixed duration**: If event has `duration_in_minutes` or `duration_in_seconds`, use directly
2. **Multi-lap with known route**: Subgroup has lap count → `lead_in + (route_distance × laps)` → estimate
3. **Known route + known distance**: Route_id resolves, event/subgroup has distance → `estimate_duration_with_distance`
4. **Known distance + unknown route**: Distance available but route unknown → `estimate_duration_for_category` (name-based multiplier)
5. **Known route only**: Route_id resolves, no distance → `estimate_duration_from_route_id` (adds lead-in) + multi-lap DB check
6. **Racing Score + route**: Route_id known, parse distance from description → estimate
7. **Fallback: event distance**: `distance_in_meters > 0` → `estimate_duration_for_category`
8. **Fallback: Racing Score description**: Parse distance from description text
9. **Fallback: name guess**: `estimate_distance_from_name` heuristic

### Lead-in Distance Handling

- **Multi-lap (step 2)**: Lead-in added by caller in `event_filtering.rs`
- **Route-only (step 5)**: Lead-in added inside `estimate_duration_from_route_id` in `estimation.rs`
- **All other paths**: Lead-in is **not** added — potential accuracy gap

### Route ID Alias Resolution

Zwift uses different internal route IDs for the same physical route depending on context (event-only vs free-ride). The `route_aliases` table maps these alternative IDs to canonical DB route IDs. `Database::get_route()` checks aliases transparently: direct lookup → alias lookup → None. See [Database Reference](DATABASE.md).

## Calibration

- **Current MAE**: 16.6% on 125 matched races
- **Target**: < 20% MAE
- **Threshold**: Regression test asserts < 30% MAE
- Run: `cargo test --lib test_race_predictions_accuracy -- --nocapture`

### Error by Terrain Type

| Terrain | MAE | n | Notes |
|---------|-----|---|-------|
| Flat (<10 m/km) | ~19% | 104 | Bias ≈ 0%, model well-calibrated |
| Hilly (10–20 m/km) | ~10% | 16 | Best accuracy segment |
| Climbing (>20 m/km) | <20% | 5 | Fixed from 43.8% by category-aware multiplier |

### Key Findings

1. Pure physics models fail (127% error)
2. Draft benefit crucial — already baked into category speeds
3. High variance is inherent to racing (same route: 32–86 min depending on pack dynamics)
4. Binary state: with pack or dropped, no gradual separation
5. Lower categories are disproportionately slower on climbs (w/kg effect)
6. Weight/height are **not** direct model inputs — the effect is captured via the category × elevation interaction

## Special Cases

| Case | Handling |
|------|----------|
| Fixed duration events | Duration used directly from event data (no estimation) |
| Time trials | No specific handling — uses same category speeds (known inaccuracy) |
| Multi-lap | Route distance × laps + lead-in |
| Racing Score events | `distanceInMeters: 0` in API; parse from description |
| Unknown routes | Name-based difficulty multiplier fallback |
| Category E | Separate category at 28 km/h |

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/ARCHITECTURE.md -->
<!-- ============================================================ -->

# Zwift Race Finder Architecture

## System Architecture

```
Zwift API → Filter Events → Estimate Duration → Display Results
     ↓                            ↑
ZwiftPower → Import → SQLite → Route Data
```

## Core Components

### 1. CLI and Orchestration (`main.rs`)
- Clap-based argument parsing (20+ flags)
- Fetches events from Zwift public API (`us-or-rly101.zwift.com`)
- Optionally fetches Racing Score from ZwiftPower (environment variables: `ZWIFTPOWER_PROFILE_ID`, `ZWIFTPOWER_SESSION_ID`)
- Dispatches to filtering, display, route discovery, or progress tracking

### 2. Duration Estimation (`duration_estimation.rs`, `estimation.rs`)
- `duration_estimation.rs`: Pure functions — category speed lookup, difficulty multipliers (piecewise linear with category-aware climbing penalty), duration math
- `estimation.rs`: Bridge — route lookup from DB (with alias resolution), lead-in addition, connects to `duration_estimation`
- Category speed and elevation are the rider/route inputs. Weight/FTP are stored but **not used** directly — the weight effect is captured through category × elevation interaction.

### 3. Event Filtering (`event_filtering.rs`)
- 9-step estimation priority (see [Algorithms](ALGORITHMS.md))
- Handles both Traditional (A/B/C/D) and Racing Score (0–650) events
- Racing Score events have `distanceInMeters: 0` — distance parsed from description text
- Tag-based filtering (`--tags`, `--exclude-tags`)

### 4. Event Display (`event_display.rs`)
- Compact table (default) or verbose multi-line format
- Colored output with time-until-start, distance, estimated duration
- Route completion indicators

### 5. Database (`database.rs`)
- SQLite at `~/.local/share/zwift-race-finder/races.db`
- Created automatically on first run
- 6 tables:

| Table | Purpose |
|-------|---------|
| `routes` | Route data (distance, elevation, lead-in, surface, slug) |
| `race_results` | Actual race times for regression testing |
| `unknown_routes` | Routes seen in events but not yet mapped |
| `route_aliases` | Maps event-only route IDs to canonical DB route IDs |
| `route_completion` | User's route completion tracking |
| `rider_stats` | Height, weight, FTP (stored but not used in estimation) |
| `route_discovery_attempts` | Tracks web search attempts to avoid repeats |

### 6. Config (`config.rs`)
- Config loading priority: `./config.toml` → `~/.config/zwift-race-finder/config.toml` → `~/.local/share/zwift-race-finder/config.toml` → defaults
- Secrets from environment variables only (no file storage for credentials)
- Defaults: score=195, category=D, duration=120min, tolerance=30min

### 7. Route Discovery (`route_discovery.rs`)
- Searches whatsonzwift.com for unknown routes
- Caches results in `route_discovery_attempts` table
- Rate-limited (500ms between requests)

## Binaries

| Binary | Purpose | Feature gate |
|--------|---------|-------------|
| `zwift-race-finder` | Main CLI | — |
| `analyze_descriptions` | Fetch events and extract description patterns | — |
| `debug_tags` | Analyze event tags from saved JSON | — |
| `import_zwift_offline_routes` | Import routes from zwift-offline fork | — |
| `test_ocr` | OCR testing | `ocr` |
| `debug_ocr` | OCR debugging | `ocr` |
| `zwift_ocr_benchmark` | OCR benchmarking | `ocr` |
| `zwift_ocr_compact` | Compact OCR extraction | `ocr` |

## API Integration

### Zwift Public API
- URL: `https://us-or-rly101.zwift.com/api/public/events/upcoming`
- No authentication required
- Maximum 200 events returned (~12 hours of data)

### ZwiftPower
- Profile scraping via environment variable credentials
- Provides Racing Score auto-detection
- Script: `scripts/refresh_zwiftpower_stats.sh`

## Route ID System

- Route IDs are unsigned 32-bit integers (`u32` in Rust, `INTEGER` in SQLite)
- Values can exceed `i32` max (2^31) — appear negative when serialized as signed integers
- Import code uses `i64` for deserialization, then casts to `u32`
- Route names change; route IDs are stable. Always use IDs as primary key.

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/DATABASE.md -->
<!-- ============================================================ -->

# Database Reference

## Location

`~/.local/share/zwift-race-finder/races.db` (SQLite, created automatically on first run)

## Schema

### routes
```sql
CREATE TABLE IF NOT EXISTS routes (
    route_id INTEGER PRIMARY KEY,
    distance_km REAL NOT NULL,
    elevation_m INTEGER NOT NULL,
    name TEXT NOT NULL,
    world TEXT NOT NULL,
    surface TEXT NOT NULL DEFAULT 'road',
    lead_in_distance_km REAL DEFAULT 0.0,
    lead_in_elevation_m INTEGER DEFAULT 0,
    lead_in_distance_free_ride_km REAL,
    lead_in_elevation_free_ride_m INTEGER,
    lead_in_distance_meetups_km REAL,
    lead_in_elevation_meetups_m INTEGER,
    slug TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### race_results
```sql
CREATE TABLE IF NOT EXISTS race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id INTEGER NOT NULL,
    event_name TEXT NOT NULL,
    actual_minutes INTEGER NOT NULL,
    zwift_score INTEGER NOT NULL,
    race_date TIMESTAMP NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);
```

### unknown_routes
```sql
CREATE TABLE IF NOT EXISTS unknown_routes (
    route_id INTEGER PRIMARY KEY,
    event_name TEXT NOT NULL,
    event_type TEXT,
    first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    times_seen INTEGER DEFAULT 1
);
```

### route_completion
```sql
CREATE TABLE IF NOT EXISTS route_completion (
    route_id INTEGER PRIMARY KEY,
    completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    actual_time_minutes INTEGER,
    notes TEXT,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);
```

### rider_stats
```sql
CREATE TABLE IF NOT EXISTS rider_stats (
    id INTEGER PRIMARY KEY,
    height_m REAL DEFAULT 1.82,
    weight_kg REAL,
    ftp_watts INTEGER,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

Note: `rider_stats` data is stored but **not used** in the duration estimation algorithm.

### route_aliases
```sql
CREATE TABLE IF NOT EXISTS route_aliases (
    alias_route_id INTEGER PRIMARY KEY,
    canonical_route_id INTEGER NOT NULL,
    notes TEXT,
    FOREIGN KEY (canonical_route_id) REFERENCES routes(route_id)
);
```

Maps alternative Zwift API route IDs (event-only variants) to canonical route IDs in the `routes` table. Zwift uses different internal IDs for the same physical route depending on whether it's a free-ride or event-only context. `Database::get_route()` checks aliases transparently on miss.

Populated by `sql/mappings/route_aliases.sql`. Currently 11 aliases covering ~2,640 previously-unresolvable event sightings.

### route_discovery_attempts
```sql
CREATE TABLE IF NOT EXISTS route_discovery_attempts (
    route_id INTEGER PRIMARY KEY,
    event_name TEXT NOT NULL,
    last_attempt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    found BOOLEAN DEFAULT 0,
    distance_km REAL,
    elevation_m INTEGER,
    world TEXT,
    surface TEXT,
    notes TEXT
);
```

## Route ID Notes

- Route IDs are `u32` in Rust (0 to 4,294,967,295)
- SQLite stores them as `INTEGER` (signed 64-bit)
- Zwift API may serialize large IDs as negative numbers (e.g., -2129086892 = 2165880404 as u32)
- Import code deserializes as `i64`, then casts to `u32`
- `--record-result` requires the route to exist in `routes` table first (FK constraint)

## Common Queries

```sql
-- Most-raced routes
SELECT r.name, r.world, COUNT(rr.id) as race_count
FROM routes r
JOIN race_results rr ON r.route_id = rr.route_id
GROUP BY r.route_id
ORDER BY race_count DESC;

-- Unknown routes by frequency
SELECT event_name, route_id, times_seen
FROM unknown_routes
ORDER BY times_seen DESC
LIMIT 20;

-- Route count
SELECT COUNT(*) FROM routes;
```

## Backup

```bash
cp ~/.local/share/zwift-race-finder/races.db \
   ~/.local/share/zwift-race-finder/races_$(date +%Y%m%d).db
```

## Reset

```bash
rm ~/.local/share/zwift-race-finder/races.db
# Next run recreates the database with empty tables
```

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/INTEGRATION_TEST_COVERAGE.md -->
<!-- ============================================================ -->

# Integration Test Coverage Analysis

**Date**: 2025-06-08  
**Purpose**: Document current integration test coverage and identify gaps

## Current Integration Test Coverage

### 1. CLI Integration Tests (`tests/integration_tests.rs`)

#### ✅ Command Line Interface Testing
- **Help command**: Validates help text and available options
- **Invalid inputs**: Tests negative duration, invalid event types
- **Database commands**: Tests `--show-unknown-routes` functionality
- **New routes flag**: Tests `--new-routes-only` behavior
- **Multiple tags**: Tests tag parsing with `--tags` and `--exclude-tags`
- **Record result**: Validates format requirements
- **Verbose mode**: Tests different output formats

#### ✅ Output Format Testing
- **Table format**: Default output validation
- **Multi-lap detection**: Tests lap counting from event names
- **Distance calculation**: Verifies multi-lap distance multiplication

#### ✅ Route Regression Testing
- **Known routes**: Validates 6 specific routes exist in database
- **Duration bounds**: Tests estimates are within reasonable ranges for Cat D
- **Framework ready**: Structure in place for Jack's actual race results

### 2. API Integration Tests (`tests/api_tests.rs`)

#### ✅ API Response Handling (with Mocks)
- **Success case**: Valid JSON response parsing
- **404 errors**: Proper error handling
- **Malformed JSON**: Graceful failure on invalid data
- **Empty response**: Handles empty event lists
- **Racing Score events**: Special handling for 0-distance events
- **Tag filtering**: Tests event filtering by tags

### 3. Regression Tests (`src/regression_test.rs`)

#### ✅ Prediction Accuracy Testing
- **Database integration**: Reads actual race results
- **Multi-lap handling**: Caches lap info for performance
- **Error analysis**: Tracks errors by route
- **Mean accuracy**: Calculates overall prediction accuracy
- **Large error reporting**: Highlights predictions > 20% error

### 4. Snapshot Tests (`tests/snapshot_tests.rs`)

#### ✅ Behavioral Snapshots
- **Flat routes**: Bell Lap, Downtown Dolphin, Three Village Loop
- **Hilly routes**: Castle to Castle, Eracing Course, Hilltop Hustle
- **Mountain routes**: Road to Sky, Mt. Fuji, Mountain Mash
- **Duration calculations**: Tests all category speeds
- **Elevation factors**: Verifies elevation impact on duration

## Integration Test Gaps

### 🔴 Missing End-to-End Tests

1. **Real API Integration**
   - No tests against actual Zwift API (requires credentials)
   - OAuth token refresh flow not tested
   - Rate limiting behavior not verified
   - Network timeout handling not tested

2. **Full Workflow Tests**
   - API → Filter → Display pipeline not tested end-to-end
   - Config file integration not tested
   - Cache behavior not verified in integration tests

3. **External Tool Integration**
   - Strava import workflow not tested
   - ZwiftPower data extraction not tested
   - Database migration scripts not tested

### 🟡 Partial Coverage

1. **Error Scenarios**
   - Database corruption recovery
   - Concurrent access to database
   - Disk full conditions
   - Invalid config file handling

2. **Performance Tests**
   - Large event list processing
   - Database query performance
   - Memory usage under load

3. **Cross-Platform Testing**
   - Windows path handling
   - macOS specific behaviors
   - Different terminal environments

## Recommendations for UOM Migration

### Critical Integration Tests to Add

1. **UOM A/B Integration Tests**
   ```rust
   #[test]
   fn test_end_to_end_filtering_ab() {
       // Run full filtering pipeline with both implementations
       // Compare final event lists
   }
   ```

2. **Database Round-Trip Tests**
   ```rust
   #[test]
   fn test_uom_database_compatibility() {
       // Store UOM values in database
       // Read back and verify consistency
   }
   ```

3. **CLI Output Compatibility**
   ```rust
   #[test]
   fn test_cli_output_unchanged() {
       // Run CLI with both implementations
       // Verify output is character-for-character identical
   }
   ```

### Integration Test Strategy for UOM

1. **Use Golden Files**: Capture current CLI output for various commands
2. **Mock Time**: Use fixed timestamps for reproducible tests
3. **Test Data**: Create minimal test database with known routes
4. **CI Integration**: Run integration tests on every commit

## Summary

The project has good integration test coverage for:
- CLI functionality and parsing
- API response handling (with mocks)
- Regression testing against historical data
- Output format verification

Key gaps for UOM migration:
- End-to-end workflow tests
- Real API integration (would need test credentials)
- Performance and stress testing
- Cross-platform compatibility

The existing integration tests provide a solid foundation for ensuring the UOM migration doesn't break user-facing functionality.
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/PHYSICAL_STATS.md -->
<!-- ============================================================ -->

# Physical Stats and Their Impact on Zwift Performance

## Why Physical Attributes Matter

### Weight (kg)
- **Power-to-weight ratio (w/kg)** is the primary determinant of climbing speed
- Heavier riders have an advantage on flats due to higher absolute power
- Lighter riders excel on climbs where w/kg matters most
- Affects drafting dynamics - heavier riders provide better draft

### Height (meters)
- **Aerodynamic drag** increases with frontal area (taller = more drag)
- **Drafting effectiveness** - taller riders get less benefit from the draft
- **CdA (drag coefficient × area)** calculations in Zwift physics engine
- Affects speed on flats and descents more than climbs

## Jack's Stats
- Height: 1.82m (6'0")
- Weight: ~86kg (from race data)
- Zwift Racing Score: 195 (Cat D)

## Performance Implications

### On Flats
- Taller riders (like Jack at 1.82m) face more air resistance
- Need higher absolute power to maintain same speed as shorter riders
- Benefit less from pack dynamics

### On Climbs
- Weight becomes the dominant factor
- At 86kg, need strong w/kg to compete on climbs
- Height penalty is minimal on steep gradients

### Optimal Race Types
Given Jack's profile (1.82m, 86kg):
- Better suited to rolling/flat courses than pure climbing races
- Can leverage higher absolute power on flats
- Should focus on races with good draft opportunities
- Time trials may be challenging due to height/drag penalty

## Future Enhancements
- Use height/weight for more accurate speed predictions
- Adjust draft benefit calculations based on height
- Factor in CdA estimates for different positions
- Consider bike choice effects (TT vs road bike aerodynamics)
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/REFACTORING_RULES.md -->
<!-- ============================================================ -->

# Refactoring Rules for Claude

<critical_contract>
WHEN YOU SEE THE WORD "REFACTOR" YOU ARE ENTERING A BINDING CONTRACT:
- You will preserve behavior EXACTLY
- You will follow the specific mechanics for each refactoring type
- You will NOT add features or "improvements"
- You will REVERT if tests fail
- Breaking this contract is a CRITICAL FAILURE
</critical_contract>

## Core Definition

**Martin Fowler (refactoring.com):**
"A disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior"

**Key Principles:**
- Small behavior-preserving transformations
- System fully working after each change
- Tests are the specification

## Catalog of Safe Refactoring Mechanics

### 1. Move Function (Between Files)

<mechanics>
MECHANICAL COPY-DELETE METHOD:

Step 1: Copy ENTIRE source file
```bash
cp src/main.rs src/parsing.rs
```

Step 2: DELETE everything except functions to move
- Use ONLY delete key
- Keep function EXACTLY as is

Step 3: DELETE moved functions from original
- Add module declaration
- Add imports

Step 4: Run tests - must pass unchanged
</mechanics>

### 2. Extract Function

<mechanics>
Step 1: Identify code fragment to extract
Step 2: Create new function with descriptive name
Step 3: COPY (not rewrite) the code fragment
Step 4: Replace original with function call
Step 5: Pass needed parameters
Step 6: Return needed values
Step 7: Run tests - must pass unchanged
</mechanics>

### 3. Rename Function/Variable

<mechanics>
Step 1: Change declaration
Step 2: Find ALL references (use IDE/grep)
Step 3: Update each reference MECHANICALLY
Step 4: Run tests - must pass unchanged

NEVER:
- Change logic while renaming
- "Fix" parameter order
- Add/remove parameters
</mechanics>

### 4. Extract Variable

<mechanics>
Step 1: Identify expression to extract
Step 2: Create variable with descriptive name
Step 3: Assign expression to variable
Step 4: Replace ALL occurrences with variable
Step 5: Run tests - must pass unchanged

Example:
```rust
// Before
if (order.quantity * order.item_price > 1000) { ... }

// After
let base_price = order.quantity * order.item_price;
if (base_price > 1000) { ... }
```
</mechanics>

### 5. Inline Function/Variable

<mechanics>
Step 1: Find all callers/uses
Step 2: Replace each call with body/value
Step 3: Remove the function/variable
Step 4: Run tests - must pass unchanged

CAUTION: Easy to change behavior accidentally
</mechanics>

### 6. Change Function Declaration

<mechanics>
MIGRATION METHOD (safer):

Step 1: Create new function with desired signature
Step 2: COPY old function body
Step 3: Update old function to call new
Step 4: Find and update each caller
Step 5: Remove old function
Step 6: Run tests after EACH step
</mechanics>

## Universal STOP Signals

<stop_signals>
If you think ANY of these, STOP IMMEDIATELY:

❌ "This would be better if..."
❌ "While I'm here..."
❌ "This parameter isn't used..."
❌ "Modern style prefers..."
❌ "This could be more efficient..."
❌ "The test is wrong..."
❌ "This catches more edge cases..."
❌ "This comment is outdated..."
❌ "This is redundant..."

THESE THOUGHTS MEAN YOU'RE REWRITING, NOT REFACTORING
</stop_signals>

## Validation for ALL Refactorings

<validation>
□ All tests pass WITHOUT modification
□ No test files changed
□ Behavior identical (not just "equivalent")
□ No new features
□ No bug fixes
□ No optimizations
□ No style updates
</validation>

## Complex Refactorings to AVOID

<danger_zone>
These require extreme care - consider refusing:

- Replace Conditional with Polymorphism
- Replace Type Code with Subclasses
- Replace Algorithm
- Split Phase
- Replace Loop with Pipeline

Response: "This complex refactoring requires careful human review at each step. I can provide the mechanics but should not execute automatically."
</danger_zone>

## Examples of Behavior Change (FAILURES)

<failures>
1. ADDING validation:
```rust
// Before
fn set_age(age: i32) { self.age = age; }

// WRONG refactor
fn set_age(age: i32) { 
    if age >= 0 { self.age = age; }  // Added validation!
}
```

2. FIXING edge cases:
```rust
// Before  
fn parse(s: &str) -> i32 { s.parse().unwrap() }

// WRONG refactor
fn parse(s: &str) -> i32 { 
    s.parse().unwrap_or(0)  // "Fixed" panic!
}
```

3. IMPROVING efficiency:
```rust
// Before
items.iter().filter(|x| x.active).collect::<Vec<_>>().len()

// WRONG refactor  
items.iter().filter(|x| x.active).count()  // "More efficient!"
```
</failures>

## Recovery Protocol

<recovery>
WHEN TESTS FAIL:
1. DO NOT debug the code
2. DO NOT modify tests
3. DO NOT try to fix
4. IMMEDIATELY revert all changes
5. Report: "Refactoring failed - behavior changed"
</recovery>

## The Refactoring Decision Tree

<decision_tree>
1. Is it a simple mechanical refactoring? → Use specific mechanics
2. Does it require creating new abstractions? → Proceed with extreme care
3. Does it touch core algorithms? → Consider refusing
4. Are there no tests? → REFUSE: "Cannot refactor without tests"
5. Do tests use mocks? → WARN: "Mock-based tests may hide behavior changes"
</decision_tree>

## Required Response Format

<response_template>
I will perform a [refactoring type] refactoring.

Refactoring Plan:
- Type: [Extract Function/Rename/Move Function/etc.]
- Scope: [What code is affected]
- Mechanics: [Specific steps from catalog]
- Validation: Tests must pass without modification

I understand that ANY behavior change means failure.
</response_template>

## The Golden Rule

**Tests failing = You failed, not the tests**

Tests are the specification. If they fail after refactoring, you changed behavior. The ONLY correct response is to revert and try again.

## Final Wisdom

Martin Fowler: "When you find you have to add a feature to a program, and the program's code is not structured in a convenient way to add the feature, first refactor the program to make it easy to add the feature, then add the feature."

The key: Refactoring is SEPARATE from feature addition. Never mix them.
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/REQUIREMENTS.md -->
<!-- ============================================================ -->

# REQUIREMENTS.md

This document specifies the functional and non-functional requirements for the Zwift Race Finder tool.

## PRIORITY UPDATE (2025-05-27)

**User Concern**: "I'm not convinced that the program is working as I'd like" - This is the highest priority issue to investigate and resolve.

### Additional Recent Priorities (from file review)
1. **Security**: ✅ OAuth token storage in plain text files (HIGH - from SECURITY_AUDIT.md) - COMPLETED 2025-05-27
   - Implemented secure storage module with environment variables, system keyring, and file options
   - Created migration scripts and documentation
   - Maintained backward compatibility
2. **Personal Data**: Multiple files contain hardcoded personal IDs that need sanitization
3. **Configuration Management**: Need seamless personal config that survives updates
4. **Physics Modeling**: Height/weight stats affect predictions but aren't fully utilized
5. **API Limitations**: 200 event hard limit requires user education and workarounds

## Project Overview

The Zwift Race Finder is a command-line tool that helps cyclists find Zwift races matching their target duration and fitness level. It predicts race completion times based on the rider's Zwift Racing Score and route characteristics, achieving 16.6% prediction accuracy on 125 matched races.

## Core Problem Statement

Zwift shows race distances but not expected durations. A 40km race might take 60 or 90 minutes depending on route profile and rider fitness. This tool solves that by predicting actual race duration for specific riders.

## Functional Requirements

### 1. Event Filtering and Discovery

#### 1.1 Event Fetching
- **FR-1.1.1**: Fetch upcoming events from Zwift Public API
- **FR-1.1.2**: Handle API limitation of 200 events maximum (~12 hours of data)
- **FR-1.1.3**: Display actual time range covered when multi-day searches exceed API limits
- **FR-1.1.4**: Notify users if API returns >250 events (future-proofing)

#### 1.2 Event Type Support
- **FR-1.2.1**: Support Traditional Category events (A/B/C/D/E) with populated distance
- **FR-1.2.2**: Support Racing Score events (0-650) with distance in description text
- **FR-1.2.3**: Filter by event type: race, fondo, group ride, workout, time trial
- **FR-1.2.4**: Default to showing only races unless specified otherwise

#### 1.3 Event Filtering
- **FR-1.3.1**: Filter events by estimated duration within tolerance range
- **FR-1.3.2**: Exclude non-cycling events (running)
- **FR-1.3.3**: Show event counts by type after fetching
- **FR-1.3.4**: Provide context-aware suggestions when no results found

### 2. Duration Prediction

#### 2.1 Route-Based Estimation
- **FR-2.1.1**: Use route_id to lookup known route data (distance, elevation, surface)
- **FR-2.1.2**: Handle multi-lap races using event_sub_groups for category-specific distances
- **FR-2.1.3**: Parse distance from event descriptions for Racing Score events
- **FR-2.1.4**: Apply elevation-based difficulty multipliers
- **FR-2.1.5**: Apply surface-type penalties (gravel, mixed surfaces)
- **FR-2.1.6**: Account for lead-in distance variations by event type (race, workout, group ride)
- **FR-2.1.7**: Consider route-specific physics (e.g., jungle has different draft dynamics)

#### 2.2 Speed Calculation
- **FR-2.2.1**: Use category-based average speeds:
  - Cat E (0-99): 28.0 km/h
  - Cat D (100-199): 30.9 km/h
  - Cat C (200-299): 33 km/h
  - Cat B (300-399): 37 km/h
  - Cat A (400-599): 42 km/h
  - Cat A+ (600+): 42.0 km/h
- **FR-2.2.2**: Support dual-speed model with pack dynamics (optional):
  - Pack speed: Category-based
  - Solo speed: 77% of pack speed
  - Drop probability based on elevation and rider weight

#### 2.3 Accuracy Targets
- **FR-2.3.1**: Maintain <20% mean absolute error on predictions
- **FR-2.3.2**: Track prediction accuracy using regression tests
- **FR-2.3.3**: Support calibration with actual race results

### 3. Data Management

#### 3.1 Database Operations
- **FR-3.1.1**: Store route data in SQLite database
- **FR-3.1.2**: Track unknown routes for future mapping
- **FR-3.1.3**: Store race results for accuracy improvement
- **FR-3.1.4**: Support rider stats (weight, FTP) for personalized predictions

#### 3.2 Route Discovery
- **FR-3.2.1**: Log unknown routes during event processing
- **FR-3.2.2**: Support manual route mapping via SQL scripts
- **FR-3.2.3**: Web scraping for route data from whatsonzwift.com
- **FR-3.2.4**: Parse route information from event descriptions
- **FR-3.2.5**: Extract and utilize hidden event tags for advanced filtering
- **FR-3.2.6**: Support route slug mapping for external URL generation
- **FR-3.2.7**: Track event-only routes that aren't available in free ride

#### 3.3 Data Import
- **FR-3.3.1**: Import race history from ZwiftPower via browser extraction
- **FR-3.3.2**: Import actual race times from Strava API
- **FR-3.3.3**: Apply route mappings to imported data
- **FR-3.3.4**: Handle OAuth authentication for Strava

### 4. User Interface

#### 4.1 Command Line Interface
- **FR-4.1.1**: Accept Zwift Racing Score as parameter
- **FR-4.1.2**: Accept target duration and tolerance
- **FR-4.1.3**: Support debug mode showing filtering details
- **FR-4.1.4**: Show unknown routes that need mapping
- **FR-4.1.5**: Record race results for calibration
- **FR-4.1.6**: Support URL-based filter parameters for sharing searches
- **FR-4.1.7**: Filter events by tags (e.g., --tags ranked,zracing)
- **FR-4.1.8**: Show route completion status when tracking enabled

#### 4.2 Output Format
- **FR-4.2.1**: Display events with estimated duration for rider's score
- **FR-4.2.2**: Show time until event starts
- **FR-4.2.3**: Use colored output for better readability
- **FR-4.2.4**: Include route details when available
- **FR-4.2.5**: Default to compact table format with columns: Event Name, Time, Distance (total with lead-in), Duration, Route Info (✓ if known)
- **FR-4.2.6**: Show only the user's selected category in output (not all categories)
- **FR-4.2.7**: Support --verbose flag for detailed multi-line output format

#### 4.3 User Guidance
- **FR-4.3.1**: Show event type summary after fetching
- **FR-4.3.2**: Provide working command examples when no results
- **FR-4.3.3**: Explain typical event durations by type
- **FR-4.3.4**: Suggest appropriate search parameters

## Non-Functional Requirements

### 5. Performance

- **NFR-5.1**: Process 200 events in under 2 seconds
- **NFR-5.2**: Database queries complete in under 100ms
- **NFR-5.3**: Minimal memory footprint (<50MB)
- **NFR-5.4**: Support concurrent API requests where beneficial

### 6. Reliability

- **NFR-6.1**: Handle API failures gracefully with retry logic
- **NFR-6.2**: Continue operation when route data unavailable
- **NFR-6.3**: Validate all data inputs to prevent crashes
- **NFR-6.4**: Maintain 25+ passing tests with >80% coverage

### 7. Security

- **NFR-7.1**: Never store API credentials in code
- **NFR-7.2**: Use secure token storage for OAuth (GPG/direnv or secure directory)
- **NFR-7.3**: Exclude sensitive files via .gitignore
- **NFR-7.4**: Support environment variables for secrets
- **NFR-7.5**: Provide security audit scripts (check_secrets.sh, sanitize_personal_data.sh)
- **NFR-7.6**: Pre-commit hooks to prevent accidental secret commits
- **NFR-7.7**: Replace personal data with placeholders before public release
- **NFR-7.8**: Support multiple secure configuration options for different user preferences
- **NFR-7.9**: Implement OAuth token refresh to prevent authentication failures

### 8. Usability

- **NFR-8.1**: Work with zero configuration (sensible defaults)
- **NFR-8.2**: Provide clear error messages
- **NFR-8.3**: Include comprehensive help text
- **NFR-8.4**: Support both simple and advanced usage

### 9. Maintainability

- **NFR-9.1**: Use Rust for type safety and performance
- **NFR-9.2**: Modular architecture with clear separation
- **NFR-9.3**: Comprehensive documentation in code
- **NFR-9.4**: Follow Rust idioms and best practices
- **NFR-9.5**: Version control with meaningful commits

### 10. Compatibility

- **NFR-10.1**: Run on Linux (primary target)
- **NFR-10.2**: Support WSL for Windows users
- **NFR-10.3**: Install to standard locations (~/.local/bin)
- **NFR-10.4**: Use SQLite for portability

## Data Requirements

### 11. Route Data

- **DR-11.1**: Store route_id as primary key (Zwift's internal ID)
- **DR-11.2**: Track distance in kilometers
- **DR-11.3**: Track elevation gain in meters
- **DR-11.4**: Track surface type (road, gravel, mixed)
- **DR-11.5**: Store route name and world
- **DR-11.6**: Store route slug for URL generation
- **DR-11.7**: Track lead-in distances (race, free ride, meetup variants)
- **DR-11.8**: Store external URLs (Strava segment, Zwift Insider, What's on Zwift)
- **DR-11.9**: Flag event-only routes vs free ride available
- **DR-11.10**: Track lap route indicator and time trial support

### 12. Race Results

- **DR-12.1**: Link results to routes via route_id
- **DR-12.2**: Store actual completion time in minutes
- **DR-12.3**: Store rider's Zwift Score at time of race
- **DR-12.4**: Track data source (Strava, ZwiftPower, manual)

### 13. Configuration

- **DR-13.1**: Support JSON configuration files (legacy)
- **DR-13.2**: Support TOML for improved readability (preferred)
- **DR-13.3**: Allow environment variable overrides
- **DR-13.4**: Provide secure storage options (GPG/direnv integration)
- **DR-13.5**: Configuration loading priority: local → secure dir → env vars → defaults
- **DR-13.6**: Separate secrets from non-secret configuration
- **DR-13.7**: Support personal wrappers that auto-load configuration

## Integration Requirements

### 14. External APIs

- **IR-14.1**: Integrate with Zwift Public API for events
- **IR-14.2**: Integrate with Strava API for race results
- **IR-14.3**: Support OAuth 2.0 authentication
- **IR-14.4**: Handle rate limiting appropriately
- **IR-14.5**: Cache API responses where beneficial

### 15. Data Sources

- **IR-15.1**: Import from ZwiftPower via browser extraction
- **IR-15.2**: Import from Strava activity exports
- **IR-15.3**: Support manual data entry
- **IR-15.4**: Web scraping for route information

## Testing Requirements

### 16. Test Coverage

- **TR-16.1**: Unit tests for core logic
- **TR-16.2**: Integration tests for API calls
- **TR-16.3**: Regression tests with real race data
- **TR-16.4**: Performance tests for large datasets
- **TR-16.5**: Security tests for credential handling

### 17. Test Data

- **TR-17.1**: Use actual race results for regression testing
- **TR-17.2**: Maintain test fixtures for predictable testing
- **TR-17.3**: Track accuracy metrics over time
- **TR-17.4**: Support test mode without API calls

## Future Enhancement Requirements

### 18. Advanced Features (Planned)

- **FER-18.1**: Real-time race tracking via Sauce4Zwift
- **FER-18.2**: Machine learning for improved predictions
- **FER-18.3**: Community data sharing for route times
- **FER-18.4**: Web interface for non-technical users
- **FER-18.5**: Mobile app with push notifications

### 19. Physics Modeling (Research Phase)

- **FER-19.1**: Implement Martin et al. power equations
- **FER-19.2**: Calculate CdA from rider dimensions (A = 0.0276 × h^0.725 × m^0.425)
- **FER-19.3**: Model grade-specific speed changes
- **FER-19.4**: Account for Zwift-specific physics (33% draft vs 25% real world)
- **FER-19.5**: Use height/weight for aerodynamic drag calculations
- **FER-19.6**: Adjust draft benefit based on rider height
- **FER-19.7**: Factor power-to-weight ratio for climbing predictions
- **FER-19.8**: Consider bike choice effects (TT vs road bike)
- **FER-19.9**: Import complete route data from zwift-data npm package (MIT licensed) including:
  - Route IDs, slugs, names, distances, elevation, lead-in distances
  - Surface type variations (cobbles, dirt, wood, brick, grass, snow)
  - External references (Strava segments, Zwift Insider, What's on Zwift)
  - Event-only routes, lap routes, time trial support flags
- **FER-19.10**: Map between different route identification systems for better matching
- **FER-19.11**: Consider zwiftmap.com architecture patterns for future visualization features
- **FER-19.12**: Track route completion history for gamification and variety scoring
- **FER-19.13**: Generate shareable configuration URLs for team/club setups
- **FER-19.14**: Support world availability schedule for event filtering
- **FER-19.15**: Implement protobuf support for certain Zwift API endpoints
- **FER-19.16**: Study GoldenCheetah concepts for reimplementation (GPL v3 prevents direct integration):
  - Use GoldenCheetah as analysis tool to validate our predictions
  - Study published papers on Critical Power (CP) and W' balance models
  - Research CdA estimation techniques from scientific literature
  - Implement our own power-duration curve fitting from first principles
  - Create clean-room implementation of TSS/CTL/ATL concepts (which are publicly documented)
- **FER-19.17**: Design race planning features using performance management concepts:
  - Predict optimal race timing based on fitness (CTL) and fatigue (ATL)
  - Estimate power targets for different race durations
  - Provide pacing strategies based on W' expenditure models
  - Generate race-specific training recommendations
  - Track performance trends to refine predictions over time

### 20. Automated Testing with Simulation Tools

- **FER-20.1**: Integrate with Zwift simulation tools that provide Bluetooth data
- **FER-20.2**: Create test scenarios with controlled power output profiles
- **FER-20.3**: Validate duration predictions against simulated race completions
- **FER-20.4**: Test edge cases (getting dropped, rejoining pack, sprint finishes)
- **FER-20.5**: Automate regression testing with multiple rider profiles
- **FER-20.6**: Compare simulated results across different routes and conditions
- **FER-20.7**: Build database of simulated race data for model training
- **FER-20.8**: Run offline and online race simulations for performance prediction:
  - Develop offline simulation engine using Zwift physics model
  - Create Monte Carlo simulations with varying field sizes and rider abilities
  - Model draft dynamics based on field size (larger field = more consistent draft)
  - Simulate position changes and draft availability throughout race
  - Validate against real race data and online test races
- **FER-20.9**: Generate dynamic race plans accounting for draft variability:
  - Calculate expected draft percentage based on field size and rider ability
  - Identify critical sections where draft loss is likely (climbs, attacks)
  - Plan power targets for drafted vs non-drafted segments
  - Provide contingency strategies for different race scenarios
  - Adjust plans based on real-time position in pack
  - Account for course-specific draft effectiveness (e.g., jungle roads vs open flats)
- **FER-20.10**: Analyze video recordings of races to validate and refine models:
  - Use OBS Studio or similar to record race footage with HUD data
  - Extract metrics from video: position in pack, speed, power, draft status
  - Identify patterns in pack behavior (splits, regrouping, sprint dynamics)
  - Correlate visual pack position with power requirements
  - Measure actual draft benefit in different scenarios
  - Document critical race moments (attacks, climbs, selections)
  - Build library of race scenarios for model training
  - Compare predicted vs actual race dynamics from video analysis
- **FER-20.11**: Investigate live data extraction from Zwift races using AI:
  - Research computer vision techniques for real-time HUD data extraction
  - Explore OCR/AI models for reading on-screen metrics (speed, power, position)
  - Design system architecture: direct capture on host vs external device
  - Evaluate Raspberry Pi with AI accelerator (Coral, Hailo) for dedicated capture
  - Implement real-time data pipeline: video → AI extraction → structured data
  - Create post-race analysis tools using extracted telemetry
  - Build Zwift/real-world comparison models using extracted data
  - Develop live race coaching features based on real-time position/power
  - Consider privacy/ToS implications of screen capture and analysis
- **FER-20.12**: Investigate suitable open-source simulation models with compatible licenses:
  - Research cycling physics simulators on GitHub (MIT, Apache, BSD licenses)
  - Evaluate OpenRoadSim or similar for peloton dynamics modeling
  - Study traffic flow models adaptable to cycling pack behavior
  - Investigate agent-based models for multi-rider simulations
  - Review computational fluid dynamics (CFD) models for draft calculations
  - Ensure license compatibility (avoid GPL for direct integration)
  - Prioritize models with documented physics equations
  - Look for validation against real-world cycling data
  - Consider models that support both online and offline simulation modes

## Success Metrics

### 21. Key Performance Indicators

- **KPI-21.1**: Prediction accuracy <20% MAE ✅ (Currently 16.6% on 125 races)
- **KPI-21.2**: Race matching rate >75% ✅ (Currently 80%)
- **KPI-21.3**: User satisfaction (via feedback)
- **KPI-21.4**: Route coverage >90% of common races
- **KPI-21.5**: Zero security incidents

## Constraints and Assumptions

### 22. Technical Constraints

- **TC-22.1**: Zwift API returns maximum 200 events
- **TC-22.2**: No official Zwift results API available
- **TC-22.3**: Racing Score events have distance=0 in API
- **TC-22.4**: Route IDs are stable but undocumented
- **TC-22.5**: No real-time telemetry available through Zwift logs (only debug info)
- **TC-22.6**: Zwift network packets encrypted since July 2022
- **TC-22.7**: FIT files saved every 10 minutes (not suitable for live analysis)
- **TC-22.8**: No local database with accessible race telemetry
- **TC-22.9**: Community packet monitoring tools broken by encryption

### 23. Assumptions

- **A-23.1**: Users know their Zwift Racing Score
- **A-23.2**: Draft benefit is ~30% in races
- **A-23.3**: Category speeds are relatively consistent
- **A-23.4**: Route characteristics affect all riders similarly
- **A-23.5**: Historical performance predicts future results

## Compliance Requirements

### 24. Legal and Ethical

- **CR-24.1**: Aim to respect Zwift's terms of service (users must verify compliance)
- **CR-24.2**: Only access public APIs
- **CR-24.3**: Don't store other users' data
- **CR-24.4**: Open source under MIT/Apache license
- **CR-24.5**: Credit data sources appropriately

## Development Methodology

### 25. AI-Assisted Development

- **DM-25.1**: Built using Claude Code for implementation
- **DM-25.2**: Human provides domain expertise and testing
- **DM-25.3**: Iterative refinement based on real data
- **DM-25.4**: Transparent development with clear reasoning
- **DM-25.5**: Version control for all changes

---

## Critical Discoveries from Development

### Pack Dynamics Model (2025-05-25)
- Getting dropped on hills explains 82.6% of race time variance
- Binary state: either with pack (30.9 km/h) or solo (23.8 km/h)
- Weight penalty significant: 86kg vs 70-75kg typical riders
- High variance is inherent to racing, not a prediction failure

### Event Type Systems (2025-05-26)
- Two mutually exclusive systems: Traditional (A/B/C/D) vs Racing Score (0-650)
- Racing Score events always have distanceInMeters: 0 in API
- Distance must be parsed from description text
- This affected ~50% of all events

### Route Discovery Insights
- Most "unknown routes" are custom event names, not actual routes
- Event organizer websites contain route details not in API
- Manual mapping more effective than automated discovery
- Route length must match typical race duration for accuracy

### AI Development Model
- Human provides domain expertise and quality control
- AI handles implementation and coding
- Transparency in reasoning catches wrong assumptions early
- Real data validation essential - assumptions will be wrong

## Recent Improvements and Current State (2025-05-27)

### Completed Improvements
1. **Code Quality** - All compilation warnings resolved, zero warnings in release build
2. **Multi-Lap Race Accuracy** - Fixed from 533% error to correct predictions (e.g., 38 min vs 6 min)
3. **Pattern Matching** - Flexible SQL matching handles event name variants
4. **Production Deployment** - Binary installed to ~/.local/bin, documentation complete
5. **Test Coverage** - Expanded from 16 to 25 tests (+56%), all passing
6. **Racing Score Events** - Fixed filtering for events with distanceInMeters: 0
7. **UX Enhancements** - Event type counts, smart suggestions, working examples

### Immediate Priorities
1. **User Functionality Concerns** - Investigate why user feels tool isn't working as desired
2. **Security** - Implement secure token storage for OAuth credentials
3. **Route Discovery** - Continue mapping high-frequency unknown routes
4. **Multi-Lap Automation** - Parse lap counts from event descriptions

### Known Issues Requiring Attention
1. **Category E** - Currently treated as Category D
2. **Rotating Race Series** - EVO CC runs different routes weekly
3. **Placeholder Route IDs** - Routes 9001-9003 need real Zwift route_ids
4. **Time Zone Display** - All times shown in local timezone

## Revision History

- 2025-05-27: Initial requirements document created
- 2025-05-27: Updated with user concerns and recent session improvements
- 2025-05-27: Comprehensive update after reviewing all 41 project *.md files
  - Added security requirements from SECURITY_AUDIT.md
  - Enhanced configuration requirements based on GPG/direnv integration
  - Added physics modeling details from PHYSICAL_STATS.md
  - Incorporated pack dynamics and event type discoveries
  - Added AI development insights from AI_DEVELOPMENT.md
- Based on: Production deployment with 16.1% accuracy achieved (now 16.6% on current DB)
- Status: Requirements now comprehensive, reflecting all documented needs and discoveries
- 2025-06-01: Added discovery insights from zwiftmap and zwift-data projects
  - Found comprehensive route database in zwift-data npm package (MIT licensed)
  - Identified need for mapping between route IDs, slugs, and names
  - Added consideration for surface types (cobbles, dirt, wood, etc.)
  - Note: Future enhancements should be added to existing sections without renumbering
- 2025-06-01: Comprehensive review against all reference sources
  - Added lead-in distance handling requirements (critical for accuracy)
  - Enhanced route data requirements with slugs, external URLs, flags
  - Added hidden event tags and URL-based filtering from ZwiftHacks
  - Included OAuth token refresh from zwift-client analysis
  - Added route completion tracking and shareable configurations
- 2025-01-06: Added table output format requirements (FR-4.2.5, FR-4.2.6, FR-4.2.7)
  - Default to compact table view with key information
  - Show only user's selected category
  - Support verbose flag for detailed output
- 2025-01-06: Added GoldenCheetah-inspired requirements (FER-19.16, FER-19.17)
  - Clarified GPL v3 prevents direct integration - must reimplement from scratch
  - Use GoldenCheetah output for validation only
  - Study published papers on Critical Power and W' models
  - Implement TSS/CTL/ATL from public formulas, not GoldenCheetah code
  - Design race planning using well-documented performance concepts
- 2025-01-06: Added race simulation requirements (FER-20.8, FER-20.9)
  - Offline simulation engine with Monte Carlo methods
  - Model draft dynamics based on field size
  - Generate race plans with draft/non-draft power targets
  - Account for course-specific draft effectiveness
- 2025-01-06: Added video analysis requirement (FER-20.10)
  - Use OBS Studio to record races with HUD data
  - Extract position, power, and draft metrics from video
  - Build library of race scenarios for model validation
  - Refine predictions based on observed pack dynamics
- 2025-01-06: Added live data extraction requirement (FER-20.11)
  - AI-based extraction of HUD data from video stream
  - Consider both host-based and Raspberry Pi architectures
  - Real-time telemetry pipeline for live coaching
  - Post-race analysis and Zwift/real-world modeling
- 2025-01-06: Added simulation model research requirement (FER-20.12)
  - Find open-source cycling/peloton simulators with compatible licenses
  - Focus on MIT/Apache/BSD to allow integration
  - Look for validated physics models and multi-agent support
- 2025-01-06: Major discovery session - Zwift telemetry limitations and live analysis solution
  - Discovered Zwift provides no real-time telemetry (logs, APIs, packets all inadequate)
  - Added comprehensive "Zwift Live Telemetry Tool" requirements (Section 21)
  - Justified video analysis approach based on lack of alternatives
  - Added technical constraints TC-22.5 through TC-22.9 documenting Zwift limitations
  - Explored hardware options: Raspberry Pi 5 + Hailo-8L justified despite initial skepticism
  - Researched Zwift ToS Section 5a(XI) - updated compliance requirements
  - Noted techniques to avoid based on community ban reports
  - Clarified users must determine their own ToS compliance
  - Removed definitive compliance claims - aim to respect ToS

### 21. Zwift Live Telemetry Tool (New Companion Application)

#### Background and Rationale
Discovery from development session (2025-01-06): Zwift provides no real-time telemetry access through:
- APIs (only pre-race event data available)
- Local logs (contain debug info, not telemetry)
- Network packets (encrypted since 2022)
- FIT files (saved every 10 minutes, not real-time)

Therefore, video analysis of the game display is the only viable method for real-time race telemetry extraction.

#### 21.1 Tool Overview
- **FER-21.1.1**: Create separate tool "zwift-live-telemetry" for real-time race monitoring
- **FER-21.1.2**: Purpose: Extract telemetry data during races for live analysis and coaching
- **FER-21.1.3**: Integration: Feed extracted data back to improve zwift-race-finder predictions
- **FER-21.1.4**: Architecture: Support both local (screen capture) and remote (HDMI capture) modes

#### 21.2 Video Capture Requirements
- **FER-21.2.1**: Local mode: Screen capture on same PC as Zwift
  - Use native OS APIs (Windows Graphics Capture, X11 on Linux)
  - Minimal performance impact on Zwift
  - No additional hardware required
- **FER-21.2.2**: Remote mode: Dedicated capture device
  - Support HDMI capture cards (USB 3.0)
  - Optional: Raspberry Pi 5 with AI accelerator for edge processing
  - Enable analysis without impacting gaming PC performance
- **FER-21.2.3**: Support common Zwift display resolutions (1080p, 1440p, 4K)
- **FER-21.2.4**: Handle dynamic HUD layouts and UI scaling

#### 21.3 Data Extraction via Computer Vision
- **FER-21.3.1**: OCR extraction of numeric HUD elements:
  - Power (watts) - current and average
  - Speed (km/h or mph based on settings)
  - Heart rate (bpm)
  - Cadence (rpm)
  - Distance completed (km/mi)
  - Time elapsed
- **FER-21.3.2**: Visual analysis of non-numeric elements:
  - Position in race (1st, 2nd, etc.)
  - Gap to leader/next rider
  - Draft status (in draft vs solo)
  - Gradient percentage
  - Power-up status
- **FER-21.3.3**: Robust detection handling:
  - Multiple HUD layouts (default, minimal, custom)
  - Transparency settings
  - UI elements overlapping data
  - Motion blur during fast segments

#### 21.4 Real-time Processing Pipeline
- **FER-21.4.1**: Frame extraction at 5-10 FPS (sufficient for telemetry)
- **FER-21.4.2**: AI model requirements:
  - Lightweight models optimized for edge deployment
  - <100ms inference time per frame
  - Pre-trained on Zwift UI elements
  - Support model updates for UI changes
- **FER-21.4.3**: Data validation and smoothing:
  - Detect and filter OCR errors
  - Interpolate missing values
  - Apply physics-based sanity checks

#### 21.5 Data Output and Integration
- **FER-21.5.1**: Real-time data streaming:
  - WebSocket server for live clients
  - JSON message format with timestamps
  - Support multiple concurrent consumers
- **FER-21.5.2**: Data persistence:
  - Store in SQLite with high-frequency telemetry table
  - Aggregate to 1-second intervals for storage efficiency
  - Link to zwift-race-finder event data
- **FER-21.5.3**: Analysis outputs:
  - Real-time power zones and effort tracking
  - Draft percentage over time
  - Position changes and race dynamics
  - Automatic segment detection

#### 21.6 Live Coaching Features
- **FER-21.6.1**: Real-time alerts:
  - Power target adherence
  - Position changes (gained/lost places)
  - Upcoming gradient changes
  - Effort sustainability warnings
- **FER-21.6.2**: Race strategy automation:
  - Optimal power targets based on race progress
  - Sprint timing recommendations
  - Recovery interval suggestions
- **FER-21.6.3**: Audio/visual feedback options:
  - Text overlay on secondary screen
  - Audio cues via TTS
  - Integration with streaming software (OBS)

#### 21.7 Privacy and Compliance (Critical - Zwift ToS)
- **FER-21.7.1**: Local processing only - no cloud upload of video
- **FER-21.7.2**: User consent for any data sharing
- **FER-21.7.3**: Design with intent to respect Zwift ToS:
  - Users must review ToS Section 5a(XI) themselves
  - Avoid interaction with Zwift platform without authorization
  - No protocol emulation or redirection
  - No network packet interception (community reports bans)
  - No memory reading or process manipulation
  - Position as "screen recording tool with analysis" (like OBS)
- **FER-21.7.4**: Passive observation approach:
  - Screen capture similar to streaming tools (OBS)
  - Visual analysis of captured frames only
  - No automation or control of Zwift gameplay
  - No data modification or injection
  - Users must determine if this provides unfair advantages
- **FER-21.7.5**: Open source to ensure transparency
- **FER-21.7.6**: Include clear disclaimers:
  - "This tool uses screen capture for analysis only"
  - "Does not interact with or modify Zwift in any way"
  - "Users must review Zwift's Terms of Service"
  - "Use at your own risk - you are responsible for compliance"
- **FER-21.7.7**: Avoid banned approaches discovered by community:
  - No "man-in-the-middle" data interception (6-month bans)
  - No reverse engineering of protocols
  - No unauthorized API access
  - Learn from tools that were shut down (packet monitors post-2022)

#### 21.8 Hardware Options (Remote Mode)
- **FER-21.8.1**: Raspberry Pi 5 configuration:
  - Hailo-8L AI hat (26 TOPS) for inference
  - Alternative: Google Coral USB (4 TOPS) for budget option
  - USB 3.0 HDMI capture card
  - Gigabit ethernet for data streaming
- **FER-21.8.2**: x86 Mini PC option:
  - Intel N100/N200 for better compatibility
  - Integrated GPU for video decode
  - Lower power than gaming PC
- **FER-21.8.3**: Performance targets:
  - 1080p30 capture and processing
  - <50ms end-to-end latency
  - <10W power consumption

#### 21.9 Development Priorities
1. Proof of concept with static screenshot analysis
2. Local screen capture implementation
3. Basic OCR for power/speed/HR
4. Real-time streaming infrastructure
5. Advanced visual analysis (position, draft)
6. Remote capture device support
7. Live coaching features
8. Integration with zwift-race-finder

- Based on: Production deployment with 16.1% accuracy achieved (now 16.6% on current DB)
- Status: Requirements now complete with insights from all reference sources

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/ROUTE_DATA_EXTRACTION.md -->
<!-- ============================================================ -->

# Zwift Route Data Extraction Documentation

This document details the investigation into extracting complete route data from Zwift, including findings, methods, and current limitations.

## Overview

Zwift route data is essential for calculating race durations. Routes have three key components:
- **Base route distance** - The core loop distance
- **Lead-in distance** - Distance from start point to the loop beginning
- **Elevation gain** - Total meters of climbing

## Route Data Sources

### 1. WAD Archive Files (Authoritative Source)

All route data is stored in compressed WAD archives within the Zwift installation:
```
C:\Program Files (x86)\Zwift\assets\Worlds\world*\data_1.wad
```

These archives contain XML files with complete route definitions:
```xml
<route>
  <name>Route Name</name>
  <nameHash>12345678</nameHash>
  <distanceInMeters>15000</distanceInMeters>
  <elevationGainInMeters>250</elevationGainInMeters>
  <leadinDistanceInMeters>500</leadinDistanceInMeters>
  <eventOnly>1</eventOnly>
  <sportType>1</sportType>
</route>
```

### 2. zwift-offline Export (Limited)

The zwift-offline project provides route data through its API, but with limitations:
- Only exports routes marked with `eventOnly='1'`
- Provides 55 routes out of 300+ total routes
- Missing free-ride routes used in many events

### 3. Third-Party Sources

Several community sources provide route data:
- ZwiftHacks.com - Comprehensive route database
- WhatsOnZwift.com - Route profiles and details
- ZwiftInsider.com - Route descriptions and strategies

## Extraction Methods

### Method 1: WAD File Extraction (Requires Tools)

```python
# From zwift-offline/scripts/get_events.py
subprocess.run(['wad_unpack.exe', os.path.join(worlds, directory, 'data_1.wad')])
routes = os.path.join('Worlds', directory, 'routes')
```

**Requirements:**
- `wad_unpack.exe` - A decompression tool (no longer publicly available)
- Access to Zwift game files

**Process:**
1. Decompress WAD files using wad_unpack.exe
2. Parse XML route definitions
3. Extract distance, elevation, and metadata

### Method 2: zwift-offline Integration (Currently Used)

```bash
# Import from running zwift-offline server
./scripts/import_from_zwift_offline.sh --skip-ssl-verify
```

**Limitations:**
- Only event routes (eventOnly='1')
- No free-ride routes
- Limited to 55 routes

### Method 3: Empirical Collection (Last Resort)

For missing routes:
1. Create a Zwift event on the target route
2. Ride the event and collect FIT file
3. Analyze to determine lead-in and lap distances

## Event System Understanding

### How Zwift Events Work

Events modify base routes using three parameters:

1. **Laps** - Repeat the route loop N times
   ```protobuf
   optional uint32 laps = 25;
   ```

2. **Distance** - Fixed total distance
   ```protobuf
   optional float distanceInMeters = 24;
   ```

3. **Duration** - Time-based events
   ```protobuf
   optional uint32 durationInSeconds = 34;
   ```

### Route Structure

```
[Start Point] → [Lead-in Distance] → [Loop Start] → [Loop Distance] → [Loop End]
                                           ↑                               ↓
                                           └─────────── (if laps > 1) ─────┘
```

### Free-Ride vs Event Routes

- **Event-only routes**: Fixed courses for organized events
- **Free-ride routes**: Open world routes with spawn points
- Both can be used for races with appropriate modifiers

## Current Implementation Status

### What We Have

1. **Routes in database** from third-party imports (historically 378; current DB has fewer — re-import needed)
2. **55 event routes** from zwift-offline integration
3. **Working import tools** for various sources
4. **License-compliant integration** via API boundary

### What We Tried

1. Created extraction scripts:
   - `get_all_routes.py` - Extract all routes from WAD files
   - `extract_all_routes_wsl.sh` - WSL wrapper for Windows

2. Investigated workarounds:
   - Merging events.txt and start_lines.txt (abandoned - no distance data)
   - Memory dump extraction (against Zwift ToS)

### Current Limitations

1. **Missing wad_unpack.exe**
   - Referenced tools no longer available:
     - github.com/h4l/zwift-routes (doesn't exist)
     - github.com/h4l/zwift-map-parser (doesn't exist)
   
2. **Incomplete route coverage**
   - Free-ride routes not accessible
   - Some event routes use free-ride base routes

## Technical Details

### World/Course ID Mapping

```python
world_to_course = {
    '1': (6, 'Watopia'),
    '2': (2, 'Richmond'),
    '3': (7, 'London'),
    '4': (8, 'New York'),
    '5': (9, 'Innsbruck'),
    '6': (10, 'Bologna'),
    '7': (11, 'Yorkshire'),
    '8': (12, 'Crit City'),
    '9': (13, 'Makuri Islands'),
    '10': (14, 'France'),
    '11': (15, 'Paris'),
    '12': (16, 'Gravel Mountain'),
    '13': (17, 'Scotland')
}
```

### Route ID Handling

- Route IDs can be negative (signed integers)
- Use route_id as identifier, never route name
- Names change, IDs are permanent

## Future Possibilities

### If wad_unpack.exe Becomes Available

1. Run `get_all_routes.py` to extract complete route database
2. Import both event and free-ride routes
3. Achieve 100% route coverage

### Alternative Tools

- **zwf** (github.com/h4l/zwf) - Works only with decompressed WADs
- **zwift-utils** (gitlab.com/r3dey3/zwift-utils) - Claims decompression support

### Community Collaboration

- Crowdsource FIT files for missing routes
- Share route data with other developers
- Contribute findings back to zwift-offline

## Recommendations

1. **Continue using current data sources** - third-party route imports provide good coverage when populated
2. **Monitor for tool availability** - Check periodically for wad_unpack.exe
3. **Document missing routes** - Track which routes need data
4. **Consider empirical collection** - For critical missing routes only

## Conclusion

The route data exists and is well-structured in Zwift's WAD files. The primary limitation is the availability of decompression tools, not the data itself. Current third-party sources and zwift-offline integration provide sufficient coverage for most use cases.
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/RUST_REFACTORING.md -->
<!-- ============================================================ -->

# Rust Refactoring Reference

Tools, mechanics, and patterns for safely restructuring Rust code. For the philosophy and AI-specific rules, see [Refactoring Rules](REFACTORING_RULES.md) and [Refactoring Explained](../explanation/REFACTORING_EXPLAINED.md).

## Tools

### Essential
```bash
cargo install cargo-edit         # Add/remove/upgrade dependencies
cargo install cargo-expand       # Expand macros
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-mutants      # Mutation testing
rustup component add clippy rustfmt
```

### Optional
```bash
cargo install cargo-nextest      # Better test runner
cargo install cargo-outdated     # Check outdated deps
cargo install cargo-audit        # Security audit
cargo install cargo-semver-checks # Breaking change detection
cargo install cargo-watch        # Auto-run on file changes
```

### IDE (rust-analyzer)
- `Ctrl+.` — Quick fixes and refactorings
- `F2` — Rename symbol
- `Ctrl+Shift+R` — Refactor menu

## Mechanical Catalog

### Extract Function
```rust
// Before
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    for item in &items {
        if !item.is_valid() { return Err(Error::InvalidItem); }
    }
    let total = items.iter().map(|i| i.value).sum();
    Ok(Summary { total })
}

// After — ownership matters: take &[Item] not Vec<Item> when borrowing
fn validate_items(items: &[Item]) -> Result<(), Error> {
    for item in items {
        if !item.is_valid() { return Err(Error::InvalidItem); }
    }
    Ok(())
}
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    validate_items(&items)?;
    Ok(Summary { total: items.iter().map(|i| i.value).sum() })
}
```

### Extract Module
```rust
// Step 1: Inline module (verify it works)
mod validation {
    use super::*;
    pub fn validate_items(items: &[Item]) -> Result<(), Error> { /* ... */ }
}

// Step 2: Move to separate file
// main.rs: mod validation;
// validation.rs: use crate::{Item, Error}; pub fn validate_items(...) { }
```

### Rename Symbol
Use rust-analyzer's `F2`. Review all occurrences before confirming. **Never** change logic while renaming.

### Change Function Signature (Migration Method)
```rust
// Step 1: Create new function with desired signature
fn calculate_v2(base: f64, rate: f64, years: u32) -> f64 { base * rate * years as f64 }

// Step 2: Old function delegates to new
#[deprecated(note = "Use calculate_v2")]
fn calculate(base: f64, rate: f64) -> f64 { calculate_v2(base, rate, 1) }

// Step 3: Update callers one by one, running tests after each
// Step 4: Remove old function
```

### Replace Loop with Iterator
```rust
// Before
let mut result = Vec::new();
for item in items {
    if item.is_active() { result.push(item.value * 2); }
}

// After — maintain lazy evaluation, don't collect intermediate results
let result: Vec<_> = items.into_iter()
    .filter(|item| item.is_active())
    .map(|item| item.value * 2)
    .collect();
```

## Ownership Patterns

### Reduce Cloning
```rust
// Bad: unnecessary clone
fn total(items: Vec<Item>) -> f64 {
    let cloned = items.clone();
    cloned.iter().map(|i| i.price).sum()
}

// Good: borrow
fn total(items: &[Item]) -> f64 {
    items.iter().map(|i| i.price).sum()
}
```

### Simplify Lifetimes
```rust
// Before: explicit lifetimes where elision works
fn find_item<'a, 'b>(name: &'a str, items: &'b [Item]) -> Option<&'b Item> { ... }

// After: lifetime elision
fn find_item(name: &str, items: &[Item]) -> Option<&Item> { ... }
```

### Choose References vs Ownership
```rust
impl Container {
    fn len(&self) -> usize { }           // Read only: &self
    fn push(&mut self, item: Item) { }   // Modify: &mut self
    fn into_vec(self) -> Vec<Item> { }   // Consume: self
}
```

## Visibility Progression

Start restrictive, expand as needed:
1. `fn` (private) → 2. `pub(super)` → 3. `pub(crate)` → 4. `pub`

## Error Handling

```rust
// Bad: string errors, lost context
fn parse() -> Result<Data, String> { Err("failed".into()) }

// Good: structured errors with context
use anyhow::{Context, Result};
fn parse() -> Result<Data> {
    let content = fs::read_to_string("config.json")
        .context("Failed to read config")?;
    serde_json::from_str(&content)
        .context("Failed to parse config")
}
```

## Common Pitfalls

| Pitfall | Fix |
|---------|-----|
| Breaking iterator chains (collecting then re-iterating) | Keep chains lazy |
| Over-using `Arc<Mutex<T>>` | Use message passing or state machines |
| Making everything async | Only async for actual I/O |
| Over-abstracting (trait for one impl) | Concrete first, extract when needed |
| Losing error type info (`Box<dyn Error>`) | Use concrete error types |

## Validation Checklist

Before refactoring:
- [ ] All tests passing, no clippy warnings
- [ ] Benchmarks run (if performance-critical)

After each change:
- [ ] `cargo check` (fast compilation check)
- [ ] `cargo test` (behaviour preserved)
- [ ] `cargo clippy -- -D warnings` (no new warnings)

After all refactoring:
- [ ] `cargo mutants` on changed modules (tests still effective)
- [ ] No test files modified (tests are the spec)
- [ ] API compatibility maintained (if public)

## Useful Aliases

```bash
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias ccl='cargo clippy -- -W clippy::all'
alias cw='cargo watch -x check -x test -x clippy'
```

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/SECURITY_AUDIT.md -->
<!-- ============================================================ -->

# Security Audit Report - Zwift Race Finder

## Summary
Overall security assessment: **MEDIUM RISK** - Some issues need addressing before public release.

## Findings

### 🔴 HIGH Priority Issues

1. **Personal Data in Multiple Files**
   - **Issue**: ZwiftPower profile ID (1106548) hardcoded in multiple scripts
   - **Files**: `scrape_zwiftpower.sh`, `export_zwiftpower_logged_in.sh`, `src/main.rs`
   - **Fix**: Run `sanitize_personal_data.sh` to replace with placeholders

2. **Session IDs in Code**
   - **Issue**: Session ID hardcoded in `src/main.rs` line 365
   - **Risk**: Could be used to access your ZwiftPower account
   - **Fix**: Remove or use environment variable

3. **Personal Race Data**
   - **Files**: `zwiftpower_results.json`, `zwiftpower_page_structure.json`
   - **Risk**: Contains your complete race history
   - **Fix**: Already in .gitignore, ensure not committed

### 🟡 MEDIUM Priority Issues

1. **Windows Username in Paths**
   - **Issue**: `/mnt/c/Users/YOUR_USERNAME/` in multiple scripts
   - **Files**: Various import scripts
   - **Fix**: Replace with generic path or environment variable

2. **Email Address**
   - **File**: `ZWIFT_API_LOG.md`
   - **Issue**: Contains `your.email@example.com`
   - **Fix**: Remove or replace with example email

3. **Browser Automation Scripts**
   - **Files**: `scrape_zwiftpower.py`, various JS files
   - **Risk**: Could be misused for scraping
   - **Mitigation**: Add usage disclaimer in README

### 🟢 LOW Priority / Good Practices

1. **SQL Injection Protection**
   - **Status**: ✅ GOOD - Uses parameterized queries
   - **Example**: `params![route_id, event_name, event_type]`

2. **API Security**
   - **Status**: ✅ GOOD - Only uses public Zwift API
   - **URL**: `https://us-or-rly101.zwift.com/api/public/events/upcoming`

3. **File Permissions**
   - **Status**: ✅ GOOD - Scripts use proper error handling
   - **Example**: `set -euo pipefail` in bash scripts

4. **Database Location**
   - **Status**: ✅ GOOD - Uses user's local data directory
   - **Path**: `~/.local/share/zwift-race-finder/races.db`

### 🔍 Additional Observations

1. **No Hardcoded Credentials**
   - No API keys, passwords, or tokens found in code
   - ZwiftPower access relies on browser-based extraction

2. **Safe External Dependencies**
   - All Cargo dependencies are well-known, maintained libraries
   - No suspicious or unmaintained dependencies

3. **Local-Only Data Storage**
   - All data stored locally in SQLite
   - No cloud services or external data transmission

## Recommendations

### Before Making Public:

1. **Run Sanitization Script**
   ```bash
   ./sanitize_personal_data.sh
   ```

2. **Verify Clean State**
   ```bash
   # Check no personal data in git
   git grep -i "1106548\|jackc\|rechung"
   
   # Ensure personal files not tracked
   git status --ignored
   ```

3. **Add Security Notice to README**
   ```markdown
   ## Security Notice
   - Never commit your ZwiftPower session IDs or profile IDs
   - Keep your `config.json` file private
   - The browser extraction scripts should only be used on your own profile
   ```

4. **Consider Adding**
   - `.env.example` file for configuration template
   - Rate limiting notice for API usage
   - Clear documentation about data privacy

### Good Security Practices Already in Place:

- ✅ Parameterized SQL queries prevent injection
- ✅ Proper error handling in scripts
- ✅ Local-only data storage
- ✅ No credential storage in code
- ✅ Public API usage only
- ✅ Comprehensive .gitignore file

## Conclusion

After running the sanitization script and addressing the high-priority issues, this repository will be safe to make public. The codebase follows good security practices for a personal project, with proper SQL handling and no credential storage.

## Session 2025-01-05: Repository Sanitization and Secure Configuration

### Problems Solved
- Created comprehensive system to sanitize personal data before making repository public
- Implemented multiple secure configuration options that survive repository updates
- Migrated from JSON to TOML configuration for better readability
- Integrated GPG/direnv for secure secret management

### Key Discoveries
- Personal data was scattered across multiple files (profile IDs, session tokens, file paths)
- Configuration needed separation between secrets and non-secret settings
- Users need zero-friction way to restore personal config after sanitization

### Solutions Implemented

1. **Sanitization System**:
   - `sanitize_personal_data.sh` - Replaces all personal identifiers with placeholders
   - `check_secrets.sh` - Standalone security scanner for pre-commit checks
   - `setup_git_hooks.sh` - Pre-commit hooks to prevent accidental secret commits

2. **Secure Configuration Options**:
   - **GPG/direnv Integration**: Secrets loaded via environment variables
   - **Local Secure Directory**: `~/.config/zwift-race-finder/` with restricted permissions
   - **GPG Encrypted**: Passphrase-protected configuration

3. **Configuration Architecture**:
   - Separated secrets (env vars) from settings (TOML files)
   - Multi-source config loading: local → secure dir → env vars → defaults
   - Smart wrappers that auto-load from preferred source

### Technical Implementation

**Config Loading Priority** (src/config.rs):
```rust
1. ./config.toml (local)
2. ~/.config/zwift-race-finder/config.toml (secure)
3. Environment variables (from direnv/ak)
4. Default values
```

**TOML Configuration Structure**:
```toml
[defaults]
zwift_score = 195
category = "D"

[import]
windows_username = "jackc"

[preferences]
default_duration = 120
default_tolerance = 30
```

### Lessons Learned
- Always separate secrets from configuration
- Provide multiple security options for different user preferences
- Use standard tools (GPG/direnv) rather than custom encryption
- Make the "right thing" (security) the easy thing (one command setup)
- TOML is more user-friendly than JSON for configuration files

---
## Key Commands

```bash
# One-time personal config setup (interactive)
./setup_personal_config.sh

# Secrets setup (direnv/ak)
direnv allow

# Security checks before committing
./check_secrets.sh

# Sanitize repository for public release
./sanitize_personal_data.sh

# Install pre-commit hooks
./setup_git_hooks.sh

# Use personal wrapper (auto-loads config)
./zwift-race-finder-personal

# Restore config from secure location
./restore_personal_config.sh

# Decrypt config (if using encryption)
./decrypt_config.sh
```

<!-- ============================================================ -->
<!-- FILE: ./docs/reference/SIMULATION_TOOLS.md -->
<!-- ============================================================ -->

# Zwift Simulation Tools for Automated Testing

This document lists tools that can simulate Bluetooth/ANT+ cycling devices for automated testing of Zwift applications.

## Device Simulation Tools

### 1. [Gymnasticon](https://github.com/ptx2/gymnasticon) (500+ stars)
- **Language**: JavaScript/Node.js
- **Purpose**: Bridge proprietary bikes (Peloton, etc.) to Zwift
- **Features**:
  - Simulates ANT+ and BLE power/cadence sensors
  - Modifiable for custom power profiles (Cat A/B/C/D riders)
  - Active development and community support
- **Testing Use**: Create repeatable test scenarios with specific power outputs

### 2. [FortiusANT](https://github.com/WouterJD/FortiusANT) (100+ stars)
- **Language**: Python
- **Purpose**: Connect old Tacx trainers to modern cycling apps
- **Features**:
  - Full ANT+ FE-C (Fitness Equipment Control) protocol
  - Grade simulation support
  - Can run headless for automation
  - Detailed logging capabilities
- **Testing Use**: Automated regression testing with grade changes

### 3. [Zwack](https://github.com/paixaop/zwack) (50+ stars)
- **Language**: JavaScript/Node.js
- **Purpose**: Pure Bluetooth LE sensor simulator
- **Features**:
  - Simulates FTMS (Fitness Machine Service)
  - Cycling Power Service
  - Heart Rate Service
  - Keyboard controls: w/s (power), a/d (cadence)
- **Testing Use**: Interactive testing of specific race scenarios

### 4. [openant](https://github.com/Tigge/openant) (300+ stars)
- **Language**: Python
- **Purpose**: ANT+ protocol library with simulators
- **Features**:
  - Complete ANT+ protocol implementation
  - Example device simulators included
  - Power, HR, speed, cadence simulation
  - Well-documented API
- **Testing Use**: Build custom test simulators for edge cases

### 5. [GoldenCheetah](https://github.com/GoldenCheetah/GoldenCheetah) (2000+ stars)
- **Language**: C++
- **Purpose**: Comprehensive cycling analytics platform
- **Features**:
  - ANT+ device simulation code
  - Reference power curve implementations
  - Extensive physiological models
- **Testing Use**: Reference implementation for power calculations

## Related Tools & APIs

### [Sauce4Zwift](https://github.com/SauceLLC/sauce4zwift) (400+ stars)
- **Purpose**: Real-time Zwift data access
- **API Endpoints**:
  - REST: `http://localhost:1080/api`
  - WebSocket: `ws://localhost:1080/api/ws/events`
- **Features**:
  - Live race data, gaps, positions
  - Headless mode for automation
  - Python client examples
- **Testing Use**: Validate predictions against live race data

### [zwift-offline](https://github.com/zoffline/zwift-offline) (912+ stars)
- **Purpose**: Offline Zwift server implementation
- **Features**:
  - Complete server protocol implementation
  - Reveals internal API structure
  - No subscription required for testing
- **Testing Use**: Test without active Zwift subscription

### [zwift-mobile-api](https://github.com/Ogadai/zwift-mobile-api) (109+ stars)
- **Purpose**: JavaScript client for Zwift's mobile API
- **Status**: ⚠️ Now requires Developer API access (restricted)
- **Features**:
  - Protocol documentation
  - Protobuf message decoding examples
- **Testing Use**: Understanding Zwift's data formats

## Testing Applications

### Automated Regression Testing
1. Use device simulators to create repeatable power profiles
2. Simulate different categories (A/B/C/D) with appropriate power outputs
3. Test edge cases: getting dropped, rejoining pack, sprint finishes
4. Validate duration predictions against simulated completions

### Test Scenarios
- **Steady State**: Constant power for entire race
- **Variable Power**: Simulate real race dynamics with surges
- **Getting Dropped**: High power start, then drop to solo pace
- **Sprint Finish**: Steady power with final sprint
- **Equipment Failure**: Power dropouts, sensor disconnections

### Implementation Example
```bash
# Using Zwack for simple testing
git clone https://github.com/paixaop/zwack.git
cd zwack
npm install
node server.js
# Use w/s keys to adjust power, a/d for cadence

# Using openant for automated testing
pip install openant
# Create custom script with specific power profile
```

## Future Integration Plans

1. **Automated Test Suite**
   - Create power profiles for each racing category
   - Run simulated races on all routes
   - Compare predicted vs actual times
   - Build regression test database

2. **Continuous Integration**
   - Nightly automated tests
   - Performance tracking dashboard
   - Alert on prediction accuracy degradation

3. **Machine Learning Dataset**
   - Generate thousands of simulated races
   - Various power profiles and race dynamics
   - Train improved prediction models

## Security & Compliance Note

When using these tools:
- Respect Zwift's Terms of Service
- Use only for testing and development
- Don't use for gaining unfair advantage in races
- Consider impact on other riders if testing in public events
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/TEST_SUITE_SUMMARY.md -->
<!-- ============================================================ -->

# Zwift Race Finder Test Suite Summary

Date: 2025-01-06
Created by: Claude with Jack

## Overview

A comprehensive test suite has been created for the Zwift Race Finder application, providing robust coverage across unit tests, integration tests, property-based tests, and performance benchmarks.

## Test Statistics

### Current Coverage
- **Library Tests**: 8 tests (all passing)
- **Main Binary Tests**: 28 tests passing, 1 ignored
- **Integration Tests**: Ready but require API credentials
- **Property Tests**: 5 comprehensive property-based tests
- **API Tests**: 8 tests with mocked API responses
- **Config Tests**: 5 tests for configuration management
- **Performance Benchmarks**: 6 benchmarks for critical paths

Total: **60+ tests** covering all major functionality

### Test Organization

```
tests/
├── README.md              # Test documentation
├── integration_tests.rs   # CLI and end-to-end tests
├── api_tests.rs          # API interaction with mocks
├── config_tests.rs       # Configuration loading tests
└── property_tests.rs     # Property-based testing

benches/
└── performance.rs        # Performance benchmarks

src/
├── main.rs              # 20 unit tests
├── database.rs          # 2 unit tests
├── secure_storage.rs    # 3 unit tests
├── route_discovery.rs   # 2 unit tests
└── regression_test.rs   # 3 regression tests
```

## Key Test Categories

### 1. Unit Tests (30 tests)
- Duration estimation algorithms
- Event filtering logic
- Route parsing and detection
- Format functions
- URL parameter parsing
- Racing Score vs Traditional category detection

### 2. Integration Tests
- CLI argument parsing
- Help command validation
- Invalid input handling
- Database creation
- Conflicting options

### 3. API Tests (with Mockito)
- Successful event fetching
- 404 error handling
- Malformed JSON handling
- Empty responses
- Racing Score event parsing
- Rate limiting
- Strava token refresh

### 4. Property-Based Tests
- Duration formatting across all ranges
- Duration estimation boundaries
- Filter logic symmetry
- URL parameter robustness
- Race result parsing edge cases

### 5. Configuration Tests
- Default configuration values
- TOML parsing
- File loading precedence
- Partial configurations
- Invalid config handling

### 6. Performance Benchmarks
- Route lookup performance
- Batch route operations
- Duration estimation calculations
- Format operations
- URL parsing
- Event filtering at scale

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --lib                    # Library tests only
cargo test --bin zwift-race-finder  # Main binary tests
cargo test integration             # Integration tests
cargo test api                     # API tests
cargo test config                  # Configuration tests
cargo test property                # Property-based tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Generate coverage report (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Test Quality Features

### 1. Realistic Test Data
- Uses actual Zwift event structures
- Real route data from Jack's racing history
- Authentic Racing Score event formats

### 2. Edge Case Coverage
- Zero distances
- Invalid formats
- Missing data
- Extreme values
- Multi-lap races

### 3. Error Handling
- API failures
- Database errors
- Invalid configurations
- Parsing failures

### 4. Performance Testing
- Database operations benchmarked
- Critical path optimizations measured
- Scale testing with 1000+ events

## Key Insights from Test Development

1. **Test-Driven Development Essential**: The attempted code reorganization failed because comprehensive tests weren't in place first. TDD prevents such issues.

2. **AI Behavior**: AI assistants may inadvertently change logic when asked to reorganize code. Tests catch these modifications.

3. **Property Testing Value**: Property-based tests found edge cases in duration formatting and parsing that traditional tests might miss.

4. **Mock Strategy**: Using Mockito for API tests allows testing error conditions without actual API calls.

5. **Regression Tests**: Using Jack's actual race history ensures predictions remain accurate as code evolves.

## Future Improvements

1. **Increase Coverage Target**: Aim for >90% code coverage
2. **Add Mutation Testing**: Use cargo-mutants to verify test quality
3. **Golden Tests**: Add snapshot tests for output format stability
4. **Fuzz Testing**: Add fuzzing for parsers
5. **Contract Tests**: Add tests to verify API contract assumptions

## Conclusion

The test suite provides strong confidence in the application's correctness and performance. With 60+ tests covering all major functionality, the project is well-protected against regressions and ready for future enhancements.
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/ZWIFT_DOMAIN.md -->
<!-- ============================================================ -->

# Zwift Domain Knowledge

## Racing Categories

### Traditional Categories (A/B/C/D/E)
- **A**: Elite racers, typically 4.0+ w/kg FTP
- **B**: Strong racers, 3.2-3.9 w/kg
- **C**: Intermediate, 2.5-3.1 w/kg
- **D**: Beginner/recreational, <2.5 w/kg
- **E**: Recovery/social, no performance requirements

### Zwift Racing Score (ZRS)
- **Range**: 0-1000 (most riders 0-650)
- **Replaces**: Traditional categories in many events
- **Dynamic**: Updates based on recent performance
- **Granular**: No sandbagging, automatic placement
- **Brackets**: Events use score ranges (e.g., 200-299)

## Key Concepts

### Draft Benefit
- **Zwift**: ~33% power savings in pack
- **Real World**: ~25% power savings
- **Impact**: Staying with pack is crucial
- **Auto-Draft**: Enabled in races (no steering needed)

### Weight Impact
- **Flats**: Power matters most (heavier = advantage)
- **Climbs**: W/kg crucial (lighter = advantage)
- **Zwift Physics**: More polarized than real world
- **Typical Racer**: 70-75kg (Jack at 86kg = climb disadvantage)

### Route Types

#### Surface Types
- **Road**: Standard tarmac, most common
- **Dirt/Gravel**: Slower speeds, MTB events
- **Mixed**: Combination surfaces

#### Elevation Profiles
- **Flat**: <5m elevation per km
- **Rolling**: 5-10m/km
- **Hilly**: 10-15m/km
- **Mountainous**: >15m/km

### Event Types

#### Race
- Full draft enabled
- Results tracked
- Categories enforced
- Typically 20-60 minutes

#### Time Trial (TT)
- No draft benefit
- Individual effort
- Usually shorter
- Pure power test

#### Group Ride
- Social pace
- Leader controlled
- May have "race" segments
- Categories optional

#### Group Workout
- Structured training
- ERG mode common
- No racing
- Power targets

### Special Concepts

#### Lead-in Distance
- Distance before start/finish banner
- Not counted in lap distance
- Varies by event type (0.2-5.7km)
- Must be added to route distance for total

#### PowerUps
- Temporary boosts in races
- Feather (weight reduction) for climbs
- Aero boost for flats
- Draft boost for breakaways
- Strategic use important

#### Route Knowledge
- **Route ID**: Internal Zwift identifier
- **Stable**: Doesn't change with event renames
- **Discovery**: Found via ZwiftHacks, testing
- **Route 9999**: Placeholder for unknown

## Worlds

### Watopia
- Always available
- Most routes
- Varied terrain
- Alpe du Zwift (biggest climb)

### Makuri Islands
- Japan-inspired
- Technical courses
- Good for crits

### London
- Flat/rolling
- Box Hill climb
- Schedule rotation

### New York
- Central Park loops
- KOM climb
- Schedule rotation

### Paris
- Champs-Élysées
- Mostly flat
- Schedule rotation

### Richmond
- UCI Worlds course
- Technical
- Schedule rotation

### Yorkshire
- UCI Worlds course
- Varied terrain
- Schedule rotation

## Race Dynamics

### Pack Behavior
- **Sticky Draft**: Easier to stay in pack than real world
- **Blob Effect**: Large groups stay together on flats
- **Selection Points**: Hills cause splits
- **Binary State**: Either in pack or dropped

### Starting Strategy
- **Hard Start**: Most races start fast
- **Position Early**: Hard to move up later
- **Warm Up**: Critical for good start
- **Power-ups**: Grab early if possible

### Finishing
- **Sprint Module**: Often decides places
- **Bike Choice**: Aero for flat finish, light for uphill
- **Power-up Timing**: Save for final effort
- **Line Choice**: Inside line shorter

## Equipment Impact

### Bike Choice
- **Tron**: Best all-around (unlock at level 10)
- **Aero**: For flat races
- **Lightweight**: For climbing races
- **Gravel**: For mixed surface

### Wheels
- **Deep Section**: For flats
- **Lightweight**: For climbs
- **Disc**: Time trials only

### Impact on Time
- Equipment can save 30-90 seconds in a race
- More important in longer events
- Critical for competitive results

## Common Terms

- **ZwiftPower**: Third-party results tracking
- **Sandbagging**: Racing below your category
- **Weight Doping**: Entering false weight
- **Sticky Watts**: Power that holds you in draft
- **Flier**: Attack off the front
- **Autocat**: Automatic category enforcement
- **WTRL**: World Tactical Racing League
- **ZRL**: Zwift Racing League
- **Double Draft**: Events with enhanced draft effect
<!-- ============================================================ -->
<!-- FILE: ./docs/reference/ZWIFT_QUICK_REFERENCE.md -->
<!-- ============================================================ -->

# Zwift Racing Quick Reference

Lookup tables for in-race decisions. For the reasoning behind these numbers, see [Zwift Racing Tactics](ZWIFT_RACING_TACTICS.md) and [Zwift Physics](ZWIFT_PHYSICS.md).

## Group Size Decision

| Group Size | Draft | Success vs Blob | Power Needed | Best For |
|------------|-------|-----------------|--------------|----------|
| Solo | 0% | <5% | 130–150% of blob | Climbs >7%, final 1–2 km |
| 2 riders | 25% | 10% | 120–130% | Emergency bridge |
| **3–5 riders** | **~35%** | **20–30%** | **110–120%** | **Coordinated attacks** |
| 6–10 riders | ~35% | 15% | 105–115% | Selection moves |
| 20+ riders | ~35% + blob | N/A | 75–85% (good position) | Main pack |

## Terrain Strategy

| Your Strength | Flat | Rolling | Mountain | Mixed |
|---------------|------|---------|----------|-------|
| Climber | Stay in blob | 3–5 rider break | Solo on >7% | Attack terrain changes |
| All-rounder | Stay in blob | Opportunistic break | Small group | Position + timing |
| Sprinter | Blob until final | Blob, sprint from rises | Survive to sprint | Blob, save matches |
| Time trialist | 3–5 rider break | 3–5 rider break | Small group | Breakaway |

## Gap Assessment

| Gap to Blob | Solo Survival | 3–5 Riders | 6+ Riders |
|-------------|---------------|------------|-----------|
| <10 sec | Bridge hard | Bridge easily | Comfortable |
| 10–30 sec | 1–2 min | 5–10 min | Probably safe |
| >30 sec | Climb only | Often survives | Usually safe |

## Blob Position Checklist

| When | Target Position | Action |
|------|-----------------|--------|
| Steady sections | Rows 5–15 | Conserve, follow wheels |
| 30 sec before climb | Top 10 | Micro-sprints forward |
| 1 km to finish | Top 5 | Committed move up |
| 500 m to finish | Top 3 | Launch or follow |

## PowerUp Timing

| PowerUp | Save For | Use When |
|---------|----------|----------|
| Feather (-10% weight) | Steepest gradient | Decisive climb or to avoid being dropped |
| Aero Helmet | Sprint finish | Final 300–500 m |
| Draft Boost (+50%) | Bridging | When catching a group |

## Power Zone Guide

| Situation | FTP % | Sustainable |
|-----------|-------|-------------|
| Race start surge | 150–200% | 1 minute |
| Solo attack | 120–130% | 5–10 min |
| Small break (3–5) | 105–115% | 15–25 min |
| Blob (good position) | 70–85% | Full race |
| Blob (poor position) | 90–95% | Full race |

## Decision Flowchart

```
Am I in the main blob (20+ riders)?
├─ YES → Am I in the front half?
│  ├─ YES → Maintain, watch for moves
│  └─ NO → Move up NOW (micro-sprints)
└─ NO → How big is the gap?
   ├─ <10 sec → Bridge solo
   ├─ 10–30 sec → Find 2–3 allies
   └─ >30 sec → Accept it, maximise current position

Is a break forming?
├─ 2 riders → Usually ignore
├─ 3–5 riders, similar w/kg → Consider joining
└─ 6+ riders → This is a split, must respond

Should I attack?
├─ Climb >7% → Solo viable
├─ 3–5 willing riders → Coordinate
├─ Final 2 km → Solo possible
└─ Otherwise → Stay in blob
```

<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/AI_DEVELOPMENT.md -->
<!-- ============================================================ -->

# AI-Assisted Development: Building Zwift Race Finder with Claude Code

This document captures the approach, lessons, and insights from building a real-world application using AI assistance without traditional coding.

## The Team

### Human: Jack (Product Owner/Manager)
- **Domain Expertise**: Active Zwift racer who understands the problem space
- **Technical Background**: 40+ years IT experience (retired professional)
- **Role**: Define problems, provide direction, validate solutions, catch assumptions

### AI: Claude Code (Developer)
- **Technical Skills**: Implements code, integrates APIs, handles debugging
- **Transparency**: Shows reasoning and decision-making process
- **Role**: Write code, explain approaches, flag assumptions, iterate on feedback

## The Management Model

Think of it as managing a very willing and enthusiastic employee who:
- Never gets tired or frustrated
- Always explains their thinking
- Sometimes makes reasonable but wrong assumptions
- Needs clear direction and context

## Key Success Factors

### 1. Clear Problem Definition
**Started with**: "I know when and how long I want to race, but not which races will actually take that long"
**Not**: "Build me a Zwift app"

This clarity guided every decision.

### 2. Domain Knowledge is Crucial
Understanding Zwift racing meant I could:
- Spot when 100km races finishing in 60 minutes didn't make sense
- Know that draft benefit matters (~30% speed increase)
- Recognize that different categories race different distances
- Understand why elevation profiles matter

### 3. Technical Experience Helps (But Coding Isn't Required)
40 years of IT experience meant:
- Knowing when SQLite beats JSON files
- Understanding API authentication patterns
- Recognizing when to pivot approaches
- Asking the right debugging questions

### 4. Transparency Enables Quality Control
Claude showing its reasoning allowed me to:
- Catch wrong assumptions early
- Spot data/description mismatches
- Understand why certain approaches were chosen
- Redirect when heading the wrong direction

## Development Process

### 1. Initial Attempt (92.8% Error)
- Started with ZwiftPower data
- Discovered "actual times" were estimates
- Lesson: Always validate your data

### 2. Strava Integration (31.2% Error)
- Pivoted to get real race times
- Required learning OAuth flows
- Lesson: Don't be afraid to change approach

### 3. Multi-Lap Fix (25.1% Error)
- Event names misleading, API had better data
- Required understanding event_sub_groups
- Lesson: Data structure matters more than descriptions

## Practical Tips

### DO:
- **Test with real data frequently** - Assumptions will be wrong
- **Keep good documentation** - Track decisions and discoveries
- **Question surprising results** - 100km in 60 minutes?
- **Use your expertise** - Domain knowledge is your superpower
- **Think system-wide** - Consider the full workflow

### DON'T:
- **Accept magic numbers** - Ask why 25 km/h?
- **Ignore your instincts** - If something seems wrong, investigate
- **Skip testing** - Real data reveals real problems
- **Fear pivoting** - Better approaches emerge with learning

## Results That Matter

- **25.1% prediction accuracy** - Better than many v1.0 products
- **151 real races analyzed** - Data-driven, not theoretical
- **Multiple pivots** - Adapted as we learned
- **Practical tool** - Actually solves the original problem

## The Bottom Line

You don't need to be a coder to build software with AI. You need:
1. A clear problem to solve
2. Domain knowledge about that problem
3. Ability to manage and direct AI (like any employee)
4. Willingness to test and iterate

The combination of human expertise and AI capability is powerful. The human provides wisdom, context, and quality control. The AI provides implementation, integration, and transparency.

This isn't about learning to code - it's about learning to direct AI effectively to solve real problems.
<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/REFACTORING_EXPLAINED.md -->
<!-- ============================================================ -->

# Understanding Refactoring: A Human's Guide

This document explains the full scope of refactoring, why AI assistants struggle with it, and how the REFACTORING_RULES.md addresses these challenges.

## What Refactoring Really Is

According to Martin Fowler (refactoring.com), refactoring is "a disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior."

The key phrase: **"without changing its external behavior"**

## The Refactoring Catalog

Refactoring isn't just moving functions between files. Fowler's catalog includes over 60 different refactorings, grouped into categories:

### Basic Refactorings
- **Extract Function**: Pull code into a new function
- **Inline Function**: Replace function calls with the function body
- **Extract Variable**: Name a complex expression
- **Inline Variable**: Replace variable references with the expression
- **Rename Function/Variable**: Change names for clarity

### Moving Features
- **Move Function**: Relocate to a better module/class
- **Move Field**: Relocate data to a better structure
- **Move Statements into Function**: Consolidate related code
- **Move Statements to Callers**: Push code up to callers

### Organizing Data
- **Replace Primitive with Object**: int age → Age class
- **Replace Array with Object**: data[0] → data.name
- **Encapsulate Variable**: Direct access → getter/setter

### Simplifying Conditionals
- **Decompose Conditional**: Complex if → multiple functions
- **Consolidate Conditional Expression**: Multiple ifs → single if
- **Replace Nested Conditional with Guard Clauses**: Deep nesting → early returns

### Dealing with Inheritance
- **Pull Up Method/Field**: Move to parent class
- **Push Down Method/Field**: Move to subclasses
- **Replace Subclass with Delegate**: Inheritance → composition

## Why AI Assistants Fail at Refactoring

### The Core Problem: Improvement Bias

When an AI sees code to refactor, multiple trained behaviors activate:

1. **Code Review Mode**: "Is this code good? How can I improve it?"
2. **Problem Solving Mode**: "What issues can I fix?"
3. **Modernization Mode**: "Is this using current best practices?"
4. **Efficiency Mode**: "Can this be optimized?"

These are helpful 95% of the time, but catastrophic during refactoring.

### Real Examples of AI Refactoring Failures

#### Example 1: Lost Functionality During Move
```rust
// Original (handles multiple formats)
fn parse_distance_from_description(description: &Option<String>) -> Option<f64> {
    if let Some(desc) = description {
        // Parse "Distance: 10 km" format
        let distance_re = Regex::new(r"Distance:\s*(\d+(?:\.\d+)?)\s*(km|miles?)").unwrap();
        if let Some(captures) = distance_re.captures(desc) {
            let value = captures[1].parse::<f64>().ok()?;
            let unit = &captures[2];
            return Some(if unit.starts_with("mile") {
                value * 1.60934  // Convert miles to km
            } else {
                value
            });
        }
        // Fallback to simple parsing
        parse_distance_from_name(desc)
    } else {
        None
    }
}

// AI's "refactored" version (lost regex parsing and miles conversion)
fn parse_distance_from_description(description: &Option<String>) -> Option<f64> {
    description.as_ref().and_then(|desc| parse_distance_from_name(desc))
}
```

#### Example 2: Added Features During Extract
```rust
// Task: Extract the validation logic
fn process_age(age: i32) {
    if age >= 0 && age <= 150 {
        self.age = age;
    }
}

// AI's extraction (added new validation!)
fn validate_age(age: i32) -> bool {
    age >= 0 && age <= 150 && age != 13  // Added "unlucky number" check!
}
```

#### Example 3: "Fixed" Edge Cases During Rename
```rust
// Task: Rename 'get_value' to 'fetch_value'
fn get_value(key: &str) -> Option<String> {
    self.map.get(key).cloned()
}

// AI's version (added "helpful" default)
fn fetch_value(key: &str) -> Option<String> {
    self.map.get(key).cloned().or_else(|| Some(String::new()))
}
```

## How Different Refactorings Challenge AI

### Move Function (Easiest)
- **Challenge**: Temptation to "clean up" while moving
- **Solution**: Mechanical copy-delete process

### Extract Function (Moderate)
- **Challenge**: Deciding what's "better" extraction
- **Solution**: Copy exact code, no rewrites

### Rename (Moderate)
- **Challenge**: Fixing "related issues" during rename
- **Solution**: Change names ONLY, nothing else

### Extract Variable (Moderate)
- **Challenge**: Simplifying the expression
- **Solution**: Extract exactly as-is

### Change Function Declaration (Hard)
- **Challenge**: "Improving" the API
- **Solution**: Migration method with old calling new

### Replace Conditional with Polymorphism (Very Hard)
- **Challenge**: Complete restructuring
- **Solution**: Often better refused by AI

## The Mechanical Process Solution

For each refactoring type, we define mechanical steps that remove decision points:

1. **No Rewriting**: Copy code exactly, character for character
2. **No Decisions**: Follow steps mechanically
3. **No Improvements**: Even obvious ones
4. **Test Driven**: Tests pass or refactoring fails

## Understanding the Rules Document

### Critical Contract
Creates psychological commitment - this isn't normal coding, it's a special mode.

### Specific Mechanics
Each refactoring type has exact steps, like a recipe. No improvisation allowed.

### STOP Signals
Catches dangerous thoughts before they become code:
- "While I'm here..." → STOP
- "This would be better..." → STOP
- "Modern style..." → STOP

### Failure Examples
Real examples of how refactoring goes wrong, making the danger concrete.

### Recovery Protocol
When tests fail, no debugging allowed. Only revert. This prevents "fixing forward" which often changes more behavior.

## When to Use These Rules

### Good for AI-Assisted Refactoring:
- Move Function/Method
- Extract Function/Method
- Extract Variable
- Rename (with care)
- Simple inlines

### Require Human Oversight:
- Change Function Declaration
- Introduce Parameter Object
- Pull Up/Push Down Method
- Encapsulate Variable

### Better Done by Humans:
- Replace Conditional with Polymorphism
- Replace Algorithm
- Split Phase
- Any refactoring touching core logic

## How to Activate the Refactoring Rules

To ensure Claude follows the refactoring rules instead of modifying code:

### Option 1: Direct Reference (Most Reliable)
Simply mention the rules when asking for refactoring:
```
"Please refactor this code following REFACTORING_RULES.md"
```

### Option 2: Use the Magic Word with Context
When you say "refactor", Claude should recognize it as entering the contract, but you can reinforce:
```
"I need you to refactor (not rewrite) these functions to a new module"
```

### Option 3: Quote the Contract
Start your request with the contract to activate the mindset:
```
"Remember: preserve behavior EXACTLY. Now refactor..."
```

### Option 4: Specify the Refactoring Type
Use the specific names from the catalog:
```
"Perform a Move Function refactoring to move parse_* functions to parsing.rs"
"Do an Extract Function refactoring on this validation logic"
```

### Option 5: Add to Project's CLAUDE.md
For permanent activation in a project, add to CLAUDE.md:
```markdown
## Refactoring Discipline
When asked to refactor, ALWAYS follow REFACTORING_RULES.md.
See REFACTORING_EXPLAINED.md for why this matters.
```

### What Triggers the Rules

The rules should activate when Claude sees:
- The word "refactor" (vs "rewrite", "improve", "fix")
- References to the rules file
- Specific refactoring type names
- The contract language

### If Claude Starts Modifying Code

Interrupt immediately:
```
"STOP - you're changing behavior. Follow REFACTORING_RULES.md"
```

### Example of a Good Refactoring Request
```
"Please perform a Move Function refactoring to move parse_distance_from_description 
and its tests from main.rs to a new parsing.rs module. Use the mechanical 
copy-delete method from REFACTORING_RULES.md"
```

The key is being explicit that you want refactoring (structure change only) not rewriting (behavior change).

## The Paradox of AI Refactoring

AI excels at:
- Understanding code intent
- Suggesting improvements
- Finding bugs
- Modernizing patterns

But these strengths become weaknesses during refactoring, where the goal is to change NOTHING about behavior.

The solution isn't to make AI "understand" refactoring better. It's to create mechanical processes that prevent the AI from using its "intelligence" in ways that break the refactoring contract.

## Key Takeaways

1. **Refactoring is about structure, not behavior**
2. **AI's helpful nature is harmful during refactoring**
3. **Mechanical processes prevent thinking/improving**
4. **Tests are the only source of truth**
5. **Different refactorings need different mechanics**
6. **Some refactorings are too complex for AI**

## Final Thought

The irony: We're using AI's intelligence to create processes that prevent it from being intelligent. But that's exactly what safe refactoring requires - mechanical transformation without creative interpretation.

Remember Martin Fowler's wisdom: First refactor to make the change easy (this might be hard), then make the easy change. Never mix refactoring with feature changes.
<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/TESTING_PHILOSOPHY.md -->
<!-- ============================================================ -->

# Testing Philosophy

Why we test the way we do, what the research says, and the lessons that shaped our approach.

## The OCR Lesson: 234 Mutations, 0 Caught

On 2025-01-10, our OCR module had property tests, snapshot tests, integration tests, fuzz tests, and benchmarks. Mutation testing revealed **0% effectiveness** — not a single mutation was caught.

This proved that test quantity ≠ test quality, and it changed our entire approach.

## What the Research Says

### Code Coverage Is a Weak Predictor

- **Microsoft Research (2020)**: "Insignificant correlation" between coverage and post-release bugs across 100 Java projects
- **Google Research (2014)**: "Coverage is not strongly correlated with test suite effectiveness" (31,000 test suites)
- **Industry consensus**: 60–80% is the optimal coverage range. Beyond that, bug detection rate slows dramatically.

Google's guidelines: "60% acceptable, 75% commendable, 90% exemplary."

### Mutation Testing Is Better (But Imperfect)

- Mutation score correlation with fault detection: 0.6–0.8 (vs 0.3–0.5 for line coverage)
- **Google**: Uses mutation testing on 1,000+ projects — but only on changed code during review
- **Facebook**: Found >50% of mutants survived their rigorous test suite
- **Key finding**: Test suite size is a major confounding factor

### The Coupling Effect (Empirically Validated)

Tests that kill simple mutants (`+` → `-`, `&&` → `||`) also kill **99% of complex mutants**. Simple mutations are sufficient.

## Our Principles

### 1. Natural Tests Over Contrived Tests

**Natural**: Given realistic inputs, does this function produce expected outputs?
```rust
assert_eq!(format_duration(90), "01:30");  // Real use case
```

**Contrived**: Can I make this code execute?
```rust
let mock_args = MockArgs::new();
let mock_config = MockConfig::new();
// 50 lines of mock setup to test that mocks work
```

If a test takes >5 minutes to write, the code design is fighting you. Refactor first.

### 2. Coverage Grows Through Usage

```
New Feature (60–70%) → User Reports → Regression Tests → High Coverage (90%+)
```

Ship at 60–70% with quality tests. Real users find edge cases you didn't imagine. Add regression tests for actual failures. Mature features naturally reach 90%+. 100% coverage on day one = contrived tests that catch nothing.

### 3. Mutation Testing During Development, Not After

**Old**: Write all tests → Run mutations → Despair at 0%.
**New**: Write test → Mutate immediately → Fix → Repeat.

```bash
# Per function (5 min)
cargo mutants --file src/parser.rs --function parse_time --timeout 30

# Per module (30 min)
cargo mutants --file src/estimation.rs --timeout 180

# Full codebase (2–4 hours, run in background)
nohup cargo mutants --jobs 8 --timeout 180 > mutation.log 2>&1 &
```

### 4. Test What Users Experience

Track **behavioural coverage** — what the user sees — not code lines:

| Behaviour | Tested | Mutation Score |
|-----------|--------|----------------|
| Filter races by duration | ✅ | 85% |
| Handle Racing Score events | ✅ | 80% |
| Estimate unknown routes | ❌ | — |

### 5. What NOT to Unit Test

- **Simple delegators**: `fn get_name(&self) -> &str { &self.name }`
- **Type conversions**: `impl From<ConfigError> for AppError`
- **Framework boilerplate**: `#[derive(Debug, Clone, Serialize)]`
- **Logging/metrics**: `info!("Processing item: {}", id);`
- **Orchestration** (main/run): Test at integration level instead

## Red Flags: Your Tests Are Bad If…

1. **Structure-only assertions**: `assert!(result.is_some())` — survives all mutations
2. **Property tests without properties**: `let _ = parse(&s);` — just checks it doesn't panic
3. **Integration tests that test nothing**: `assert!(telemetry.speed.is_some())` — no value check
4. **The mock-everything anti-pattern**: 50 lines of mock setup, then `assert mock_db.called`
5. **The snapshot-everything anti-pattern**: Locks in current bugs as "correct"

**Fix**: Replace with concrete assertions: `assert_eq!(result, Some(42))`.

## The Test Hierarchy

| Level | Purpose | Rust Tool | When |
|-------|---------|-----------|------|
| Unit tests | Verify pure functions | `#[test]` + concrete assertions | During development |
| Property tests | Verify invariants across input space | `proptest` | After unit tests reveal complexity |
| Snapshot tests | Catch output regressions | `insta` | For complex outputs, API responses |
| Integration tests | Verify components together | `tests/` directory | After units prove individual correctness |
| E2E tests | Verify user workflows | `assert_cmd` | Define early, test throughout |

**Coverage targets**: Unit 60% → Integration 80% → E2E 95%.

## Current Project Metrics

- **MAE**: 16.6% on 125 matched races (target: <20%)
- **169 tests** passing across lib + integration + property + snapshot
- **100% natural test rate** for tested functions
- Regression test with real race data runs on every change

## References

1. Microsoft Research (2020). Code Coverage and Post-release Defects.
2. Google Research (2014). Coverage is not strongly correlated with test suite effectiveness.
3. Papadakis et al. (2018). Are Mutation Scores Correlated with Real Fault Detection?
4. Google (2018). State of Mutation Testing at Google.
5. Facebook (2021). What It Would Take to Use Mutation Testing in Industry.

<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/ZWIFT_PHYSICS_EQUATIONS.md -->
<!-- ============================================================ -->

# Zwift Physics Equations and Sources

## Overview

This document provides a comprehensive reference for the physics equations used in Zwift, with proper attribution to their sources. It distinguishes between officially documented values, community reverse-engineered formulas, and academic research.

### Key Understanding

Zwift appears to use established mathematical models from cycling physics research (primarily Martin et al. 1998) but applies its own coefficients and assumptions. The exact implementation details are proprietary, leading the community to reverse-engineer behaviors through empirical testing. Since only power output can be changed during a race (height, weight, and equipment are fixed at start), understanding these relationships is crucial for race performance.

### What We Know vs What We Assume

**Confirmed by Zwift or Testing:**
- Rolling resistance values for different surfaces (Zwift Insider tests)
- Draft savings percentages (24.7-33%)
- Environmental constants (no wind, fixed air density)
- General physics relationships (power/weight on climbs, power/CdA on flats)

**Community Reverse-Engineered:**
- CdA formula with specific coefficients
- Equipment CdA base value (0.1647)
- Height/weight impact on aerodynamics

**Unknown/Assumed:**
- Exact implementation of physics equations
- All coefficient values in Zwift's code
- How different equipment CdA values are calculated
- Future changes to the physics engine

## Power Equation (Academic Foundation)

### Martin et al. (1998) Equation
The fundamental cycling power equation from "Validation of a Mathematical Model for Road Cycling Power":

```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)ρ·CD·A·v³
```

Where:
- **P** = Power (watts)
- **M** = Mass of rider + bike (kg)
- **g** = Gravitational acceleration (9.81 m/s²)
- **v** = Velocity (m/s)
- **G** = Grade (slope percentage)
- **Crr** = Rolling resistance coefficient
- **ρ** = Air density (kg/m³)
- **CD** = Drag coefficient
- **A** = Frontal area (m²)

**Source**: Martin, J.C., Milliken, D.L., Cobb, J.E., McFadden, K.L., & Coggan, A.R. (1998). Journal of Applied Biomechanics, 14(3), 276-291.

### Power Components
- **Rolling resistance**: 10-20% of total power
- **Gravitational resistance**: 10-20% on climbs
- **Aerodynamic drag**: 56-96% of total power (largest component)

## Zwift-Specific Values

### CdA (Coefficient of Drag × Area)

#### Frontal Area Formula
```
A = 0.0276 × h^0.725 × m^0.425 + 0.1647
```

Where:
- **h** = Height in meters
- **m** = Mass in kilograms
- **0.1647** = Equipment CdA (bike + wheels)

#### Formula Origins
This formula appears to be based on the **Du Bois Body Surface Area (BSA)** formula:
- Standard Du Bois: `BSA = 0.007184 × Weight^0.425 × Height^0.725`
- The exponents (0.725, 0.425) match exactly
- The coefficient 0.0276 appears to be Zwift's scaling factor to convert BSA to frontal area

**Source**: Community reverse-engineered
- **Status**: NOT officially documented by Zwift
- **Discovery Method**: Systematic testing and data analysis by community members
- **Forum Discussions**:
  - [CdA dependency on height issue](https://forums.zwift.com/t/cda-dependency-on-height-issue/561927)
  - [How does watts/CdA work for TT in Zwift?](https://forums.zwift.com/t/how-does-watts-cda-work-for-tt-in-zwift/147046)
  - [CdA for tall riders](https://forums.zwift.com/t/cda-for-tall-riders/520042)
  - [Zwift TT tests - TrainerRoad](https://www.trainerroad.com/forum/t/zwift-tt-tests-take-2-cda-relationship-to-rider-weight-does-this-look-right/56932)

#### Height/Weight Controversy
The community has identified significant issues with this implementation:
- Taller riders face disproportionate aerodynamic penalties
- The relationship doesn't accurately reflect real-world physics
- Creates fairness concerns in competitive racing

### Rolling Resistance (Crr) Values

**Source**: Zwift Insider testing (https://zwiftinsider.com/crr/)

#### Confirmed Values
- **Road wheels on pavement**: Crr = 0.004
- **MTB wheels on pavement**: Crr = 0.009
- **Gravel wheels on dirt**: Crr = 0.018

#### Surface Penalties
- **November 2023 Update**: Road bikes get ~80W penalty on dirt surfaces
- **Dirt surfaces**: More than double the rolling resistance for road wheels

### Pack Dynamics

**Source**: Zwift Insider (https://zwiftinsider.com/road-bike-drafting-pd41/)

- **Draft benefit**: 24.7-33% power savings (position dependent)
- **"Sticky draft"**: Wattage windows for maintaining draft
- **Dynamic CdA**: 3% reduction during attacks (>20% power increase)

### Environmental Constants

**Status**: Officially implemented in Zwift

- **Air density**: Fixed at 1.225 kg/m³
- **Wind**: None (0 m/s always)
- **Temperature effects**: None
- **Altitude effects**: None on air density

## Speed Relationships

### Fundamental Relationships
- **On flats**: Speed ∝ ∛(Power/CdA)
- **On climbs**: Speed ∝ Power/Weight

### Category-Based Pack Speeds (Empirical)
Based on analysis of 151 races:
- **Cat A**: 37.5 km/h
- **Cat B**: 35.0 km/h
- **Cat C**: 32.5 km/h
- **Cat D**: 30.9 km/h

**Source**: Regression analysis of actual race data

## Empirical Testing Approach

Since Zwift's exact implementation is proprietary, the only way to understand the physics better is through empirical testing:

### Testing Variables
- **Height**: Can be adjusted in Zwift settings (though should match real life)
- **Weight**: Can be adjusted in Zwift settings (should be accurate for fair play)
- **Power**: The only variable changeable during a race
- **Equipment**: Fixed at race start (different bikes/wheels have different CdA values)

### Testing Methodology
1. **Controlled Time Trials**: Same power, different heights/weights
2. **Real-Time Monitoring**: Tools like Sauce4Zwift for live data
3. **Statistical Analysis**: Large sample sizes to account for draft variations
4. **Community Collaboration**: Shared data across multiple testers

## Fairness and Gameplay Implications

### The Height/Weight Dilemma
Zwift requires riders to use their real-world height and weight for fair competition. However, if the physics model doesn't accurately reflect reality, this creates unintended consequences:

1. **Height Penalty**: Taller riders are slower than shorter riders at the same W/kg
2. **Weight Effects**: Complex interactions between weight, CdA, and gradient
3. **Gaming the System**: Some riders may be tempted to misrepresent their dimensions

### Fixed vs Variable Factors
During a race, only power output can be changed:
- **Fixed at Start**: Height, weight, bike choice, wheel choice
- **Variable**: Power output (watts)
- **Implication**: Understanding the physics is crucial for equipment selection and pacing strategy

## Important Notes

1. **Community vs Official**: Most formulas are community-discovered, not officially documented
2. **Simplifications**: Zwift simplifies many real-world factors for gameplay balance
3. **Updates**: Values may change with game updates without notice
4. **Validation**: Community values validated through extensive empirical testing
5. **Proprietary Nature**: Zwift's exact implementation remains a trade secret

## References

### Academic Papers
- Martin et al. (1998): Base power equation
- Chung (2003): Virtual elevation method for CdA testing

### Community Resources
- Zwift Insider: Rolling resistance tests, draft analysis
- TrainerRoad Forums: CdA formula discussions
- Zwift Forums: Height/weight impact discussions

### Tools for Testing
- Sauce4Zwift: Real-time power/speed monitoring
- ZwiftPower: Historical race data analysis
- Virtual elevation method: CdA validation technique
<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/ZWIFT_PHYSICS.md -->
<!-- ============================================================ -->

# Zwift Physics

How Zwift models cycling physics, where it simplifies reality, and what the academic research says.

## The Foundation: Martin et al. (1998)

Zwift's physics derive from the standard cycling power equation:

```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)·ρ·CD·A·v³
```

| Symbol | Meaning | Zwift treatment |
|--------|---------|-----------------|
| P | Power (watts) | Measured from trainer |
| M | Rider + bike mass (kg) | User-entered weight + bike |
| g | Gravity (9.81 m/s²) | Standard |
| v | Velocity (m/s) | Calculated |
| G | Gradient (%) | From route data; **halved on descents** |
| Crr | Rolling resistance | Fixed per surface type |
| ρ | Air density (kg/m³) | Fixed at 1.225 — no altitude/temperature effects |
| CD·A | Drag coefficient × area | Simplified formula per equipment |

## Zwift's Simplifications

### What's Removed
- **Wind** — no crosswinds, headwinds, or echelons
- **Temperature and altitude** — air density is constant
- **Bearing friction and drivetrain losses** — not modelled
- **Position changes** — CdA is fixed per equipment choice
- **Fatigue** — repeated efforts don't compound

### What's Modified
- **Descent gradients halved** — an 8% descent feels like 4%
- **Braking removed in races** — groups stay together on descents
- **Binary draft** — full benefit or nothing (no gradual falloff)
- **Sticky draft** — wattage windows keep you attached

## Aerodynamics (CdA)

### Frontal Area Formula (Community Reverse-Engineered)

```
A = 0.0276 × h^0.725 × m^0.425 + 0.1647
```

- **h** = height (metres), **m** = mass (kg)
- **0.1647** = equipment CdA (bike + wheels)
- Exponents match the Du Bois body surface area formula
- **Not officially documented** — discovered through systematic community testing

**Controversy**: Taller riders face disproportionate aerodynamic penalties that may not reflect reality.

### Speed Relationships
- **Flats**: Speed ∝ ∛(Power / CdA) — aerodynamics dominate
- **Climbs**: Speed ∝ Power / Weight — w/kg is king

## Rolling Resistance (Crr)

Source: [Zwift Insider testing](https://zwiftinsider.com/crr/)

| Surface | Crr | Notes |
|---------|-----|-------|
| Road wheels on pavement | 0.004 | Standard |
| MTB wheels on pavement | 0.009 | 2.25× penalty |
| Gravel wheels on dirt | 0.018 | 4.5× penalty |
| Road bike on dirt | ~0.004 + 80W penalty | November 2023 update |

## Draft Physics

### Zwift's Model vs Real World

| Aspect | Real world | Zwift |
|--------|-----------|-------|
| Behind one rider | ~30% savings | 25% savings |
| Optimal peloton position | 90–95% drag reduction | ~35% maximum |
| Draft falloff | Gradual with distance | Binary (in or out) |
| Crosswind effect | Echelons form | No crosswinds |
| Uphill draft | Reduced but present | Still 10–15% |

### Academic Research on Real Pelotons

**Blocken et al. (2018)** — CFD study of 121-rider peloton:
- Riders in rows 12–14 experience only 5–10% of solo rider drag
- Nearly 3 billion computational cells used
- Optimal position provides 90–95% drag reduction

**Olds (Sports Engineering)** — Group velocity modelling:
- Speed increases rapidly up to 5–6 riders, then gradually to ~20
- Diminishing returns beyond 20 riders
- Real-world optimal breakaway: 5–7 riders

**Swiss Side wind tunnel** — Distance effects:
- 10 cm gap: 65% drag reduction
- 2.64 m gap: 48% reduction
- 10 m gap: 23% reduction
- Benefits measurable up to 50 m

**Sports Engineering (2021)** — Uphill drafting:
- 7.5% gradient at 6 m/s: 7% power savings
- 7.5% gradient at 8 m/s: 12% power savings

### Zwift's "Blob Effect"
Academic models don't predict the blob effect because it emerges from Zwift-specific mechanics:
- Continuous churn (no fatigue for front riders)
- No team blocking
- Perfect efficiency in position changes
- Result: 20+ rider groups go 2–3 km/h faster than equivalent real groups

## Environmental Constants

| Parameter | Value | Notes |
|-----------|-------|-------|
| Air density | 1.225 kg/m³ | Fixed, no variation |
| Wind speed | 0 m/s | Always |
| Gravity | 9.81 m/s² | Standard |
| Temperature effects | None | Not modelled |
| Altitude effects on air | None | Not modelled |

## Category-Based Pack Speeds (Empirical)

Calibrated from 151 real races:

| Category | Score Range | Pack Speed (km/h) |
|----------|-------------|-------------------|
| E | 0–99 | 28.0 |
| D | 100–199 | 30.9 |
| C | 200–299 | 33.0 |
| B | 300–399 | 37.0 |
| A | 400–599 | 42.0 |
| A++ | 600+ | 45.0 |

These speeds already include average draft benefit — they're empirical, not derived from physics models.

## What Matters for Performance

### Weight (kg)
- w/kg is the primary determinant of climbing speed
- Heavier riders have an advantage on flats (higher absolute power)
- At 86 kg vs 70 kg typical, major climb disadvantage

### Height (m)
- Increases frontal area → more drag
- Taller riders get less benefit from the draft
- Affects flats and descents more than climbs

### The Only Variable in a Race
Once a race starts, you control exactly **one thing**: power output. Height, weight, equipment, and physics are all fixed. Understanding the physics lets you spend those watts wisely.

## References

1. Martin, J.C., et al. (1998). "Validation of a Mathematical Model for Road Cycling Power." Journal of Applied Biomechanics, 14(3), 276–291.
2. Blocken, B., et al. (2018). "Aerodynamic drag in cycling pelotons." Journal of Wind Engineering and Industrial Aerodynamics.
3. Olds, T. (2018). "Optimizing the breakaway position in cycle races." Sports Engineering.
4. Swiss Side (2023). Wind tunnel testing of cycling aerodynamics and drafting effects.
5. Zwift Insider testing articles on rolling resistance, pack dynamics, and draft measurements.
6. Zwift community forum discussions on CdA formula and height/weight effects.

<!-- ============================================================ -->
<!-- FILE: ./docs/explanation/ZWIFT_RACING_TACTICS.md -->
<!-- ============================================================ -->

# Zwift Racing Tactics

How Zwift racing differs from road racing, and how to use those differences to your advantage.

## The Fundamental Shift

> "Virtual racing is just like real life racing at 3X speed. All the same tactics and strategies work, but it compresses time, and everything speeds up in your head."

Zwift is a cycling-themed game, not a cycling simulator. Accepting this is liberating — success comes from mastering its actual rules, not wishing they were different.

## What You Control

- **During a race**: Only your power output
- **Before a race**: Equipment selection (within level limits), warm-up
- **Always**: Tactical decisions, race selection, positioning

Everything else — physics, draft model, terrain — is fixed. Mastering the rules means optimising the one variable you have.

## What Transfers from Road Racing

- **Power-to-weight** still determines climbing speed
- **Positioning** still saves energy (even more critical in Zwift)
- **Energy conservation** still matters — save matches for key moments
- **Race reading** — watch w/kg numbers instead of body language

## What Doesn't Transfer

- **No team tactics** — no domestiques, no blocking, no lead-outs
- **No technical skills** — no cornering, bike handling, or echelons
- **No environmental factors** — no wind, rain, or road furniture
- **No equipment changes** — bike choice is fixed at race start

## Zwift's Unique Physics

### Binary Draft Model
Unlike the gradual falloff in real cycling, Zwift's draft is binary:
- **In the draft**: Full benefit (25–35% power savings)
- **Out of the draft**: Zero benefit
- **Consequence**: Small gaps are catastrophic. You're either in or out.

### The Blob Effect
Large groups (20+ riders) travel 2–3 km/h faster than physics predicts because:
- Continuous rotation at front artificially increases speed
- No team blocking possible — can't disrupt the chase
- No fatigue model — repeated efforts don't compound
- Result: Breakaways need 110–120% of blob w/kg to survive

### Modified Terrain
- **Descent gradients halved** — 8% real feels like 4%
- **No braking in races** — removed for more realistic descents
- **Sticky draft** — can get trapped behind riders; need a sharp surge to escape

## Attack Timing — Different from Road

### Climbing Attacks
**Road**: Attack at the base when gradients kick up.
**Zwift**: Wait for the steep section mid-climb.

> "The hard ramp usually happens in the middle. That's when I like to go, when it gets really hard."

### Descent Attacks
**Road**: Attack during the descent using technical skill.
**Zwift**: Attack *before* the crest — carry speed over the top.

> "A downhill attack starts at the last little kick rolling into the crest."

### Sprint Timing
- Longer sprints possible (no fatigue model)
- Aero PowerUp is essential — "rare that a sprint is won without aero boost"
- Top 5 by 1 km to go, top 3 by 500 m

## Pack Dynamics

### Positioning Within the Blob

| Position | Effect |
|----------|--------|
| Front 3 rows | High power required, dealing with churn |
| **Rows 5–15** | **Sweet spot**: max draft, manageable surges |
| Back 25% | Risk of gaps, harder to respond to attacks |
| Edges | Less draft protection |

**Movement technique**: Micro-sprints — 2–3 hard pedal strokes at 150% FTP, then immediately return to blob pace. Avoid overshooting (sticky draft).

**Pre-climb**: Move to top 10 position 30 seconds before the climb starts.

### When to Leave the Blob

- Major climbs (>7% gradient) — blob advantage minimised
- Technical sections — dirt, tight turns fragment groups
- Late race (<20% remaining) — less time for organised chase
- 3–5 similar-strength riders willing to commit

## Breakaway Strategy

### Why 90% of Breakaways Fail
1. Blob speed advantage (2–3 km/h faster than physics)
2. No team tactics to block or disrupt chase
3. Binary draft — no gradual benefit as gap shrinks
4. Perfect information — everyone sees the gap on display

### Optimal Breakaway Size: 3–5 Riders
- Sufficient draft benefit (~35% savings in rotation)
- Manageable coordination (TTT-style pulls)
- All riders must be within 0.2 w/kg of each other

### Power Requirements

| Strategy | Power vs Blob Average |
|----------|----------------------|
| Solo attack | 130–150% |
| 3-rider break | 115–125% |
| 5-rider break | 110–120% |
| In blob (good position) | 75–85% |

### Solo Moves
Only viable on climbs >7% or in the final 1–2 km. Success rate <5%.

## Draft Savings by Position and Group Size

### By Position in Line
| Position | Power Savings |
|----------|---------------|
| Behind one rider | 25% |
| 3rd in line | 33% |
| 4th in line | 37% |
| 4+ riders deep | ~35% (plateau) |

### By Group Size
| Group Size | Draft Benefit | Best Use |
|------------|---------------|----------|
| 2 riders | 25% | Emergency bridge |
| 3–5 riders | ~35% | Coordinated breakaway |
| 6–10 riders | ~35% (churn begins) | Selection group |
| 20+ riders | ~35% (max + blob effect) | Main pack |

No confirmed "8-rider maximum" for draft calculations — benefits plateau around 35%. Uphill draft still provides 10–15% benefit even on gradients.

## Route-Specific Tactics

### Flat Routes
- Blob dominates, breakaways have <5% success
- Save everything for the sprint
- Position is everything

### Rolling Routes
- 3–5 rider breaks can work between climbs
- Watch for selection points at gradient changes
- Elastic effect — blob fragments then reforms

### Mountain Routes
- Natural selection by w/kg
- Solo efforts viable on >7% gradients
- Less blob advantage
- Small groups form organically

## Race Phase Strategy (1–2 Hour Race)

### Minutes 0–5: Survival
- Expect 150–200% FTP for the first minute
- Find a sustainable group
- Ignore early breaks

### Minutes 5–30: Settle
- Position rows 10–20
- Note strong riders
- Conserve energy

### Middle of Race: Observe
- Maintain position 5–15
- Watch for selection points
- Test legs on short efforts

### Final 20%: Commit
- Move to top 15 positions
- Follow moves matching your strength
- Use remaining PowerUps
- Empty the tank — no regrets

## PowerUp Usage

| PowerUp | Best Use | Timing |
|---------|----------|--------|
| Feather (-10% weight) | Decisive climbs | Save for the steepest pitch |
| Aero Helmet | Sprint finish | Final 300–500 m |
| Draft Boost (+50% draft) | Bridging to a group | When catching riders |

## Common Mistakes

1. **Attacking climb bases** — wait for the steep section
2. **Trying to recover on descents** — gradients halved, can't coast
3. **Gradual position changes** — binary draft needs decisive moves
4. **Ignoring PowerUps** — they're mandatory for winning
5. **Going solo on flats** — almost never works
6. **Following stronger riders' breaks** — match your w/kg, not theirs
7. **Fighting the blob** — accept the physics, use positioning instead

<!-- ============================================================ -->
<!-- FILE: ./docs/project-history/ACCURACY_TIMELINE.md -->
<!-- ============================================================ -->

# Zwift Race Finder Accuracy Timeline

## Accuracy Progression Over Development

### Initial State (Session 2025-05-25 Early)
- **92.8% mean absolute error** - Initial predictions were way off
- **Root Cause**: Comparing estimates to estimates! The "actual_minutes" in database were calculated as `distance ÷ 30 km/h`, not real race times
- **Lesson**: Need real data, not calculated estimates

### After Strava Integration (Session 2025-05-25 Mid)
- **92.8% → 31.2% error** (66% improvement!)
- **What Changed**: 
  - Integrated Strava API to get actual race times
  - Fixed base speed from 25 km/h to 30.9 km/h (based on 151 real races)
  - Fixed incorrect route distances (e.g., KISS Racing: 100km → 35km)
- **Key Insight**: Real data revealed we were using wrong base speeds

### After Multi-Lap Fix (Session 2025-05-25 Late Afternoon)
- **31.2% → 25.1% error** (Below 30% target!)
- **What Changed**:
  - Fixed multi-lap race predictions (was showing 21 min for 67-74 min races)
  - Started using event_sub_groups for per-category distances
  - Added lap detection and distance parsing
- **Key Insight**: Different categories race different distances in same event

### After Pack Dynamics Model (Session 2025-05-25 Evening)
- **25.1% → 36.9% error** (Regression!)
- **What Changed**:
  - Implemented dual-speed model with drop probability
  - Accounted for getting dropped on hills (binary state: pack vs solo)
  - Added weight penalty calculations
- **Why It Regressed**: Model became more complex but revealed inherent variance in racing

### After Route Mapping Fix (Session 2025-05-25 Night)
- **36.9% → 25.7% error** (Back below 30% target!)
- **What Changed**:
  - Fixed EVO CC races incorrectly mapped to Bell Lap (14.1km vs actual 45km)
  - Added comprehensive test suite to prevent future mapping errors
- **Key Insight**: Single route mapping error caused 11.2% accuracy degradation

## Summary

The accuracy journey shows classic software development patterns:
1. **Start with wrong assumptions** (92.8% error)
2. **Get real data** (31.2% error)
3. **Fix edge cases** (25.1% error)
4. **Add complexity that reveals new problems** (36.9% error)
5. **Fix data quality issues** (25.7% error)

### After Category Speed Calibration (2025-06-19)
- **25.7% → 17.9% error**
- **What Changed**:
  - Removed dual-speed pack/solo model (was never called in production)
  - Relied on empirical category speeds which already include draft benefit
  - Category E added at 28.0 km/h
- **Key Insight**: Simple empirical speeds outperform complex physics models

### After Climb Speed Modeling Fix (2026-03-15)
- **17.9% → 16.6% error**
- **What Changed**:
  - Fixed `estimate_duration_with_distance()` — was using name-based multiplier, ignoring elevation data in the database. Routes like "Road to Sky" (59.5 m/km) got multiplier 1.0 instead of ~0.35.
  - Replaced 5-tier step-function elevation multiplier with 9-breakpoint piecewise linear interpolation for smooth transitions
  - Added category-aware climbing penalty: lower categories (D, E) are disproportionately slower on climbs >15 m/km due to lower w/kg
  - Added route alias table mapping event-only Zwift route IDs to canonical DB route IDs (11 aliases, ~2,640 event sightings resolved)
- **Key Insight**: The "should we use rider weight?" question turned out to be a misframing. The real issue was (a) a code path ignoring elevation data entirely and (b) a multiplier that didn't go low enough for steep climbs or distinguish between categories. The w/kg effect is better captured as a category × elevation interaction than raw weight input.
- **Climbing MAE**: 43.8% → <20%
- **Flat MAE**: 18.8% → unchanged (no regression)

## Summary

The accuracy journey shows classic software development patterns:
1. **Start with wrong assumptions** (92.8% error)
2. **Get real data** (31.2% error)
3. **Fix edge cases** (25.1% error)
4. **Add complexity that reveals new problems** (36.9% error)
5. **Fix data quality issues** (25.7% error)
6. **Simplify model** (17.9% error)
7. **Fix bugs in elevation handling** (16.6% error)

Current accuracy of **16.6%** is good given:
- High inherent variance in racing (same route can vary 32-86 minutes)
- Binary nature of pack dynamics (with pack or dropped)
- Only 5 climbing data points — model will improve with more race data

The variance isn't a prediction failure — it's the nature of bike racing!
<!-- ============================================================ -->
<!-- FILE: ./docs/project-history/FEEDBACK.md -->
<!-- ============================================================ -->

# Zwift Race Finder - User Feedback

Thank you for using Zwift Race Finder! Your feedback helps improve prediction accuracy and user experience.

## How to Provide Feedback

### 1. GitHub Issues (Preferred)
Report issues or suggestions at: https://github.com/jchidley/zwift-race-finder/issues

Include:
- Your Zwift Racing Score or category
- The command you ran
- What happened vs what you expected
- Any error messages

### 2. Accuracy Feedback
Help improve predictions by recording your actual race times:

```bash
# After completing a race, record the result:
zwift-race-finder --record-result "route_id,actual_minutes,event_name"

# Example:
zwift-race-finder --record-result "3379779247,22,Stage 4: Makuri May"
```

### 3. Route Mapping
Found an unknown route? Help by:
1. Running `zwift-race-finder --show-unknown-routes`
2. Looking up the route on ZwiftHacks.com or Zwift Insider
3. Creating an issue with the route details

## Common Issues & Solutions

### "No matching events found"
- Most races are 20-30 minutes, try: `zwift-race-finder -d 30 -t 30`
- Time trials are less common, try: `zwift-race-finder -e all`
- API only shows ~12 hours of events (200 event limit)

### Inaccurate predictions
- Record your actual times to improve calibration
- Check if it's a multi-lap race (we've fixed most of these)
- Report consistently wrong predictions via GitHub

### Unknown routes
- New routes need manual mapping
- Use `--show-unknown-routes` to see what needs mapping
- Contribute route data via GitHub issues

## Current Accuracy: 16.6%
We've exceeded our <20% target! Help us get even better by sharing your race results.

## Privacy
All feedback is voluntary. Race results are stored locally and never shared without your consent.
<!-- ============================================================ -->
<!-- FILE: ./docs/project-history/HISTORICAL_DISCOVERIES.md -->
<!-- ============================================================ -->

# Historical Discoveries

## Overview

This document captures the evolution of understanding during Zwift Race Finder development, preserving key discoveries and insights that shaped the project.

## 2025-05-25: Drop Dynamics Discovery

### The Problem
Bell Lap race showed extreme variance: 32-86 minutes for the same route and category.

### Investigation Process
1. Initial hypothesis: Physics model would explain times
2. Implementation: Martin et al. (1998) cycling power model
3. Result: 127% error - completely wrong

### The Discovery
Racing isn't about steady-state physics - it's about pack dynamics:
- **Binary State**: Either with pack (fast) or dropped (slow)
- **No Middle Ground**: Transition is quick and decisive
- **Weight Penalty**: 86kg vs 70kg typical = major climb disadvantage
- **Cascade Effect**: Drop early → race mostly solo → 2-3x longer time

### Impact
- Explained 82.6% of variance with simple dual-speed model
- Pack speed: Category-based (D: 30.9 km/h)
- Solo speed: 77% of pack speed
- Weighted by drop probability

## 2025-05-26: Racing Score Event Discovery

### The Problem
Tool found traditional races but missed Racing Score events entirely.

### Investigation Process
1. User report: "Not finding races"
2. API inspection: Events exist but filtered out
3. Pattern noticed: `distanceInMeters: 0` for some events

### The Discovery
Zwift has two mutually exclusive event systems:
- **Traditional**: A/B/C/D/E with distance populated
- **Racing Score**: Score ranges with distance = 0
- **Key Signal**: `rangeAccessLabel` field presence
- **Distance Location**: Only in description text

### Solution
```rust
fn is_racing_score_event(subgroup: &EventSubgroup) -> bool {
    subgroup.distance_in_meters == 0.0 && 
    subgroup.range_access_label.is_some()
}
```

### Impact
- Tool now finds all event types
- Better UX with event type summaries
- Proper distance parsing from descriptions

## 2025-05-26: UX Enhancement Insights

### The Problem
Users getting "No events found" with no guidance.

### The Discovery
Context-aware messages dramatically improve user experience:
- Show what was searched
- Explain why no results
- Provide working examples
- Give actionable next steps

### Implementation
```
No races found matching your criteria.
Searched for: races, 120 ± 15 minutes, Category D
Try: cargo run -- -d 30 -t 30
```

### Impact
- Users understand the issue
- Self-service problem solving
- Fewer support requests

## 2025-05-27: Configuration Evolution

### The Journey
1. Started with command-line args only
2. Added config file support
3. Discovered need for multiple levels
4. Implemented hierarchical config

### The Pattern
```
Priority: CLI args > Environment > Local > User > System > Defaults
```

### Key Insight
Users need flexibility in how they configure:
- Quick overrides via CLI
- Persistent settings in config
- Wrapper scripts with env vars
- System-wide defaults

## 2025-06-02: Lead-in Distance Impact

### The Problem
Systematic underestimation of race times.

### Investigation
1. Compared actual vs predicted times
2. Noticed consistent shortfall
3. Researched route structures

### The Discovery
Lead-in distance is significant but hidden:
- Varies by event type: 0.2-5.7km
- Not included in route distance
- Different for races vs free rides
- Must be added for accurate total

### Impact
- Added lead_in_km to database schema
- Updated all calculations
- Improved accuracy significantly

## 2025-06-02: Data Source Discoveries

### WhatsOnZwift Situation
- Has permission from Zwift for data
- No public API available
- Must parse web pages
- Most comprehensive route data

### Zwift API Limitations
- Developer accounts restricted
- Public API limited
- No route database endpoint
- Must discover empirically

### Strava Integration Value
- Zwift exports rides automatically
- Contains actual route data
- Provides ground truth
- Enables regression testing

### The Strategy
Combine multiple sources:
1. zwift-data npm package for bulk import
2. Manual curation for accuracy
3. Strava for validation
4. User reports for unknowns

## API Evolution Insights

### Zero as Signal Pattern
- `distanceInMeters: 0` → check description
- `null` often means "look elsewhere"
- Empty arrays vs missing fields
- Field presence indicates type

### Browser DevTools Power
- Faster than documentation
- Shows actual behavior
- Reveals undocumented fields
- Network tab is truth

### Empirical Development
When documentation lacks:
1. Make minimal API calls
2. Inspect actual responses
3. Test edge cases
4. Document findings

## Technical Discoveries

### SQLite Performance
- Perfect for this use case
- No server overhead
- Fast local queries
- Easy backups

### Rust Advantages
- Compiler catches API changes
- Type system prevents nulls
- Fast enough for real-time
- Great error messages

### Testing Insights
- Regression tests with real data crucial
- Property tests find edge cases
- Integration tests catch API changes
- Mutation testing reveals test quality

## Lessons Learned

### On Estimation
1. Perfect accuracy impossible in racing
2. Good enough is good enough
3. Users understand variance
4. Empirical beats theoretical

### On APIs
1. Undocumented doesn't mean unavailable
2. Multiple sources improve reliability
3. Cache everything possible
4. Plan for API changes

### On User Experience
1. Clear error messages crucial
2. Examples better than explanations
3. Progressive disclosure works
4. Context-aware help valued

### On Development
1. Start simple, iterate based on data
2. Real user data beats assumptions
3. Fast feedback loops essential
4. Document discoveries immediately

## Future Considerations

### Potential Improvements
- Machine learning on race results
- Power-based personalization
- Weather integration
- Social features

### Resolved Questions
- **Why do some routes have multiple IDs?** — Zwift uses different internal route IDs for the same physical route depending on context (event-only vs free-ride). Discovered 2026-03-15 by cross-referencing the `unknown_routes` table with the zwift-offline export. Solved with `route_aliases` table (11 aliases, ~2,640 sightings resolved).
- **Should we use rider weight/height for predictions?** — No. The w/kg effect on climbs is better modeled as a category × elevation interaction. Lower categories are disproportionately slower on climbs (Cat D: 48% of flat speed on >20 m/km, Cat C: 54%). Adding category-aware climbing penalties reduced climbing MAE from 43.8% to <20% without needing weight input.

### Unresolved Questions
- How does Zwift calculate Racing Score?
- What determines lead-in distance?
- Can we predict pack dynamics better?

This historical record helps future developers understand not just what the code does, but why it evolved this way.
<!-- ============================================================ -->
<!-- FILE: ./docs/screenshots/README.md -->
<!-- ============================================================ -->

# Zwift Event Screenshots

These screenshots from the Zwift mobile app show the Makuri May event series, specifically Stage 4: Three Village Loop.

## Screenshots

### 1. makuri-may-stage4-monthly-gc-overview.jpeg
Shows the Monthly GC Competition overview for Stage 4. Explains that this is a General Classification competition where race times accumulate across all stages. Includes links to results and Racing Score FAQ. Shows category ranges from 0-159 up to 650+.

### 2. makuri-may-stage4-category-requirements.jpeg  
Details the requirements for entering the Advanced category (650+ Racing Score):
- Heart Rate Monitor required
- Zwift Racing Score of over 650
- Smart Trainer/Bike or Power Meter required
- Explains that hourly ZRacing will be split into five tighter categories

### 3. makuri-may-stage4-three-village-loop-route.jpeg
Route details for Stage 4: Three Village Loop
- 2 laps
- 23.5 km total distance
- 190m total elevation
- Shows route map with the loop highlighted
- Category 0-650 shown

### 4. makuri-may-event-description-stages.jpeg
Event description showing all 4 stages:
- Stage 1: Bridges and Boardwalks (May 5-11) - 3 laps, 20.5km, 191m
- Stage 2: Turf N Surf (May 12-18) - 1 lap, 24.7km, 198m
- Stage 3: Castle to Castle (May 19-25) - 1 lap, 23.2km, 141m
- Stage 4: Three Village Loop (May 26-June 1) - 2 laps, 23.5km, 191m

### 5. zwift-events-list-today.jpeg
Shows Jack Chidley's (Level 50) event list for Monday, including:
- 11:40 ZWC Hill Climb Battle (12.1km, 0-1000 score)
- 12:10 Stage 4: Makuri May (2 laps, 19 participants, 0-650 score)
- 12:20 Zwift TT Club Racing (1 lap, 3 participants, 0-1000 score)

### 6. makuri-may-stage4-category-schedule.jpeg
Shows scheduled Stage 4 events for different Racing Score categories:
- 270-390 Score category at 12:12
- 160-270 Score category at 12:13 (marked as "Recommended for you")
- Both showing 2 laps, with 7-8 participants signed up

### 7. makuri-may-all-stages-overview.jpeg
Complete overview of all Makuri May stages with full details:
- Stage 2: Turf N Surf - 1 lap, 24.7km, 198m elevation, PowerUps: Feather, Anvil
- Stage 3: Castle to Castle - 1 lap, 23.2km, 141m elevation, PowerUps: Draft, Aero  
- Stage 4: Three Village Loop - 2 laps, 23.5km, 191m elevation

## Key Observations

1. This is a multi-stage event running throughout May 2025
2. Events are split by Racing Score categories (e.g., 0-159, 160-269, 270-389, etc.)
3. Stage 4 (Three Village Loop) is 2 laps for a total of 23.5km with 191m elevation
4. The event uses Zwift Racing Score for category enforcement
5. Different PowerUps are available on different stages
6. All stages are in the Makuri Islands world