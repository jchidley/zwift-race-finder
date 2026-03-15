# Zwift API Log - Executive Summary

## Project Overview
AI-assisted development of a Zwift race finder tool that predicts race durations based on route characteristics and rider profile. Achieved production-ready accuracy of 23.6% mean absolute error (from initial 92.8%).

## Major Breakthroughs

### 1. Pack Dynamics Discovery (2025-05-25)
- **Root Cause**: Getting dropped on hills explains 82.6% of race time variance
- **Binary State**: Either with pack (30.9 km/h) or solo (23.8 km/h) - no middle ground
- **Weight Impact**: Jack at 86kg vs typical 70-75kg = major disadvantage on climbs
- **Key Insight**: High variance is inherent to racing dynamics, not a prediction failure

### 2. Event Type Revelation (2025-05-26)
- **Two Systems**: Traditional (A/B/C/D) vs Racing Score (0-650) events
- **Critical Difference**: Racing Score events always have `distanceInMeters: 0`
- **Solution**: Parse distance from description text for Racing Score events
- **Impact**: Fixed "no races found" bug affecting ~50% of events

## Accuracy Progression
- Initial physics model: 92.8% error (overestimated by 127%)
- Category-based model: 36.9% error
- Pack dynamics model: 25.7% error
- Production model: 23.6% error (exceeded <30% target)

## Key Technical Solutions

### Duration Estimation Algorithm
1. Use route_id for known route lookup
2. Check event_sub_groups for category-specific distances
3. Apply dual-speed model with drop probability
4. Handle both Traditional and Racing Score events
5. Fallback to category-based estimation

### Data Integration
- ZwiftPower browser extraction (151 historical races)
- Route mapping system (routes.db)
- Regression testing against actual times
- Dynamic route discovery for unknowns

## Critical Lessons Learned

### Zwift ≠ Real World
- Draft benefit: 33% in Zwift vs 25% real world
- Physics models accurate for real cycling fail in Zwift
- Pack dynamics dominate individual power metrics

### Data Quality Insights
- Event names unstable, route_ids stable
- Official Zwift API incomplete (missing distances)
- ZwiftHacks.com authoritative for route_ids
- Community resources essential (Zwift Insider)

## Current Status
- Tool in production use by Jack
- Handles both event types seamlessly
- Logs unknown routes for continuous improvement
- Ready for community release pending documentation