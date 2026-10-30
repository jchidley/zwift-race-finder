# How to Test the Project

Use focused tests while changing a component, then run the default-feature suite before declaring the change complete. The [Test Suite Reference](../reference/TEST_SUITE_SUMMARY.md) describes what each target actually covers.

## Prerequisites

On Debian/Ubuntu:

```bash
sudo apt-get install -y libssl-dev pkg-config
```

Install Rust with `rustup`. OCR tests additionally need the native libraries and model files described in [`tools/ocr/SETUP_GUIDE.md`](../../tools/ocr/SETUP_GUIDE.md).

## Run focused tests

```bash
# Pure library modules
cargo test --lib

# Main binary-private helpers
cargo test --bin zwift-race-finder

# Individual integration targets
cargo test --test api_tests
cargo test --test config_tests
cargo test --test property_tests
cargo test --test snapshot_tests
```

For a specific function or regression, pass its test-name filter:

```bash
cargo test test_name_fragment
```

## Run the accuracy regression

Any duration-formula change requires:

```bash
cargo test --lib test_race_predictions_accuracy -- --nocapture
```

This test reads race results from the normal application database. Record the before/after mean absolute error; do not treat a result from an empty or different database as comparable evidence.

## Run CLI integration tests carefully

```bash
cargo test --test integration_tests
```

This target recursively executes `cargo run --bin zwift-race-finder`. Some cases may call the live Zwift API and create or read `~/.local/share/zwift-race-finder/races.db`. It is not a hermetic fixture suite.

## Run the complete default-feature suite

```bash
cargo test
```

Run OCR tests only when the feature and native prerequisites are available:

```bash
cargo test --features ocr
```

## Update snapshots or golden data

Review ordinary snapshot failures before accepting new output. Use Insta's review flow rather than blindly replacing snapshots.

Golden generators and validators are ignored by default. Their source comments name the intended commands, including:

```bash
cargo test generate_golden_baseline_improved -- --ignored
cargo test validate_test_routes -- --ignored --nocapture
cargo test validate_against_race_history -- --ignored --nocapture
```

These commands can depend on local data. Inspect the affected files and generated diff before retaining updates.

## Use mutation testing when it answers a real risk

For important estimation or filtering logic, run a targeted campaign after the focused tests pass:

```bash
cargo mutants --file src/duration_estimation.rs
```

Choose the changed module or function. A surviving consequential mutation indicates a test gap; timeouts and unviable mutants require inspection rather than automatic acceptance. Mutation testing is not required for every documentation or reversible low-impact edit.

## Diagnose prerequisite failures

If `openssl-sys` reports that it cannot run `pkg-config` or locate OpenSSL, install/expose the packages listed above and retry. Do not interpret that build failure as a failing application test.
