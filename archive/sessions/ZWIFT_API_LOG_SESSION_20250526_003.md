# ZWIFT API LOG - Session 2025-05-26 #3: Event Description Parsing

## Session Overview
Implemented comprehensive event description parsing to extract route names and lap counts from event descriptions, solving the problem where most "unknown routes" are actually custom event names containing real route information.

## Key Accomplishments

### 1. Description Parsing Implementation
Created `parse_route_from_description()` in `route_discovery.rs` with 5 regex patterns:
- "X laps of Route Name" → `3 laps of Volcano Circuit`
- "Route Name x N" → `Mountain Route x 2`  
- "Nx Route Name" → `2x Bell Lap`
- "Route Name (N laps)" → `Three Village Loop (3 laps)`
- "Stage X: Route Name" → `Stage 4: Makuri May` (assumes 1 lap)

### 2. Enhanced Unknown Route Logging
Modified `log_unknown_route()` to parse descriptions before logging:
```rust
fn log_unknown_route(event: &ZwiftEvent) {
    if let Some(parsed) = route_discovery::parse_route_from_description(&description) {
        // Log as: "Event Name -> Route Name (N laps)"
        let event_name_with_route = format!("{} -> {} ({} laps)",
            event.name, parsed.route_name, parsed.laps);
    }
}
```

### 3. Database Enhancements
- Added `get_route_by_name()` for name-based route lookups
- Created `get_route_data_enhanced()` to try description parsing as fallback
- Ready for integration into main duration estimation

### 4. New CLI Feature
Added `--analyze-descriptions` to batch analyze event descriptions:
- Fetches current events from API
- Parses all descriptions for route patterns
- Shows frequency-sorted results
- Helps identify most common route patterns

## Testing Results
All parsing patterns tested and working:
- ✅ Basic patterns parsing correctly
- ✅ Real example: "Stage 4: Makuri May: Three Village Loop || Advanced" 
     → "Stage 4: Makuri May: Three Village Loop || Advanced -> Makuri Three Village Loop (1 laps)"
- ✅ All 9 new tests passing (route_discovery module tests)

## Impact on Unknown Routes
Running `--show-unknown-routes` now shows enhanced entries that reveal actual routes:
- Old: `3379779247 | 68 | Stage 4: Makuri May: Three Village Loop || Advanced`
- New: `3379779247 | 68 | Stage 4: Makuri May: Three Village Loop || Advanced -> Makuri Three Village Loop (1 laps)`

This immediately shows what route is actually being used in custom-named events!

## Technical Details

### Code Architecture
```rust
// Parsing result structure
pub struct ParsedEventDescription {
    pub route_name: String,
    pub laps: u32,
}

// Enhanced route lookup combining ID and name parsing
fn get_route_data_enhanced(event: &ZwiftEvent) -> Option<(u32, f64)> {
    // First try direct route_id lookup
    if let Some(route_id) = event.route_id {
        if let Some(route_data) = get_route_data(route_id) {
            return Some((route_id, route_data.distance_km));
        }
    }
    
    // Fallback to description parsing
    if let Some(parsed) = parse_route_from_description(&description) {
        if let Ok(Some(route)) = db.get_route_by_name(&parsed.route_name) {
            let total_distance = route.distance_km * parsed.laps as f64;
            return Some((route.route_id, total_distance));
        }
    }
}
```

### Key Files Modified
- `src/route_discovery.rs`: Added parsing function and tests
- `src/main.rs`: Enhanced logging, added CLI option, created enhanced lookup
- `src/database.rs`: Added `get_route_by_name()` function

## Next Session Priority
1. **Integrate `get_route_data_enhanced()` into main filtering logic** - Currently created but not used
2. Handle duplicate route names (e.g., Innsbruck KOM After Party has IDs 13 & 1431)
3. Test with live data to measure accuracy improvement
4. Consider fuzzy string matching for partial route name matches

## Lessons Learned
- Event descriptions contain valuable structured data often overlooked
- Regex patterns can reliably extract route information from natural language
- Most "unknown routes" aren't unknown - they're custom event names wrapping known routes
- Clean architecture (separate parsing from business logic) enables easy testing