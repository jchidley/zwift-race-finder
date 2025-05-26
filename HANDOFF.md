# Project: Zwift Race Finder
Updated: 2025-05-26 11:05 UTC

## Current State
Status: API limitation identified - 200 events max (~12 hours)
Target: Clear UX communication about API limits
Latest: Discovered API ignores pagination/date params, always returns 200 events

## Essential Context
- Debug fix working perfectly ✅
- API hard limit: 200 events (~12 hours of data)
- No working pagination, offset, or date filters found
- Added warning for multi-day searches
- Need to show actual time range covered by events

## Next Step
Implement display of event time range (e.g., "Events through May 26, 11:00 PM")

## If Blocked
Search GitHub for other Zwift API solutions that may have workarounds