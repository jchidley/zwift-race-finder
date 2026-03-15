# ZWIFT API LOG SESSION - 2025-05-27 002
# Web Research for High-Frequency Event Mapping

## Session 2025-05-27: Manual Route Discovery via Web Search

Discovered that high-frequency community events often have their own websites mentioned in event descriptions. This led to successful mapping of three major event series.

### Key Accomplishments
- Mapped "Restart Monday Mash" (55x) → Mountain Mash route (5.7km, 335m elevation)
- Mapped "TEAM VTO POWERPUSH" (37x) → Tempus Fugit route (18.9km, 16m elevation)
- Mapped "The Bump Sprint Race" (27x) → Tick Tock route (19.2km, 59m elevation)
- Added all three routes to database with proper specifications
- Updated manual_route_mappings.sql with discovered routes
- Documented findings in ROUTE_MAPPING_RESEARCH.md

### Discoveries
- Event descriptions often contain organizer websites with detailed route information
- Web searches for "[event name] zwift route" highly effective for community events
- Many "unknown routes" are actually using standard Zwift routes with custom event names
- Route IDs from unknown_routes table often match the actual routes being used

### Technical Details

#### Mountain Mash (Restart Monday Mash)
- Route ID: 1917017591
- Used for Zwift Games Categories C&D climbing stage
- Starts in Jungle Pens, goes up Epic KOM reverse
- Duration: 18-32 minutes depending on w/kg

#### Tempus Fugit (TEAM VTO POWERPUSH)
- Route ID: 2128890027
- Flattest route in Zwift (only 16m elevation over 18.9km)
- Format: 40min warm-up + 7.1km Team Time Trial segment
- Organized by Team Virtual Training Oceania

#### Tick Tock (The Bump Sprint Race)
- Route ID: 3366225080
- One-lap race format, "drag race from the start"
- Organized by Team Endurance Nation
- Designed to help riders improve threshold power

### Implementation
```sql
-- Added to manual_route_mappings.sql
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(1917017591, 5.7, 335, "Mountain Mash", "Watopia", "road"),
(2128890027, 18.9, 16, "Tempus Fugit", "Watopia", "road"),
(3366225080, 19.2, 59, "Tick Tock", "Watopia", "road");
```

### Next Session Priority
- Analyze which of Jack's 104 unmapped races are causing high prediction errors
- Focus on mapping those specific races to achieve <30% accuracy target
- Consider additional web searches for remaining high-frequency events