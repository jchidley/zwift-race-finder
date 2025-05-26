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