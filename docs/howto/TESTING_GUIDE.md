# Testing Guide

How to run tests, add tests, and use mutation testing in this project.

## Running Tests

```bash
# All tests
cargo test

# Specific suites
cargo test --lib                    # Unit tests only
cargo test --test api_tests         # API tests (mocked)
cargo test --test integration_tests # CLI end-to-end
cargo test --test property_tests    # Property-based
cargo test --test snapshot_tests    # Output stability

# Regression accuracy (requires race_results in DB)
cargo test --lib test_race_predictions_accuracy -- --nocapture

# Benchmarks
cargo bench
```

## Adding Tests

### Unit Test for a Pure Function

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(90), "01:30");
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(61), "01:01");
    }
}
```

**Then immediately mutate**:
```bash
cargo mutants --file src/duration_estimation.rs --function format_duration --timeout 30
```

Fix any surviving mutations before moving on.

### Property Test for Invariants

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn duration_always_positive(distance in 1.0..200.0, elevation in 0.0..2000.0) {
        let duration = estimate_duration(distance, elevation);
        prop_assert!(duration > 0.0);
    }

    #[test]
    fn longer_distance_takes_longer(d1 in 10.0..100.0, d2 in 100.0..200.0) {
        let t1 = estimate_duration(d1, 0.0);
        let t2 = estimate_duration(d2, 0.0);
        prop_assert!(t2 > t1, "Longer route should take more time");
    }
}
```

### Integration Test for CLI

```rust
use assert_cmd::Command;

#[test]
fn test_help_shows_options() {
    Command::cargo_bin("zwift-race-finder")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("duration"));
}
```

## Mutation Testing

### Quick Check (Single Function)
```bash
cargo mutants --file src/estimation.rs --function estimate_duration --timeout 30
```

### Module Check (Before Commit)
```bash
cargo mutants --file src/duration_estimation.rs --timeout 180
```

### Full Codebase (Background)
```bash
nohup cargo mutants --jobs 8 --timeout 180 > mutation_$(date +%Y%m%d).log 2>&1 &
tail -f mutation_*.log | grep -E "MISSED|CAUGHT|tested"
```

### Interpreting Results

- **Killed**: Test caught the mutation ✅
- **Survived**: Tests didn't catch it — add/improve tests
- **Timeout**: Mutation caused infinite loop (often fine to ignore)

Target: >75% kill rate for core logic. Don't chase 100%.

### Code Movement Problem

If you refactored between starting mutation tests and analysing results, mutation reports reference old line numbers. Solution:

```bash
# Tag before mutation testing
git tag pre-mutation-$(date +%Y%m%d)
cargo mutants
# Later, compare: git diff pre-mutation-$(date +%Y%m%d)
```

## Golden Tests

Golden tests capture expected output for known inputs, preventing regressions.

### Generate Baseline
```bash
cargo test generate_golden_baseline_improved -- --ignored
```

### Test Data Selection
Focused on 11 representative routes × 11 distances × 12 scores = 1,452 cases (reduced from 9,414 by picking representatives of each terrain type and category boundary).

### No Database Dependency
Golden tests use `estimate_duration_for_category` (pure function), not the database. Tests are reproducible across environments.

### Tests That Need Database Access
For functions that require a real database, use isolated test databases:

```rust
use zwift_race_finder::test_db::TestDatabase;

#[test]
fn test_with_database() {
    let test_db = TestDatabase::new().unwrap();
    test_db.seed_test_routes().unwrap();
    // Run tests — database auto-deleted when test_db drops
}
```

## Test Data Validation

After changing routes in the database or adjusting the algorithm:

```bash
# Validate test routes against production DB
cargo test validate_test_routes -- --ignored --nocapture

# Validate against race history
cargo test validate_against_race_history -- --ignored --nocapture
```

Good results: <10% difference in mean/std dev between test routes and all routes.

## Tools

```bash
# Install
cargo install cargo-mutants    # Mutation testing
cargo install cargo-nextest    # Faster test runner
cargo install cargo-tarpaulin  # Coverage reports

# Usage
cargo mutants --file src/mod.rs --timeout 30
cargo nextest run
cargo tarpaulin --out Html --ignore-tests
```

## When to Write Tests

| Situation | Action |
|-----------|--------|
| Fixing a bug | Write regression test first |
| New calculation | Unit test + property test + mutate |
| New CLI flag | Integration test |
| Refactoring | Run existing tests, never modify them |
| Before release | Full mutation test run |
