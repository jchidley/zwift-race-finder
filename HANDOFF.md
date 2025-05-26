# Project: Zwift Race Finder
Updated: 2025-05-27 00:00 UTC

## Current State
Status: World detection + route ID extraction complete and tested
Target: Implement batch discovery to handle 185 unknown routes efficiently
Latest: Successfully reducing API calls by ~10x per route with world detection

## Essential Context
- World detection working perfectly (e.g., "STAGE 3: RACE MAKURI" → makuri-islands)
- Real route IDs extracted from whatsonzwift.com (no more 9999 placeholders)
- 185 unknown routes × 500ms = ~8 minutes (timeouts at 2 minutes)
- Cache + world detection create multiplicative performance gains
- High-frequency events: "Restart Monday Mash" (51), "TEAM VTO" (37)

## Next Step
Implement batch discovery: process 10-20 routes at a time with progress saving

## If Blocked
Manual mapping for top 10 high-frequency events would cover ~250 occurrences

## Related Documents
- todo.md - Active tasks (batch discovery needed)
- PROJECT_WISDOM.md - Technical insights and patterns
- CLAUDE.md - Project-specific AI instructions
- ZWIFT_API_LOG_RECENT.md - Latest API discoveries
- sessions/ZWIFT_API_LOG_SESSION_20250526_005.md - Latest work log (world detection)
- plan.md - Project roadmap and architecture
- README.md - User documentation