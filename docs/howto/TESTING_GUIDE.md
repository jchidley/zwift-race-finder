# How to Test the Project

Use Bash from the checkout root with stable Rust/Cargo via `rustup` (no minimum Rust version is declared). Run focused tests for the changed contract, then applicable default-feature, formatting and lint gates. The [Test Suite Reference](../reference/TEST_SUITE_SUMMARY.md) describes their boundaries; a listed command is not evidence it passed.

For documentation-only changes, inspect changed links and command/flag contracts and run `git diff --check`. Builds, live searches and application-state access are not required.

## Prerequisites

On Debian/Ubuntu, builds need `libssl-dev` and `pkg-config`. If missing, install only with the required privilege approval; do not run setup merely to validate documentation:

```bash
sudo apt-get install -y libssl-dev pkg-config
```

OCR tests additionally need the native libraries and model files described in [`tools/ocr/SETUP_GUIDE.md`](../../tools/ocr/SETUP_GUIDE.md). Default-feature success does not qualify OCR.

## Isolate writable application state

Database/library tests can open default paths, not just CLI tests. Use fresh HOME
and all three XDG directories, preserve the installed Cargo/Rustup homes, and
exclude inherited credentials and config overrides. For example, define this
Bash function in the current shell; it creates temporary state and removes only
that directory when the command ends:

```bash
zrf_check() (
    export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
    export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
    state=$(mktemp -d) || exit 1
    trap 'rm -rf -- "$state"' EXIT
    export HOME="$state"
    export XDG_DATA_HOME="$state/data"
    export XDG_CONFIG_HOME="$state/config"
    export XDG_CACHE_HOME="$state/cache"
    unset ZWIFTPOWER_PROFILE_ID ZWIFTPOWER_SESSION_ID ZRF_ZP_DEBUG
    unset ZWIFT_SCORE ZWIFT_CATEGORY ZWIFT_DEFAULT_DURATION
    unset ZWIFT_DEFAULT_TOLERANCE ZWIFT_DEFAULT_DAYS
    "$@"
)
```

Keep the checkout root as cwd so fixtures resolve. Subprocess CLI tests additionally
clear their environment and use temporary cwd/state. Some config and cache tests
still mutate process-wide cwd/environment; until those tests are migrated, use
`-- --test-threads=1` for affected targets and the complete suite. This limits
in-process interference, not network dependence. The function does not sandbox
arbitrary tests or block network access. Its empty database deliberately provides
**no accuracy qualification**.

## Run focused tests

```bash
# Library modules (includes database/default-path consumers)
zrf_check cargo test --lib -- --test-threads=1

# Main binary-private helpers
zrf_check cargo test --bin zwift-race-finder -- --test-threads=1

# Individual integration targets
zrf_check cargo test --test api_tests
zrf_check cargo test --test config_tests -- --test-threads=1
zrf_check cargo test --test property_tests -- --test-threads=1
zrf_check cargo test --test snapshot_tests -- --test-threads=1
zrf_check cargo test --test duration_consistency_tests -- --test-threads=1
zrf_check cargo test --test personal_estimation_tests -- --test-threads=1
zrf_check cargo test --test history_import_tests
zrf_check cargo test --test client_route_import_tests
```

For a specific function or regression, pass its test-name filter:

```bash
zrf_check cargo test test_name_fragment -- --test-threads=1
```

## Run the accuracy regression

Any duration-formula change requires before/after comparison on the **same approved
populated dataset**, preferably a backed-up isolated copy selected through
HOME/XDG paths. Do not use `zrf_check`'s fresh empty state for this check. Do not
copy or import private data merely to validate documentation. With the populated
data path deliberately selected and inherited ZwiftPower credentials excluded:

```bash
cargo test --lib test_race_predictions_accuracy -- --nocapture
```

This test opens the application database selected by HOME/XDG and may initialize
or migrate it. Preserve the original data and use the same populated copy/cohort
for both runs. Record MAE in minutes, MAPE, usable sample count and exclusions for
unknown historical scores or missing geometry. Never infer scores to fill gaps.

The current test prints both metrics but asserts only MAPE below 30%; it also
passes when no usable comparisons exist. Treat zero samples as **unqualified**,
not a successful accuracy check. The older product target's below-20% “mean
absolute error” wording is ambiguous; resolving its metric is separate from the
current test threshold. Neither this documentation nor a passing test changes it.

This regression exercises the retained legacy category formula, not the personal estimator. Personal
changes additionally need chronological shipped-code evaluation using only earlier
observations, comparable event geometry, out-of-sample MAE and time-window
inclusion results. Fitted residuals are not forecast intervals. See
[Algorithms](../reference/ALGORITHMS.md#validation-and-calibration).

## Run CLI integration tests carefully

```bash
zrf_check cargo test --test integration_tests -- --test-threads=1
```

This target executes Cargo's built binary via `CARGO_BIN_EXE_zwift-race-finder`,
not recursive Cargo. Its subprocess helper clears inherited environment and uses
temporary HOME/XDG/cwd. Other in-process helpers can still open default paths,
which is why outer isolation matters. Several cases call the live Zwift API:
network failures are not deterministic fixture failures, and loopback adapter
tests do not qualify the production endpoint. No full fixture-driven CLI pipeline
is currently provided.

## Run the complete default-feature suite

```bash
zrf_check cargo test -- --test-threads=1
cargo fmt -- --check
cargo clippy -- -D warnings
```

CI specifies these gates, but that is not evidence of a current pass. Existing
formatting/lint failures are recorded in `STATUS.md`; report relevant failures
without broad suppression or unrelated auto-formatting. An empty-state accuracy
test pass inside this suite does not replace the populated-data comparison.

Run OCR tests only when the feature and native prerequisites are available:

```bash
zrf_check cargo test --features ocr -- --test-threads=1
```

## Evaluate saved personal predictions

Use Node.js from the checkout root. Keep rider data and reports outside Git. This
command reads saved predictions; it does not fetch events, rerun a model or import
history. The output must be a new file and is created with private permissions.

```bash
node tools/analysis/evaluate_predictions.mjs /absolute/private/predictions.jsonl /absolute/private/evaluation.json
node --test tests/prediction_evaluation_tests.mjs
```

Each JSONL row contains a `target` with a unique source `result_id` and positive
`actual_seconds`, plus consistently named model objects (such as `B0`, `R1`,
`H1`, `RW1`). Each model supplies positive `raw_minutes` or an explicit
`unavailable` reason. Example schema with synthetic data:

```json
{"target":{"result_id":"synthetic-1","actual_seconds":2532.517},"B0":{"raw_minutes":43.6126802983357}}
```

The report retains unrounded predicted seconds, signed error (predicted minus
measured), absolute error, mean absolute error and mean signed error. Optional
legacy `rounded_minutes` fields are ignored. Inclusive window decisions use raw
seconds for 20–40, 40–50, 50–70, 75–105 and 40–60 minutes. These windows overlap;
summed mistakes are not distinct races. Unsupported predictions count as not
selected; an all-unavailable model has null error metrics, not zero error. Model
means cover supported targets, so compare availability and paired cohorts before
ranking models with different coverage.

The evaluator verifies data shape, not source authenticity or training chronology.
Qualify source/geometry and use strictly earlier history in the producing replay.
Keep frozen predictions and earlier reports unchanged; generate a separate precision
report. Do not combine retrospective geometry replays with prospective forecasts
as though their inputs or validation status were identical. Numerical precision
is not a claim of predictive accuracy, and rounded source distances remain uncertain.

## Update snapshots or golden data

Review ordinary snapshot failures before accepting new output. Use Insta's review flow rather than blindly replacing snapshots.

Golden generators and validators are ignored by default. Available commands include:

```bash
cargo test generate_golden_baseline_improved -- --ignored
cargo test validate_test_routes -- --ignored --nocapture
cargo test validate_against_race_history -- --ignored --nocapture
```

These commands can depend on local data and write expected artifacts. Inspect
source and prerequisites before execution, use isolated state or an explicitly
approved populated copy as appropriate, and review the generated diff. Do not
regenerate outputs merely to make a failing check pass.

## Use mutation testing when it answers a real risk

For important estimation or filtering logic, consider a targeted campaign after
the focused tests pass, when stronger fault-detection evidence is justified. If
`cargo-mutants` is installed, use the same state isolation:

```bash
zrf_check cargo mutants --file src/duration_estimation.rs
```

Choose the changed module or function. A surviving consequential mutation indicates a test gap; timeouts and unviable mutants require inspection rather than automatic acceptance. Mutation testing is not required for every documentation or reversible low-impact edit.

## Diagnose prerequisite failures

If `openssl-sys` reports that it cannot run `pkg-config` or locate OpenSSL, install/expose the packages listed above and retry. Do not interpret that build failure as a failing application test.
