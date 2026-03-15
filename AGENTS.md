# AGENTS.md

## Overview
Zwift Race Finder predicts race durations from Zwift event data and a rider's Racing Score, then filters events by target duration. Rust CLI, SQLite database, Zwift public API.

## Commands
| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Run | `cargo run --release --bin zwift-race-finder -- --zwift-score 195 --duration 30 --tolerance 15` |
| All tests | `cargo test` |
| Regression suite | `cargo test regression_test -- --nocapture` |
| Full regression (slow) | `cargo test regression_test -- --nocapture --ignored` |
| Import routes | `./scripts/import_from_zwift_offline.sh` |

## Build prerequisites
Debian/Ubuntu: `sudo apt-get install -y libssl-dev pkg-config`

## Project structure
```
src/main.rs                  # CLI entry point
src/duration_estimation.rs   # Core prediction algorithm
src/event_filtering.rs       # Event matching and filtering
src/event_display.rs         # Output formatting
src/database.rs              # SQLite (path: ~/.local/share/zwift-race-finder/races.db)
src/config.rs                # Config loading (./config.toml or ~/.config/zwift-race-finder/config.toml)
src/category.rs              # Racing Score → category mapping and speeds
src/bin/                     # 7 secondary binaries (OCR, debug, import)
data/zwift_offline_export/   # Route data (JSON)
sql/                         # Schema migrations and mappings
docs/                        # Diátaxis: tutorial/ howto/ reference/ explanation/
archive/                     # Historical docs — do not modify
```

## Multiple binaries
This crate has 8 binaries. Always use `--bin zwift-race-finder` with `cargo run`. OCR binaries require `--features ocr` and additional system deps (leptonica, tesseract).

## Boundaries
- Never change duration formula behavior without running `cargo test regression_test -- --nocapture` and comparing MAE.
- Never use route names as identifiers. Always use `route_id` (can be negative/signed).
- Never commit credentials or personal data (Strava tokens, ZwiftPower IDs).
- Do not modify files under `archive/` — they are historical records.

## Domain invariants
- Lead-in distance (0.2–5.7 km) must be included in total time calculations.
- Drop state is binary: in-pack or dropped. No intermediate states.
- Category boundaries: E=0-99, D=100-199, C=200-299, B=300-399, A=400-599, A+=600+.
- Category speeds: E=28, D=30.9, C=33, B=37, A=42 km/h. Empirical from real races, already include draft benefit. Change only with regression evidence.
- Racing Score events have `distanceInMeters: 0` in the API — distance must be parsed from the event description text.
- Two mutually exclusive event systems: Traditional (A/B/C/D/E) vs Racing Score (0–650).

## Database
- 126 routes, 125 race results from Strava (2022–2025), 75 unique routes with results.
- Current MAE: 17.4% on 125 races (target: <20%).
- Route import: `cargo run --bin import_zwift_offline_routes -- --input-dir data/zwift_offline_export` (uses app DB path automatically; ignore the `--database` flag in help — it's dead code).
- Strava import: `uv run python` script in project root (see commit history) or `tools/import/strava/`.

## Troubleshooting
| Symptom | Fix |
|---------|-----|
| Route not found | Run `--show-unknown-routes`, find `route_id` on ZwiftHacks, add to DB via SQL |
| Prediction looks wrong | Check lead-in distance and Racing Score input. Run regression suite. |
| Build fails (OpenSSL) | Install `libssl-dev pkg-config` |
| `cargo run` ambiguous binary | Add `--bin zwift-race-finder` |
| OCR build fails | Needs `--features ocr` plus leptonica/tesseract system libs |
| Regression test: "no races found" | Race results table empty. Re-import from Strava. |

## Documentation
Human-readable docs follow Diátaxis in `docs/`:
- `docs/tutorial/` — getting started
- `docs/howto/` — task guides (deployment, data import, config, secrets)
- `docs/reference/` — algorithms, architecture, database, requirements
- `docs/explanation/` — Zwift physics, racing tactics, testing philosophy
