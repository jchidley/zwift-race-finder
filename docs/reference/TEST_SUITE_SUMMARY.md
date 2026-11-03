# Test Suite Reference

This page maps test targets to the behaviour they exercise. Exact test counts are intentionally omitted because they drift as tests are added; Cargo metadata and the source files are authoritative.

## Commands

| Scope | Command |
|---|---|
| Default-feature suite | `cargo test` |
| Library tests | `cargo test --lib` |
| Main binary tests | `cargo test --bin zwift-race-finder` |
| Category-fallback accuracy regression (approved populated data) | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| Production HTTP adapter with loopback fixtures | `cargo test --test api_tests` |
| CLI integration tests | `cargo test --test integration_tests` |
| Configuration tests (process-cwd mutation; run serially) | `cargo test --test config_tests` |
| Core property tests | `cargo test --test property_tests` |
| Additional properties | `cargo test --test properties_tests` |
| Output snapshots | `cargo test --test snapshot_tests` |
| OCR tests | `cargo test --features ocr` |

These are target selectors, not safe-state wrappers or verified passes. Follow the
[testing guide's isolation procedure](../howto/TESTING_GUIDE.md#isolate-writable-application-state)
for writable tests, preserving Cargo/Rustup homes and excluding inherited secrets.
Use serial test execution for targets that mutate cwd/environment. Some ignored
tests require populated data, OCR models/images, or explicit baseline generation;
inspect each reason before enabling it. The focused accuracy test is **not ignored**
and can pass on empty data.

## In-module tests

Most reusable modules contain adjacent unit tests:

- `src/category.rs`, `src/duration_estimation.rs`, and `src/estimation.rs`: category mapping and fallback duration calculations;
- `src/personal_estimation.rs`: score-independent personal history fit, chronology, support thresholds and explicit fallback; live integration coverage checks filter/display use of personal minutes;
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
| `tests/api_tests.rs` | Production API adapter against loopback `mockito`: typed event/subgroup decoding, unsigned route IDs, optional fields, malformed payloads, HTTP errors and real tag helpers. |
| `tests/integration_tests.rs` | Runs the built binary with isolated HOME/XDG and no inherited secrets. Several cases still call the live Zwift API. |
| `tests/config_tests.rs` | TOML/default behaviour and current-directory config loading. Some tests change process cwd; subprocess migration is not implemented. |
| `tests/property_tests.rs`, `tests/properties_tests.rs` | Generated inputs for parsing, formatting, filtering, and estimation invariants. |
| `tests/snapshot_tests.rs` | Historical synthetic arithmetic fixtures; their labels/IDs are not current route metadata. |
| `tests/duration_consistency_tests.rs` | Compact/verbose duration agreement with filtering, fixed-duration priority, fallback visibility, explicit-distance/lap lead-in rules, and rejection of unestimable events. |
| `tests/personal_estimation_tests.rs` | Isolated measured-history import → live personal estimate → filter/compact/verbose agreement; chronology, aliases, score independence, formats/provenance, mixed-rider rejection, fixed durations and explicit fallback. |
| `tests/elevation_consistency_tests.rs` | Known zero versus unknown, exact lap plus lead-in totals, proportional distance totals, and invalid/overflow cases. |
| `tests/client_route_import_tests.rs` | Temporary synthetic-database client precedence, precision, Unicode/nulls, legacy identity correction, history preservation, preview and forced rollback. Preview may initialize/migrate schema. |
| `tests/history_import_tests.rs` | Synthetic legacy migration, nullable scores, source-qualified reconciliation, preservation, conflict rejection, forced rollback, repeatability and byte-for-byte unchanged target after preview. |
| `tests/export_client_routes_tests.ps1` | Standalone PowerShell 7 collector tests for ID precedence, units, missing/zero, hashes, XML safety, invalid input, and output protection. |
| `tests/golden_tests.rs` and `tests/golden/` | Baseline generation and validation; generators/validators are ignored by default. |
| `tests/ocr_*` | OCR characterization, fuzz/property, snapshot, and integration behaviour; prerequisites vary by target. |

## Important limitations

- No default Criterion benchmark suite is implemented. The optional
  `zwift_ocr_benchmark` executable is a separate OCR diagnostic.

- The loopback HTTP suite exercises production request/decoding and HTTP error handling, not availability or compatibility of the hard-coded live endpoint.
- CLI integration is not hermetic: several cases call the live API. Subprocess state is isolated, but in-process default-path consumers still require outer isolation.
- Some config/cache tests mutate process-global cwd/environment; isolation from user files does not establish parallel-test safety.
- Database tests do not comprehensively cover migration scripts, corruption, concurrent writers, or disk-full behaviour.
- No current automated test substitutes a local fixture endpoint for the complete fetch → filter → display workflow.
- The accuracy regression depends on local database contents and checks the retained legacy category formula. It prints MAE in minutes and MAPE but asserts only MAPE below 30%; zero usable comparisons also pass. Report counts/exclusions and treat empty-data results as unqualified. This does not validate personal-estimator accuracy or resolve the product target's ambiguous percentage wording.
- Filter/display tests check agreement under their fixtures. Callers still resolve estimates independently against time/database state; a once-per-event immutable snapshot is not implemented.
- OCR coverage is split between default-disabled feature code, ignored model/image tests, and Python tooling.

For task-oriented instructions, see [How to Test the Project](../howto/TESTING_GUIDE.md).
Current validation results belong in `STATUS.md`, not this test map.
