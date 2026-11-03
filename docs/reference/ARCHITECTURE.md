# Zwift Race Finder Architecture

## System Architecture

```
Zwift API → Select subgroup/geometry → Resolve estimate → Filter/display
                                               ↑
Selected client export → SQLite routes/aliases ─┤
Measured history export → SQLite observations ─┘

Optional ZwiftPower profile lookup/cache → rider score
```

## Engineering constraints

Keep the existing concrete CLI/library modules; SQL and row mapping belong to `database.rs` and its `database_history.rs` child. Pure numerical fitting takes explicit inputs, including its date cutoff, without opening a database or making HTTP calls. Introduce abstractions only for a demonstrated need, not to turn this CLI into a service framework.

`event_filtering::resolve_event()` owns subgroup/geometry resolution and estimate priority; `estimate_event()` is its optional adapter. Duration filtering and both presentation modes use it; display must not add another formula or lead-in. The current callers invoke it independently: this is a shared resolver, **not** a once-per-event snapshot of time and database state. Passing a single immutable estimate through the pipeline remains a separate migration, not a documented guarantee.

Use `u32` route IDs, explicit units in names, optional unknown measurements, and source-qualified measured history. Preserve known zero, measured seconds, event geometry, chronology and rider boundaries. Do not infer scores or treat training residuals as uncertainty. Exact numerical rules remain in [Algorithms](ALGORITHMS.md).

For new work, follow the [selected first-party examples with tests](../code-truth/PATTERNS.md#selected-first-party-examples). They demonstrate bounded patterns, not blanket approval of their modules. Keep OCR feature-gated; do not require native OCR dependencies for the default CLI. Keep consequential invariants in types/APIs, derive positive and negative tests from requirements, preserve unrelated work and use bounded recovery rather than broad rollback.

## Core Components

### 1. CLI and Orchestration (`main.rs`, `api.rs`, `commands.rs`, `zwiftpower.rs`)
- `main.rs`: Clap-based argument parsing, top-level filtering, and display orchestration
- `api.rs`: Zwift API client — fetch_events() from `us-or-rly101.zwift.com`
- `commands.rs`: unknown-route inspection; completion, manual recording, discovery and description-analysis commands are retired
- `zwiftpower.rs`: ZwiftPower profile scraping, user stats fetching/caching

### 2. Duration Estimation (`personal_estimation.rs`, `duration_estimation.rs`, `estimation.rs`)
- Fixed durations win. Supported ordinary races use earlier same-route pace or personal distance/ascent fitting; unsupported or insufficient history remains unknown, with no category-speed substitution.
- `personal_estimation.rs`: pure score-independent fitting with an explicit date; a database adapter qualifies provenance, rider identity, formats and canonical route IDs.
- `duration_estimation.rs`: Retained legacy pure functions — category speed lookup, difficulty multipliers (piecewise linear with category-aware climbing penalty), duration math
- `estimation.rs`: Bridge — route lookup from DB (with alias resolution), lead-in addition, connects to `duration_estimation`
- Category helpers remain for historical regression and standalone callers, not the finder duration path. Physical-stat settings/accessors are removed; legacy stored values remain unused.

### 3. Event Filtering (`event_filtering.rs`)
- Ordered estimation fallback tree (see [Algorithms](ALGORITHMS.md))
- Selects a traditional subgroup by exact label/category token, or a Racing Score subgroup by numeric range. Out-of-range scores do not select another restricted subgroup.
- Prefers positive finite subgroup distance, then positive subgroup laps with one known lead-in, then event distance and documented route/text fallbacks. Zero distance/laps are not usable geometry; explicit total distance never receives another lead-in.
- Tag-based filtering (`--tags`, `--exclude-tags`)

### 4. Event Display (`event_display.rs`)
- Compact table (default) or verbose multi-line format
- Colored output with time-until-start, distance, estimated duration
- Personal estimates remain labelled experimental; unknown-duration omissions are disclosed. Completion indicators are retired.

### 5. Database (`database.rs`, `database_history.rs`)
- SQLite at `~/.local/share/zwift-race-finder/races.db`
- Schema created automatically on first run; fresh route tables remain empty until explicit client import. Existing rows/history are retained.
- Connections enable foreign keys before transactional initialization/migration. Imports validate complete batches and commit atomically; unsupported history schemas fail rather than being rebuilt destructively.
- Active stores and preserved legacy tables:

| Table | Purpose |
|-------|---------|
| `routes` | Route data (distance, elevation, lead-in, surface, slug) |
| `race_results` | Measured seconds, optional historical scores, event geometry and source-qualified observations |
| `route_sources` | Client route source precedence and provenance |
| `unknown_routes` | Routes seen in events but not yet mapped |
| `route_aliases` | Maps event-only route IDs to canonical DB route IDs |
| `route_completion` | Legacy databases only: preserved but no longer created, read or written by the application |
| `rider_stats` | Legacy databases only; existing values preserved, no current readers/writers |
| `route_discovery_attempts` | Legacy databases only: retained historical web searches; no longer created or used |

### 6. Config (`config.rs`)
- Environment overrides the first existing file among `./config.toml`, `~/.config/zwift-race-finder/config.toml`, and `~/.local/share/zwift-race-finder/config.toml`, then defaults; files are not merged. The intended CLI-over-config contract has a known gap: duration/tolerance/days are overridden when their parsed value equals the built-in default, even if that default was explicitly supplied. See [Configuration](CONFIGURATION.md).
- Secrets from environment variables only (no credential fields in TOML)
- Defaults include score=195, category=D, duration=120min, and tolerance=30min

### 7. Route Metadata and Description Parsing
- [Client-only import](../howto/CLIENT_ROUTE_DATA.md) accepts an explicitly selected client cache and extracted client XML for missing IDs; exports stay private outside Git.
- No bundled route dataset or supported community/zwift-offline import remains.
- The [measured-history procedure](../howto/ZWIFTPOWER_EXPORT_STEPS.md) owns export, backup and preview/apply steps. Importing supported observations can change personal predictions without changing the model; [Algorithms](ALGORITHMS.md) owns the active estimate policy.
- Discovery and description-analysis commands are removed; route metadata comes from explicit client imports.
- Event-distance/elevation/lap parsing in `parsing.rs` remains supported. The route-name guessing module is removed.

## Binaries

| Binary | Purpose | Feature gate |
|--------|---------|-------------|
| `zwift-race-finder` | Main CLI | — |
| `debug_tags` | Analyze event tags from saved JSON | — |
| `import_client_routes` | Import a private client route export with provenance; preview may initialize/migrate schema | — |
| `import_zwiftpower_history` | Import measured source-qualified history; preview leaves the target unchanged | — |
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

For call relationships, persistence ownership and change propagation, see the [code-truth architecture](../code-truth/ARCHITECTURE.md). The map directs implementation work against the current diff; it is not evidence of clean release gates.

## Route ID System

- Route IDs are unsigned 32-bit integers (`u32` in Rust, `INTEGER` in SQLite)
- Values can exceed `i32` max (2^31) — appear negative when serialized as signed integers
- Validate external IDs and use checked conversions where representation changes; do not truncate or reinterpret out-of-range input
- Route names change; route IDs are stable. Always use IDs as primary key.
