# Integration Test Coverage Analysis

**Date**: 2025-06-08  
**Purpose**: Document current integration test coverage and identify gaps

## Current Integration Test Coverage

### 1. CLI Integration Tests (`tests/integration_tests.rs`)

#### ✅ Command Line Interface Testing
- **Help command**: Validates help text and available options
- **Invalid inputs**: Tests negative duration, invalid event types
- **Database commands**: Tests `--show-unknown-routes` functionality
- **New routes flag**: Tests `--new-routes-only` behavior
- **Multiple tags**: Tests tag parsing with `--tags` and `--exclude-tags`
- **Record result**: Validates format requirements
- **Verbose mode**: Tests different output formats

#### ✅ Output Format Testing
- **Table format**: Default output validation
- **Multi-lap detection**: Tests lap counting from event names
- **Distance calculation**: Verifies multi-lap distance multiplication

#### ✅ Route Regression Testing
- **Known routes**: Validates 6 specific routes exist in database
- **Duration bounds**: Tests estimates are within reasonable ranges for Cat D
- **Framework ready**: Structure in place for Jack's actual race results

### 2. API Integration Tests (`tests/api_tests.rs`)

#### ✅ API Response Handling (with Mocks)
- **Success case**: Valid JSON response parsing
- **404 errors**: Proper error handling
- **Malformed JSON**: Graceful failure on invalid data
- **Empty response**: Handles empty event lists
- **Racing Score events**: Special handling for 0-distance events
- **Tag filtering**: Tests event filtering by tags

### 3. Regression Tests (`src/regression_test.rs`)

#### ✅ Prediction Accuracy Testing
- **Database integration**: Reads actual race results
- **Multi-lap handling**: Caches lap info for performance
- **Error analysis**: Tracks errors by route
- **Mean accuracy**: Calculates overall prediction accuracy
- **Large error reporting**: Highlights predictions > 20% error

### 4. Snapshot Tests (`tests/snapshot_tests.rs`)

#### ✅ Behavioral Snapshots
- **Flat routes**: Bell Lap, Downtown Dolphin, Three Village Loop
- **Hilly routes**: Castle to Castle, Eracing Course, Hilltop Hustle
- **Mountain routes**: Road to Sky, Mt. Fuji, Mountain Mash
- **Duration calculations**: Tests all category speeds
- **Elevation factors**: Verifies elevation impact on duration

## Integration Test Gaps

### 🔴 Missing End-to-End Tests

1. **Real API Integration**
   - No tests against actual Zwift API (requires credentials)
   - OAuth token refresh flow not tested
   - Rate limiting behavior not verified
   - Network timeout handling not tested

2. **Full Workflow Tests**
   - API → Filter → Display pipeline not tested end-to-end
   - Config file integration not tested
   - Cache behavior not verified in integration tests

3. **External Tool Integration**
   - Strava import workflow not tested
   - ZwiftPower data extraction not tested
   - Database migration scripts not tested

### 🟡 Partial Coverage

1. **Error Scenarios**
   - Database corruption recovery
   - Concurrent access to database
   - Disk full conditions
   - Invalid config file handling

2. **Performance Tests**
   - Large event list processing
   - Database query performance
   - Memory usage under load

3. **Cross-Platform Testing**
   - Windows path handling
   - macOS specific behaviors
   - Different terminal environments

## Recommendations for UOM Migration

### Critical Integration Tests to Add

1. **UOM A/B Integration Tests**
   ```rust
   #[test]
   fn test_end_to_end_filtering_ab() {
       // Run full filtering pipeline with both implementations
       // Compare final event lists
   }
   ```

2. **Database Round-Trip Tests**
   ```rust
   #[test]
   fn test_uom_database_compatibility() {
       // Store UOM values in database
       // Read back and verify consistency
   }
   ```

3. **CLI Output Compatibility**
   ```rust
   #[test]
   fn test_cli_output_unchanged() {
       // Run CLI with both implementations
       // Verify output is character-for-character identical
   }
   ```

### Integration Test Strategy for UOM

1. **Use Golden Files**: Capture current CLI output for various commands
2. **Mock Time**: Use fixed timestamps for reproducible tests
3. **Test Data**: Create minimal test database with known routes
4. **CI Integration**: Run integration tests on every commit

## Summary

The project has good integration test coverage for:
- CLI functionality and parsing
- API response handling (with mocks)
- Regression testing against historical data
- Output format verification

Key gaps for UOM migration:
- End-to-end workflow tests
- Real API integration (would need test credentials)
- Performance and stress testing
- Cross-platform compatibility

The existing integration tests provide a solid foundation for ensuring the UOM migration doesn't break user-facing functionality.