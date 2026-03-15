# Duplicate Test Analysis Report

## Summary

Found **6 duplicate test functions** that exist in both `main.rs` and other modules. Additionally, identified **44 test functions** in `main.rs` that should be moved to their appropriate modules based on their functionality.

## Duplicate Tests (Exist in Both Files)

These tests are exact duplicates and should be removed from `main.rs`:

1. **test_duration_estimation_for_cat_d**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

2. **test_edge_case_estimations**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

3. **test_estimate_duration_for_category**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

4. **test_get_route_difficulty_multiplier**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

5. **test_get_route_difficulty_multiplier_from_elevation**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

6. **test_specific_route_multipliers**
   - Location: `main.rs` and `duration_estimation.rs`
   - Action: Remove from `main.rs`

## Tests That Should Be Moved

### To `event_filtering.rs` (13 tests)
- test_boolean_operators_in_filtering
- test_display_filter_stats
- test_display_filter_stats_arithmetic
- test_duration_filtering
- test_event_type_filtering
- test_filter_events_comparison_operators
- test_filter_events_distance_conversion
- test_filter_events_duration_arithmetic
- test_filter_events_duration_arithmetic_mutations
- test_filter_events_increment_operations
- test_filters_out_running_events
- test_generate_filter_description
- test_racing_score_event_filtering

### To `event_display.rs` (9 tests)
- test_display_filter_stats (overlaps with event_filtering)
- test_display_filter_stats_arithmetic (overlaps with event_filtering)
- test_prepare_event_row
- test_prepare_event_row_distance_conversion
- test_prepare_event_row_time_formatting
- test_prepare_event_row_time_formatting_mutations
- test_print_event_percentage_calculation
- test_print_event_percentage_calculations
- test_show_route_progress_percentage

### To `route_discovery.rs` (5 tests)
- test_get_multi_lap_distance
- test_multi_lap_race_detection
- test_route_id_regression_with_actual_results
- test_show_route_progress_percentage (overlaps with event_display)
- test_database_route_validation (overlaps with database)

### To `database.rs` (2 tests)
- test_database_route_validation
- test_log_unknown_route

### Tests That Should Remain in `main.rs` (14 tests)

These tests appear to be testing main.rs-specific functionality or cross-module integration:

- test_boolean_operators_mutations
- test_comparison_operators
- test_division_edge_cases
- test_generate_no_results_suggestions
- test_generate_no_results_suggestions_for_other
- test_generate_no_results_suggestions_for_race
- test_generate_no_results_suggestions_for_tt
- test_match_arm_coverage
- test_match_arm_coverage_mutations
- test_negation_operator_mutations
- test_negation_operators
- test_racing_score_event_with_zero_distance
- test_regression_common_zwift_races

## Action Items

1. **Immediate**: Remove the 6 duplicate tests from `main.rs`
2. **Next**: Move the categorized tests to their appropriate modules
3. **Verify**: Ensure all moved tests still pass in their new locations
4. **Consider**: Some tests like `test_display_filter_stats` might need to be split if they test functionality from multiple modules

## Notes

- Some tests overlap multiple categories (e.g., `test_display_filter_stats` involves both filtering and display)
- The mutation-related tests appear to be testing for proper handling of mutation testing scenarios
- Integration tests like `test_regression_common_zwift_races` should likely remain in `main.rs` or move to `tests/`