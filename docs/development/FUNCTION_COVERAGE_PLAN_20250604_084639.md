# Function Coverage Plan - Achieving 100% Coverage
**Date**: June 4, 2025, 08:46:39  
**Current State**: 41.77% function coverage in main.rs (66/158 functions covered)  
**Goal**: 100% function coverage across all modules

## Executive Summary

This plan outlines the approach to achieve 100% function coverage for the Zwift Race Finder project. We have identified 32 uncovered functions in main.rs alone, which we'll categorize and test systematically, evaluating whether each test is natural (indicates good code) or contrived (suggests potential refactoring opportunities).

## Current Coverage Status

| Module | Functions | Covered | Uncovered | Coverage |
|--------|-----------|---------|-----------|----------|
| main.rs | 158 | 66 | 92 | 41.77% |
| config.rs | 29 | 9 | 20 | 31.03% |
| database.rs | 44 | 21 | 23 | 47.73% |
| regression_test.rs | 11 | 9 | 2 | 81.82% |
| route_discovery.rs | 46 | 7 | 39 | 15.22% |
| secure_storage.rs | 19 | 10 | 9 | 52.63% |
| **Total** | **307** | **122** | **185** | **39.74%** |

## Categorization of Uncovered Functions in main.rs

### Category 1: Pure Utility Functions (High Priority - Natural Tests Expected)
These functions have no side effects and should have straightforward, natural tests:

1. **format_duration** - Formats minutes to HH:MM string
2. **parse_lap_count** - Extracts lap count from event names
3. **parse_distance_from_description** - Parses distance from description text
4. **estimate_distance_from_name** - Estimates distance from event name patterns
5. **get_category_from_score** - Maps score to category letter
6. **get_route_difficulty_multiplier_from_elevation** - Calculates difficulty multiplier
7. **is_racing_score_event** - Checks if event uses racing score system
8. **get_multi_lap_distance** - Calculates total distance for multi-lap races
9. **default_sport** - Returns default sport value

**Expected Outcome**: All should have natural tests. If any feel contrived, the function may need refactoring.

### Category 2: Data Preparation Functions (High Priority - Mostly Natural)
These prepare data for display or processing:

1. **prepare_event_row** - Formats event data for table display
2. **generate_filter_description** - Creates human-readable filter description
3. **estimate_duration_for_category** - Estimates race duration by category

**Expected Outcome**: Should have natural tests with realistic event data.

### Category 3: Display/Output Functions (Medium Priority - Mixed)
These handle console output:

1. **print_event** - Prints detailed event information
2. **print_events_table** - Prints events in table format
3. **display_filter_stats** - Shows filtering statistics
4. **log_unknown_route** - Logs unknown routes (has side effects)

**Expected Outcome**: May require output capture or mocking. Tests might feel slightly contrived.

### Category 4: API/Network Functions (Medium Priority - Integration Tests)
These interact with external services:

1. **fetch_events** - Main API call to Zwift
2. **fetch_zwiftpower_stats** - Authenticated ZwiftPower API
3. **fetch_zwiftpower_public** - Public ZwiftPower scraping
4. **get_user_stats** - Orchestrates stat fetching with caching

**Expected Outcome**: Will need mocking or test fixtures. Some tests may be contrived.

### Category 5: File/Cache Functions (Medium Priority - IO Tests)
These handle file system operations:

1. **get_cache_file** - Determines cache file path
2. **load_cached_stats** - Reads from cache
3. **save_cached_stats** - Writes to cache
4. **get_route_data** - Database/fallback lookup

**Expected Outcome**: Natural tests with temp directories.

### Category 6: Business Logic Functions (High Priority - Natural)
Core application logic:

1. **filter_events** - Main filtering pipeline
2. **estimate_duration_for_category** - Duration calculation logic

**Expected Outcome**: Should have very natural tests with various event scenarios.

### Category 7: CLI Command Handlers (Low Priority - Often Contrived)
These are main entry points for commands:

1. **main** - Application entry point
2. **show_unknown_routes** - CLI command
3. **discover_unknown_routes** - CLI command (async)
4. **analyze_event_descriptions** - CLI command (async)
5. **record_race_result** - CLI command
6. **mark_route_complete** - CLI command
7. **show_route_progress** - CLI command

**Expected Outcome**: Often require full application setup. Tests may be contrived or better suited as integration tests.

## Testing Strategy

### Phase 1: Pure Functions (Week 1)
- Start with Category 1 functions
- Each should have 3-5 test cases covering normal and edge cases
- Document any that feel contrived

### Phase 2: Business Logic (Week 1)
- Test Categories 2, 3, and 6
- Focus on realistic scenarios from actual Zwift usage
- Use real event data where possible

### Phase 3: IO and Network (Week 2)
- Test Categories 4 and 5
- Use test fixtures for network responses
- Use temp directories for file operations
- Consider whether integration tests are more appropriate

### Phase 4: CLI Handlers (Week 2)
- Evaluate each CLI handler
- Some may be better tested through integration tests
- Document which ones have contrived tests

## Success Criteria

1. **Quantitative Goals**:
   - Achieve 100% function coverage in main.rs
   - Improve overall project function coverage to >80%
   - All tests must pass reliably

2. **Qualitative Goals**:
   - Document test quality (natural vs contrived) for each function
   - Identify functions that may need refactoring based on test difficulty
   - Ensure tests actually validate behavior, not just execute code

## Expected Challenges

1. **Network Functions**: May require extensive mocking
2. **Async Functions**: Need tokio test runtime
3. **CLI Handlers**: May have too much setup for unit tests
4. **Display Functions**: Output testing can be fragile

## Next Steps

1. Begin with Category 1 (pure utility functions)
2. Write tests for 3-5 functions at a time
3. Run coverage after each batch to verify improvement
4. Document test quality in session notes
5. Refactor functions where tests reveal issues

## Measuring Progress

After each testing session:
1. Run: `cargo llvm-cov --summary-only --ignore-filename-regex "src/bin/.*"`
2. Document: Functions tested, coverage improvement, test quality
3. Update: This plan with findings and adjustments

## Decision Framework

For each uncovered function, ask:
1. Does this function have a clear, testable purpose?
2. Can I write a natural test that validates real behavior?
3. If the test feels contrived, why? 
   - Is it because the function is too tightly coupled?
   - Should this be an integration test instead?
   - Is the function even necessary?

## Expected Timeline

- **Day 1-2**: Categories 1 & 2 (Pure functions and data prep)
- **Day 3-4**: Categories 3 & 6 (Display and business logic)
- **Day 5-6**: Categories 4 & 5 (IO and network)
- **Day 7**: Category 7 (CLI handlers) and cleanup

## Notes

- Prioritize natural tests over coverage metrics
- Document functions that might benefit from refactoring
- Consider creating test utilities for common patterns
- Some CLI handlers might be better left to integration tests