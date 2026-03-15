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
