# Project Wisdom Session: 2025-05-27

## Session: Manual Route Mappings for Custom Events

This session focused on creating manual route mappings for high-frequency custom race events that couldn't be automatically discovered.

### Key Accomplishments
- Researched 9 high-frequency race series to understand their typical routes and durations
- Created `manual_route_mappings.sql` with SQL scripts to map custom events to appropriate routes
- Created `ROUTE_MAPPING_RESEARCH.md` documenting findings for each race series
- Applied mappings successfully, reducing unmapped events from 112 to 104
- Fixed critical EVO CC mapping error that was causing 72% prediction errors

### Discoveries

#### Custom Events vs Route Names
Most "unknown routes" are actually custom event names (club races, team events) that use standard routes but with custom branding. These require manual mapping rather than automated discovery.

#### Route Length Critical for Accuracy
Initial mapping of EVO CC to Volcano Flat (12.1km) caused massive errors because EVO CC races are 60-90 minutes. Correcting to Watopia's Waistband (40.8km) improved accuracy from 43.9% to 34.0% error rate.

#### Race Series Patterns
- **EVO CC**: Rotates routes weekly, 60-90 minute races
- **Sydkysten**: Consistent 29.6km distance
- **Tofu Tornado**: Variable format (regular 32km vs XL 70km)
- **CAT & MOUSE**: Chase format with 40km distance
- **DBR**: Danish series using Zwift Racing Score
- **ZHR Morning Tea**: London crit, 49km
- **TT Club Watopia's Waistband**: Specific route, 25.4km

### Technical Details

Created placeholder route IDs for series without confirmed Zwift route_ids:
```sql
-- Placeholder routes for future discovery
(9001, 29.6, 150, "Sydkysten Typical Route", "Various", "road"),
(9002, 40.0, 200, "CAT & MOUSE Chase Route", "Various", "road"),
(9003, 49.0, 300, "ZHR Morning Tea Route", "London", "road")
```

Key SQL mapping example:
```sql
-- EVO CC Race Series - corrected to longer route
UPDATE race_results 
SET route_id = 2363965193  -- Watopia's Waistband 40.8km
WHERE event_name LIKE 'EVO CC%Race Series%';
```

### Impact on Accuracy
- Regression test error improved from 43.9% to 34.0%
- Still above 30% target but significant progress
- Demonstrates importance of matching route distance to typical race duration

### Next Session Priority
Research and map remaining high-frequency unmapped events:
- "Restart Monday Mash" (55 occurrences)
- "TEAM VTO POWERPUSH" (37 occurrences)
- "The Bump Sprint Race" (27 occurrences)

Consider implementing date-based mapping for rotating series like EVO CC.