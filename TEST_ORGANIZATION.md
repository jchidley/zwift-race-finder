# Test Organization

This document describes how tests are organized in the Zwift Race Finder codebase.

## Test Distribution

Tests are placed in the same module as the code they test, following Rust best practices:

### Module Tests

Each module contains its own test module with unit tests for functions in that module:

- **duration_estimation.rs** - Tests for duration calculation functions
  - `test_get_route_difficulty_multiplier_from_elevation()`
  - `test_estimate_duration_for_category()`
  - `test_duration_estimation_for_cat_d()`
  - `test_get_route_difficulty_multiplier()`
  - `test_specific_route_multipliers()`
  - `test_edge_case_estimations()`
  - `test_more_elevation_multipliers()`

- **event_filtering.rs** - Tests for event filtering functions
  - `test_filter_stats_total()`
  - `test_filter_by_sport()`
  - `test_filter_by_time()`
  - `test_filter_by_event_type_*()` (multiple variants)
  - `test_filter_by_tags()`
  - `test_filter_by_excluded_tags()`
  - `test_event_matches_duration_*()` (multiple variants)

- **parsing.rs** - Tests for parsing functions
  - `test_estimate_distance_from_name()`
  - `test_parse_distance_from_description()`
  - `test_parse_description_data()`
  - `test_parse_lap_count()`
  - `test_parse_distance_from_name()`

- **category.rs** - Tests for category-related functions
- **route_discovery.rs** - Tests for route discovery functions
- **database.rs** - Tests for database operations
- **secure_storage.rs** - Tests for secure storage functionality

### Integration Tests in main.rs

The main.rs file contains integration tests that test the interaction between multiple modules:

- `test_filter_events()` - Tests the main event filtering pipeline
- `test_prepare_event_row()` - Tests event display preparation
- `test_display_filter_stats()` - Tests filter statistics display
- `test_generate_no_results_suggestions()` - Tests user suggestion generation
- Various mutation testing edge cases

### Test Organization Principles

1. **Unit tests go with their module** - Each function is tested in the same file where it's defined
2. **Integration tests stay in main.rs** - Tests that orchestrate multiple modules remain in main
3. **Test helpers are module-local** - Helper functions like `create_test_event()` are duplicated where needed
4. **Regression tests in separate module** - Complex regression tests have their own module

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib duration_estimation::tests
cargo test --lib event_filtering::tests
cargo test --lib parsing::tests

# Run only integration tests
cargo test --bin zwift-race-finder

# Run with output for debugging
cargo test -- --nocapture
```

## Future Improvements

1. Consider extracting common test utilities to a shared test_utils module
2. Add property-based tests for edge cases
3. Add performance benchmarks for critical paths