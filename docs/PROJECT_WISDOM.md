# Project Wisdom

Knowledge and insights discovered during development of Zwift Race Finder.
Older insights archived to PROJECT_WISDOM_ARCHIVE_20250602.md

## Active Insights

### 2025-06-02: Lead-in Distance Critical for Accuracy
Insight: Lead-in distance varies by event type (race vs free ride vs meetup) and can be significant (0.2-5.7 km)
Impact: Ignoring lead-in was causing systematic underestimation of race duration
Key Learning: Always check for "hidden" distance components in racing/sports applications

### 2025-06-02: Database Schema Evolution Pattern
Insight: Adding columns requires updating ALL database access points - queries, inserts, tests, and seed data
Impact: Test failures revealed incomplete migration despite main code working
Best Practice: Create migration checklist: schema → struct → queries → inserts → tests → seeds

### 2025-06-02: External Data Import Strategy
Insight: Large reference datasets (zwift-data) best imported via script rather than hardcoding
Impact: 264 routes imported with accurate data, maintainable via re-run
Pattern: Parse external format → map to internal schema → upsert with conflict handling

### 2025-06-02: URL Generation from Slugs
Insight: Route slugs enable external service integration without APIs
Impact: Users get direct links to detailed route information on WhatsOnZwift
Design: Store slugs during import, generate URLs on display

### 2025-05-27: Configuration Management Success
Insight: Multi-level config with environment overrides provides maximum flexibility
Impact: Users can configure via files, env vars, or wrapper scripts
Key Pattern: env → local → ~/.config → ~/.local/share → defaults

### 2025-05-27: Secure Storage Design Pattern
Insight: Support multiple storage backends with automatic fallback (env → keyring → file)
Impact: Users get best available security without configuration burden

### 2025-05-26: Racing Score vs Traditional Categories
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D and Racing Score (0-650)
Impact: Filtering logic excluded half the events due to `distanceInMeters: 0`

### 2025-05-26: Zero as API Signal
Insight: Some APIs use 0/null to mean "check elsewhere", not actual zero
Impact: Changed filtering to accept 0 and check alternatives

### 2025-05-26: Field Presence Type Detection
Insight: `rangeAccessLabel` presence identifies Racing Score events
Impact: Differentiate event types without explicit type field

### 2025-05-26: Browser DevTools Power
Insight: Browser tools reveal undocumented API behavior quickly
Impact: Found Racing Score pattern in minutes vs hours

### 2025-06-02: WhatsOnZwift Data Sources Discovery
Insight: WhatsOnZwift has permission from Zwift to display route/workout data but provides no public API
Impact: Third-party tools must parse web pages or use indirect sources like zwift-data npm package
Key Learning: Popular services often have special agreements unavailable to indie developers
Details:
- Zwift's developer API requires special accounts not available to hobby developers
- WhatsOnZwift likely has privileged access through their partnership
- Our approach: Use zwift-data package + public Zwift endpoints + manual curation
- Web scraping tools exist (wozzwo) but check ToS first

### 2025-06-02: Strava as Accurate Zwift Route Data Source
Insight: Zwift exports completed rides to Strava with accurate route information embedded
Impact: Strava activities provide ground truth data for route details, distances, and elevation
Key Learning: Sometimes the best API is an indirect one - use data exhaust from integrations
Details:
- Zwift automatically syncs rides to Strava (when connected)
- Strava activities contain actual route names, distances, elevation profiles
- Can download activity data via Strava API with proper authentication
- Our project uses this: strava_auth.sh → strava_fetch_activities.sh → import to DB
- This gives us real-world validation data for route predictions

### 2025-06-02: Club Events as Route Discovery Tool
Insight: Zwift Companion app allows creating club events with specific routes for controlled testing
Impact: Can systematically map all routes by creating events and analyzing the exported data
Key Learning: When APIs are locked down, create your own data through controlled experiments
Details:
- Club owners can create events on any free-ride route
- Participants' rides export to Strava/Garmin with full route data
- Captures actual distances including lead-in variations
- Different event types (race/ride/meetup) may have different lead-ins
- Strategy: Create "Route Discovery" club events to map entire Zwift ecosystem

### 2025-06-02: WhatsOnZwift Route URL Patterns
Insight: WhatsOnZwift has accurate route data but reverse routes don't have separate pages
Impact: Need to map reverse routes to their forward equivalents for data lookup
Key Learning: External data sources may have different organization than internal models
Details:
- URL pattern: https://whatsonzwift.com/world/{world}/route/{slug}
- Reverse routes (e.g., "hilly-route-rev") must map to base route ("hilly-route")
- Data includes: distance, elevation, lead-in distance, lead-in elevation
- Most comprehensive public source for Zwift route data