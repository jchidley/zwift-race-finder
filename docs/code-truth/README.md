# Code-Truth Map

This commit-scoped map helps a human direct implementation work and detect misplaced changes. Human usage documentation starts at [`docs/README.md`](../README.md); operational agent rules are in [`AGENTS.md`](../../AGENTS.md).

Behavioural baseline: `v0.3.0` (`05edb8a`). The cleanup removes
comments and obsolete documentation, not model behaviour. This map directs changes;
it is not a claim of a clean release or a substitute for inspecting the current diff.

## System shape

Zwift Race Finder is a Rust CLI and library. The main command fetches public Zwift events, selects the rider's event subgroup, resolves fixed durations, earlier personal history or unknown duration without category-speed substitution, filters by a requested time window, and renders a compact or verbose result. SQLite stores route knowledge, aliases, race results and unknown routes. Legacy completion/rider-stat/discovery tables are preserved but no longer created or used.

Optional OCR modules and binaries extract telemetry from screenshots. Import and analysis tools support route and race-data maintenance but are not part of the normal event-search path.

## Reading order

For the normal CLI path:

1. `src/main.rs` — Clap arguments and orchestration.
2. `src/api.rs` — fixed public event endpoint and response handling.
3. `src/event_filtering.rs` — filter order and duration-match decision tree.
4. `src/personal_estimation.rs`, `src/estimation.rs` and `src/duration_estimation.rs` — personal fitting, route bridge and retained legacy formula helpers.
5. `src/database.rs` and `src/database_history.rs` — schema, imports, aliases and measured observations.
6. `src/event_display.rs` — table/verbose presentation.
7. `src/config.rs` and `src/zwiftpower.rs` — defaults, environment secrets, optional score lookup, and cache.

Read the durable [architecture constraints](../reference/ARCHITECTURE.md) before
changing boundaries, then this map's [`ARCHITECTURE.md`](ARCHITECTURE.md) and
[`DECISIONS.md`](DECISIONS.md) for observed implementation and recorded decisions.
Follow the [selected examples and matching tests](PATTERNS.md#selected-first-party-examples)
for the affected pattern; current code is not automatically exemplary.

## Repository map

```text
Cargo.toml                         # Package, features, binary gates, dependencies
src/main.rs                       # Main CLI; binary-private command/ZwiftPower adapters
src/lib.rs                        # Public reusable module surface
src/api.rs                        # Public Zwift event HTTP request
src/commands.rs                   # Special CLI command handlers
src/config.rs                     # TOML/env configuration and secret loading
src/database.rs                   # Database API, schema, route/alias SQL
src/database_history.rs           # Private child: measured-history SQL/migration/import
src/category.rs                   # Racing Score category and empirical speed mapping
src/duration_estimation.rs        # Retained standalone category difficulty/duration calculations
src/personal_estimation.rs        # Earlier measured history → score-independent personal duration
src/estimation.rs                 # Database route lookup → calculation bridge
src/event_filtering.rs            # Event filters and estimation fallback order
src/event_display.rs              # Compact and verbose output
src/models.rs                     # API/domain data structures
src/parsing.rs                    # Description, distance, lap, and URL parsing
src/cache.rs                      # 24-hour user-stat JSON cache
src/zwiftpower.rs                 # Optional profile scraping and fallback stats
src/ocr_*.rs                      # Optional OCR implementation (`ocr` feature)
src/bin/                          # Secondary analysis, import, debug, and OCR binaries
sql/                              # Operator-run mappings and migration SQL
src/bin/import_client_routes.rs   # Explicit private client-export import
scripts/                          # Supported import/refresh entrypoints
tools/                            # Data, OCR, debugging, and maintenance utilities
tests/                            # Integration/property/snapshot/golden/OCR targets
docs/                             # Human docs, code-truth snapshot, and history
archive/                          # Historical evidence; do not modify
```

## Change-to-location map

| Desired change | Start here | Also inspect |
|---|---|---|
| CLI flag or command dispatch | `src/main.rs` | `src/commands.rs`, CLI integration tests, reference docs |
| Event fetch/error behaviour | `src/api.rs` | `src/errors.rs`, production-adapter loopback tests in `tests/api_tests.rs`; live endpoint remains separate |
| Event inclusion/exclusion | `src/event_filtering.rs` | `src/event_analysis.rs`, `src/parsing.rs`, property tests |
| Personal duration/history fit | `src/personal_estimation.rs` | `src/event_filtering.rs`, history storage, display, personal integration tests |
| Retained category formula or thresholds | `src/duration_estimation.rs`, `src/category.rs` | `src/estimation.rs`, filtering/display call sites, regression and snapshots |
| Lead-in or lap handling | `src/event_filtering.rs`, `src/estimation.rs` | route schema/data and regression tests |
| Route identity or aliases | `src/database.rs` | import scripts, SQL mappings, `u32` parsing tests |
| Database schema | `src/database.rs`, `src/database_history.rs` | affected queries/row mapping, preservation/rollback tests, database reference; initialization already migrates supported schemas |
| Config/defaults/secrets | `src/config.rs` | `config.example.toml`, config tests, secrets how-to |
| Output columns or wording | `src/event_display.rs` | snapshots and CLI integration tests |
| Route metadata collection/import | `scripts/export_client_routes.ps1`, `src/bin/import_client_routes.rs`, `src/database.rs` | private client exports, source precedence, provenance, client import tests |
| Internal event-description parsing | `src/parsing.rs` | event-distance/elevation/lap parsing remains supported; route-name guessing and analysis/discovery commands removed |
| OCR | `src/ocr_*`, OCR binaries | `tools/ocr/`, feature-gated tests, native prerequisites |

## Test locations and hazards

The maintained test map is [`docs/reference/TEST_SUITE_SUMMARY.md`](../reference/TEST_SUITE_SUMMARY.md). Two boundaries are easy to misstate:

- `tests/api_tests.rs` calls the production `fetch_events_from_url()` adapter against loopback Mockito responses; the normal CLI retains its fixed endpoint.
- `tests/integration_tests.rs` runs the built binary with isolated HOME/XDG state; several cases still reach the live API.

Generated snapshots and golden inputs should be reviewed, not hand-edited casually. Proptest regression files preserve minimized failures.
