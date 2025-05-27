# Project Wisdom - Recent Insights

## Recent Discoveries (2025-05-27)

### Configuration Management Success
Insight: Multi-level config with environment overrides provides maximum flexibility
Impact: Users can configure via files, env vars, or wrapper scripts
Key Pattern: env → local → ~/.config → ~/.local/share → defaults

### Data Directory Strategy
Insight: ~/.local/share/ survives system updates better than ~/.config/
Impact: User settings persist across tool updates
Lesson: Separate volatile config from persistent user data

### TOML Over JSON
Insight: TOML more readable for end users with clear comments
Impact: Better user experience for configuration files
Trade-off: Slightly more complex parsing but worth it

### Testing Configuration Systems
Insight: Config systems need real-world testing beyond unit tests
Impact: Discovered edge cases with default value handling
Pattern: Create test config → Run tool → Verify behavior → Test env overrides

### Documentation Types Matter
Insight: Different audiences need different documentation
Impact: Created CONFIG_MANAGEMENT.md (users) vs SIMULATION_TOOLS.md (developers)
Best Practice: Clear examples and security warnings in user docs

### Secure Storage Design Pattern
Insight: Support multiple storage backends with automatic fallback (env → keyring → file)
Impact: Users get best available security without configuration burden

### Backward Compatibility First
Insight: Security improvements should never break existing workflows
Impact: New secure scripts alongside originals, migration at user's pace

### Environment Variables for CI/CD
Insight: CI/CD environments need stateless token storage
Impact: Env vars as primary option enables GitHub Actions integration

## Recent Discoveries (2025-05-26)

### Racing Score vs Traditional Categories
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D and Racing Score (0-650)
Impact: Filtering logic excluded half the events due to `distanceInMeters: 0`

### Distance in Description Text
Insight: Racing Score events embed distance in description ("Distance: 23.5 km")
Impact: Parse description when API fields are empty

### Route ID Without Data
Insight: Events have routeId but no details - websites maintain separate databases
Impact: Local route database strategy validated

### WhatsonZwift Search Technique
Insight: Use `site:https://whatsonzwift.com [route]` for accurate route data
Impact: Reliable method for mapping unknown routes

### Zero as API Signal
Insight: Some APIs use 0/null to mean "check elsewhere", not actual zero
Impact: Changed filtering to accept 0 and check alternatives

### Field Presence Type Detection
Insight: `rangeAccessLabel` presence identifies Racing Score events
Impact: Differentiate event types without explicit type field

### Browser DevTools Power
Insight: Browser tools reveal undocumented API behavior quickly
Impact: Found Racing Score pattern in minutes vs hours

### Hierarchical Log Management
Insight: Large logs (66KB+) slow LLM loading - use Summary/Recent/Archives
Impact: 13x context reduction while preserving all history