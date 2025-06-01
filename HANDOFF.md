# Project: Zwift Race Finder
Updated: 2025-01-06 20:19 UTC

## Current State
Status: Lap handling working; user notes description field has distance/elevation
Target: Understand how companion app shows distance/elevation for all events
Latest: User emphasized description field contains race data (not API fields)

## Essential Context
- Lap calculation working: Shows "21.2 km (2 laps)" correctly
- API returns 0.0 for distance in Racing Score events
- We calculate from route DB + lap count successfully
- User: "description field often contains this data" (distance/elevation)
- Need to parse description for complete race information

## Next Step
Parse event descriptions to extract distance/elevation data

## If Blocked
None - need to examine description field contents

## Related Documents
- todo.md - Active tasks and project status
- REQUIREMENTS.md - Comprehensive requirements document
- PROJECT_WISDOM.md - Technical insights and patterns
- CLAUDE.md - Project-specific AI instructions
- sessions/ZWIFT_API_LOG_SESSION_20250527_*.md - Recent work sessions