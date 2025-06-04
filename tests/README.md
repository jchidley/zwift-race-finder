# Zwift Race Finder Test Suite

This directory contains the comprehensive test suite for zwift-race-finder. The tests are organized to ensure code quality, prevent regressions, and validate functionality.

## Test Organization

### Unit Tests (in `src/main.rs`)
- **30+ unit tests** covering core functionality
- Duration estimation algorithms
- Event filtering logic
- Route parsing and matching
- URL parameter parsing
- Format functions

### Integration Tests (`tests/`)
- `integration_tests.rs` - CLI argument parsing and end-to-end workflows
- `api_tests.rs` - API interaction with mocked responses
- `config_tests.rs` - Configuration loading and precedence
- `property_tests.rs` - Property-based testing for edge cases

### Regression Tests (`src/regression_test.rs`)
- Tests against actual race data from Jack's history
- Validates accuracy of duration predictions
- Current accuracy: 23.6% mean absolute error

### Performance Benchmarks (`benches/`)
- `performance.rs` - Benchmarks for critical functions
- Database operations
- Duration calculations
- Event filtering performance

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test integration
cargo test api
cargo test config
cargo test property

# Run tests with output
cargo test -- --nocapture

# Run only unit tests in main.rs
cargo test --bin zwift-race-finder

# Run regression tests
cargo test regression

# Run benchmarks
cargo bench

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Test Coverage Areas

### ✅ Well Covered
- Duration estimation algorithms
- Event filtering (type, duration, tags)
- Route parsing from descriptions
- Racing Score vs Traditional category detection
- Database operations (CRUD)
- Configuration loading
- Format functions
- URL parameter parsing

### 🔄 Partially Covered
- API error handling (mocked)
- Token refresh logic
- Multi-lap race detection
- Route completion tracking

### ❌ Not Yet Covered
- Live API integration tests (would require credentials)
- Full OAuth flow
- Concurrent request handling
- Network timeout scenarios

## Writing New Tests

### Unit Test Example
```rust
#[test]
fn test_new_feature() {
    // Arrange
    let input = create_test_data();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected_value);
}
```

### Integration Test Example
```rust
#[test]
fn test_cli_workflow() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--duration", "60"])
        .output()
        .expect("Failed to execute");
        
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Event"));
}
```

### Property Test Example
```rust
proptest! {
    #[test]
    fn test_property(value in 0..1000) {
        // Property should hold for all values
        assert!(some_property(value));
    }
}
```

## Test Data

### Mock Events
Tests use realistic mock Zwift event data with:
- Various event types (RACE, GROUP_RIDE, TIME_TRIAL)
- Racing Score and Traditional category events
- Different distances and routes
- Tag variations

### Database Fixtures
- Test routes with known distances/elevation
- Historical race results for regression testing
- Unknown route entries for discovery testing

## Continuous Integration

Add to `.github/workflows/test.yml`:

```yaml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo bench --no-run
```

## Known Issues

1. **Mock Server Port Conflicts**: Mockito tests may fail if port 1234 is in use
2. **Database Path Tests**: Some tests use environment variables that may conflict
3. **Time-Sensitive Tests**: Event filtering tests may fail near date boundaries

## Future Improvements

1. **Add mutation testing** with cargo-mutants
2. **Increase API mock coverage** for edge cases
3. **Add fuzz testing** for parsers
4. **Create golden tests** for output format stability
5. **Add visual regression tests** for table output