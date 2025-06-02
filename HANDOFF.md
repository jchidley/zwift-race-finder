# Project: Zwift Race Finder
Updated: 2025-06-02 00:10 UTC

## Current State
Status: Requirements enhanced with insights from all reference sources
Target: Implement lead-in distance handling for improved accuracy
Latest: Added 15+ new requirements based on zwiftmap, zwift-data, and zwift-client analysis

## Essential Context
- Lead-in distance varies by event type - critical missing piece for accuracy
- Hidden event tags enable advanced filtering (from ZwiftHacks analysis)
- Route slugs needed for external URL generation
- Three MIT-licensed repos cloned locally for reference
- REQUIREMENTS.md now comprehensive with all discovered insights
- Previous session left uncommitted: database.rs, main.rs changes

## Next Step
1. Review and commit pending database.rs, main.rs changes
2. Implement lead-in distance handling (FR-2.1.6) - highest impact on accuracy
3. Add route slug support (DR-11.6) for external URL integration
4. Consider importing zwift-data route database

## If Blocked
Check reference repos: zwift-data-reference/src/routes.ts for data structure

## If Blocked
Check PROJECT_WISDOM.md for tag discovery and URL parsing patterns

## Failed Approaches
Initial API endpoint missing /upcoming - fixed in debug script

## Related Documents
- REQUIREMENTS.md - Updated with ZwiftHacks integration requirements (FER-20)
- ZWIFTHACKS_TECHNIQUES.md - Analysis of valuable techniques
- ROUTE_TRACKING_IDEAS.md - Detailed implementation plans
- CLAUDE.md - Project-specific AI instructions