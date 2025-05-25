# Zwift Race Finder - Project Plan

## Current Status (Post-Cleanup)
✅ Major cleanup complete - removed 28 dead files
✅ Files renamed for clarity (e.g., `zwiftpower_profile_extractor.js`)
✅ Successfully imported 163 historical races from ZwiftPower (but with FAKE times!)
✅ Strava API integration complete and ready to use
✅ Database structure supports real data
⚡ Testing post-cleanup functionality
❌ Current "actual_minutes" are estimates (distance ÷ 30 km/h) NOT real times
❌ Regression tests showing 92.8% error because we're comparing estimates to estimates
❌ Route distances wrong (KISS Racing ≠ 100km)

## Goal
Create accurate race duration predictions by using ACTUAL race times (not estimates) to calibrate the model.

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

## Why This Project Failed Initially
1. **No actual race times** - Used estimates instead of real data
2. **Wrong route distances** - KISS Racing at 100km? Really?
3. **No draft accounting** - Solo estimates for pack races

## The Fix Is Simple
1. Use Strava API to get REAL race times ✅ (already built!)
2. Fix obvious route errors
3. Let the data tell us the actual speeds

## Technical Decisions
- **Why SQLite?** Portable, simple, perfect for this use case
- **Why route_id as primary key?** Zwift's internal ID, stable across event name changes
- **Why placeholder 9999?** Allows immediate testing while mapping routes incrementally
- **Why Strava API?** Only reliable source for actual race completion times
- **Why not Zwift API?** No public API for personal results, developer API restricted

## Immediate Action Plan

### Step 0: Test & Commit Cleanup (15 min) ⚡ CURRENT
```bash
# Test everything still works
cargo test
cargo run
./import_zwiftpower_dev.sh --help

# Commit to GitHub
git add -A
git commit -m "refactor: major cleanup - remove dead code and rename files"
git push
```

### Step 1: Get Real Data (1 hour)
```bash
# This is ALL we need to do first!
cd ~/tools/rust/zwift-race-finder
./strava_auth.sh                    # Set up authentication
./strava_fetch_activities.sh        # Fetch your Zwift races
./strava_import_to_db.sh           # Import REAL times
./strava_analyze.py                # See your actual speeds
```

### Step 2: Fix Obvious Errors (30 min)
- KISS Racing: Change from 100km to ~35km
- Check multi-lap races ("3 Laps" in name)
- Run regression test - should improve dramatically

### Step 3: Iterate (ongoing)
- Map more routes as needed
- Adjust speed model based on real data
- Target <20% prediction error

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