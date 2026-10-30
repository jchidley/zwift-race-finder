# Test Suite Reference

This page maps test targets to the behaviour they exercise. Exact test counts are intentionally omitted because they drift as tests are added; Cargo metadata and the source files are authoritative.

## Commands

| Scope | Command |
|---|---|
| Default-feature suite | `cargo test` |
| Library tests | `cargo test --lib` |
| Main binary tests | `cargo test --bin zwift-race-finder` |
| Accuracy regression | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| Mock HTTP tests | `cargo test --test api_tests` |
| CLI integration tests | `cargo test --test integration_tests` |
| Configuration tests | `cargo test --test config_tests` |
| Core property tests | `cargo test --test property_tests` |
| Additional properties | `cargo test --test properties_tests` |
| Output snapshots | `cargo test --test snapshot_tests` |
| OCR tests | `cargo test --features ocr` |

Some ignored tests require the owner's populated race database, OCR models/images, or explicit baseline generation. Inspect each `#[ignore]` reason before enabling it.

## In-module tests

Most reusable modules contain adjacent unit tests:

- `src/category.rs`, `src/duration_estimation.rs`, and `src/estimation.rs`: category mapping and duration calculations;
- `src/event_filtering.rs`, `src/event_analysis.rs`, and `src/parsing.rs`: event selection, subgroup selection, and text parsing;
- `src/event_display.rs` and `src/formatting.rs`: display preparation and formatting;
- `src/database.rs`: schema creation and selected database operations;
- `src/parsing.rs`: event-distance, elevation and lap parsing;
- `src/regression_test.rs`: prediction error against locally stored race results;
- OCR modules: parsing and image-processing behaviour when `ocr` is enabled.

`src/main.rs` also contains binary-private tests for orchestration helpers that are not exposed by the library.

## Integration targets

| Target | Actual boundary exercised |
|---|---|
| `tests/api_tests.rs` | `reqwest` and JSON behaviour against a local `mockito` server. These tests do **not** call production `src/api.rs::fetch_events()`. |
| `tests/integration_tests.rs` | Spawns `cargo run --bin zwift-race-finder`. Several cases can call the live Zwift API or create/use the normal user SQLite database. |
| `tests/config_tests.rs` | TOML/default behaviour and current-directory config loading. Some tests change process cwd. |
| `tests/property_tests.rs`, `tests/properties_tests.rs` | Generated inputs for parsing, formatting, filtering, and estimation invariants. |
| `tests/snapshot_tests.rs` | Historical synthetic arithmetic fixtures; their labels/IDs are not current route metadata. |
| `tests/duration_consistency_tests.rs` | Compact/verbose duration agreement with filtering, fixed-duration priority, fallback visibility, explicit-distance/lap lead-in rules, and rejection of unestimable events. |
| `tests/elevation_consistency_tests.rs` | Known zero versus unknown, exact lap plus lead-in totals, proportional distance totals, and invalid/overflow cases. |
| `tests/client_route_import_tests.rs` | Private-database client precedence, precision, Unicode/nulls, legacy identity correction, history preservation, dry-run and forced rollback. |
| `tests/export_client_routes_tests.ps1` | Standalone PowerShell 7 collector tests for ID precedence, units, missing/zero, hashes, XML safety, invalid input, and output protection. |
| `tests/golden_tests.rs` and `tests/golden/` | Baseline generation and validation; generators/validators are ignored by default. |
| `tests/ocr_*` | OCR characterization, fuzz/property, snapshot, and integration behaviour; prerequisites vary by target. |

## Important limitations

- The former `performance` and `ocr_benchmarks` Cargo targets were empty placeholders and have been removed. No Criterion benchmark suite is currently implemented. The separate `zwift_ocr_benchmark` executable remains available with the `ocr` feature.
- The inactive `tests/uom_integration_tests.rs` and `tests/behavioral/` scaffolding has been removed: it depended on an undeclared `uom-migration` feature and absent UOM implementations. The real `src/ab_testing.rs` utility and its unit tests remain.

- The mock HTTP suite verifies HTTP/JSON mechanics, not the hard-coded production endpoint or its error mapping.
- CLI integration is not hermetic: network availability and user-state files can affect it.
- Database tests do not comprehensively cover migration scripts, corruption, concurrent writers, or disk-full behaviour.
- No current automated test substitutes a local fixture endpoint for the complete fetch → filter → display workflow.
- The accuracy regression depends on local database contents; a passing result is not reproducible from the repository alone.
- OCR coverage is split between default-disabled feature code, ignored model/image tests, and Python tooling.

For task-oriented instructions, see [How to Test the Project](../howto/TESTING_GUIDE.md). Historical UOM migration coverage notes are preserved in [project history](../project-history/UOM_INTEGRATION_TEST_COVERAGE_2025.md).

## History-model verification — 2026-10-30

Current isolated default suite: **195 passed, 0 failed, 7 ignored**. This adds seven parser and six history migration/import tests to the 182-test client-only state. Browser exporter checks use synthetic data (`node tests/history_export_tests.js`). Release binaries and all-feature test compilation pass; OCR execution is not requalified. Private before/after legacy score-based regression: **13.3% MAPE on the same six records**; after import it explicitly excludes 188 unknown-score records. This is not MAE or evidence of a new predictor. See [history audit](../project-history/HISTORY_MODEL_2026-10-30.md).

## CLI simplification — 2026-10-30

After removing completion marking/filtering and description-analysis/discovery commands, the isolated default suite passes **190 tests, 0 failures, 7 ignored**. Five executions were removed: four dedicated obsolete tests and the duplicate binary-side description-parser test (the library parser and its test remain). The standalone analysis binary is removed. Existing stored completion/discovery data is preserved; no discovery-attempt table is created in fresh databases. Direct release-binary checks verify removed options are absent from help and rejected before database creation.

### Follow-up: remove completion presentation and manual recording

The subsequent cleanup removes progress/indicators and manual result entry, their database APIs and three dedicated tests. At that stage the isolated default suite passed **187 tests, 0 failures, 7 ignored**. History query and nullable-score migration coverage remain, using synthetic SQL fixtures and the supported importer rather than the removed insertion API. Fresh databases no longer create the completion table; existing tables/data are preserved.

The final associated-code sweep removes the unused-for-estimation route-name guessing module and its remaining test: **186 passed, 0 failed, 7 ignored**. The actual event-distance/elevation/lap parsers remain.
