# Zwift Race Finder - TODO

## 📊 Current Status
- **Prediction Error**: 25.7% (down from 92.8%!) ✅ BELOW 30% TARGET!
- **Real Race Data**: 151 races from Strava  
- **Multi-Lap Handling**: FIXED - now using event_sub_groups
- **Pack Model**: Implemented - recognizes draft dominance in racing
- **Test Suite**: Complete with route validation - all tests passing ✅
- **Next Goal**: Get below 20% error by refining pack dynamics

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

## 🔒 Security & Privacy Tasks

### Priority 0: Secure Token Storage
- [ ] Implement environment variable support for Strava tokens
- [ ] Add token encryption at rest
- [ ] Create secure token refresh mechanism
- [ ] Document secure deployment practices
- [ ] Consider using system keyring for token storage

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

### AI Development Insights
1. **Domain knowledge essential** - Knowing Zwift racing guided better solutions
2. **Technical experience valuable** - 40 years IT helped spot issues and guide architecture
3. **Management approach works** - Treating AI as enthusiastic employee needing direction
4. **Transparency enables debugging** - Seeing AI's reasoning catches problems early
5. **Data validates assumptions** - Real-world testing revealed multiple wrong assumptions