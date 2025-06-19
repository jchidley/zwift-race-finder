# Phase 4 Coverage Plan - CLI and Integration Testing Strategy
**Date**: June 4, 2025, 11:39  
**Current Coverage**: 52.07% (81/169 functions uncovered)  
**Target**: Evaluate remaining functions and reach 70%+ coverage

## Executive Summary
Phase 4 focuses on evaluating the remaining 81 uncovered functions to determine the best testing approach. Based on initial analysis, we'll prioritize business logic functions for unit tests while recommending integration tests for CLI handlers and network functions.

## Function Category Analysis

### 1. High Priority - Business Logic (Natural Unit Tests Expected)
These functions should have straightforward unit tests:

#### Core Calculation Functions
- `calculate_drop_probability` - Physics calculations
- `estimate_duration_with_distance` - Core duration logic
- `get_rider_weight` - User stats calculations
- `apply_elevation_penalty` - Route difficulty calculations
- `process_multi_lap_event` - Multi-lap race handling

#### Filtering and Validation
- `filter_events` - Event filtering logic
- `validate_filters` - Input validation
- `validate_duration_range` - Range validation
- `is_event_in_time_range` - Time filtering

#### Data Processing
- `parse_event_tags` - Tag parsing logic
- `extract_route_info` - Route data extraction
- `normalize_event_name` - Name standardization
- `calculate_pace_from_speed` - Pace calculations

**Estimated functions in category**: ~20-25  
**Expected test quality**: Natural  
**Priority**: HIGH

### 2. Medium Priority - Database Functions (Natural Tests with Test DB)
Functions that interact with SQLite can use in-memory databases:

- `get_route_data` - Route lookups
- `store_unknown_route` - Logging unknown routes
- `update_route_cache` - Cache management
- `get_recent_results` - Historical data queries
- `cleanup_old_data` - Database maintenance

**Estimated functions in category**: ~10-12  
**Expected test quality**: Natural with test fixtures  
**Priority**: MEDIUM

### 3. Low Priority - CLI/Main Functions (Better as Integration Tests)
These functions handle command-line arguments and program flow:

- `main` - Program entry point
- `run` - Main execution logic
- `show_unknown_routes` - CLI command handler
- `record_race_result` - CLI command handler
- `parse_cli_args` - Argument parsing
- `handle_subcommand` - Command routing

**Estimated functions in category**: ~8-10  
**Expected test quality**: Contrived if unit tested  
**Priority**: LOW (defer to integration tests)

### 4. Low Priority - Network/API Functions (Mock-Heavy Tests)
These async functions would require extensive mocking:

- `fetch_events` - Zwift API calls
- `fetch_zwiftpower_stats` - ZwiftPower scraping
- `fetch_zwiftpower_public` - Public stats fetching
- `refresh_auth_token` - OAuth token refresh
- `check_api_health` - API availability checks

**Estimated functions in category**: ~8-10  
**Expected test quality**: Contrived with mocks  
**Priority**: LOW (consider integration tests)

### 5. Very Low Priority - Display Functions (Snapshot Tests)
Output formatting functions:

- `display_events` - Table rendering
- `format_table_output` - Column formatting
- `display_filter_stats` - Stats output (already tested)
- `print_summary` - Summary formatting
- `format_error_message` - Error display

**Estimated functions in category**: ~10-12  
**Expected test quality**: Contrived/Snapshot  
**Priority**: VERY LOW

### 6. Utility Functions (Mixed Priority)
Helper functions with varying complexity:

- `get_cache_expiry` - Time calculations (Natural)
- `sanitize_filename` - String manipulation (Natural)
- `merge_configs` - Config merging (Natural)
- `format_relative_time` - Time formatting (Natural)
- Error handling utilities (Contrived)

**Estimated functions in category**: ~15-20  
**Expected test quality**: Mixed  
**Priority**: MEDIUM

## Implementation Strategy

### Phase 4A: Business Logic Functions (Target: +15% coverage)
1. Start with core calculation functions
2. Test filtering and validation logic
3. Focus on functions with clear inputs/outputs
4. Expected: 20-25 natural tests

### Phase 4B: Database Functions (Target: +5% coverage)
1. Create test fixtures with known routes
2. Use in-memory SQLite for isolation
3. Test CRUD operations
4. Expected: 10-12 natural tests

### Phase 4C: Utility Functions (Target: +5% coverage)
1. Test pure utility functions first
2. Skip error handling utilities if contrived
3. Focus on data transformation functions
4. Expected: 10-15 natural tests

### Phase 4D: Evaluation Report
1. Document which functions were skipped and why
2. Calculate final coverage achieved
3. Identify any surprising insights
4. Recommend integration test strategy for skipped functions

## Success Criteria
- Achieve 70%+ function coverage (current: 52.07%)
- Maintain high ratio of natural tests (target: >80%)
- Document clear rationale for skipped functions
- Provide integration test recommendations

## Execution Plan
1. **Hour 1**: Implement Phase 4A business logic tests
2. **Hour 2**: Implement Phase 4B database tests
3. **Hour 3**: Implement Phase 4C utility tests
4. **Hour 4**: Create evaluation report and recommendations

## Key Principles
- Prioritize natural tests over coverage percentage
- Skip functions requiring heavy mocking
- Document why functions were skipped
- Recommend appropriate testing strategy (unit vs integration)

## Expected Outcomes
- Function coverage: 70-75%
- ~40-50 new test functions
- Clear documentation of testing decisions
- Integration test plan for remaining functions

## Risk Factors
- Some "business logic" functions may require I/O
- Database tests might reveal schema issues
- Coverage tool anomalies may persist
- Time constraints may limit full implementation

## Next Steps After Phase 4
1. Implement integration tests for CLI commands
2. Create mock-free API tests using recorded responses
3. Investigate coverage tool anomalies
4. Consider property-based testing for complex logic
5. Set up continuous coverage monitoring