# Project Wisdom Archive - 2025-06-02

This archive contains consolidated wisdom from:
- PROJECT_WISDOM.md (original insights)
- PROJECT_WISDOM_SUMMARY.md (universal insights)
- PROJECT_WISDOM_RECENT.md (recent discoveries)

## Universal Development Insights (from PROJECT_WISDOM_SUMMARY.md)

### 1. Data Quality > Algorithm Sophistication
Single data error degraded accuracy by 11.2%. The path from 92.8% to 25.7% error wasn't about better algorithms - it was about better data.

### 2. Test Reality, Not Assumptions
Tested estimates against estimates for weeks. Always validate test data comes from actual source of truth, not derived calculations.

### 3. Variance Can Be a Feature
Same conditions yielded 32-86 minute variations. High variance may be inherent to the domain, not a model flaw.

### 4. Binary States Often Dominate
Despite complex physics, outcomes were binary: with pack or dropped. Seek simple dominant patterns before complex models.

### 5. Use Stable Identifiers
Event names changed, route IDs didn't. Always identify and use the most persistent identifier in your data model.

### 6. Zero Means "Look Elsewhere"
APIs returning 0 often signal alternate data locations. Check descriptions, metadata, or related fields.

### 7. Build Fallback Chains
Multiple strategies (route→distance→description→defaults) ensure resilience. Design degradation paths for missing data.

### 8. Plan Before Debugging
When confused, resist random changes. Create systematic investigation plans for efficient problem solving.

### 9. Field Presence Reveals Types
Missing type fields? Use presence patterns (`rangeAccessLabel`) to differentiate data variants.

### 10. Browser DevTools for API Discovery
Official clients reveal API behavior. Check browser network tab before building blind.

### 11. Hierarchical Logs for AI Context
Large logs slow LLMs. Summary/Recent/Archive pattern achieves 13x reduction while preserving history.

### 12. 80% Match Rate is Success
Perfect data integration is impossible. Focus on sufficient quality for your use case.

## Historical Insights (from original PROJECT_WISDOM.md)

### 2025-01-06: Event Descriptions Contain Complete Race Information
Insight: The description field in events contains distance and elevation data that mirrors what the companion app shows
Impact: Rather than relying solely on API fields (which return 0.0 for Racing Score events), we should parse descriptions to extract the complete race information including distance in km and elevation gain in meters. This data is already being shown in the companion app.

### 2025-01-06: Route Discovery Pattern - Event Titles Contain Route Names
Insight: Event titles often contain route names after delimiters (-, :, |) - "Zwift Epic Race - Sacre Bleu" indicates the route is "Sacre Bleu"
Impact: Extracting route names from event titles enables automatic route discovery. The tool can parse titles, extract potential route names, and search the database or web for matching routes before marking them as unknown.

### 2025-01-06: Web Search Effectiveness for Route Discovery
Insight: Searching for route names directly (e.g., "Zwift Sacre Bleu route") yields better results than searching by numeric route ID
Impact: When implementing automated route discovery, use extracted route names in web searches rather than route IDs. Route IDs are internal identifiers not commonly referenced in public documentation.

### 2025-01-06: API Data Structure Contains Complete Race Information
Insight: All racing events contain lap count and racing score ranges in their subgroups, not in the main event data
Impact: Distance calculations must check subgroup.laps field and multiply by route distance. The API provides 0.0 for event distance but complete lap info in subgroups. This explains why some races show correct multi-lap distances (they use the subgroup data) while others don't.

### 2025-05-27: Requirements Gathering Process Validates User Concerns
Insight: User's concern "not working as I'd like" was effectively addressed through systematic requirements documentation rather than code changes
Impact: Created comprehensive REQUIREMENTS.md from 41-file review. Discovered security issues (OAuth in plaintext), config management needs, and physics modeling opportunities. Sometimes documentation IS the solution.

### 2025-05-27: Security Vulnerability Discovery Through Documentation
Insight: Comprehensive file review revealed OAuth tokens stored in plain text files - a critical security issue that wasn't apparent from normal usage
Impact: Security requirements now documented as HIGH priority. Shows value of systematic documentation reviews for uncovering non-obvious issues.

### 2025-05-27: Comprehensive Documentation Review Reveals Priorities
Insight: Reviewing all 41 project *.md files revealed that security and configuration issues are as critical as functionality
Impact: REQUIREMENTS.md now comprehensive with security requirements (NFR-7.6-7.8), enhanced config management (DR-13.5-13.7), and physics modeling details (FER-19.5-19.8). Highest priority remains user's functionality concern.

### 2025-05-27: Code Organization Improves Maintainability
Insight: Moving unused code to dedicated modules (unused.rs, utils.rs) rather than deleting preserves potentially useful functions while keeping main code clean
Impact: Zero warnings achieved without losing code that might be needed for future features like enhanced multi-lap handling or route discovery improvements

### 2025-05-27: Production Deployment Success
Insight: Achieving <20% accuracy unlocked production readiness - users trust predictions that are "close enough"
Impact: README updated with "PRODUCTION READY" status and 16.1% accuracy achievement. Shows importance of realistic accuracy targets over perfection.

## Recent Discoveries (from PROJECT_WISDOM_RECENT.md)

### Configuration Management Success (2025-05-27)
Insight: Multi-level config with environment overrides provides maximum flexibility
Impact: Users can configure via files, env vars, or wrapper scripts
Key Pattern: env → local → ~/.config → ~/.local/share → defaults

### Data Directory Strategy (2025-05-27)
Insight: ~/.local/share/ survives system updates better than ~/.config/
Impact: User settings persist across tool updates
Lesson: Separate volatile config from persistent user data

### TOML Over JSON (2025-05-27)
Insight: TOML more readable for end users with clear comments
Impact: Better user experience for configuration files
Trade-off: Slightly more complex parsing but worth it

### Testing Configuration Systems (2025-05-27)
Insight: Config systems need real-world testing beyond unit tests
Impact: Discovered edge cases with default value handling
Pattern: Create test config → Run tool → Verify behavior → Test env overrides

### Documentation Types Matter (2025-05-27)
Insight: Different audiences need different documentation
Impact: Created CONFIG_MANAGEMENT.md (users) vs SIMULATION_TOOLS.md (developers)
Best Practice: Clear examples and security warnings in user docs

### Secure Storage Design Pattern (2025-05-27)
Insight: Support multiple storage backends with automatic fallback (env → keyring → file)
Impact: Users get best available security without configuration burden

### Backward Compatibility First (2025-05-27)
Insight: Security improvements should never break existing workflows
Impact: New secure scripts alongside originals, migration at user's pace

### Environment Variables for CI/CD (2025-05-27)
Insight: CI/CD environments need stateless token storage
Impact: Env vars as primary option enables GitHub Actions integration

### Racing Score vs Traditional Categories (2025-05-26)
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D and Racing Score (0-650)
Impact: Filtering logic excluded half the events due to `distanceInMeters: 0`

### Distance in Description Text (2025-05-26)
Insight: Racing Score events embed distance in description ("Distance: 23.5 km")
Impact: Parse description when API fields are empty

### Route ID Without Data (2025-05-26)
Insight: Events have routeId but no details - websites maintain separate databases
Impact: Local route database strategy validated

### WhatsonZwift Search Technique (2025-05-26)
Insight: Use `site:https://whatsonzwift.com [route]` for accurate route data
Impact: Reliable method for mapping unknown routes

### Zero as API Signal (2025-05-26)
Insight: Some APIs use 0/null to mean "check elsewhere", not actual zero
Impact: Changed filtering to accept 0 and check alternatives

### Field Presence Type Detection (2025-05-26)
Insight: `rangeAccessLabel` presence identifies Racing Score events
Impact: Differentiate event types without explicit type field

### Browser DevTools Power (2025-05-26)
Insight: Browser tools reveal undocumented API behavior quickly
Impact: Found Racing Score pattern in minutes vs hours

### Hierarchical Log Management (2025-05-26)
Insight: Large logs (66KB+) slow LLM loading - use Summary/Recent/Archives
Impact: 13x context reduction while preserving all history