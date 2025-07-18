# Zwift Race Finder - Project Plan

## Racing Score Event Support - FIXED! (2025-05-26)

### Problem & Solution
Racing Score events were not showing because they have `distanceInMeters: 0` in the API. Fixed by:
1. ✅ Added `is_racing_score_event()` to detect these events
2. ✅ Implemented `parse_distance_from_description()` to extract distance from text
3. ✅ Updated Three Village Loop route data (was 39.8km, fixed to 10.6km)
4. ✅ Modified filtering to handle both Traditional and Racing Score events
5. ✅ Tool now successfully shows races (tested: Three Village Loop showing 20min duration)

### Key Discovery
Zwift has two mutually exclusive event types:
- **Traditional**: A/B/C/D categories with `distanceInMeters` populated
- **Racing Score**: Score ranges (0-650) with `distanceInMeters: 0`, distance only in description

### User Tip
Jack shared: Use `site:https://whatsonzwift.com` search to find accurate route data!

## UX Improvements - COMPLETE! (2025-05-26)

### Problem & Solution
Default search (90-120min races) returned no results - poor user experience. Fixed by:
1. ✅ Added event type summary: "Found: 91 group rides, 52 races, 33 group workouts, 5 time trials"
2. ✅ Context-aware suggestions when no results found
3. ✅ Provides working command examples like `cargo run -- -d 30 -t 30`
4. ✅ Explains event duration patterns (races: 20-30min, TT/groups: 60-90min)

### Key Insight
Most races are short (20-30 minutes), while users expected longer events. The tool now educates users about typical event durations and guides them to successful searches.

## API Limitation Discovery (2025-05-26)

### Problem
Multi-day searches (-n 3) don't show events beyond ~12 hours due to API limit.

### Investigation Results
- Zwift API returns max 200 events (about 12 hours of data)
- Tested parameters: limit, offset, eventStartsAfter, eventStartsBefore - all ignored
- No alternative endpoints found (/scheduled, /calendar return 404)
- This is a hard limit on the public API endpoint

### Solution Implemented ✅ (2025-05-26)
- Added warning when requesting multiple days
- ✅ Display actual time range covered: "Events from May 26, 12:15 PM to May 27, 12:15 AM"
- ✅ Clear message when requested days exceed available data
- ✅ Added notification if API returns >250 events (future-proofing)
- Users can search different time windows throughout the day for more coverage

## Current Status (Production Ready - UX Enhanced!)
✅ Major cleanup complete - removed 28 dead files
✅ Files renamed for clarity (e.g., `zwiftpower_profile_extractor.js`)
✅ Successfully imported 151 real race times from Strava
✅ Fixed route distances (KISS Racing: 100→35km)
✅ Event filtering bug FIXED (2025-05-26) - Racing Score events now supported!
✅ Updated base speed: 25→30.9 km/h based on actual data
✅ Implemented multi-lap race handling using event_sub_groups
✅ Mean prediction error: 23.6% (exceeded 30% target!)
✅ Fixed EVO CC mapping issue (was on wrong routes)
✅ Added comprehensive test suite with route validation
✅ All tests passing - ready for confident refactoring
✅ Implemented pack dynamics model with drop probability
✅ Created accuracy timeline: 92.8% → 31.2% → 25.1% → 36.9% → 25.7% → 23.6%
✅ Fixed Strava import SQL issues (SQLite UPDATE limitations)
✅ Achieved 80% race matching rate (131/163 races)
✅ Secured OAuth tokens with .gitignore
✅ Added Racing Score event support with description parsing
✅ Implemented hierarchical log management (66KB → <5KB)
✅ Created project-context-manager tool (extracted to ~/tools/project-context-manager)
✅ Enhanced UX with event type counts & smart suggestions (2025-05-26)
✅ Expanded test coverage: 16→25 tests (+56% increase) - all critical features tested
✅ Discovered API limitation: 200 events max (~12 hours), no working pagination
✅ Added time range display and API behavior monitoring (2025-05-26)
✅ Researched API workarounds - confirmed 200 event hard limit
✅ Manually mapped 2 popular routes (Three Village Loop, Glasgow Crit Circuit)
✅ Implemented route discovery with direct site APIs (2025-05-26)
🚧 Pivoting to parse event descriptions for route names and lap counts
⚡ Production ready - published to GitHub

## Goal
Create accurate race duration predictions by using ACTUAL race times (not estimates) to calibrate the model.

## Development Approach
This project is built using Claude Code (AI-assisted development) with:
- **Domain Expert**: 40+ years IT experience, active Zwift racer understanding the problem space
- **AI Developer**: Claude Code handling implementation details and coding
- **Management Model**: Treating AI as an enthusiastic employee requiring clear direction
- **Success Metric**: Real-world accuracy (currently 25.7% error, achieved <30% target)

## Architecture Overview
```
Data Sources:
├── Zwift Public API → Upcoming events, route_id, laps, distance
├── Strava API → Actual race times, performance data
└── ZwiftPower → Historical race list (limited data)
                    ↓
            SQLite Database
            ├── race_results (with actual times from Strava)
            ├── routes (route metadata)
            ├── strava_activities (raw Strava data)
            └── unknown_routes (tracking)
                    ↓
            Rust Program (predictions)
```

## Key Problems We Solved
1. **Fake race times** - ZwiftPower exports had estimates (distance ÷ 30 km/h), not real times
2. **Wrong route distances** - KISS Racing was 100km instead of 35km
3. **Multi-lap races** - Different categories race different distances (event_sub_groups)
4. **Draft benefit** - Races are ~30% faster than solo riding
5. **Racing Score events** - API returns distanceInMeters: 0, fixed by parsing description text

## How We Fixed It
1. ✅ Integrated Strava API for real race times (151 races imported)
2. ✅ Fixed route distances using actual race data
3. ✅ Implemented per-category distance handling from event_sub_groups
4. ✅ Updated base speeds: Cat D = 30.9 km/h (was 25 km/h)
5. ✅ Handle Racing Score events by parsing description and using route DB

## Technical Decisions
- **Why SQLite?** Portable, simple, perfect for this use case
- **Why route_id as primary key?** Zwift's internal ID, stable across event name changes
- **Why placeholder 9999?** Allows immediate testing while mapping routes incrementally
- **Why Strava API?** Only reliable source for actual race completion times
- **Why not Zwift API?** No public API for personal results, developer API restricted

## Current Accuracy & Next Steps

### Achievement Unlocked! 🎉
- **Prediction Error**: 23.6% (was 92.8%)
- **Target**: Was 30%, now EXCEEDED!
- **Race Matching**: 80% (131/163 races matched with Strava)
- **Test Coverage**: 25 tests passing ✅ (56% increase from 16)
- **Security**: OAuth tokens protected

### What Made the Difference
1. **Real race times from Strava** (not estimates)
2. **Correct route distances** from actual data
3. **Multi-lap handling** via event_sub_groups
4. **Accurate base speed** (30.9 km/h for Cat D)
5. **Fixed route mappings** (EVO CC was on wrong routes)
6. **Comprehensive test suite** prevents regressions

### Immediate Next Steps (Physics Phase)
```bash
# 1. Add physical stats to database
sqlite3 ~/.local/share/zwift-race-finder/races.db
"CREATE TABLE rider_stats (
    id INTEGER PRIMARY KEY,
    height_m REAL DEFAULT 1.82,
    weight_kg REAL,
    ftp_watts INTEGER,
    updated_at TEXT
);"

# 2. Extract weight from Zwift profile
# Note: Zwift weight is more accurate than Strava
# Riders update it regularly for fair racing

# 3. Calculate CdA from height/weight
# A = 0.0276 × h^0.725 × m^0.425
```

## Future Projects

### ZwiftPower Individual Event Scraping
Individual ZwiftPower event pages contain actual race times that aren't available elsewhere:
- Example: https://zwiftpower.com/events.php?zid=4943630
- Shows: "Jack Chidley 1:51:42" (actual race duration)
- This data isn't available in profile exports or Strava
- Could scrape individual event pages to get precise race times
- Keep zwiftpower_event_extractor.js for potential future use

### Device Simulation & Testing

#### Fitness Device Emulators for Automated Testing
These GitHub projects can simulate power meters, heart rate monitors, and trainers for testing race predictions without actually riding:

1. **[Gymnasticon](https://github.com/ptx2/gymnasticon)** (500+ stars)
   - JavaScript/Node.js - Bridge proprietary bikes to Zwift
   - Simulates ANT+ and BLE power/cadence sensors
   - Perfect for creating custom test scenarios
   - Can modify to simulate any power profile (Cat A/B/C/D)

2. **[FortiusANT](https://github.com/WouterJD/FortiusANT)** (100+ stars)
   - Python - Makes old Tacx trainers work with Zwift
   - Full ANT+ FE-C implementation with grade simulation
   - Excellent reference for understanding trainer control protocol
   - Can run headless for automated testing

3. **[Zwack](https://github.com/paixaop/zwack)** (50+ stars)
   - JavaScript/Node.js - Pure BLE sensor simulator
   - Simulates FTMS, Cycling Power, and Heart Rate services
   - Keyboard controls for real-time power/cadence adjustment
   - Ideal for testing specific race scenarios

4. **[openant](https://github.com/Tigge/openant)** (300+ stars)
   - Python ANT+ library with device simulators
   - Examples include power, HR, speed, cadence simulation
   - Build custom simulators for regression testing

5. **[GoldenCheetah](https://github.com/GoldenCheetah/GoldenCheetah)** (2000+ stars)
   - C++ - Comprehensive cycling analytics
   - Contains ANT+ device simulation code
   - Reference implementation for power curves

### Testing Applications
- **Automated regression testing**: Simulate races with known power profiles
- **Edge case testing**: Power spikes, dropouts, unusual patterns
- **Category simulation**: Test predictions for different rider categories
- **Duration validation**: Verify predictions match simulated efforts
- **Multi-rider scenarios**: Test draft calculations with multiple simulators

## Future Projects: API Integrations & Data Sources

### Zwift Ecosystem APIs

1. **[Sauce4Zwift](https://github.com/SauceLLC/sauce4zwift)** (400+ stars)
   - **Real-time API**: `http://localhost:1080/api` (REST) and `ws://localhost:1080/api/ws/events` (WebSocket)
   - Access to live race data, gaps, positions
   - Headless mode for automated data collection
   - Python client example available
   - Could provide real-time validation of predictions during races

2. **[zwift-offline](https://github.com/zoffline/zwift-offline)** (912+ stars)
   - Complete offline Zwift server implementation
   - Reveals internal API structure and protocols
   - Useful for understanding Zwift's data formats
   - Could enable testing without Zwift subscription

3. **[zwift-mobile-api](https://github.com/Ogadai/zwift-mobile-api)** (109+ stars)
   - JavaScript client for Zwift's mobile API
   - ⚠️ Now requires Developer API access (restricted)
   - Still valuable as protocol documentation
   - Shows how to decode protobuf messages

### Alternative Data Sources

1. **Strava API** ✅ (Already integrated)
   - Provides actual race completion times
   - Well-documented public API
   - OAuth authentication
   - Rate limit: 200 requests/15min, 2000/day

2. **Garmin Connect API**
   - No official API, but community solutions exist
   - Could provide additional performance metrics
   - Heart rate variability, training load data

3. **TrainingPeaks API**
   - Limited public API
   - Mainly for workout creation/import
   - Could be useful for structured workout analysis

4. **Intervals.icu**
   - Has API for activity data
   - Good for performance tracking
   - Free alternative to TrainingPeaks

### Integration Opportunities

1. **Real-time Race Tracking**
   - Use Sauce4Zwift API during races
   - Compare predictions to actual performance
   - Auto-calibrate model based on live data

2. **Multi-source Data Fusion**
   - Combine Zwift events + Strava times + Sauce4Zwift live data
   - Build comprehensive performance database
   - Machine learning opportunities

3. **Community Data Sharing**
   - Build API for users to share route times
   - Crowdsource route mapping
   - Anonymous performance benchmarks

4. **Testing Infrastructure**
   - Combine device emulators with API monitoring
   - Automated nightly regression tests
   - Performance tracking dashboard

## Next Phase: Advanced Physics-Based Modeling

### Multi-Lap Race Handling ✅ COMPLETE
- ✅ Use event_sub_groups for per-category distances
- ✅ Parse distance from event names as fallback
- ✅ Calculate laps from total distance ÷ base route distance
- ✅ Fixed 3R Volcano Flat Race (was 21 min, now correctly 71 min)

### ZwiftPower Event Page Scraping
- Individual event pages contain precise race times
- Example: https://zwiftpower.com/events.php?zid=4943630
- Shows actual finish times for all participants
- Could build comprehensive time database across all categories

### Physics-Based Speed Model (Target: <20% Error)

#### Implement Martin et al. (1998) Power Equation
```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)ρ·CD·A·v³
```
- Replace category-based speed with physics calculations
- Account for rider weight, height → CdA
- Surface-specific rolling resistance (Crr)
- Gradient-dependent speed adjustments

#### Zwift-Specific Adaptations
Based on our research findings:
- **CdA Calculations**: `A = 0.0276·h^0.725·m^0.425`
  - Note: This formula appears to be reverse-engineered by the Zwift community
  - Not officially documented by Zwift
  - Equipment CdA base value: 0.1647 (also community-discovered)
- **Pack Dynamics**: 24.7-33% draft savings (confirmed by Zwift Insider)
- **Surface Penalties** (Source: Zwift Insider rolling resistance tests):
  - Road: Crr = 0.004 (confirmed)
  - Gravel on dirt: Crr = 0.018 (not 0.008 as previously stated)
  - MTB on pavement: Crr = 0.009 (confirmed)
  - Dirt: 80W reduction for road bikes (Nov 2023 update)
- **Speed Relationships**:
  - Flats: Speed ∝ ∛(Power/CdA)
  - Hills: Speed ∝ Power/Weight

#### Implementation Plan
1. **Add rider physical stats** (height, weight from Strava/ZwiftPower)
2. **Calculate personalized CdA** based on height/weight
3. **Estimate FTP/power** from historical performance
4. **Apply physics model** instead of category speeds
5. **Validate against real times** from Strava

### Research Papers for Reference
- **Martin et al. (1998)**: "Validation of a Mathematical Model for Road Cycling Power"
  - R² = 0.97 correlation with measured power
  - Standard error only 2.7W
  - Provides base physics equations used in cycling
- **Chung (2003)**: Virtual elevation method for CdA testing
- **Community Resources**:
  - Zwift Insider rolling resistance tests: https://zwiftinsider.com/crr/
  - Zwift speed calculations: https://zwiftinsider.com/zwift-speeds/
  - CdA formula source: Community reverse-engineering (TrainerRoad forums, Zwift forums)
  - Note: Specific CdA formula not officially documented by Zwift

### Expected Outcomes
- Reduce error from 25.1% → <20%
- ✅ Handle multi-lap races correctly (DONE!)
- Account for elevation profiles accurately
- Personalized predictions based on rider characteristics
- Better draft modeling (24.7-33% power savings)

## Next Feature: Automatic Route Discovery (Planned)

### Problem
- 30+ unknown routes encountered during events
- Manual route mapping is tedious and reactive
- Users get poor estimates for unknown routes

### Solution Design
- Detect unknown routes during event processing
- Automatically search whatsonzwift.com or zwiftinsider.com
- Parse distance/elevation from search results
- Update database with discovered route data
- Rate limit searches (10+ minutes between attempts per route)
- Use discovered data immediately for better estimates

### Expected Benefits
- Self-improving route database
- Better estimates for new/beta routes
- Reduced manual maintenance
- Community benefit as routes are discovered