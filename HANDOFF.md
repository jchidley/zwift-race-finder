# Handoff - 2025-05-26

## Current Status
Successfully enhanced route discovery module with:
1. **Caching**: In-memory cache prevents repeated API calls for same events
2. **Rate Limiting**: 500ms delay between requests to be respectful
3. **Optimized Search**: Reduced worlds to check from 10 to 5 most common
4. **Better Logging**: Shows search progress for debugging

The tool now successfully finds races using known routes:
- Found and displays "Stage 4: Makuri May: Three Village Loop" races
- Correctly estimates 20 minutes for Cat D riders
- Database integration working properly

## What Works
✅ Route parsing from descriptions (extracts route name and lap count)
✅ Database schema for tracking discovery attempts  
✅ Web scraping with caching and rate limiting
✅ Integration with main race finder - known routes work perfectly
✅ Unknown route logging for manual investigation

## Recent Improvements
- Added Arc<Mutex<HashMap>> cache to RouteDiscovery
- Implemented 500ms delay between HTTP requests
- Reduced worlds to check: watopia, makuri-islands, london, new-york, france
- Fixed database cleanup (removed 20+ already-known routes from unknown_routes table)

## Known Issues
1. **Discovery Timeout**: Full discovery of 185 unknown routes takes too long
   - Each route checks 5 worlds = 925 HTTP requests
   - With 500ms delay = 7.7 minutes minimum
   - Need better heuristics to guess correct world

2. **Route ID Placeholder**: Discovered routes use placeholder ID 9999
   - Need to extract actual route_id from whatsonzwift URLs
   - Or use a better unique identifier system

## Next Steps (Priority Order)
1. **Improve World Detection**
   - Parse world hints from event names (e.g., "Makuri May" → makuri-islands)
   - Check event descriptions for world clues
   - Build mapping of common event series to worlds

2. **Batch Discovery Process**  
   - Run discovery in smaller batches (10-20 routes at a time)
   - Add progress saving between batches
   - Consider background/async discovery

3. **Extract Real Route IDs**
   - Parse route_id from whatsonzwift.com URLs
   - Or implement own ID generation scheme
   - Update database schema if needed

4. **Manual Mapping Documentation**
   - Create guide for adding routes manually
   - Document common patterns that fail discovery
   - Build curated list of important routes

## Related Documents
- todo.md - Active tasks
- PROJECT_WISDOM.md - Technical insights and patterns
- CLAUDE.md - Project-specific AI instructions
- ZWIFT_API_LOG_RECENT.md - Latest API discoveries
- plan.md - Project roadmap and architecture
- README.md - User documentation