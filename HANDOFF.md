# Project: Zwift Race Finder
Updated: 2025-05-26 16:40 UTC

## Current State
Status: Route discovery pivoting to event description parsing
Target: Extract route names from event descriptions (e.g., "3 laps of Volcano Circuit")
Latest: Identified that event descriptions likely contain actual route names with lap counts

## Essential Context
- Direct site APIs work but event names ≠ route names
- Key Insight: Event descriptions often mention actual routes
- Example patterns: "3 laps of [Route]", "[Route] x 2", "Stage 4: [Route]"
- This could solve 189 unknown routes without manual mapping
- Already have description field in API responses

## Next Step
Parse event descriptions with regex to extract route names and lap counts

## If Blocked
Fall back to manual mapping table for high-frequency events

## Related Documents
- todo.md - Active tasks (description parsing now priority)
- route_discovery.rs - Module ready for description parsing addition
- ZWIFT_API_LOG_RECENT.md - Technical implementation details
- CLAUDE.md - Project-specific AI instructions
- PROJECT_WISDOM.md - Technical insights
- plan.md - Project roadmap and architecture
- README.md - User documentation