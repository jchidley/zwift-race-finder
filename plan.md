# Zwift Race Finder - Regression Testing Plan

## Current Status
✅ Successfully imported 163 historical races from ZwiftPower
✅ Database schema aligned between Rust program and import tools
✅ All races currently use placeholder route_id 9999
✅ 109 unique event names across 163 races
✅ Database structure supports proper route mapping

## Goal
Build comprehensive regression testing using Jack's actual race history to validate the race duration estimation algorithms.

## Architecture Overview
```
ZwiftPower Data → Import Script → SQLite Database → Rust Program
                                        ↓
                              race_results table (actual times)
                              routes table (route metadata)
                              unknown_routes table (tracking)
```

## Phase 1: Route Mapping (Current)
1. **Identify Common Routes**
   - 3R Racing (13 occurrences)
   - EVO CC Race Series (8 occurrences)
   - Team DRAFT Monday Race (6 occurrences)
   - Create mapping table for event_name → route_id

2. **Research Route Data**
   - Use ZwiftHacks.com for official route IDs
   - Use Zwift Insider for distance/elevation data
   - Build comprehensive routes table

3. **Update Import Process**
   - Enhance dev_import_results.sh to use route mappings
   - Properly populate unknown_routes for unmapped events

## Phase 2: Regression Testing
1. **Test Data Preparation**
   - Group races by route_id
   - Calculate actual vs predicted times
   - Identify outliers for investigation

2. **Build Test Suite**
   - Create test_regression.rs
   - Compare actual race times with predictions
   - Calculate accuracy metrics (MAE, RMSE, R²)

3. **Improve Estimation Model**
   - Analyze prediction errors by category
   - Adjust difficulty multipliers based on data
   - Consider adding factors: draft benefit, race type

## Phase 3: Continuous Improvement
1. **Automated Updates**
   - Schedule regular ZwiftPower imports
   - Track prediction accuracy over time
   - Auto-adjust model parameters

2. **Route Discovery**
   - Flag new/unknown routes
   - Semi-automated route research
   - Community contribution system

## Technical Decisions
- **Why SQLite?** Portable, simple, perfect for this use case
- **Why route_id as primary key?** Zwift's internal ID, stable across event name changes
- **Why placeholder 9999?** Allows immediate testing while mapping routes incrementally

## Next Steps Priority
1. Map top 10 most frequent routes
2. Write regression test that works with current data
3. Fix unknown_routes tracking to show all unmapped events
4. Document route mapping process
5. Create route research helper script