# ZWIFT_API_LOG.md

This log documents our journey understanding and integrating with the Zwift API for race predictions.

## Summary of Previous Work (Archived to ZWIFT_API_LOG_2025-05-25.md)

### Key Discoveries
1. **Zwift API**: Successfully integrated with official API at `https://us-or-rly101.zwift.com/api/public/events`
2. **Route System**: Zwift uses internal route IDs that remain stable across event name changes
3. **Pack Dynamics**: Draft benefit is 33% in Zwift vs 25% in real world, making pack position critical
4. **Drop Mechanics**: Binary state - either with pack (30.9 km/h) or solo (23.8 km/h) with no middle ground

### Major Milestones
- Initial accuracy: 92.8% error (physics model failed in Zwift environment)
- Calibrated to category speeds: 36.9% error
- Fixed route mapping issues: 25.7% error (current best)
- Discovered variance is inherent to racing (32-86 min for same route)

### Technical Implementation
- Dual-speed model accounts for drop probability based on elevation/weight
- Route mapping from event names to route_ids for accurate distance/elevation
- Regression testing against 151 actual race results
- Comprehensive test suite to prevent regressions

## Current Status (2025-05-25)

### Model Performance
- **Accuracy**: 25.7% mean absolute error
- **Status**: Acceptable given inherent race variance
- **Key Insight**: High variance reflects real racing dynamics, not prediction failure

### Next Steps
- Continue monitoring and mapping new routes as they appear
- Gather more race data to refine drop probability calculations
- Consider race-specific factors (field size, time of day) for better predictions

## Session 2025-05-25 Evening: Final Improvements & Production Ready

### What We Accomplished
1. **Fixed Accuracy Regression**: Discovered EVO CC races mapped to wrong route (11.2% error)
2. **Added Comprehensive Tests**: 4 new tests preventing future mapping errors
3. **Code Cleanup**: Removed dead physics code and unused constants
4. **Documentation**: Created accuracy timeline showing full journey
5. **Integration Test**: Added API connectivity test (run with --ignored)

### Key Technical Work
- Fixed 7 failing tests by updating speed expectations to 30.9 km/h
- Cleaned up warnings by removing unused physics functions
- Added test for route mapping consistency (catches bad mappings)
- Archived previous log with comprehensive summary

### Final Status
- **All tests passing** ✅
- **Accuracy below target** (25.7% < 30%) ✅
- **Code cleaned up** with minimal warnings
- **Documentation complete** including accuracy timeline
- **Production ready** for daily use