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