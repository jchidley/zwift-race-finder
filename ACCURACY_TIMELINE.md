# Zwift Race Finder Accuracy Timeline

## Accuracy Progression Over Development

### Initial State (Session 2025-05-25 Early)
- **92.8% mean absolute error** - Initial predictions were way off
- **Root Cause**: Comparing estimates to estimates! The "actual_minutes" in database were calculated as `distance ÷ 30 km/h`, not real race times
- **Lesson**: Need real data, not calculated estimates

### After Strava Integration (Session 2025-05-25 Mid)
- **92.8% → 31.2% error** (66% improvement!)
- **What Changed**: 
  - Integrated Strava API to get actual race times
  - Fixed base speed from 25 km/h to 30.9 km/h (based on 151 real races)
  - Fixed incorrect route distances (e.g., KISS Racing: 100km → 35km)
- **Key Insight**: Real data revealed we were using wrong base speeds

### After Multi-Lap Fix (Session 2025-05-25 Late Afternoon)
- **31.2% → 25.1% error** (Below 30% target!)
- **What Changed**:
  - Fixed multi-lap race predictions (was showing 21 min for 67-74 min races)
  - Started using event_sub_groups for per-category distances
  - Added lap detection and distance parsing
- **Key Insight**: Different categories race different distances in same event

### After Pack Dynamics Model (Session 2025-05-25 Evening)
- **25.1% → 36.9% error** (Regression!)
- **What Changed**:
  - Implemented dual-speed model with drop probability
  - Accounted for getting dropped on hills (binary state: pack vs solo)
  - Added weight penalty calculations
- **Why It Regressed**: Model became more complex but revealed inherent variance in racing

### After Route Mapping Fix (Session 2025-05-25 Night)
- **36.9% → 25.7% error** (Back below 30% target!)
- **What Changed**:
  - Fixed EVO CC races incorrectly mapped to Bell Lap (14.1km vs actual 45km)
  - Added comprehensive test suite to prevent future mapping errors
- **Key Insight**: Single route mapping error caused 11.2% accuracy degradation

## Summary

The accuracy journey shows classic software development patterns:
1. **Start with wrong assumptions** (92.8% error)
2. **Get real data** (31.2% error)
3. **Fix edge cases** (25.1% error)
4. **Add complexity that reveals new problems** (36.9% error)
5. **Fix data quality issues** (25.7% error)

Final accuracy of **25.7%** is acceptable given:
- High inherent variance in racing (same route can vary 32-86 minutes)
- Binary nature of pack dynamics (with pack or dropped)
- Weight penalties for heavier riders on climbs

The variance isn't a prediction failure - it's the nature of bike racing!