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
src/main.rs                  # CLI entry point, filter_events, 26 unit tests (629 prod + 888 test)
src/api.rs                   # Zwift API client — fetch_events()
src/commands.rs              # CLI subcommand handlers (show-unknown, discover, record-result, etc.)
src/zwiftpower.rs            # ZwiftPower stats fetching
src/duration_estimation.rs   # Core prediction: distance / (category_speed * difficulty_multiplier)
src/event_filtering.rs       # Event matching and filtering (292 prod + 541 test)
src/event_display.rs         # Output formatting (804 prod + 475 test)
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
- Total tests: 145 passing (86 lib + 36 bin + 12 integration + 11 property)
- Source: 37 files, ~11,660 LOC

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

## Assessed and deliberately deferred (overhaul 2026-03-15)
These were evaluated during a code-overhaul and deliberately left unchanged:
- **DRY: route name extraction** — `event.route.as_deref().unwrap_or(&event.name)` repeated 10× across event_display.rs and event_filtering.rs. Extraction would add indirection for a single expression — not worth the risk.
- **DRY: distance conversion** — `dist_m / METERS_PER_KILOMETER` repeated 10× across same files. Same assessment.
- **DRY: duration estimation pattern** — `estimate_duration_for_category(distance_km, route_name, zwift_score)` called 13× with similar setup. Each call site has different branching context — extraction would create a complex helper.
- **Monolith: event_display.rs** (804 prod LOC) — large but cohesive (all display logic). Tests constrain splitting. Not blocking progress.
- **Monolith: database.rs** (868 prod LOC) — 22 pub methods, all SQLite CRUD. Cohesive single concern. Not blocking progress.
- **God functions: prepare_event_row, event_matches_duration** — complex but well-tested. Decomposition would increase cross-function context for minimal clarity gain.

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
