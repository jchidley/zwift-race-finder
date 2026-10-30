# Zwift Race Finder Architecture

## System Architecture

```
Zwift API → Filter Events → Estimate Duration → Display Results
     ↓                            ↑
ZwiftPower → Import → SQLite → Route Data
```

## Core Components

### 1. CLI and Orchestration (`main.rs`, `api.rs`, `commands.rs`, `zwiftpower.rs`)
- `main.rs`: Clap-based argument parsing, top-level filtering, and display orchestration
- `api.rs`: Zwift API client — fetch_events() from `us-or-rly101.zwift.com`
- `commands.rs`: CLI subcommand handlers (show-unknown, retired discover error, record-result, mark-complete, show-progress, analyze-descriptions)
- `zwiftpower.rs`: ZwiftPower profile scraping, user stats fetching/caching

### 2. Duration Estimation (`duration_estimation.rs`, `estimation.rs`)
- `duration_estimation.rs`: Pure functions — category speed lookup, difficulty multipliers (piecewise linear with category-aware climbing penalty), duration math
- `estimation.rs`: Bridge — route lookup from DB (with alias resolution), lead-in addition, connects to `duration_estimation`
- Category speed and elevation are the rider/route inputs. Weight/FTP are stored but **not used** directly — the weight effect is captured through category × elevation interaction.

### 3. Event Filtering (`event_filtering.rs`)
- Ordered estimation fallback tree (see [Algorithms](ALGORITHMS.md))
- Handles both Traditional (A/B/C/D) and Racing Score (0–650) events
- Racing Score events have `distanceInMeters: 0` — distance parsed from description text
- Tag-based filtering (`--tags`, `--exclude-tags`)

### 4. Event Display (`event_display.rs`)
- Compact table (default) or verbose multi-line format
- Colored output with time-until-start, distance, estimated duration
- Route completion indicators

### 5. Database (`database.rs`)
- SQLite at `~/.local/share/zwift-race-finder/races.db`
- Schema created automatically on first run; fresh route tables remain empty until explicit client import. Existing rows/history are retained.
- 7 tables:

| Table | Purpose |
|-------|---------|
| `routes` | Route data (distance, elevation, lead-in, surface, slug) |
| `race_results` | Actual race times for regression testing |
| `unknown_routes` | Routes seen in events but not yet mapped |
| `route_aliases` | Maps event-only route IDs to canonical DB route IDs |
| `route_completion` | Legacy databases only: preserved but no longer created, read or written by the application |
| `rider_stats` | Height, weight, FTP (stored but not used in estimation) |
| `route_discovery_attempts` | Legacy databases only: retained historical web searches; no longer created or used |

### 6. Config (`config.rs`)
- Config loading priority: `./config.toml` → `~/.config/zwift-race-finder/config.toml` → `~/.local/share/zwift-race-finder/config.toml` → defaults
- Secrets from environment variables only (no credential fields in TOML)
- Defaults include score=195, category=D, duration=120min, and tolerance=30min

### 7. Route Metadata and Description Parsing
- [Client-only import](../howto/CLIENT_ROUTE_DATA.md) accepts an explicitly selected client cache and extracted client XML for missing IDs; exports stay private outside Git.
- No bundled route dataset or supported community/zwift-offline import remains.
- Discovery and description-analysis commands are removed; route metadata comes from explicit client imports.
- Event-distance/elevation/lap parsing in `parsing.rs` remains supported. The route-name guessing module is removed.

## Binaries

| Binary | Purpose | Feature gate |
|--------|---------|-------------|
| `zwift-race-finder` | Main CLI | — |
| `debug_tags` | Analyze event tags from saved JSON | — |
| `import_client_routes` | Import a private client route export with provenance | — |
| `test_ocr` | OCR testing | `ocr` |
| `debug_ocr` | OCR debugging | `ocr` |
| `zwift_ocr_benchmark` | OCR benchmarking | `ocr` |
| `zwift_ocr_compact` | Compact OCR extraction | `ocr` |

## API Integration

### Zwift Public API
- URL: `https://us-or-rly101.zwift.com/api/public/events/upcoming`
- No authentication required
- The implementation assumes roughly 200 events/~12 hours; this external behaviour is not a tested contract

### ZwiftPower
- Profile scraping via environment variable credentials
- Provides Racing Score auto-detection
- Script: `scripts/refresh_zwiftpower_stats.sh`

## Implementation detail

For call relationships, persistence ownership, implicit contracts, and change propagation at checkpoint `ad3e87c`, see the [code-truth architecture](../code-truth/ARCHITECTURE.md).

## Route ID System

- Route IDs are unsigned 32-bit integers (`u32` in Rust, `INTEGER` in SQLite)
- Values can exceed `i32` max (2^31) — appear negative when serialized as signed integers
- Import code uses `i64` for deserialization, then casts to `u32`
- Route names change; route IDs are stable. Always use IDs as primary key.
