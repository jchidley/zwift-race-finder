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