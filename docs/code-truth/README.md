# Code-Truth Map

This commit-scoped map helps a human direct implementation work and detect misplaced changes. Human usage documentation starts at [`docs/README.md`](../README.md); operational agent rules are in [`AGENTS.md`](../../AGENTS.md).

```yaml
commit: ad3e87c19010e5b33de873a148f5f590acfbf570
short_commit: ad3e87c
branch: main
commit_date: 2026-03-16T08:00:22Z
working_tree: modified_with_untracked
```

Client-only route retirement is a working-tree update to this snapshot: no bundled
route dataset, community importers, website scraping, or automatic route seeds.
Client exports remain private outside Git; existing database rows/history remain.
Cache freshness is not verified here.

The snapshot was verified while documentation changes were already present. Re-check code before relying on it after another implementation commit.

## System shape

Zwift Race Finder is a Rust CLI and library. The main command fetches public Zwift events, selects the rider's event subgroup, estimates durations from distance/category/elevation data, filters by a requested time window, and renders a compact or verbose result. SQLite stores route knowledge, aliases, race results, rider stats and unknown routes. Legacy completion/discovery tables are preserved but no longer created or used.

Optional OCR modules and binaries extract telemetry from screenshots. Import and analysis tools support route and race-data maintenance but are not part of the normal event-search path.

## Reading order

For the normal CLI path:

1. `src/main.rs` — Clap arguments and orchestration.
2. `src/api.rs` — fixed public event endpoint and response handling.
3. `src/event_filtering.rs` — filter order and duration-match decision tree.
4. `src/estimation.rs` and `src/duration_estimation.rs` — route lookup and calculation.
5. `src/database.rs` — schema, client route imports, aliases and results.
6. `src/event_display.rs` — table/verbose presentation.
7. `src/config.rs` and `src/zwiftpower.rs` — defaults, environment secrets, optional score lookup, and cache.

Read [`ARCHITECTURE.md`](ARCHITECTURE.md) before changing boundaries and [`DECISIONS.md`](DECISIONS.md) before changing identifiers, persistence, or the duration model.

## Repository map

```text
Cargo.toml                         # Package, features, binary gates, dependencies
src/main.rs                       # Main CLI; binary-private API/command adapters
src/lib.rs                        # Public reusable module surface
src/api.rs                        # Public Zwift event HTTP request
src/commands.rs                   # Special CLI command handlers
src/config.rs                     # TOML/env configuration and secret loading
src/database.rs                   # All application SQL and schema creation
src/category.rs                   # Racing Score category and empirical speed mapping
src/duration_estimation.rs        # Pure difficulty and duration calculations
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
| Event fetch/error behaviour | `src/api.rs` | `src/errors.rs`, `tests/api_tests.rs` limitation noted in test reference |
| Event inclusion/exclusion | `src/event_filtering.rs` | `src/event_analysis.rs`, `src/parsing.rs`, property tests |
| Duration formula or category thresholds | `src/duration_estimation.rs`, `src/category.rs` | `src/estimation.rs`, filtering/display call sites, regression and snapshots |
| Lead-in or lap handling | `src/event_filtering.rs`, `src/estimation.rs` | route schema/data and regression tests |
| Route identity or aliases | `src/database.rs` | import scripts, SQL mappings, `u32` parsing tests |
| Database schema | `src/database.rs` | every query in that file, SQL migration/mapping scripts, database reference |
| Config/defaults/secrets | `src/config.rs` | `config.example.toml`, config tests, secrets how-to |
| Output columns or wording | `src/event_display.rs` | snapshots and CLI integration tests |
| Route metadata collection/import | `scripts/export_client_routes.ps1`, `src/bin/import_client_routes.rs`, `src/database.rs` | private client exports, source precedence, provenance, client import tests |
| Internal event-description parsing | `src/parsing.rs` | event-distance/elevation/lap parsing remains supported; route-name guessing and analysis/discovery commands removed |
| OCR | `src/ocr_*`, OCR binaries | `tools/ocr/`, feature-gated tests, native prerequisites |

## Test locations and hazards

The maintained test map is [`docs/reference/TEST_SUITE_SUMMARY.md`](../reference/TEST_SUITE_SUMMARY.md). Two boundaries are easy to misstate:

- `tests/api_tests.rs` uses `reqwest` against Mockito but does not call production `fetch_events()`.
- `tests/integration_tests.rs` runs Cargo recursively and can reach the live API and normal user database.

Generated snapshots and golden inputs should be reviewed, not hand-edited casually. Proptest regression files preserve minimized failures.
