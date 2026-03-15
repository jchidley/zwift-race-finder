# Project Wisdom Archive - 2025-06-02

This archive contains consolidated wisdom from:
- PROJECT_WISDOM.md (original insights)
- PROJECT_WISDOM_SUMMARY.md (universal insights)
- PROJECT_WISDOM_RECENT.md (recent discoveries)

## Universal Development Insights (from PROJECT_WISDOM_SUMMARY.md)

### 1. Data Quality > Algorithm Sophistication
Single data error degraded accuracy by 11.2%. The path from 92.8% to 25.7% error wasn't about better algorithms - it was about better data.

### 2. Test Reality, Not Assumptions
Tested estimates against estimates for weeks. Always validate test data comes from actual source of truth, not derived calculations.

### 3. Variance Can Be a Feature
Same conditions yielded 32-86 minute variations. High variance may be inherent to the domain, not a model flaw.

### 4. Binary States Often Dominate
Despite complex physics, outcomes were binary: with pack or dropped. Seek simple dominant patterns before complex models.

### 5. Use Stable Identifiers
Event names changed, route IDs didn't. Always identify and use the most persistent identifier in your data model.

### 6. Zero Means "Look Elsewhere"
APIs returning 0 often signal alternate data locations. Check descriptions, metadata, or related fields.

### 7. Build Fallback Chains
Multiple strategies (route→distance→description→defaults) ensure resilience. Design degradation paths for missing data.

### 8. Plan Before Debugging
When confused, resist random changes. Create systematic investigation plans for efficient problem solving.

### 9. Field Presence Reveals Types
Missing type fields? Use presence patterns (`rangeAccessLabel`) to differentiate data variants.

### 10. Browser DevTools for API Discovery
Official clients reveal API behavior. Check browser network tab before building blind.

### 11. Hierarchical Logs for AI Context
Large logs slow LLMs. Summary/Recent/Archive pattern achieves 13x reduction while preserving history.

### 12. 80% Match Rate is Success
Perfect data integration is impossible. Focus on sufficient quality for your use case.

## Historical Insights (from original PROJECT_WISDOM.md)

### 2025-01-06: Event Descriptions Contain Complete Race Information
Insight: The description field in events contains distance and elevation data that mirrors what the companion app shows
Impact: Rather than relying solely on API fields (which return 0.0 for Racing Score events), we should parse descriptions to extract the complete race information including distance in km and elevation gain in meters. This data is already being shown in the companion app.

### 2025-01-06: Route Discovery Pattern - Event Titles Contain Route Names
Insight: Event titles often contain route names after delimiters (-, :, |) - "Zwift Epic Race - Sacre Bleu" indicates the route is "Sacre Bleu"
Impact: Extracting route names from event titles enables automatic route discovery. The tool can parse titles, extract potential route names, and search the database or web for matching routes before marking them as unknown.

### 2025-01-06: Web Search Effectiveness for Route Discovery
Insight: Searching for route names directly (e.g., "Zwift Sacre Bleu route") yields better results than searching by numeric route ID
Impact: When implementing automated route discovery, use extracted route names in web searches rather than route IDs. Route IDs are internal identifiers not commonly referenced in public documentation.

### 2025-01-06: API Data Structure Contains Complete Race Information
Insight: All racing events contain lap count and racing score ranges in their subgroups, not in the main event data
Impact: Distance calculations must check subgroup.laps field and multiply by route distance. The API provides 0.0 for event distance but complete lap info in subgroups. This explains why some races show correct multi-lap distances (they use the subgroup data) while others don't.

### 2025-05-27: Requirements Gathering Process Validates User Concerns
Insight: User's concern "not working as I'd like" was effectively addressed through systematic requirements documentation rather than code changes
Impact: Created comprehensive REQUIREMENTS.md from 41-file review. Discovered security issues (OAuth in plaintext), config management needs, and physics modeling opportunities. Sometimes documentation IS the solution.

### 2025-05-27: Security Vulnerability Discovery Through Documentation
Insight: Comprehensive file review revealed OAuth tokens stored in plain text files - a critical security issue that wasn't apparent from normal usage
Impact: Security requirements now documented as HIGH priority. Shows value of systematic documentation reviews for uncovering non-obvious issues.

### 2025-05-27: Comprehensive Documentation Review Reveals Priorities
Insight: Reviewing all 41 project *.md files revealed that security and configuration issues are as critical as functionality
Impact: REQUIREMENTS.md now comprehensive with security requirements (NFR-7.6-7.8), enhanced config management (DR-13.5-13.7), and physics modeling details (FER-19.5-19.8). Highest priority remains user's functionality concern.

### 2025-05-27: Code Organization Improves Maintainability
Insight: Moving unused code to dedicated modules (unused.rs, utils.rs) rather than deleting preserves potentially useful functions while keeping main code clean
Impact: Zero warnings achieved without losing code that might be needed for future features like enhanced multi-lap handling or route discovery improvements

### 2025-05-27: Production Deployment Success
Insight: Achieving <20% accuracy unlocked production readiness - users trust predictions that are "close enough"
Impact: README updated with "PRODUCTION READY" status and 16.1% accuracy achievement. Shows importance of realistic accuracy targets over perfection.

## Recent Discoveries (from PROJECT_WISDOM_RECENT.md)

### Configuration Management Success (2025-05-27)
Insight: Multi-level config with environment overrides provides maximum flexibility
Impact: Users can configure via files, env vars, or wrapper scripts
Key Pattern: env → local → ~/.config → ~/.local/share → defaults

### Data Directory Strategy (2025-05-27)
Insight: ~/.local/share/ survives system updates better than ~/.config/
Impact: User settings persist across tool updates
Lesson: Separate volatile config from persistent user data

### TOML Over JSON (2025-05-27)
Insight: TOML more readable for end users with clear comments
Impact: Better user experience for configuration files
Trade-off: Slightly more complex parsing but worth it

### Testing Configuration Systems (2025-05-27)
Insight: Config systems need real-world testing beyond unit tests
Impact: Discovered edge cases with default value handling
Pattern: Create test config → Run tool → Verify behavior → Test env overrides

### Documentation Types Matter (2025-05-27)
Insight: Different audiences need different documentation
Impact: Created CONFIG_MANAGEMENT.md (users) vs SIMULATION_TOOLS.md (developers)
Best Practice: Clear examples and security warnings in user docs

### Secure Storage Design Pattern (2025-05-27)
Insight: Support multiple storage backends with automatic fallback (env → keyring → file)
Impact: Users get best available security without configuration burden

### Backward Compatibility First (2025-05-27)
Insight: Security improvements should never break existing workflows
Impact: New secure scripts alongside originals, migration at user's pace

### Environment Variables for CI/CD (2025-05-27)
Insight: CI/CD environments need stateless token storage
Impact: Env vars as primary option enables GitHub Actions integration

### Racing Score vs Traditional Categories (2025-05-26)
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D and Racing Score (0-650)
Impact: Filtering logic excluded half the events due to `distanceInMeters: 0`

### Distance in Description Text (2025-05-26)
Insight: Racing Score events embed distance in description ("Distance: 23.5 km")
Impact: Parse description when API fields are empty

### Route ID Without Data (2025-05-26)
Insight: Events have routeId but no details - websites maintain separate databases
Impact: Local route database strategy validated

### WhatsonZwift Search Technique (2025-05-26)
Insight: Use `site:https://whatsonzwift.com [route]` for accurate route data
Impact: Reliable method for mapping unknown routes

### Zero as API Signal (2025-05-26)
Insight: Some APIs use 0/null to mean "check elsewhere", not actual zero
Impact: Changed filtering to accept 0 and check alternatives

### Field Presence Type Detection (2025-05-26)
Insight: `rangeAccessLabel` presence identifies Racing Score events
Impact: Differentiate event types without explicit type field

### Browser DevTools Power (2025-05-26)
Insight: Browser tools reveal undocumented API behavior quickly
Impact: Found Racing Score pattern in minutes vs hours

### Hierarchical Log Management (2025-05-26)
Insight: Large logs (66KB+) slow LLM loading - use Summary/Recent/Archives
Impact: 13x context reduction while preserving all history# PROJECT_WISDOM.md

## Zwift Race Finder - Key Discoveries

### 2025-05-25: Accuracy Journey Reveals Data Quality Importance
Insight: The path from 92.8% to 25.7% error wasn't about better algorithms - it was about better data
Impact: Single route mapping error (EVO CC) caused 11.2% accuracy degradation. Data quality > algorithm sophistication

### 2025-05-25: Racing Variance is a Feature, Not a Bug
Insight: Same rider, same route can vary 32-86 minutes based on pack dynamics
Impact: Trying to achieve <20% error is futile - the variance is inherent to bicycle racing

### 2025-05-25: Binary States Dominate Zwift Racing
Insight: You're either with the pack (30.9 km/h) or dropped and solo (23.8 km/h) - no middle ground
Impact: Complex physics models fail because Zwift racing is about draft, not watts

### 2025-05-25: Test What You Ship, Not What You Think You Ship
Insight: We were testing estimates against estimates for weeks without realizing it
Impact: Integration with Strava for real race times revealed our entire baseline was wrong

### 2025-05-25: Route IDs > Event Names
Insight: Event names change but route IDs are stable - always use the most stable identifier
Impact: Reduced unknown routes from 50+ to <10 by focusing on route_id mapping

### 2025-05-26: SQLite Correlated Subqueries are a Trap
Insight: SQLite's UPDATE statement limitations force creative solutions - temp tables > complex subqueries
Impact: Hours of debugging could be avoided by using simpler, more SQLite-friendly patterns

### 2025-05-26: 80% Match Rate is Excellent, Not a Problem
Insight: Not all races will be in both systems - 80% matching between ZwiftPower and Strava is actually great
Impact: Stop trying to match everything; focus on having enough good data for accurate predictions

### 2025-05-26: Zwift API Returns Zero Distance for Races
Insight: The Zwift API returns `distance_in_meters: 0.0` for most race events, breaking duration estimation
Impact: This explains why "no races found" - we were filtering out all races with unknown routes because we couldn't estimate their duration without distance data

### 2025-05-26: Fallback Estimation Chain is Critical
Insight: Need multiple fallback strategies: route_id → distance → event name → subgroups
Impact: Makes the tool resilient to incomplete API data - if one method fails, try the next

### 2025-05-26: Default Duration Range Too Narrow
Insight: Default 90-150 minute range misses most races (crits are 40min, Three Village Loop is 77min)
Impact: Users need guidance on using tolerance parameter for better results (-t 60 catches most races)

### 2025-05-26: Stop and Plan When Confused
Insight: When a "fix" stops working mysteriously, resist the urge to add more debug prints - create a systematic investigation plan first
Impact: Saves time by avoiding random changes and ensures thorough understanding of the problem

### 2025-05-26: Racing Score vs Traditional Categories Discovery
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D categories and Racing Score ranges (0-650)
Impact: Our filtering logic was excluding all Racing Score events because they have distanceInMeters: 0 in the API

### 2025-05-26: Distance Data Location for Racing Score Events
Insight: Racing Score events return 0 for distance in API but embed it in description text ("Distance: 23.5 km")
Impact: We need to parse description text for these events instead of relying on distanceInMeters field

### 2025-05-26: Route ID Without Route Data
Insight: Events have routeId (e.g., 3379779247) but no route details in API response - webpage must have separate route database
Impact: Our local route database strategy is correct; we just need to map more route IDs

### 2025-05-26: Finding Route Data via WhatsonZwift Search
Insight: Use site search `site:https://whatsonzwift.com [Event Name]` to find route details when mapping unknown routes
Impact: Reliable method for finding route distance/elevation data - e.g., found Three Village Loop details this way

### 2025-05-26: API Design Patterns - Zero Means "Look Elsewhere"
Insight: Some APIs use 0 or null values as signals to check other fields - not actual zero values
Impact: Changed our filtering logic to accept 0 distance and check other data sources (description, route DB)

### 2025-05-26: Multiple Event Type Detection via Field Presence
Insight: Presence of specific fields (like rangeAccessLabel) can identify event variants when type field is missing
Impact: Successfully differentiated Racing Score from Traditional events, enabling proper handling of each

### 2025-05-26: Browser DevTools for API Reverse Engineering
Insight: When documentation is lacking, browser developer tools can reveal API behavior and data structures
Impact: Discovered the Racing Score event pattern and API endpoint structure in minutes vs hours of guessing

### 2025-05-26: Hierarchical Log Management for LLM Context Efficiency
Insight: Large log files (66KB+) slow LLM loading - hierarchical structure (Summary/Recent/Archives) reduces context to <5KB
Impact: 13x reduction in loaded context while preserving all historical data - pattern applicable to any growing log file# Project Wisdom Session: 2025-05-27

## Session: Manual Route Mappings for Custom Events

This session focused on creating manual route mappings for high-frequency custom race events that couldn't be automatically discovered.

### Key Accomplishments
- Researched 9 high-frequency race series to understand their typical routes and durations
- Created `manual_route_mappings.sql` with SQL scripts to map custom events to appropriate routes
- Created `ROUTE_MAPPING_RESEARCH.md` documenting findings for each race series
- Applied mappings successfully, reducing unmapped events from 112 to 104
- Fixed critical EVO CC mapping error that was causing 72% prediction errors

### Discoveries

#### Custom Events vs Route Names
Most "unknown routes" are actually custom event names (club races, team events) that use standard routes but with custom branding. These require manual mapping rather than automated discovery.

#### Route Length Critical for Accuracy
Initial mapping of EVO CC to Volcano Flat (12.1km) caused massive errors because EVO CC races are 60-90 minutes. Correcting to Watopia's Waistband (40.8km) improved accuracy from 43.9% to 34.0% error rate.

#### Race Series Patterns
- **EVO CC**: Rotates routes weekly, 60-90 minute races
- **Sydkysten**: Consistent 29.6km distance
- **Tofu Tornado**: Variable format (regular 32km vs XL 70km)
- **CAT & MOUSE**: Chase format with 40km distance
- **DBR**: Danish series using Zwift Racing Score
- **ZHR Morning Tea**: London crit, 49km
- **TT Club Watopia's Waistband**: Specific route, 25.4km

### Technical Details

Created placeholder route IDs for series without confirmed Zwift route_ids:
```sql
-- Placeholder routes for future discovery
(9001, 29.6, 150, "Sydkysten Typical Route", "Various", "road"),
(9002, 40.0, 200, "CAT & MOUSE Chase Route", "Various", "road"),
(9003, 49.0, 300, "ZHR Morning Tea Route", "London", "road")
```

Key SQL mapping example:
```sql
-- EVO CC Race Series - corrected to longer route
UPDATE race_results 
SET route_id = 2363965193  -- Watopia's Waistband 40.8km
WHERE event_name LIKE 'EVO CC%Race Series%';
```

### Impact on Accuracy
- Regression test error improved from 43.9% to 34.0%
- Still above 30% target but significant progress
- Demonstrates importance of matching route distance to typical race duration

### Next Session Priority
Research and map remaining high-frequency unmapped events:
- "Restart Monday Mash" (55 occurrences)
- "TEAM VTO POWERPUSH" (37 occurrences)
- "The Bump Sprint Race" (27 occurrences)

Consider implementing date-based mapping for rotating series like EVO CC.