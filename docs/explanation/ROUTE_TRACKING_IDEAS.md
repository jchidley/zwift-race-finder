# Route Tracking Ideas from ZwiftHacks

## Discovered Features

### 1. Route Completion Tracking (HIGH VALUE)
**What ZwiftHacks Does:**
- Checkbox system to mark routes as completed
- No user account required - uses browser local storage
- Optional sync to server with unique URL
- Shows completion badges in event listings

**Implementation Ideas for Zwift Race Finder:**
```bash
# Track completed routes locally
zwift-race-finder --mark-complete 3765339356
zwift-race-finder --show-progress

# Generate personal tracking URL
zwift-race-finder --generate-tracking-url
# Output: Your tracking URL: zrf-track-a8f3b2c9d4e5f6

# Sync from URL
zwift-race-finder --sync-from zrf-track-a8f3b2c9d4e5f6
```

**Database Schema:**
```sql
CREATE TABLE route_completion (
    route_id INTEGER PRIMARY KEY,
    completed_at TIMESTAMP,
    actual_time_minutes INTEGER,
    notes TEXT
);

CREATE TABLE tracking_sync (
    sync_key TEXT PRIMARY KEY,
    last_sync TIMESTAMP,
    route_data TEXT -- JSON of completed routes
);
```

### 2. Route Statistics Display (MEDIUM VALUE)
**What ZwiftHacks Shows:**
- Distance, elevation, XP rewards
- Climb difficulty rating
- Lead-in distance
- Route type (event only, always available)

**Enhancement for Our Tool:**
- Show route statistics in event listings
- Calculate total XP possible from matching events
- Highlight "new routes" user hasn't completed

### 3. Route Filtering by Availability (MEDIUM VALUE)
**What ZwiftHacks Does:**
- Filter by "Always Open" vs "Event Only"
- Show which routes need events to access

**Implementation Ideas:**
```bash
# Show only events with rare routes
zwift-race-finder --route-type event-only

# Show events with routes you haven't completed
zwift-race-finder --new-routes-only
```

### 4. Badge Progress Integration (HIGH VALUE)
**What ZwiftHacks Does:**
- Shows badge icon for routes with achievements
- Tracks which badges you've earned

**Implementation Ideas:**
- Add badge column to routes table
- Show "🏅 Badge Available" in event listings
- Track badge completion separately from route completion

### 5. World-Based Organization (LOW VALUE)
**Current State:** We already filter by world in route discovery

**Enhancement:** Could add world statistics:
```bash
zwift-race-finder --world-stats
# Output: Watopia: 45/67 routes completed (67%)
#         France: 12/18 routes completed (66%)
```

## Proposed Implementation Priority

### Phase 1: Basic Route Tracking
1. Add `route_completion` table
2. Implement `--mark-complete` and `--show-progress` commands
3. Show completion status in event listings with ✓ mark

### Phase 2: Sync Capability
1. Generate unique sync keys
2. Store completion data as JSON
3. Allow import/export of progress

### Phase 3: Enhanced Filtering
1. Add `--new-routes-only` filter
2. Add `--route-type` filter for event-only routes
3. Calculate and show potential XP gains

### Phase 4: Statistics and Gamification
1. Progress bars for each world
2. "Routes discovered this month" stats
3. Achievements for milestones (50%, 75%, 100% completion)

## Privacy Considerations
Following ZwiftHacks' approach:
- No personal data required
- Optional sync with anonymous keys
- Local-first storage
- Clear data ownership

## Integration with Existing Features
- Route completion affects duration estimates (familiar routes = more accurate predictions)
- Completed routes could have historical average times
- "Personal best" tracking for each route
- Suggest events with routes close to completion (e.g., "Only 3 routes left in France!")

## CLI Examples
```bash
# Mark route as completed after a race
zwift-race-finder --record-result "3765339356,38,Glasgow Crit" --mark-complete

# Show progress summary
zwift-race-finder --show-progress
# Output: Overall: 127/203 routes (63%)
#         Watopia: 45/67 ✓✓✓✓✓✓✓░░░
#         France: 12/18 ✓✓✓✓✓✓✓✓░░

# Find events with new routes
zwift-race-finder -d 60 -t 30 --new-routes-only

# Export progress for backup
zwift-race-finder --export-progress > my-routes-backup.json
```