# Project: Zwift Race Finder
Updated: 2025-06-01 23:28 UTC

## Current State
Status: Fixed missing race issue, added route name extraction requirements
Target: Enhance route discovery using event titles and descriptions
Latest: Added "Sacre Bleu" route (71.2km) to database - "Zwift Epic Race - Sacre Bleu" now shows

## Essential Context
- Route ID 136140280 was missing from database, causing event filtering
- Web search found "Sacre Bleu" route details (71.2 km, 396m elevation)
- Added requirements FER-19.9.1-6 for extracting route names from event titles
- Common patterns: "Event - Route Name", "Event: Route Name", "Route: name" in descriptions
- Tool currently requires exact route ID match or manual mapping

## Next Step
Implement route name extraction to automatically find routes from event titles/descriptions

## If Blocked
Manual route mapping via SQL still works as fallback

## Failed Approaches
None recently - web search successfully found route data

## Related Documents
- REQUIREMENTS.md - Updated with route name extraction requirements (FER-19.9)
- todo.md - Active tasks and project status
- PROJECT_WISDOM.md - Technical insights and patterns
- CLAUDE.md - Project-specific AI instructions