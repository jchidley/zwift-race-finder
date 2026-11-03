# Zwift Race Finder project instructions

## Working root and scope

Run commands and Pi sessions from the canonical checkout root, `/home/jack/git/zwift-race-finder` on this machine. This root file applies repository-wide; optional OCR tooling is under `tools/ocr/` with its own setup documentation. Human usage starts at [README.md](README.md).

## Commands and validation

Use Bash on Linux/WSL with Rust/Cargo via `rustup` (stable; no declared minimum Rust version). All commands below assume the checkout root. Searches call the live API and can initialize/migrate local state; import paths are placeholders for private files, not supplied fixtures.

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
| Preview client routes (may initialize/migrate schema) | `cargo run --release --bin import_client_routes -- --input /path/to/client-routes.json --dry-run` |

Debian/Ubuntu builds require `libssl-dev` and `pkg-config`. OCR binaries additionally require `--features ocr`, Leptonica, and Tesseract. This crate has multiple binaries, so always specify `--bin zwift-race-finder` when running the main CLI through Cargo.

For code changes, run focused tests first, then the default-feature suite when its network/data prerequisites are appropriate. CI also specifies `cargo fmt -- --check` and `cargo clippy -- -D warnings`; do not auto-format unrelated changes. For documentation-only edits, check changed links, command/flag contracts, and `git diff --check`; a live search or full test suite is not required. See the [testing guide](docs/howto/TESTING_GUIDE.md) and [test boundary reference](docs/reference/TEST_SUITE_SUMMARY.md).

Use isolated `HOME`, `XDG_DATA_HOME`, `XDG_CONFIG_HOME`, and `XDG_CACHE_HOME` for tests that can write application state, retaining the real `CARGO_HOME` and `RUSTUP_HOME` so the installed toolchain remains accessible. Do not pass inherited ZwiftPower credentials into validation. The accuracy regression is the deliberate exception: it needs the same approved populated database for comparable before/after MAE; an isolated empty database does not qualify accuracy.

## Boundaries

- Never change duration-formula behaviour without running the accuracy regression and reporting the before/after MAE on the same populated data. That regression checks the retained legacy category formula; do not present it as qualification of the experimental personal estimator.
- Use `route_id` (`u32`) as route identity. Route names are display and fallback data, not identifiers; IDs can exceed `i32`.
- Route metadata comes only from selected client cache/extracted client XML; fresh databases have no route seeds. Keep client exports outside Git.
- Historical Racing Score is optional; never infer it from category, current score or other rankings. Preserve measured seconds and event distance/laps separately from base route metadata.
- Never commit credentials or personal rider data, including ZwiftPower session/profile values, Strava tokens, and private race exports.
- Do not modify `archive/`; it contains historical records. Review generated snapshots/golden data and proptest regression files rather than blindly replacing them.
- Back up the database before applying imports/schema upgrades; follow [client route import](docs/howto/CLIENT_ROUTE_DATA.md) and [history import](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md). Client-route `--dry-run` may initialize/migrate the database; measured-history `--dry-run` uses an in-memory copy and leaves the target unchanged.
- Do not run deployment, privileged setup, or real-data imports merely to validate documentation. Normal searches and unknown-route inspection can initialize the database; `--help` exits before that work.
- Preserve the active estimator unless explicitly changing it: fixed time first; supported ordinary races use earlier personal same-route pace or distance/ascent fitting; unsupported personal estimates remain unknown and are excluded from duration matches; do not substitute category speed/elevation. Score selects subgroup geometry, not personal pace. Keep chronological cutoff, source/rider boundaries and experimental accuracy qualification. See `docs/reference/ALGORITHMS.md`.

## Architecture and gotchas

Before substantive changes, read the applicable constraints in [Architecture](docs/reference/ARCHITECTURE.md) and [Algorithms](docs/reference/ALGORITHMS.md), then the relevant [code-truth map](docs/code-truth/README.md). Follow the [selected first-party examples and matching tests](docs/code-truth/PATTERNS.md#selected-first-party-examples) for adapters, pure fitting, estimate consistency and safe imports. These examples cover named patterns, not blanket approval of existing code. Surface a mismatch and propose the smallest justified alternative rather than forcing an abstraction.

- The main executable starts in `src/main.rs`; reusable modules are exported by `src/lib.rs`. Only `src/commands.rs` and `src/zwiftpower.rs` remain binary-private; API, configuration and database modules are shared through the library.
- On Linux, SQLite is created at `~/.local/share/zwift-race-finder/races.db` (subject to XDG overrides); user-stat cache is `~/.cache/zwift-race-finder/user_stats.json`. `src/database.rs` and its `src/database_history.rs` child own schema evolution and application SQL. `Database::get_route()` resolves aliases after direct route lookup.
- Lead-in distance is added for subgroup lap-count estimates and route-only estimates. Estimates based on an event-provided distance do not add lead-in.
- Configuration precedence is environment variables, then the first existing file among `./config.toml`, the user config directory, and the user data directory, then defaults; files are not merged. Secrets come only from environment variables. Optional ZwiftPower lookup can run even when `--zwift-score` is supplied.
- The Zwift event client uses one public regional endpoint and has no failover. Time trials require a declared duration; ordinary-race personal fitting does not support them.
- CLI subprocess tests run the built binary with isolated HOME/XDG state, but still call the live Zwift API. Other database/library tests need isolated HOME/XDG state too; the complete suite is not hermetic.

## Documentation ownership

- `STATUS.md`: current work and verified blockers.
- `TASKS.md`: unresolved work and deferred improvements.
- `docs/README.md`: human documentation map and governance.
- `docs/code-truth/`: commit-scoped implementation map for directing changes.
- `docs/reference/`: stable current facts; `docs/project-history/` and `archive/` are historical evidence.

Keep volatile counts and current work out of this file. Verify implementation details against code and the code-truth snapshot before changing them.
