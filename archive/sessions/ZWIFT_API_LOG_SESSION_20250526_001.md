# Zwift API Development Log - 2025-05-26

## Session: Event Filtering Bug Investigation

### Context
After successfully implementing fixes that got the tool working (showing 28 races with `-t 60`), the tool suddenly stopped finding any races. User confirmed that events definitely have distance information.

### Problem Description
- `zwift-race-finder -t 60` returns "No matching events found"
- `zwift-race-finder -d 60 -t 30` also returns no matches
- API returns 200 events with ~45 cycling races
- We had added 20 routes to database
- Tool briefly worked then stopped

### Initial Debug Attempt
Added debug output showing:
```
Debug: First 5 races:
  Name: Stage 4: Makuri May: Three Village Loop
  Route ID: Some(3379779247)
  Distance: Some(0.0) meters
  Duration: None minutes
  Subgroups: 0 groups
```

This revealed races have `distance_in_meters: Some(0.0)`, which we thought we had fixed.

### Key Discovery
The fix we implemented (adding fallback estimation for unknown routes) appeared to work temporarily but something changed. Either:
1. The API data structure changed
2. Our fix has an edge case
3. The test that showed it working was a fluke

### Solution Approach
Instead of adding more debug prints randomly, created a systematic investigation plan:

1. **Ranked Hypotheses**:
   - Duration calculation returns None (most likely)
   - Distance in unexpected field (subgroups?)
   - Route lookup failing
   - Filter logic bug
   - API data changed

2. **Debug Tools to Build**:
   - `--debug-events` flag for comprehensive tracing
   - `--save-events` to capture API responses
   - Show WHY each event is rejected
   - Calculate durations even if filtered

3. **Test Strategy**:
   - Extract filtering to separate functions
   - Unit test each filter independently
   - Test with known good data

### Technical Details
The filtering pipeline:
1. Sport filter (CYCLING only)
2. Event type filter (RACE, TT, etc.)
3. Time window filter (next N days)
4. Duration filter (estimates vs target range)

The duration filter is complex with multiple fallbacks:
- Route ID with distance → estimate
- Route ID alone → use base route distance
- Distance provided → use category estimation
- Event name parsing → guess distance

### Decision Rationale
Rather than continue debugging blindly, we need to:
1. Capture real API data to analyze offline
2. Trace each event through the pipeline
3. Identify exactly where events are rejected
4. Fix with test coverage to prevent regression

### Next Steps
1. Implement `--debug-events` flag following the plan
2. Capture API response for offline analysis
3. Identify root cause
4. Fix with comprehensive tests

### Lessons Learned
- When a "working" fix suddenly breaks, stop and plan
- Systematic debugging beats random changes
- Need better test coverage for API data variations
- Should have event data fixtures for reproducible testing

## Session: Root Cause Discovery (08:00-10:30 UTC)

### The Investigation
Jack provided screenshots from Zwift Companion app showing:
- Filter: Group D, Category Enforcement ON, Race/Time Trial types
- Makuri May Stage 4 event with Racing Score 110-210 (D category)
- Website filters showing Racing Score as separate from A/B/C/D categories

### Browser DevTools Analysis
Used developer console to examine https://www.zwift.com/uk/events/view/4967256:
1. Found API endpoint: `https://us-or-rly101.zwift.com/api/public/events/4967256`
2. API response revealed: **`distanceInMeters: 0`** for Racing Score events
3. Distance only exists in description text: "Distance: 23.5 km"

### Root Cause: Two Mutually Exclusive Event Types

1. **Traditional Category Events**:
   - Uses A/B/C/D/E labels
   - Has `distanceInMeters` populated
   - Shows colored circles on website

2. **Racing Score Events**:
   - Uses score ranges (0-650, split into brackets)
   - **Always has `distanceInMeters: 0`**
   - Has `rangeAccessLabel` field (e.g., "160-270")
   - Distance ONLY in description text

### API Response Structure
```json
{
  "id": 4967256,
  "name": "Stage 4: Makuri May: Three Village Loop",
  "distanceInMeters": 0.0,  // Always 0 for Racing Score events!
  "laps": 2,
  "routeId": 3379779247,
  "categoryEnforcement": true,
  "eventType": "RACE",
  "eventSubgroups": [{
    "subgroupLabel": "D",
    "distanceInMeters": 0.0,  // Also 0 in subgroups
    "laps": 2,
    "rangeAccessLabel": "160-270"  // This identifies Racing Score type
  }]
}
```

### How Zwift Website Shows Distance
The website shows 23.5 km correctly by either:
1. Parsing from description text using regex
2. Having internal route database (10.6 km/lap × 2 + 2.3 km lead-in)
3. Hardcoding known event distances

### Web Search Results
- No existing solutions found online
- No documentation about this API behavior
- Route ID 3379779247 not in any public database
- This appears to be undocumented "feature" not bug

### Solution Implementation Plan
1. **Database Update**:
   ```sql
   INSERT INTO routes VALUES (3379779247, 10.6, 93, 'Three Village Loop', 'Makuri Islands', 'road');
   ```

2. **Code Changes**:
   - Don't filter out events with `distanceInMeters: 0`
   - Check for `rangeAccessLabel` to detect Racing Score events
   - Parse distance from description: `/Distance:\s*([\d.]+)\s*km/`
   - Use route database as fallback

3. **Handle Both Event Types**:
   - Traditional: Use existing logic
   - Racing Score: New parsing logic

### Key Insights
- The "bug" is actually Zwift's API design for Racing Score events
- Our filtering was too strict (requiring distance > 0)
- Need to handle two fundamentally different event structures
- Jack's mobile app handles both types seamlessly

## Session: Racing Score Events Fix Implementation (11:00-12:00 UTC)

### The Fix
After identifying the root cause, Claude implemented a solution:

1. **Added Helper Functions**:
   - `is_racing_score_event()` - Checks for rangeAccessLabel in event subgroups
   - `parse_distance_from_description()` - Extracts distance using regex from description

2. **Fixed Route Data**:
   - Three Village Loop was showing 39.8km (wrong)
   - Updated to correct 10.6km based on Jack's knowledge
   - Jack tip: Use `site:https://whatsonzwift.com` search for accurate data

3. **Updated Event Filtering**:
   - Now accepts events with distanceInMeters: 0 if they're Racing Score events
   - Falls back to description parsing or route database lookup
   - Handles both Traditional and Racing Score event types seamlessly

### Implementation Details
```rust
// Check if Racing Score event
fn is_racing_score_event(event: &Event) -> bool {
    event.event_sub_groups.iter()
        .any(|sg| sg.range_access_label.is_some())
}

// Parse distance from description
fn parse_distance_from_description(description: &str) -> Option<f64> {
    let re = Regex::new(r"Distance:\s*([\d.]+)\s*km").unwrap();
    re.captures(description)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
}
```

### Test Results
- Tool now successfully shows races!
- Three Village Loop races showing 20 minute duration (correct for 10.6km)
- Both Traditional and Racing Score events working

### Technical Learnings
1. **API Design Pattern**: Some APIs use 0 or null as a signal for "look elsewhere"
2. **Multiple Data Sources**: When primary field is empty, check description/metadata
3. **Event Type Detection**: Use presence of specific fields to identify variants
4. **Fallback Strategies**: Route DB → Description parsing → Category defaults

### Development Process Insights
- Initial panic ("it stopped working!") led to systematic investigation
- Browser DevTools crucial for understanding API behavior
- Real-world testing immediately validated the fix
- User domain knowledge (correct route distance) essential for accuracy