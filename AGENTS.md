# Zwift Race Finder project instructions

## Commands

| Task | Command |
|---|---|
| Build | `cargo build --release` |
| Run | `cargo run --release --bin zwift-race-finder -- --zwift-score 195 --duration 30 --tolerance 15` |
| All default-feature tests | `cargo test` |
| Accuracy regression | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| API tests | `cargo test --test api_tests` |
| CLI integration tests | `cargo test --test integration_tests` |
| Property tests | `cargo test --test property_tests` |
| Show unknown routes | `cargo run --release --bin zwift-race-finder -- --show-unknown-routes` |
| Import measured history | `cargo run --release --bin import_zwiftpower_history -- --input /path/to/history.json --dry-run` |
| Import client routes | `cargo run --release --bin import_client_routes -- --input /path/to/client-routes.json` |

Debian/Ubuntu builds require `libssl-dev` and `pkg-config`. OCR binaries additionally require `--features ocr`, Leptonica, and Tesseract. This crate has multiple binaries, so always specify `--bin zwift-race-finder` when running the main CLI through Cargo.

## Boundaries

- Never change duration-formula behaviour without running the accuracy regression and reporting the before/after MAE.
- Use `route_id` (`u32`) as route identity. Route names are display and fallback data, not identifiers; IDs can exceed `i32`.
- Route metadata comes only from selected client cache/extracted client XML; fresh databases have no route seeds. Keep client exports outside Git.
- Historical Racing Score is optional; never infer it from category, current score or other rankings. Preserve measured seconds and event distance/laps separately from base route metadata.
- Never commit credentials or personal rider data, including ZwiftPower session/profile values, Strava tokens, and private race exports.
- Do not modify `archive/`; it contains historical records.
- Preserve the production model unless the task explicitly changes it: distance divided by category speed and an elevation/category difficulty multiplier. See `docs/reference/ALGORITHMS.md`.

## Architecture and gotchas

- The main executable starts in `src/main.rs`; reusable modules are exported by `src/lib.rs`. Binary-private adapters (`src/api.rs`, `src/commands.rs`, `src/zwiftpower.rs`) coexist with library modules.
- SQLite is created at `~/.local/share/zwift-race-finder/races.db`. `src/database.rs` owns schema creation and SQL. `Database::get_route()` resolves aliases after direct route lookup.
- Lead-in distance is added for subgroup lap-count estimates and route-only estimates. Estimates based on an event-provided distance do not add lead-in.
- Configuration precedence is environment variables, `./config.toml`, the user config directory, the user data directory, then defaults. Secrets come only from environment variables.
- The Zwift event client uses one public regional endpoint and has no failover. Time trials currently use draft-inclusive race speeds.
- `tests/integration_tests.rs` invokes Cargo recursively, can call the live Zwift API, and can create/use the normal user database; it is not an isolated hermetic CLI suite.

## Documentation ownership

- `STATUS.md`: current work and verified blockers.
- `TASKS.md`: unresolved work and deferred improvements.
- `docs/README.md`: human documentation map and governance.
- `docs/code-truth/`: commit-scoped implementation map for directing changes.
- `docs/reference/`: stable current facts; `docs/project-history/` and `archive/` are historical evidence.

Keep volatile counts and current work out of this file. Verify implementation details against code and the code-truth snapshot before changing them.
