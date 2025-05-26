# Zwift Race Finder - TODO

## 📊 Current Status
- **Prediction Error**: 23.6% (down from 92.8%!) ✅ EXCEEDED 30% TARGET!
- **Real Race Data**: 131 matched races from Strava (80% match rate)
- **Multi-Lap Handling**: FIXED - now using event_sub_groups
- **Pack Model**: Implemented - recognizes draft dominance in racing
- **Test Suite**: Complete with route validation - all tests passing ✅
- **Security**: OAuth tokens protected with .gitignore
- **Repository**: Published to GitHub with all fixes

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

### AI Development Insights
1. **Domain knowledge essential** - Knowing Zwift racing guided better solutions
2. **Technical experience valuable** - 40 years IT helped spot issues and guide architecture
3. **Management approach works** - Treating AI as enthusiastic employee needing direction
4. **Transparency enables debugging** - Seeing AI's reasoning catches problems early
5. **Data validates assumptions** - Real-world testing revealed multiple wrong assumptions