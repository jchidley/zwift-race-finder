# Project Wisdom

## Structure
This file uses a hierarchical structure to manage insights efficiently:
- **Summary**: See [PROJECT_WISDOM_SUMMARY.md](./PROJECT_WISDOM_SUMMARY.md) for key universal insights (< 2KB)
- **Recent**: See [PROJECT_WISDOM_RECENT.md](./PROJECT_WISDOM_RECENT.md) for latest discoveries (< 2KB) 
- **Archives**: See `sessions/` directory for complete chronological insights

## Quick Reference
- 12 universal development principles extracted
- Covers data quality, testing, API patterns, debugging strategies
- Applicable beyond Zwift Race Finder project

## Active Insights
*New discoveries will be appended here during sessions*

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
Impact: 16.1% error is sufficient for real-world use. Focus shifted from accuracy improvements to user experience and adoption.

### 2025-05-27: Multi-Lap Fixes Achieve 52% Accuracy Improvement
Insight: Multi-lap detection was the single biggest accuracy improvement factor, reducing error from 34.0% to 16.1%
Impact: Exceeded <20% target by significant margin. Production ready with 16.1% mean absolute error across 46 races.

### 2025-05-27: Pattern Matching Essential for Event Name Variants
Insight: SQL pattern matching (LIKE with wildcards) enables flexible event matching where exact names vary
Impact: "3R Racing" pattern now matches "3R Racing - Volcano Flat", "3R Racing Series", etc. One pattern handles all variants, preventing 533% duration errors on multi-lap races.

### 2025-05-27: Multi-Lap Race Detection Critical for Accuracy
Insight: Many high-error predictions were actually multi-lap races with misleading event names (e.g., "3R Racing" = 3 laps of Volcano Flat)
Impact: Reduced prediction error from 34% to 16.1% by creating multi_lap_events table and checking for known patterns. Simple database lookup beats complex parsing.

### 2025-05-26: Web Scraping Architecture Pattern
Insight: Separate discovery mechanism from business logic for flexibility
Impact: When Google search failed, only needed to modify search function, not entire discovery flow. Clean architecture = easy pivots.

### 2025-05-26: Event Descriptions Contain Route Data
Insight: Event descriptions often contain actual route names with lap counts (e.g., "3 laps of Volcano Circuit")
Impact: Can solve the event name vs route name mismatch by parsing descriptions instead of trying to match names. The data we need was there all along, just in a different field.

### 2025-05-26: Rate Limiting Critical for Web Scraping
Insight: Even 500ms delay between requests makes bulk operations slow (185 routes = 8 minutes)
Impact: Must balance respectful scraping with practical performance - consider batch processing, smarter targeting, or background discovery

### 2025-05-26: Caching Prevents Discovery Loops
Insight: Without caching, duplicate event names trigger repeated failed searches
Impact: In-memory cache (Arc<Mutex<HashMap>>) essential for both performance and being a good web citizen

### 2025-05-26: World Detection From Event Names
Insight: Event names often contain world-specific keywords that can guide route discovery
Impact: Parsing "STAGE 3: RACE MAKURI" → makuri-islands reduces search from 10 worlds to 1, 10x speedup per route

### 2025-05-26: Route IDs Hidden in JavaScript
Insight: whatsonzwift.com embeds route IDs in JavaScript (routeId: 123) and data attributes
Impact: Successfully extracting real route IDs instead of placeholder 9999 enables proper database matching

### 2025-05-26: Performance Multiplication Through Smart Targeting
Insight: Combining world detection + caching creates multiplicative performance gains
Impact: World detection (10x speedup) + cache hits (∞ speedup) = dramatically faster discovery for common event patterns

### 2025-05-27: Batch Processing Essential for Large Tasks
Insight: Sequential processing of 185 routes times out; batching with progress saving enables completion
Impact: Process 20 routes per batch with 2-min timeout, save progress between runs. Users can chip away at large discovery tasks.

### 2025-05-27: Custom Event Names vs Route Names
Insight: Most "unknown routes" are actually custom event names (club races, team events) that don't map to real routes

### 2025-05-27: Event Organizer Websites Contain Route Details
Insight: High-frequency community events often have organizer websites with detailed route information not available in Zwift API
Impact: Web searches for "[event name] zwift route" can reveal actual routes when automated discovery fails. Found Mountain Mash, Tempus Fugit, and Tick Tock routes this way.
Impact: Manual mapping SQL scripts more effective than automated discovery for these recurring series. Database shows patterns.

### 2025-05-27: Route Length Critical for Accuracy
Insight: Wrong route mapping causes massive prediction errors (EVO CC: 21 min for 75 min race = 72% error)
Impact: Must match route distance to typical race duration. EVO CC needed 40.8km route, not 12.1km. Always verify with actual race times.
Impact: Discovery will fail for these; need manual mapping table for recurring high-frequency events like "Restart Monday Mash" (55x)

### 2025-01-06: Description Parsing Patterns for Racing Score Events
Insight: Zwift API returns 0.0 for distance in Racing Score events, but includes the data in description text
Impact: Implemented parse_description_data() to extract "Distance: X km/miles", "Elevation: Y m/ft", and "N laps" patterns. Supports both metric and imperial units with automatic conversion. This complements route database lookups for complete race information.

### 2025-01-06: Development Testing Strategies
Insight: DNS connectivity issues can prevent API testing during development
Impact: Consider creating offline test data fixtures or mock API responses for development. Real API testing should be separate from unit tests to ensure development can continue without network dependencies.