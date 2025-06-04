# Coverage Baseline Session
**Date**: June 4, 2025, 08:46  
**Purpose**: Document current coverage state before beginning work

## Current Test Count
- Total tests passing: 70
- Total test files: 8 (including property tests)

## Functions Already Tested in main.rs

Based on the existing tests, these functions are covered:
1. `parse_lap_count` ✓
2. `parse_distance_from_name` ✓
3. `find_user_subgroup` ✓
4. `get_route_difficulty_multiplier_from_elevation` ✓
5. `get_route_difficulty_multiplier` ✓
6. `get_category_from_score` ✓
7. `get_category_speed` ✓
8. `count_events_by_type` ✓
9. `format_event_type` ✓
10. `generate_no_results_suggestions` ✓
11. `is_racing_score_event` ✓
12. `parse_distance_from_description` ✓
13. `parse_description_data` ✓

## Priority Order for Testing

Based on the plan, here's the order I'll tackle the functions:

### Immediate (Pure Functions with Clear Purpose):
1. `format_duration` - Simple formatter, should be trivial
2. `estimate_distance_from_name` - Pattern matching, good test candidate
3. `default_sport` - Trivial one-liner
4. `get_multi_lap_distance` - Simple calculation

### Next (Data Processing):
5. `prepare_event_row` - Table formatting
6. `generate_filter_description` - String building
7. `estimate_duration_for_category` - Core business logic

### Then (I/O Functions):
8. `get_cache_file` - Path construction
9. `load_cached_stats` - File reading
10. `save_cached_stats` - File writing

### Finally (Complex/Network):
11. `filter_events` - Main filtering pipeline
12. `display_filter_stats` - Console output
13. Network functions (may need mocking)

## Test Quality Tracking

I'll track each function as:
- ✅ Natural test (good code)
- ⚠️ Somewhat contrived (consider refactoring)
- ❌ Very contrived (needs refactoring)
- ⏭️ Skipped (better as integration test)

## Success Metrics
- Starting: 41.77% function coverage in main.rs
- Target: 100% function coverage
- Quality: >80% of tests should be natural