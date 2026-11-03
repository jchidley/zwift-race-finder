# Test Suite

Use this page as a quick command index. The maintained description of test boundaries and limitations is in [`docs/reference/TEST_SUITE_SUMMARY.md`](../docs/reference/TEST_SUITE_SUMMARY.md).

## Common commands

| Purpose | Command |
|---|---|
| All default-feature tests | `cargo test` |
| Library tests | `cargo test --lib` |
| Main binary tests | `cargo test --bin zwift-race-finder` |
| Accuracy regression | `cargo test --lib test_race_predictions_accuracy -- --nocapture` |
| Mock HTTP tests | `cargo test --test api_tests` |
| CLI integration tests | `cargo test --test integration_tests` |
| Client route import tests (private databases) | `cargo test --test client_route_import_tests` |
| Config tests | `cargo test --test config_tests` |
| Property tests | `cargo test --test property_tests` |
| Snapshot tests | `cargo test --test snapshot_tests` |
| OCR tests | `cargo test --features ocr` |

Route metadata inputs are now [client-only](../docs/howto/CLIENT_ROUTE_DATA.md).
Community route-import tests are retired with their importers; existing route rows
and history are not deleted, and fresh databases contain no seeded routes.

## Before running

Debian/Ubuntu needs `libssl-dev` and `pkg-config`. OCR tests have additional native and model/image prerequisites.

The accuracy regression reads the application database selected by HOME/XDG. Run the suite with isolated state to protect normal data. CLI subprocess tests use the built binary with their own isolated HOME/XDG and no inherited secrets, but some still use the live Zwift API. API adapter tests are loopback-only.

## Performance history

- `cargo test --lib history::`: synthetic export parsing and missingness/precision checks.
- `cargo test --test history_import_tests`: private-database migration, unknown/zero scores, preserved rows/objects/sequence, provenance uniqueness, repeat import, conflicts, rollback and non-writing preview.
- `node tests/history_export_tests.js`: synthetic browser export checks; no browser or personal data.
