# Test Suite

## Running tests

| Purpose | Command |
|---------|---------|
| All tests | `cargo test` |
| Regression accuracy | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| API tests | `cargo test --test api_tests` |
| Integration tests | `cargo test --test integration_tests` |
| Config tests | `cargo test --test config_tests` |
| Property tests | `cargo test --test property_tests` |
| Snapshot tests | `cargo test --test snapshot_tests` |
| Benchmarks | `cargo bench` |
| Unit tests only | `cargo test --lib` |

## Current state (2026-03-15)

- **169 tests passing** (90 lib + 80 integration/property/snapshot)
- **Regression accuracy**: 17.9% MAE on 125 matched races
- **6 tests ignored** (golden tests, full regression on 7500+ races)

## Test locations

| Location | Tests | Purpose |
|----------|-------|---------|
| `src/main.rs` | 26 | CLI logic, event filtering, URL parsing |
| `src/duration_estimation.rs` | 11 | Duration math, difficulty multipliers |
| `src/event_filtering.rs` | 25 | Filter logic, category matching |
| `src/event_display.rs` | 18 | Output formatting |
| `src/regression_test.rs` | 4 | Accuracy vs real race data |
| `src/category.rs` | 4 | Score→category, speeds |
| `tests/api_tests.rs` | 6 | API interaction with mocks |
| `tests/config_tests.rs` | 6 | Config loading, precedence |
| `tests/integration_tests.rs` | 12 | CLI end-to-end workflows |
| `tests/property_tests.rs` | 7 | Property-based edge cases |
| `tests/properties_tests.rs` | 9 | Additional properties |
| `tests/snapshot_tests.rs` | 4 | Output format stability |
