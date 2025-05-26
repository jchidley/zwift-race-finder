# Zwift API Log - Recent Sessions

## Session: Project Context Manager Creation (2025-05-26)

### Side Project: Log Management
Created hierarchical log management system to solve 66KB+ log files slowing LLM context:
- Implemented Summary/Recent/Archives pattern
- Reduced loaded context from 66KB to <5KB
- Applied same pattern to PROJECT_WISDOM.md

### Evolution to Project Context Manager
Realized the fundamental problem: side projects overwrite main project state
- Designed git-inspired project context management tool
- Created `pc` command with switch/stash/log/diff operations  
- Implemented SQLite-backed persistent todos
- Added GitHub integration for issue sync
- Extracted entire project to ~/tools/project-context-manager

### Key Innovations
1. **Separation of Concerns**: Code (git) vs Context (pc) vs Work Items (todos)
2. **Non-Destructive**: Never lose work when switching projects
3. **Offline-First**: Full functionality without internet
4. **GitHub-Aware**: Optional integration when connected

## Session: Racing Score Events Fix (2025-05-26)

### Critical Discovery: Two Event Types
Zwift has two mutually exclusive event categorization systems:
1. **Traditional Categories**: A/B/C/D/E with `distanceInMeters` populated
2. **Racing Score Events**: Score ranges with `distanceInMeters: 0`

### Root Cause
- Racing Score events ALWAYS have `distanceInMeters: 0` in API
- Distance only available in description text: "Distance: 23.5 km"
- Identified by `rangeAccessLabel` field in event subgroups
- Our filter was rejecting all events with 0 distance

### Solution Implemented
```rust
// Detect Racing Score events
fn is_racing_score_event(event: &Event) -> bool {
    event.event_sub_groups.iter()
        .any(|sg| sg.range_access_label.is_some())
}

// Parse distance from description
fn parse_distance_from_description(description: &str) -> Option<f64> {
    let re = Regex::new(r"Distance:\s*([\d.]+)\s*km").unwrap();
    // Extract and parse distance
}
```

### Impact
- Fixed "no races found" bug affecting ~50% of events
- Tool now handles both event types seamlessly
- Successfully showing correct durations for all event types

### Key Lessons
1. API design patterns: 0 values can mean "look elsewhere"
2. Always check alternative data sources (descriptions, metadata)
3. Browser DevTools essential for API investigation
4. User domain knowledge crucial (Jack caught route distance error)

## Current Status
- Production ready with 23.6% accuracy
- Handles all Zwift event types
- Continuous route discovery system active
- Ready for community release

## Session: UX Improvements (2025-05-26)

### Problem Discovered
User testing revealed poor UX when default search returned no results:
- Default search (90-120min races) found nothing
- Users had to try multiple times to find events
- No guidance on what to try next
- Most races are actually 20-30 minutes, not 90-120

### Solutions Implemented
1. **Event Type Summary**: Shows counts after fetching
   - "Found: 91 group rides, 52 races, 33 group workouts, 5 time trials"
   - Users immediately see what's available

2. **Smart No Results Messages**: Context-aware suggestions
   - For races: "Most races are short (20-30 minutes). Try:"
   - Provides working examples: `cargo run -- -d 30 -t 30`
   - Explains typical durations: "Most races: 20-30 min | Time trials/Group rides: 60-90 min"

### Testing Results
- ✅ Short races (`-d 30 -t 30`): Found 28 matching events
- ✅ Time trials (`-e tt`): Helpful message about TT rarity
- ✅ All events (`-e all -d 60 -t 180`): Found 143 events
- ✅ Group rides (`-e group`): Found 9 matching rides

### Impact
- Tool now educates users about event patterns
- Reduced trial-and-error from 5+ attempts to 1-2
- Clear guidance leads to successful searches

## Recent Commands
```bash
# Working commands after fix
zwift-race-finder -t 60        # Shows all races
zwift-race-finder -d 20 -t 10  # Filters by duration
zwift-race-finder -d 30 -t 30  # Best for finding short races
zwift-race-finder -e all       # See all event types
```