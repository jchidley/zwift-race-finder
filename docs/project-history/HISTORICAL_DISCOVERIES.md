# Historical Discoveries

## Overview

This document captures the evolution of understanding during Zwift Race Finder development, preserving key discoveries and insights that shaped the project.

## 2025-05-25: Drop Dynamics Discovery

### The Problem
Bell Lap race showed extreme variance: 32-86 minutes for the same route and category.

### Investigation Process
1. Initial hypothesis: Physics model would explain times
2. Implementation: Martin et al. (1998) cycling power model
3. Result: 127% error - completely wrong

### The Discovery
Racing isn't about steady-state physics - it's about pack dynamics:
- **Binary State**: Either with pack (fast) or dropped (slow)
- **No Middle Ground**: Transition is quick and decisive
- **Weight Penalty**: 86kg vs 70kg typical = major climb disadvantage
- **Cascade Effect**: Drop early → race mostly solo → 2-3x longer time

### Impact
- Explained 82.6% of variance with simple dual-speed model
- Pack speed: Category-based (D: 30.9 km/h)
- Solo speed: 77% of pack speed
- Weighted by drop probability

## 2025-05-26: Racing Score Event Discovery

### The Problem
Tool found traditional races but missed Racing Score events entirely.

### Investigation Process
1. User report: "Not finding races"
2. API inspection: Events exist but filtered out
3. Pattern noticed: `distanceInMeters: 0` for some events

### The Discovery
Zwift has two mutually exclusive event systems:
- **Traditional**: A/B/C/D/E with distance populated
- **Racing Score**: Score ranges with distance = 0
- **Key Signal**: `rangeAccessLabel` field presence
- **Distance Location**: Only in description text

### Solution
```rust
fn is_racing_score_event(subgroup: &EventSubgroup) -> bool {
    subgroup.distance_in_meters == 0.0 && 
    subgroup.range_access_label.is_some()
}
```

### Impact
- Tool now finds all event types
- Better UX with event type summaries
- Proper distance parsing from descriptions

## 2025-05-26: UX Enhancement Insights

### The Problem
Users getting "No events found" with no guidance.

### The Discovery
Context-aware messages dramatically improve user experience:
- Show what was searched
- Explain why no results
- Provide working examples
- Give actionable next steps

### Implementation
```
No races found matching your criteria.
Searched for: races, 120 ± 15 minutes, Category D
Try: cargo run -- -d 30 -t 30
```

### Impact
- Users understand the issue
- Self-service problem solving
- Fewer support requests

## 2025-05-27: Configuration Evolution

### The Journey
1. Started with command-line args only
2. Added config file support
3. Discovered need for multiple levels
4. Implemented hierarchical config

### The Pattern
```
Priority: CLI args > Environment > Local > User > System > Defaults
```

### Key Insight
Users need flexibility in how they configure:
- Quick overrides via CLI
- Persistent settings in config
- Wrapper scripts with env vars
- System-wide defaults

## 2025-06-02: Lead-in Distance Impact

### The Problem
Systematic underestimation of race times.

### Investigation
1. Compared actual vs predicted times
2. Noticed consistent shortfall
3. Researched route structures

### The Discovery
Lead-in distance is significant but hidden:
- Varies by event type: 0.2-5.7km
- Not included in route distance
- Different for races vs free rides
- Must be added for accurate total

### Impact
- Added lead_in_km to database schema
- Updated all calculations
- Improved accuracy significantly

## 2025-06-02: Data Source Discoveries

### WhatsOnZwift Situation
- Has permission from Zwift for data
- No public API available
- Must parse web pages
- Most comprehensive route data

### Zwift API Limitations
- Developer accounts restricted
- Public API limited
- No route database endpoint
- Must discover empirically

### Strava Integration Value
- Zwift exports rides automatically
- Contains actual route data
- Provides ground truth
- Enables regression testing

### The Strategy
Combine multiple sources:
1. zwift-data npm package for bulk import
2. Manual curation for accuracy
3. Strava for validation
4. User reports for unknowns

## API Evolution Insights

### Zero as Signal Pattern
- `distanceInMeters: 0` → check description
- `null` often means "look elsewhere"
- Empty arrays vs missing fields
- Field presence indicates type

### Browser DevTools Power
- Faster than documentation
- Shows actual behavior
- Reveals undocumented fields
- Network tab is truth

### Empirical Development
When documentation lacks:
1. Make minimal API calls
2. Inspect actual responses
3. Test edge cases
4. Document findings

## Technical Discoveries

### SQLite Performance
- Perfect for this use case
- No server overhead
- Fast local queries
- Easy backups

### Rust Advantages
- Compiler catches API changes
- Type system prevents nulls
- Fast enough for real-time
- Great error messages

### Testing Insights
- Regression tests with real data crucial
- Property tests find edge cases
- Integration tests catch API changes
- Mutation testing reveals test quality

## Lessons Learned

### On Estimation
1. Perfect accuracy impossible in racing
2. Good enough is good enough
3. Users understand variance
4. Empirical beats theoretical

### On APIs
1. Undocumented doesn't mean unavailable
2. Multiple sources improve reliability
3. Cache everything possible
4. Plan for API changes

### On User Experience
1. Clear error messages crucial
2. Examples better than explanations
3. Progressive disclosure works
4. Context-aware help valued

### On Development
1. Start simple, iterate based on data
2. Real user data beats assumptions
3. Fast feedback loops essential
4. Document discoveries immediately

## Future Considerations

### Potential Improvements
- Machine learning on race results
- Power-based personalization
- Weather integration
- Social features

### Unresolved Questions
- Why do some routes have multiple IDs?
- How does Zwift calculate Racing Score?
- What determines lead-in distance?
- Can we predict pack dynamics better?

This historical record helps future developers understand not just what the code does, but why it evolved this way.