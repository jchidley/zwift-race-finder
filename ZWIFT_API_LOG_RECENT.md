# Zwift API Log - Recent Sessions

## Session: Secure OAuth Token Storage Implementation (2025-05-27-009)
**Goal**: Implement secure storage for OAuth tokens to address HIGH priority security requirement

### Summary
Successfully implemented comprehensive secure storage solution with three backends: environment variables (CI/CD), system keyring (desktop), and file storage (backward compatible). Created migration scripts and documentation while maintaining 100% backward compatibility.

**Key Results**:
- Created `src/secure_storage.rs` with automatic backend detection
- Built migration scripts: `strava_auth_secure.sh`, `strava_fetch_activities_secure.sh`
- Added comprehensive documentation: SECURE_TOKEN_MIGRATION.md
- All tests passing, ready for production use

**Status**: Security requirement COMPLETE - Ready for next priority

### Key Accomplishments
1. **Flexible Storage Module**:
   - Environment variables for CI/CD workflows
   - System keyring integration for desktop security
   - File storage with 600 permissions for compatibility
   - Automatic fallback: env → keyring → file

2. **Zero Breaking Changes**:
   - Existing users' setups continue working
   - Progressive enhancement based on available backends
   - Clear migration path at user's own pace

3. **Production Ready**:
   - Full test coverage for all backends
   - Comprehensive documentation
   - Ready for GitHub Actions integration

### Next Session Priority
Configuration management (priority #2) - Personal data that survives updates

[Full session details in sessions/ZWIFT_API_LOG_SESSION_20250527_009.md]

## Session: Comprehensive Requirements Review (2025-05-27-008)
**Goal**: Review all project documentation and update REQUIREMENTS.md based on latest user needs

### Summary
Systematically reviewed all 41 *.md files in the project to capture requirements, with the most recent user needs taking precedence. User clarified that creating comprehensive requirements documentation WAS the solution to their "not working as I'd like" concern. Tool verified working correctly with all tests passing.

**Key Results**:
- Created FILES_REVIEW_LIST.md to track systematic review of 41 files
- Created comprehensive REQUIREMENTS.md addressing all concerns
- Identified security issues: OAuth tokens in plain text (HIGH priority)
- Tool functionality verified: runs correctly, 26/26 tests pass
- Committed requirements documentation to git

**Status**: Requirements gathering complete, awaiting user direction on priorities

### Key Accomplishments
1. **Systematic File Review**:
   - Listed all 41 *.md files sorted by modification time
   - Reviewed each file from newest to oldest
   - Captured all requirements with recent needs taking precedence

2. **Security Requirements Added**:
   - Bitwarden integration for OAuth token storage
   - Pre-commit hooks to prevent secret commits
   - Sanitization scripts for public release
   - Multiple secure config options

3. **Critical Discoveries Documented**:
   - Pack dynamics explain 82.6% of variance
   - Two event type systems (Traditional vs Racing Score)
   - Manual mapping more effective than automated discovery
   - AI + human expertise = successful development

### Discoveries
- **Highest Priority**: User stated "I'm not convinced that the program is working as I'd like"
- **Security Issues**: OAuth tokens in plain text, hardcoded personal IDs
- **Configuration Gaps**: Need seamless personal config that survives updates
- **Underutilized Data**: Height/weight collected but not fully used in predictions
- **API Communication**: 200 event limit needs better user education

### Technical Details
Updated REQUIREMENTS.md sections:
- Priority update section highlighting user concern
- Security requirements (7.6-7.8) for token storage and sanitization
- Configuration requirements (13.5-13.7) for loading hierarchy
- Physics modeling requirements (19.5-19.8) for height/weight utilization
- Critical discoveries section documenting key insights

### Next Session Priority
**Implement security improvements for OAuth token storage** - HIGH priority issue identified during requirements review.

[Full session details in sessions/ZWIFT_API_LOG_SESSION_20250527_008.md]

## Session: Production Deployment (2025-05-27-006)
**Goal**: Deploy to production with comprehensive documentation

### Summary
Successfully deployed Zwift Race Finder to production with 16.1% accuracy. Created user documentation, tested production installation, and prepared for community feedback.

**Key Results**:
- Production binary installed to ~/.local/bin
- Created FEEDBACK.md and DEPLOYMENT.md guides
- Updated README with 16.1% accuracy metrics
- Production test: Found 34 races in 20-40min range

**Status**: PRODUCTION READY - Monitoring for user feedback

## Session: Multi-Lap Accuracy Achievement (2025-05-27-005)
**Goal**: Measure overall accuracy improvement from multi-lap detection fixes

### Summary
Multi-lap detection implementation resulted in dramatic accuracy improvement. Mean Absolute Percentage Error dropped from 34.0% to 16.1%, exceeding the <20% target. The fix particularly improved predictions for criterium races which commonly use multi-lap formats.

**Key Results**:
- Mean Absolute Percentage Error: 16.1% (was 34.0%)
- Exceeded <20% accuracy target
- Major fixes: 3R Racing (21→63 min), Team DRAFT (47→94 min), EVR Winter (64→128 min)
- All regression tests passing

**Impact**: Production-ready with proven accuracy exceeding project requirements

**Next**: Monitor production usage for any remaining high-error event types

[Full details reflect regression test results from Session 2025-05-27-003]

## Session: Multi-Lap Detection Production Testing (2025-05-27-004)
**Goal**: Verify multi-lap detection works with live API data

### Summary
Successfully tested and verified multi-lap detection with live Zwift API data. Pattern matching enhancement allows flexible event name matching, fixing 533% duration errors on criterium races. Zwift Crit Racing Club now correctly shows as 6 laps (38 minutes) instead of single lap (6 minutes).

**Key Fix**: Enhanced `get_multi_lap_info()` with two-stage SQL lookup (exact match + pattern match)

**Impact**: Production-ready multi-lap detection significantly improves accuracy for all multi-lap events

**Next**: Run regression test to measure overall accuracy improvement

[Full details in sessions/ZWIFT_API_LOG_SESSION_20250527_004.md]

## Session: Multi-Lap Race Detection Implementation (2025-05-27-003)
**Goal**: Improve prediction accuracy by recognizing multi-lap race patterns

### Key Accomplishments
1. **Root Cause Analysis**:
   - Regression test showed 34% error, with some races at 67% error
   - "3R Racing" predicted 21 min but actual was 52-79 min
   - "Team DRAFT Monday Race" predicted 47 min but actual was 75-91 min
   - Realized these were multi-lap races with misleading names

2. **Multi-Lap Detection System**:
   - Created `multi_lap_events` table in database
   - Added `get_multi_lap_info()` method to database.rs
   - Modified filtering logic to check for multi-lap events
   - Updated display to show lap information
   - Enhanced regression test to apply lap multipliers

3. **Key Mappings Discovered**:
   - "3R Racing" = 3 laps of Volcano Flat (36.9km total)
   - "Team DRAFT Monday Race" = 2 laps of Castle to Castle (49km total)
   - "EVR Winter Series" = 2 laps of Watopia Flat Route (66.8km total)
   - KISS Racing & DIRT Dadurday = Single lap (not multi-lap as initially thought)

### Results
- **Prediction accuracy improved from 34.0% to 16.1%!**
- Exceeded target of <30% error rate
- Major prediction fixes:
  - 3R Racing: 21→63 minutes
  - Team DRAFT: 47→94 minutes
  - EVR Winter: 64→128 minutes

### Technical Details
```sql
-- Create multi-lap events table
CREATE TABLE multi_lap_events (
    event_name_pattern TEXT PRIMARY KEY,
    route_id INTEGER NOT NULL,
    lap_count INTEGER NOT NULL,
    notes TEXT
);

-- Example entries
INSERT INTO multi_lap_events VALUES
('3R Racing', 3369744027, 3, 'Generic 3R Racing is 3 laps of Volcano Flat'),
('Team DRAFT Monday Race', 3742187716, 2, 'Monday race is 2 laps of Castle to Castle');
```

### Key Discovery
Many race series use generic names that hide their multi-lap nature. A simple database lookup for known patterns dramatically improves accuracy over complex parsing attempts.

### Next Priority
Test multi-lap detection with live API data to ensure production readiness

## Session: Web Research for Event Mapping (2025-05-27-002)
**Goal**: Map high-frequency events using web search for route details

### Key Accomplishments
1. **Discovered Web Research Method**:
   - Event organizer websites contain route details not in API
   - Web searches for "[event name] zwift route" highly effective
   - Found routes for 3 major event series

2. **Successfully Mapped**:
   - Restart Monday Mash (55x) → Mountain Mash (5.7km, 335m)
   - TEAM VTO POWERPUSH (37x) → Tempus Fugit (18.9km, 16m)
   - The Bump Sprint Race (27x) → Tick Tock (19.2km, 59m)

3. **Database Updates**:
   - Added all 3 routes with proper specifications
   - Updated manual_route_mappings.sql
   - Removed from unknown_routes table

### Key Discovery
Many "unknown routes" are standard Zwift routes with custom event names. Event descriptions and organizer websites are valuable data sources.

### Next Priority
Map Jack's remaining 104 unmapped races to achieve <30% accuracy target

## Session: Manual Route Mappings (2025-05-27-001)
**Goal**: Create manual mappings for high-frequency custom race events

### Key Accomplishments
1. **Researched 9 Race Series**: 
   - EVO CC, Sydkysten, Tofu Tornado, CAT & MOUSE, DBR, ZHR, TT Club
   - Documented typical routes, distances, and race formats

2. **Created Mapping Infrastructure**:
   - `manual_route_mappings.sql` - SQL scripts for applying mappings
   - `ROUTE_MAPPING_RESEARCH.md` - Documentation of findings
   - Placeholder route IDs (9001-9003) for series without confirmed IDs

3. **Applied Mappings Successfully**:
   - Reduced unmapped events from 112 to 104
   - Fixed critical EVO CC error (was 12.1km, now 40.8km)
   - Regression error improved from 43.9% to 34.0%

### Key Discovery
Route length must match race duration for accurate predictions. Initial EVO CC mapping to Volcano Flat (12.1km) predicted 21 minutes for 75-minute races (72% error). Correcting to Watopia's Waistband (40.8km) fixed the issue.

### Next Priority
Map remaining high-frequency events: "Restart Monday Mash" (55x), "TEAM VTO POWERPUSH" (37x)

## Session: Batch Discovery Implementation (2025-05-27-001)
**Goal**: Implement batch processing for route discovery to handle timeouts

### Key Accomplishments
1. **Batch Processing**: Process 20 routes per batch with 2-min timeout
2. **Prioritization**: Sort by frequency - "Restart Monday Mash" (55x) first
3. **Progress Saving**: Can resume discovery across multiple runs
4. **Graceful Handling**: Detects approaching timeout, saves state

### Results
- First batch: 15 routes processed, 0 found (all custom event names)
- 170 routes remaining for future runs
- Performance: ~6 seconds per route with respectful delays

### Next Priority
Create manual mapping table for recurring custom events

## Session: World Detection and Route ID Extraction (2025-05-26-005)
**Goal**: Enhance route discovery with intelligent world detection and real route ID extraction

### Key Accomplishments
1. **World Detection**: Parse event names for world-specific keywords
   - "makuri", "neokyo", "yumezi" → makuri-islands
   - "london", "box hill", "keith hill" → london
   - "alpe", "volcano", "jungle" → watopia
   - Detected world checked first, reducing API calls by ~10x

2. **Route ID Extraction**: Extract real IDs from whatsonzwift.com
   - Regex patterns: `routeId: 123`, `data-route-id="123"`, `/api/routes/123`
   - No more placeholder 9999 IDs
   - Enables proper database matching

3. **Performance Boost**: Multiplicative gains
   - World detection: 10x reduction (10 worlds → 1)
   - Combined with cache: effectively infinite speedup for duplicates

### Testing Results
```
STAGE 3: RACE MAKURI— Turf N Surf -> makuri-islands
Box Hill Climb Race -> london
Central Park Loop -> new-york
Alpe du Zwift -> watopia
```

### Next Priority
Implement batch discovery (10-20 routes at a time) to handle 185 unknown routes without timeout

## Session: Route Discovery Enhancement (2025-05-26-004)
**Goal**: Add caching and optimization to route discovery module

### Key Improvements
1. **Caching System**: Added in-memory cache to RouteDiscovery
   - Uses Arc<Mutex<HashMap>> for thread-safe caching
   - Prevents repeated lookups for same event names
   - Caches both successes and failures

2. **Rate Limiting**: Implemented respectful scraping
   - 500ms delay between HTTP requests
   - Prevents overwhelming external servers
   - Makes discovery more sustainable

3. **Optimized Search**: Reduced scope for better performance
   - Limited to 5 most common worlds (was 10)
   - Prioritized: watopia, makuri-islands, london, new-york, france
   - Added progress logging for debugging

### Results
- Route discovery now works but is slow for bulk operations
- Successfully found "Three Village Loop" races (20 min estimates)
- Cleaned up database: removed 20+ already-known routes

### Known Issues
- Full discovery of 185 routes would take ~8 minutes
- Placeholder route_id (9999) needs proper extraction
- Need better world detection heuristics

## Session: Event Description Parsing Implementation (2025-05-26-003)
**Goal**: Parse route information from event descriptions to handle unknown route IDs

### Solution Implemented
Created comprehensive regex-based parsing for event descriptions:

1. **Parsing Patterns Added** (5 total):
   - "X laps of Route Name" → `3 laps of Volcano Circuit`
   - "Route Name x N" → `Mountain Route x 2`
   - "Nx Route Name" → `2x Bell Lap`
   - "Route Name (N laps)" → `Three Village Loop (3 laps)`
   - "Stage X: Route Name" → `Stage 4: Makuri May` (assumes 1 lap)

2. **Code Architecture**:
   ```rust
   // In route_discovery.rs
   pub fn parse_route_from_description(description: &str) -> Option<ParsedEventDescription> {
       // Returns route_name and lap count
   }
   
   // Enhanced logging to show parsed route
   fn log_unknown_route(event: &ZwiftEvent) {
       if let Some(parsed) = parse_route_from_description(&description) {
           // Log as: "Event Name -> Route Name (N laps)"
       }
   }
   ```

3. **Database Enhancement**:
   - Added `get_route_by_name()` for name-based lookups
   - Created `get_route_data_enhanced()` to try description parsing fallback
   - Ready for integration into duration estimation

### Testing Results
All parsing patterns tested and working:
- ✅ "3 laps of Volcano Circuit" → Route: "Volcano Circuit", Laps: 3
- ✅ "Mountain Route x 2" → Route: "Mountain", Laps: 2  
- ✅ "Stage 4: Makuri May" → Route: "Makuri May", Laps: 1
- ✅ Real example: "Stage 4: Makuri May: Three Village Loop || Advanced" 
     → "Makuri Three Village Loop (1 laps)"

### Impact on Unknown Routes
Running `--show-unknown-routes` now shows enhanced entries:
- Old: `3379779247 | 68 | Stage 4: Makuri May: Three Village Loop || Advanced`
- New: `3379779247 | 68 | Stage 4: Makuri May: Three Village Loop || Advanced -> Makuri Three Village Loop (1 laps)`

This immediately reveals the actual route being used!

### New CLI Feature
Added `--analyze-descriptions` to batch analyze event descriptions:
- Fetches current events from API
- Parses all descriptions for route patterns
- Shows frequency-sorted results
- Helps identify most common route patterns for manual mapping

### Next Steps
1. Integrate `get_route_data_enhanced()` into main filtering logic
2. Handle duplicate route names (e.g., Innsbruck KOM After Party has IDs 13 & 1431)
3. Test with live data to measure accuracy improvement
4. Consider fuzzy string matching for partial route name matches

## Session: Racing Score Events Fix (2025-05-26-002)

### Critical Discovery: Two Event Types
Zwift has two mutually exclusive event categorization systems:
1. **Traditional Categories**: A/B/C/D/E with `distanceInMeters` populated
2. **Racing Score Events**: Score ranges with `distanceInMeters: 0`

### Root Cause
- Racing Score events ALWAYS have `distanceInMeters: 0` in API
- Distance only available in description text: "Distance: 23.5 km"
- Identified by `rangeAccessLabel` field in event subgroups
- Our filter was rejecting all events with 0 distance

### Solution Implemented
```rust
// Detect Racing Score events
fn is_racing_score_event(event: &Event) -> bool {
    event.event_sub_groups.iter()
        .any(|sg| sg.range_access_label.is_some())
}

// Parse distance from description
fn parse_distance_from_description(description: &str) -> Option<f64> {
    let re = Regex::new(r"Distance:\s*([\d.]+)\s*km").unwrap();
    // Extract and parse distance
}
```

### Impact
- Fixed "no races found" bug affecting ~50% of events
- Tool now handles both event types seamlessly
- Successfully showing correct durations for all event types

### Key Lessons
1. API design patterns: 0 values can mean "look elsewhere"
2. Always check alternative data sources (descriptions, metadata)
3. Browser DevTools essential for API investigation
4. User domain knowledge crucial (Jack caught route distance error)