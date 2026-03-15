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
The production algorithm is `estimate_duration_for_category`: `distance_km / (category_speed * difficulty_multiplier)`. Category speeds: E=28, D=30.9, C=33, B=37, A=42, A++=45 km/h. These are empirical from real races and already include draft benefit.

Lead-in distance is added to all route-based estimates (`estimate_duration_from_route_id`) and multi-lap calculations.

## Current metrics
- MAE: 17.9% on 125 matched races (target: <20%)
- Routes in DB: 126
- Total tests: 169 passing (across lib + 7 integration test files)

## Troubleshooting
| Symptom | Fix |
|---------|-----|
| Route not found | Run `--show-unknown-routes`, find `route_id` on ZwiftHacks, update route data |
| Prediction looks wrong | Check lead-in distance and Racing Score input. Run regression test. |
| Build fails (OpenSSL) | Install `libssl-dev pkg-config` |
| `cargo run` ambiguous binary | Add `--bin zwift-race-finder` |
| OCR build fails | Needs `--features ocr` plus leptonica/tesseract system libs |

## Documentation
Human-readable docs follow Diátaxis in `docs/`:
- `docs/tutorial/` — getting started
- `docs/howto/` — task guides (deployment, data import, config, secrets)
- `docs/reference/` — algorithms, architecture, database, requirements
- `docs/explanation/` — Zwift physics, racing tactics, testing philosophy
