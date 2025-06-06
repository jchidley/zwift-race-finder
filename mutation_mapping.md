# Mutation Results Mapping

This document maps functions from the old mutation results (when code was in main.rs) to their current locations after refactoring.

## Function Location Mapping

### Functions Moved from main.rs to event_display.rs:
- `prepare_event_row` - now at event_display.rs:495
- `print_events_table` - now at event_display.rs:600
- `display_filter_stats` - now at event_display.rs:709
- `log_unknown_route` - now at event_display.rs:78
- `display_event_header` - now at event_display.rs:23
- `display_route_info` - now at event_display.rs:45
- `display_duration_info` - now at event_display.rs:109
- `display_route_based_duration` - now at event_display.rs:138
- `calculate_actual_distance` - now at event_display.rs:185
- `display_calculated_duration` - now at event_display.rs:229
- `display_unknown_route_duration` - now at event_display.rs:264
- `display_distance_based_duration` - now at event_display.rs:300
- `display_estimated_duration` - now at event_display.rs:316
- `display_category_enforcement` - now at event_display.rs:340
- `display_subgroups` - now at event_display.rs:347
- `display_description_info` - now at event_display.rs:403
- `display_external_url` - now at event_display.rs:446

### Functions Still in main.rs:
- `analyze_event_descriptions` - main.rs:930
- `discover_unknown_routes` - main.rs:736
- `record_race_result` - main.rs:866
- `show_route_progress` - main.rs:1024
- `mark_route_complete` - main.rs:1004
- `filter_events` - main.rs:374
- `generate_no_results_suggestions` - main.rs:129
- `generate_filter_description` - main.rs:201
- `fetch_zwiftpower_public` - main.rs:271

### Functions in Other Modules:
- Event filtering functions moved to event_filtering.rs
- Duration estimation functions in duration_estimation.rs
- Route discovery functions in route_discovery.rs
- Database functions in database.rs
- Config functions in config.rs

## Analysis of 649 Missed Mutations

### By Module (from missed.txt):
- main.rs: ~200 mutations (many now moved to event_display.rs)
- event_display.rs: ~150 mutations
- database.rs: ~100 mutations 
- route_discovery.rs: ~50 mutations
- event_filtering.rs: ~40 mutations
- config.rs: ~40 mutations
- duration_estimation.rs: ~30 mutations
- secure_storage.rs: ~10 mutations
- Other files: ~30 mutations

### Most Common Mutation Types:
1. Arithmetic operators: `*` → `+`, `/` → `*`, `-` → `+`
2. Comparison operators: `>` → `<`, `==` → `!=`, `<=` → `>`
3. Boolean operators: `&&` → `||`
4. Assignment operators: `+=` → `*=`, `+=` → `-=`
5. Return value mutations: returning empty/default values

### High Priority Areas (based on frequency):
1. Event display calculations (distance, duration, elevation)
2. Filter statistics calculations
3. Route discovery parsing
4. Database query results
5. Configuration defaults

## Key Mutations to Address

### Critical Calculations:
1. Line 2: `src/main.rs:677:74: replace > with < in prepare_event_row` → Now event_display.rs
2. Line 11: `src/event_display.rs:271:39: replace / with % in display_distance_based_duration`
3. Line 50: `src/duration_estimation.rs:93:75: replace * with + in calculate_duration_with_dual_speed`
4. Line 114: `src/event_display.rs:223:47: replace / with * in display_calculated_duration`

### Boolean Logic:
1. Line 3: `src/main.rs:584:32: replace && with || in print_events_table` → Now event_display.rs
2. Line 4: `src/event_filtering.rs:78:73: replace || with && in log_unknown_route` → Now event_display.rs
3. Line 30: `src/main.rs:875:66: replace && with || in record_race_result`

### Boundary Conditions:
1. Line 5: `src/event_filtering.rs:200:33: replace <= with > in event_matches_duration`
2. Line 59: `src/event_filtering.rs:159:37: replace <= with > in event_matches_duration`

## Recommendations

1. **Focus on arithmetic operations in calculations** - These are critical for correct duration and distance estimates
2. **Test boundary conditions** - Many <= and >= comparisons lack edge case tests
3. **Verify boolean logic** - && vs || mutations indicate missing tests for compound conditions
4. **Add tests for default/empty returns** - Many functions that return Option or Result lack tests for None/empty cases
5. **Test error paths** - Functions returning Result<()> often just tested for Ok() case

## Next Steps

1. Write tests for the most critical arithmetic mutations in event_display.rs
2. Add boundary condition tests for comparison operators
3. Create tests for boolean logic combinations
4. Add negative test cases for functions returning Option/Result