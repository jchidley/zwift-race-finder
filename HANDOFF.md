# Project: Zwift Race Finder
Updated: 2025-06-03 00:00 UTC

## Current State
Status: Lead-in distance feature complete, PROJECT_WISDOM consolidated
Target: Fix tests and verify accuracy improvements
Latest: Consolidated all WISDOM files per wrap-session process

## Essential Context
- Lead-in distance now displayed for all races (e.g., "Lead-in: 0.2 km")
- WhatsOnZwift URLs generated for routes with slugs
- 250 new routes imported from zwift-data-reference
- Database schema updated with lead-in fields (6 new columns)
- Duration calculations now include lead-in distance
- Tests need fixing due to schema changes

## Completed Today
1. ✅ Implemented lead-in distance handling (FR-2.1.6)
   - Updated database schema with 6 lead-in columns
   - Modified duration calculations to include lead-in
   - Display lead-in distance in race output
2. ✅ Added route slug support (DR-11.6)
   - Store route slugs in database
   - Generate WhatsOnZwift URLs for known routes
3. ✅ Imported zwift-data route database
   - Created import_zwift_data_routes.py script
   - Imported 264 routes with accurate lead-in data

## Next Step
Fix failing database tests due to schema changes, then run regression tests to measure accuracy improvement from lead-in distance implementation

## If Blocked
Check test failures with: cargo test database::tests::test_database_creation -- --nocapture


## Related Documents
- REQUIREMENTS.md - Updated with ZwiftHacks integration requirements (FER-20)
- ZWIFTHACKS_TECHNIQUES.md - Analysis of valuable techniques
- ROUTE_TRACKING_IDEAS.md - Detailed implementation plans
- CLAUDE.md - Project-specific AI instructions