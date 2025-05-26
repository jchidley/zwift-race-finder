# Zwift Race Finder - TODO

## 📊 Current Status
- **Prediction Error**: 16.1% (down from 92.8%!) ✅ EXCEEDED <20% TARGET!
- **Real Race Data**: 131 matched races from Strava (80% match rate)
- **Multi-Lap Handling**: FIXED - pattern matching enables flexible event detection
- **Pack Model**: Implemented - recognizes draft dominance in racing
- **Test Suite**: Complete with route validation - all tests passing ✅
- **Security**: OAuth tokens protected with .gitignore
- **Repository**: Published to GitHub with all fixes
- **Production Status**: ✅ DEPLOYED - Binary installed, documentation complete, ready for users

## ✅ Completed Tasks
- [x] Major cleanup: removed 28 dead files
- [x] Renamed files with sensible names
- [x] Committed cleanup to GitHub
- [x] Ran Strava import - got 151 real race times
- [x] Fixed route distances (KISS Racing: 100→35km)
- [x] Updated base speed: 25→30.9 km/h based on actual data
- [x] Implemented multi-lap race detection using event_sub_groups
- [x] Fixed Volcano Flat 3 Laps prediction (21→71 min)
- [x] Achieved <30% prediction error target!
- [x] Created rider_stats table and weight import
- [x] Implemented pack-based model (draft dominates in races)
- [x] Fixed EVO CC route mapping (was on wrong routes)
- [x] Added route mapping consistency test
- [x] Added multi-lap race detection test
- [x] Added edge case tests (sprint, gran fondo, Alpe)
- [x] Updated all test expectations for 30.9 km/h speed
- [x] Cleaned up dead code (physics functions, unused constants)
- [x] Added integration test for Zwift API
- [x] Archived ZWIFT_API_LOG.md with date
- [x] Created ACCURACY_TIMELINE.md documentation
- [x] Fixed all 7 failing tests after speed update
- [x] Fixed strava_import_to_db.sh SQL errors (SQLite UPDATE limitations)
- [x] Achieved 80% race matching rate with Strava
- [x] Added strava_config.json to .gitignore for security
- [x] Created strava_config.json.example template
- [x] Achieved 23.6% prediction accuracy (exceeded target!)
- [x] **Fixed Racing Score event filtering** - Added support for events with distanceInMeters: 0 (2025-05-26)
- [x] Documented Racing Score vs Traditional event types
- [x] Implemented is_racing_score_event() and parse_distance_from_description()
- [x] Fixed Three Village Loop route data (39.8km → 10.6km)
- [x] Tool now shows races again! (tested with Three Village Loop showing 20min)
- [x] Implemented hierarchical log management (66KB → <5KB for LLM context)
- [x] Created project-context-manager tool (extracted to separate repository)
- [x] Enhanced UX with event type counts (e.g., "Found: 91 group rides, 52 races...")
- [x] Added context-aware "no results" suggestions with working examples
- [x] Tested all scenarios (short races, TT, group rides) - all working
- [x] **Expanded test coverage** (2025-05-26) - Added 9 new tests (16→25 total, +56%)
- [x] Implemented comprehensive Racing Score event tests (4 tests)
- [x] Implemented UX feature tests (5 tests)
- [x] Fixed debug output showing in normal mode
- [x] Extracted UX logic into testable functions for better architecture
- [x] **Investigated API limitation** (2025-05-26) - Confirmed 200 event hard limit (~12 hours)
- [x] Tested pagination/date parameters - all ignored by API
- [x] Added warning for multi-day searches explaining limitation

## 🔒 Security & Privacy Tasks

### Priority 0: Secure Token Storage
- [ ] Implement environment variable support for Strava tokens
- [ ] Add token encryption at rest
- [ ] Create secure token refresh mechanism
- [ ] Document secure deployment practices
- [ ] Consider using system keyring for token storage

## ✅ Racing Score Events - FIXED! (2025-05-26)

### What We Fixed
- [x] Updated Three Village Loop in routes database (was 39.8km, now 10.6km)
- [x] Modified filtering to accept events with distanceInMeters: 0
- [x] Added is_racing_score_event() function (checks rangeAccessLabel)
- [x] Implemented parse_distance_from_description() with regex
- [x] Tested with Three Village Loop races - working perfectly!
- [x] Updated documentation about Racing Score handling

## 🧪 Test Coverage Improvement Plan (2025-05-26) ✅ PHASE 1 COMPLETE!

### Completed Test Areas
1. **Racing Score Events** (Critical - recent bug fix) ✅ COMPLETE!
   - [x] Test `is_racing_score_event()` detection
   - [x] Test `parse_distance_from_description()` parsing
   - [x] Test events with distanceInMeters: 0
   - [x] Test full event filtering with Racing Score events

2. **UX Features** (High - new functionality) ✅ COMPLETE!
   - [x] Test event type counting logic
   - [x] Test "no results" message generation
   - [x] Test command example accuracy (via suggestions tests)

### Test Statistics
- **Total Tests**: 25 (up from 16)
- **New Tests Added**: 9 (+56% increase)
- **Coverage Areas**: Racing Score events, UX features, edge cases
- **All Tests**: ✅ PASSING

### Remaining Test Areas
3. **Core Functions** (High - no coverage)
   - [ ] Test distance parsing functions
   - [ ] Test user subgroup matching
   - [ ] Test caching mechanisms

4. **Edge Cases** (Medium)
   - [ ] Test malformed API responses
   - [ ] Test extreme route data
   - [ ] Test timezone handling

Target: >80% code coverage with all critical paths tested

## ✅ Completed: Auto Route Discovery (2025-05-26)

### Route Discovery Implementation
- [x] Implemented route discovery module with web scraping
- [x] Added database table for tracking search attempts  
- [x] Created CLI option --discover-routes
- [x] Integrated discovery flow into main program
- [x] Found 189 unknown routes needing discovery
- [x] Discovered Google search scraping doesn't work (blocked/changed)
- [x] Implemented direct site search APIs (whatsonzwift.com, zwiftinsider.com)
- [x] Discovered most "unknown routes" are custom event names, not route names
- [x] **Parse Event Descriptions** - Successfully extracting route names + lap counts
  - Implemented 5 regex patterns for common formats
  - Enhanced `--show-unknown-routes` to display parsed route names
  - Created `--analyze-descriptions` CLI option for batch analysis
  - Example: "Stage 4: Makuri May" → "Makuri Three Village Loop (1 laps)"
- [x] **Enhanced with Caching & Rate Limiting** (2025-05-26)
  - Added Arc<Mutex<HashMap>> cache to prevent repeated API calls
  - Implemented 500ms rate limiting for respectful scraping
  - Reduced worlds to check from 10 to 5 most common
  - Cleaned up database (removed 20+ already-known routes)
  - Successfully finding races with known routes (Three Village Loop = 20 min)

## ✅ Completed: Route Discovery Optimization (2025-05-27)

### Priority 0: Improve World Detection ✅ COMPLETED (2025-05-26)
- [x] Parse world hints from event names (e.g., "Makuri May" → makuri-islands)
- [x] Check event names for world-specific keywords (makuri, london, etc.)
- [x] Detected world checked first, reducing API calls by ~10x per route
- [x] Extract real route IDs from whatsonzwift.com URLs (no more placeholder 9999)

### Priority 1: Batch Discovery Process ✅ COMPLETED (2025-05-27)
- [x] Run discovery in smaller batches (10-20 routes at a time)
- [x] Add progress saving between batches
- [x] Consider background/async discovery
- [x] Implement smarter targeting based on route frequency
- [x] Sort by frequency to prioritize high-value routes
- [x] Show frequency count during discovery: "[55x] Searching for..."
- [x] Graceful timeout handling with progress notification

## ✅ Completed: Manual Route Mappings (2025-05-27)

### Priority 1: Create Manual Mapping Table ✅ COMPLETED
- [x] Researched actual routes used by high-frequency custom events
  - EVO CC Race Series → Watopia's Waistband (40.8km)
  - Sydkysten Race → Placeholder route 9001 (29.6km)
  - Tofu Tornado Race → Volcano Flat (varies 32-70km)
  - CAT & MOUSE KZR CHASE → Placeholder route 9002 (40km)
  - DBR races → Watopia Flat Route
  - ZHR Morning Tea Race → Placeholder route 9003 (49km)
  - Zwift TT Club - Watopia's Waistband → route 3733109212
- [x] Built SQL script for common event → route mappings (`manual_route_mappings.sql`)
- [x] Documented event series patterns in `ROUTE_MAPPING_RESEARCH.md`
- [x] Applied mappings: reduced unmapped events from 112 to 104
- [x] Improved accuracy: regression error from 43.9% to 34.0%

## ✅ Completed: Multi-Lap Race Detection (2025-05-27)

### Multi-Lap Implementation
- [x] Created multi_lap_events table for known multi-lap races
- [x] Added database method get_multi_lap_info()
- [x] Updated filtering logic to check for multi-lap events
- [x] Modified display code to show lap information
- [x] Enhanced regression test to apply lap multipliers
- [x] **Achieved 16.1% accuracy (exceeded <20% target!)**
- [x] **Production Deployment** (2025-05-27) - Built release binary, installed to ~/.local/bin
- [x] **Created User Documentation** - FEEDBACK.md for collecting user reports
- [x] **Created Deployment Guide** - DEPLOYMENT.md with installation and troubleshooting
- [x] **Updated README** - Latest 16.1% accuracy metrics and achievement details
- [x] **Production Testing** - Verified finding 34 races in 20-40 minute range
- [x] Fixed "3R Racing" with pattern matching (533% error improvement!)
- [x] Production-tested with live API data - working perfectly

### Priority 1: Production Testing
- [x] Test multi-lap detection with live API data - WORKING! Pattern matching enables flexible event matching
- [x] Enhanced get_multi_lap_info() with SQL pattern matching (exact match + LIKE)
- [x] Added Zwift Crit Racing Club (6 laps) - fixed 533% error!
- [ ] Add more multi-lap events as discovered
- [ ] Consider automating lap detection from event descriptions

### Priority 2: Remaining Improvements
- [ ] Create date-based mapping for rotating series (EVO CC changes weekly)
- [ ] Replace placeholder route IDs (9001-9003) with real Zwift route_ids
- [ ] Document multi-lap detection in README.md

### Priority 2: Integrate Description Parsing
- [ ] Integrate `get_route_data_enhanced()` into main filtering logic
- [ ] Test with events that have unknown route IDs but parseable descriptions
- [ ] Measure accuracy improvement from description parsing
- [ ] Handle case/spacing mismatches between parsed names and DB names

### Priority 3: Alternative Approaches
- [ ] Build manual mapping table for recurring events
  - "EVO CC Race Series" → rotates through specific routes
  - "KISS Racing" → usually specific criterium routes
- [ ] Add community submission process for route mappings
- [ ] Consider parsing ZwiftPower event pages for route info

### Priority 2: Optimize Discovery for High-Value Routes
- [ ] Prioritize high-frequency unknown routes
  - Stage 4: Makuri May (68 occurrences)
  - Restart Monday Mash (42 occurrences)
  - TEAM VTO POWERPUSH (37 occurrences)
  - Focus on routes with >10 occurrences first

## 🎯 Next Phase: Get Below 20% Error

### Priority 1: Refine Pack Dynamics Model
- [x] Implemented simplified pack-based model
- [ ] Analyze races with highest prediction errors
- [ ] Account for race size (bigger fields = more consistent draft)
- [ ] Consider route-specific pack dynamics (climbs split packs)
- [ ] Add time-of-day factor (peak hours = bigger fields)
- [ ] Test impact of category density on pack formation

### Priority 2: Better Route Data
- [ ] Build elevation profile database
- [ ] Add grade-specific speed calculations
- [ ] Import more route mappings from ZwiftHacks
- [ ] Handle surface-specific rolling resistance

### Priority 3: Enhanced Data Collection
- [ ] Scrape individual ZwiftPower event pages for exact times
- [ ] Import power data from Strava (for FTP estimation)
- [ ] Add route elevation profiles from ZwiftHacks API
- [ ] Track draft vs non-draft events

## 🚀 Future Enhancements

### Near Term
- [ ] Add database backup automation
- [ ] Add time trial support (no draft calculation)
- [ ] Surface-specific bike recommendations
- [ ] Weather condition effects (if Zwift adds them)
- [ ] Automate weekly Strava sync
- [ ] Create CI/CD pipeline with automated tests

### Long Term
- [ ] Real-time predictions during races (Sauce4Zwift API)
- [ ] Community data sharing API
- [ ] Device emulation for automated testing
- [ ] Machine learning model from race history
- [ ] Power-based pacing recommendations

## 🐛 Current Known Issues
- [x] ~~Racing Score events have distanceInMeters: 0~~ - FIXED! (2025-05-26)
- [ ] Some routes still need distance corrections
- [ ] Network connectivity affects API calls
- [ ] Category E not properly mapped (treated as D)
- [ ] Race series (like EVO CC) run different routes each week
- [ ] Some struct fields show as "never read" but are used in DB queries

## 💡 Key Learnings

### Technical Discoveries
1. **Event subgroups are crucial** - Different categories race different distances
2. **Real data beats estimates** - Strava integration was game-changing
3. **Draft matters** - 30.9 km/h in races vs 25 km/h solo
4. **Multi-lap races** - Must use total distance, not base route
5. **Route mapping critical** - Wrong mapping (EVO CC) caused 11.2% accuracy drop
6. **Test coverage essential** - Comprehensive tests prevent regression
7. **Racing Score events different** - API returns distanceInMeters: 0, distance in description only
8. **Two event systems** - Traditional (A/B/C/D) vs Racing Score (0-650 ranges) are mutually exclusive
9. **Site search tip** - Use `site:https://whatsonzwift.com` to find accurate route data
10. **UX matters** - Event type counts and smart suggestions reduce user friction significantly
11. **Manual mapping effective** - Custom event names need SQL scripts, not automated discovery
12. **Route length critical** - Must match distance to duration (EVO CC: 40.8km not 12.1km)

### AI Development Insights
1. **Domain knowledge essential** - Knowing Zwift racing guided better solutions
2. **Technical experience valuable** - 40 years IT helped spot issues and guide architecture
3. **Management approach works** - Treating AI as enthusiastic employee needing direction
4. **Transparency enables debugging** - Seeing AI's reasoning catches problems early
5. **Data validates assumptions** - Real-world testing revealed multiple wrong assumptions
6. **Web search effective** - Event organizer websites often contain route details not in API