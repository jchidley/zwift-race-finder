# REQUIREMENTS.md

This document specifies the functional and non-functional requirements for the Zwift Race Finder tool.

## PRIORITY UPDATE (2025-05-27)

**User Concern**: "I'm not convinced that the program is working as I'd like" - This is the highest priority issue to investigate and resolve.

### Additional Recent Priorities (from file review)
1. **Security**: ✅ OAuth token storage in plain text files (HIGH - from SECURITY_AUDIT.md) - COMPLETED 2025-05-27
   - Implemented secure storage module with environment variables, system keyring, and file options
   - Created migration scripts and documentation
   - Maintained backward compatibility
2. **Personal Data**: Multiple files contain hardcoded personal IDs that need sanitization
3. **Configuration Management**: Need seamless personal config that survives updates
4. **Physics Modeling**: Height/weight stats affect predictions but aren't fully utilized
5. **API Limitations**: 200 event hard limit requires user education and workarounds

## Project Overview

The Zwift Race Finder is a command-line tool that helps cyclists find Zwift races matching their target duration and fitness level. It predicts race completion times based on the rider's Zwift Racing Score and route characteristics, achieving 16.1% prediction accuracy using real race data.

## Core Problem Statement

Zwift shows race distances but not expected durations. A 40km race might take 60 or 90 minutes depending on route profile and rider fitness. This tool solves that by predicting actual race duration for specific riders.

## Functional Requirements

### 1. Event Filtering and Discovery

#### 1.1 Event Fetching
- **FR-1.1.1**: Fetch upcoming events from Zwift Public API
- **FR-1.1.2**: Handle API limitation of 200 events maximum (~12 hours of data)
- **FR-1.1.3**: Display actual time range covered when multi-day searches exceed API limits
- **FR-1.1.4**: Notify users if API returns >250 events (future-proofing)

#### 1.2 Event Type Support
- **FR-1.2.1**: Support Traditional Category events (A/B/C/D/E) with populated distance
- **FR-1.2.2**: Support Racing Score events (0-650) with distance in description text
- **FR-1.2.3**: Filter by event type: race, fondo, group ride, workout, time trial
- **FR-1.2.4**: Default to showing only races unless specified otherwise

#### 1.3 Event Filtering
- **FR-1.3.1**: Filter events by estimated duration within tolerance range
- **FR-1.3.2**: Exclude non-cycling events (running)
- **FR-1.3.3**: Show event counts by type after fetching
- **FR-1.3.4**: Provide context-aware suggestions when no results found

### 2. Duration Prediction

#### 2.1 Route-Based Estimation
- **FR-2.1.1**: Use route_id to lookup known route data (distance, elevation, surface)
- **FR-2.1.2**: Handle multi-lap races using event_sub_groups for category-specific distances
- **FR-2.1.3**: Parse distance from event descriptions for Racing Score events
- **FR-2.1.4**: Apply elevation-based difficulty multipliers
- **FR-2.1.5**: Apply surface-type penalties (gravel, mixed surfaces)
- **FR-2.1.6**: Account for lead-in distance variations by event type (race, workout, group ride)
- **FR-2.1.7**: Consider route-specific physics (e.g., jungle has different draft dynamics)

#### 2.2 Speed Calculation
- **FR-2.2.1**: Use category-based average speeds:
  - Cat D (0-199): 30.9 km/h
  - Cat C (200-299): 33 km/h
  - Cat B (300-399): 37 km/h
  - Cat A (400+): 42 km/h
- **FR-2.2.2**: Support dual-speed model with pack dynamics (optional):
  - Pack speed: Category-based
  - Solo speed: 77% of pack speed
  - Drop probability based on elevation and rider weight

#### 2.3 Accuracy Targets
- **FR-2.3.1**: Maintain <20% mean absolute error on predictions
- **FR-2.3.2**: Track prediction accuracy using regression tests
- **FR-2.3.3**: Support calibration with actual race results

### 3. Data Management

#### 3.1 Database Operations
- **FR-3.1.1**: Store route data in SQLite database
- **FR-3.1.2**: Track unknown routes for future mapping
- **FR-3.1.3**: Store race results for accuracy improvement
- **FR-3.1.4**: Support rider stats (weight, FTP) for personalized predictions

#### 3.2 Route Discovery
- **FR-3.2.1**: Log unknown routes during event processing
- **FR-3.2.2**: Support manual route mapping via SQL scripts
- **FR-3.2.3**: Web scraping for route data from whatsonzwift.com
- **FR-3.2.4**: Parse route information from event descriptions
- **FR-3.2.5**: Extract and utilize hidden event tags for advanced filtering
- **FR-3.2.6**: Support route slug mapping for external URL generation
- **FR-3.2.7**: Track event-only routes that aren't available in free ride

#### 3.3 Data Import
- **FR-3.3.1**: Import race history from ZwiftPower via browser extraction
- **FR-3.3.2**: Import actual race times from Strava API
- **FR-3.3.3**: Apply route mappings to imported data
- **FR-3.3.4**: Handle OAuth authentication for Strava

### 4. User Interface

#### 4.1 Command Line Interface
- **FR-4.1.1**: Accept Zwift Racing Score as parameter
- **FR-4.1.2**: Accept target duration and tolerance
- **FR-4.1.3**: Support debug mode showing filtering details
- **FR-4.1.4**: Show unknown routes that need mapping
- **FR-4.1.5**: Record race results for calibration
- **FR-4.1.6**: Support URL-based filter parameters for sharing searches
- **FR-4.1.7**: Filter events by tags (e.g., --tags ranked,zracing)
- **FR-4.1.8**: Show route completion status when tracking enabled

#### 4.2 Output Format
- **FR-4.2.1**: Display events with estimated duration for rider's score
- **FR-4.2.2**: Show time until event starts
- **FR-4.2.3**: Use colored output for better readability
- **FR-4.2.4**: Include route details when available

#### 4.3 User Guidance
- **FR-4.3.1**: Show event type summary after fetching
- **FR-4.3.2**: Provide working command examples when no results
- **FR-4.3.3**: Explain typical event durations by type
- **FR-4.3.4**: Suggest appropriate search parameters

## Non-Functional Requirements

### 5. Performance

- **NFR-5.1**: Process 200 events in under 2 seconds
- **NFR-5.2**: Database queries complete in under 100ms
- **NFR-5.3**: Minimal memory footprint (<50MB)
- **NFR-5.4**: Support concurrent API requests where beneficial

### 6. Reliability

- **NFR-6.1**: Handle API failures gracefully with retry logic
- **NFR-6.2**: Continue operation when route data unavailable
- **NFR-6.3**: Validate all data inputs to prevent crashes
- **NFR-6.4**: Maintain 25+ passing tests with >80% coverage

### 7. Security

- **NFR-7.1**: Never store API credentials in code
- **NFR-7.2**: Use secure token storage for OAuth (Bitwarden, GPG, or secure directory)
- **NFR-7.3**: Exclude sensitive files via .gitignore
- **NFR-7.4**: Support environment variables for secrets
- **NFR-7.5**: Provide security audit scripts (check_secrets.sh, sanitize_personal_data.sh)
- **NFR-7.6**: Pre-commit hooks to prevent accidental secret commits
- **NFR-7.7**: Replace personal data with placeholders before public release
- **NFR-7.8**: Support multiple secure configuration options for different user preferences
- **NFR-7.9**: Implement OAuth token refresh to prevent authentication failures

### 8. Usability

- **NFR-8.1**: Work with zero configuration (sensible defaults)
- **NFR-8.2**: Provide clear error messages
- **NFR-8.3**: Include comprehensive help text
- **NFR-8.4**: Support both simple and advanced usage

### 9. Maintainability

- **NFR-9.1**: Use Rust for type safety and performance
- **NFR-9.2**: Modular architecture with clear separation
- **NFR-9.3**: Comprehensive documentation in code
- **NFR-9.4**: Follow Rust idioms and best practices
- **NFR-9.5**: Version control with meaningful commits

### 10. Compatibility

- **NFR-10.1**: Run on Linux (primary target)
- **NFR-10.2**: Support WSL for Windows users
- **NFR-10.3**: Install to standard locations (~/.local/bin)
- **NFR-10.4**: Use SQLite for portability

## Data Requirements

### 11. Route Data

- **DR-11.1**: Store route_id as primary key (Zwift's internal ID)
- **DR-11.2**: Track distance in kilometers
- **DR-11.3**: Track elevation gain in meters
- **DR-11.4**: Track surface type (road, gravel, mixed)
- **DR-11.5**: Store route name and world
- **DR-11.6**: Store route slug for URL generation
- **DR-11.7**: Track lead-in distances (race, free ride, meetup variants)
- **DR-11.8**: Store external URLs (Strava segment, Zwift Insider, What's on Zwift)
- **DR-11.9**: Flag event-only routes vs free ride available
- **DR-11.10**: Track lap route indicator and time trial support

### 12. Race Results

- **DR-12.1**: Link results to routes via route_id
- **DR-12.2**: Store actual completion time in minutes
- **DR-12.3**: Store rider's Zwift Score at time of race
- **DR-12.4**: Track data source (Strava, ZwiftPower, manual)

### 13. Configuration

- **DR-13.1**: Support JSON configuration files (legacy)
- **DR-13.2**: Support TOML for improved readability (preferred)
- **DR-13.3**: Allow environment variable overrides
- **DR-13.4**: Provide secure storage options (Bitwarden integration)
- **DR-13.5**: Configuration loading priority: local → secure dir → env vars → defaults
- **DR-13.6**: Separate secrets from non-secret configuration
- **DR-13.7**: Support personal wrappers that auto-load configuration

## Integration Requirements

### 14. External APIs

- **IR-14.1**: Integrate with Zwift Public API for events
- **IR-14.2**: Integrate with Strava API for race results
- **IR-14.3**: Support OAuth 2.0 authentication
- **IR-14.4**: Handle rate limiting appropriately
- **IR-14.5**: Cache API responses where beneficial

### 15. Data Sources

- **IR-15.1**: Import from ZwiftPower via browser extraction
- **IR-15.2**: Import from Strava activity exports
- **IR-15.3**: Support manual data entry
- **IR-15.4**: Web scraping for route information

## Testing Requirements

### 16. Test Coverage

- **TR-16.1**: Unit tests for core logic
- **TR-16.2**: Integration tests for API calls
- **TR-16.3**: Regression tests with real race data
- **TR-16.4**: Performance tests for large datasets
- **TR-16.5**: Security tests for credential handling

### 17. Test Data

- **TR-17.1**: Use actual race results for regression testing
- **TR-17.2**: Maintain test fixtures for predictable testing
- **TR-17.3**: Track accuracy metrics over time
- **TR-17.4**: Support test mode without API calls

## Future Enhancement Requirements

### 18. Advanced Features (Planned)

- **FER-18.1**: Real-time race tracking via Sauce4Zwift
- **FER-18.2**: Machine learning for improved predictions
- **FER-18.3**: Community data sharing for route times
- **FER-18.4**: Web interface for non-technical users
- **FER-18.5**: Mobile app with push notifications

### 19. Physics Modeling (Research Phase)

- **FER-19.1**: Implement Martin et al. power equations
- **FER-19.2**: Calculate CdA from rider dimensions (A = 0.0276 × h^0.725 × m^0.425)
- **FER-19.3**: Model grade-specific speed changes
- **FER-19.4**: Account for Zwift-specific physics (33% draft vs 25% real world)
- **FER-19.5**: Use height/weight for aerodynamic drag calculations
- **FER-19.6**: Adjust draft benefit based on rider height
- **FER-19.7**: Factor power-to-weight ratio for climbing predictions
- **FER-19.8**: Consider bike choice effects (TT vs road bike)
- **FER-19.9**: Import complete route data from zwift-data npm package (MIT licensed) including:
  - Route IDs, slugs, names, distances, elevation, lead-in distances
  - Surface type variations (cobbles, dirt, wood, brick, grass, snow)
  - External references (Strava segments, Zwift Insider, What's on Zwift)
  - Event-only routes, lap routes, time trial support flags
- **FER-19.10**: Map between different route identification systems for better matching
- **FER-19.11**: Consider zwiftmap.com architecture patterns for future visualization features
- **FER-19.12**: Track route completion history for gamification and variety scoring
- **FER-19.13**: Generate shareable configuration URLs for team/club setups
- **FER-19.14**: Support world availability schedule for event filtering
- **FER-19.15**: Implement protobuf support for certain Zwift API endpoints

### 20. Automated Testing with Simulation Tools

- **FER-20.1**: Integrate with Zwift simulation tools that provide Bluetooth data
- **FER-20.2**: Create test scenarios with controlled power output profiles
- **FER-20.3**: Validate duration predictions against simulated race completions
- **FER-20.4**: Test edge cases (getting dropped, rejoining pack, sprint finishes)
- **FER-20.5**: Automate regression testing with multiple rider profiles
- **FER-20.6**: Compare simulated results across different routes and conditions
- **FER-20.7**: Build database of simulated race data for model training

## Success Metrics

### 21. Key Performance Indicators

- **KPI-21.1**: Prediction accuracy <20% MAE ✅ (Currently 16.1%)
- **KPI-21.2**: Race matching rate >75% ✅ (Currently 80%)
- **KPI-21.3**: User satisfaction (via feedback)
- **KPI-21.4**: Route coverage >90% of common races
- **KPI-21.5**: Zero security incidents

## Constraints and Assumptions

### 22. Technical Constraints

- **TC-22.1**: Zwift API returns maximum 200 events
- **TC-22.2**: No official Zwift results API available
- **TC-22.3**: Racing Score events have distance=0 in API
- **TC-22.4**: Route IDs are stable but undocumented

### 23. Assumptions

- **A-23.1**: Users know their Zwift Racing Score
- **A-23.2**: Draft benefit is ~30% in races
- **A-23.3**: Category speeds are relatively consistent
- **A-23.4**: Route characteristics affect all riders similarly
- **A-23.5**: Historical performance predicts future results

## Compliance Requirements

### 24. Legal and Ethical

- **CR-24.1**: Respect Zwift's terms of service
- **CR-24.2**: Only access public APIs
- **CR-24.3**: Don't store other users' data
- **CR-24.4**: Open source under MIT/Apache license
- **CR-24.5**: Credit data sources appropriately

## Development Methodology

### 25. AI-Assisted Development

- **DM-25.1**: Built using Claude Code for implementation
- **DM-25.2**: Human provides domain expertise and testing
- **DM-25.3**: Iterative refinement based on real data
- **DM-25.4**: Transparent development with clear reasoning
- **DM-25.5**: Version control for all changes

---

## Critical Discoveries from Development

### Pack Dynamics Model (2025-05-25)
- Getting dropped on hills explains 82.6% of race time variance
- Binary state: either with pack (30.9 km/h) or solo (23.8 km/h)
- Weight penalty significant: 86kg vs 70-75kg typical riders
- High variance is inherent to racing, not a prediction failure

### Event Type Systems (2025-05-26)
- Two mutually exclusive systems: Traditional (A/B/C/D) vs Racing Score (0-650)
- Racing Score events always have distanceInMeters: 0 in API
- Distance must be parsed from description text
- This affected ~50% of all events

### Route Discovery Insights
- Most "unknown routes" are custom event names, not actual routes
- Event organizer websites contain route details not in API
- Manual mapping more effective than automated discovery
- Route length must match typical race duration for accuracy

### AI Development Model
- Human provides domain expertise and quality control
- AI handles implementation and coding
- Transparency in reasoning catches wrong assumptions early
- Real data validation essential - assumptions will be wrong

## Recent Improvements and Current State (2025-05-27)

### Completed Improvements
1. **Code Quality** - All compilation warnings resolved, zero warnings in release build
2. **Multi-Lap Race Accuracy** - Fixed from 533% error to correct predictions (e.g., 38 min vs 6 min)
3. **Pattern Matching** - Flexible SQL matching handles event name variants
4. **Production Deployment** - Binary installed to ~/.local/bin, documentation complete
5. **Test Coverage** - Expanded from 16 to 25 tests (+56%), all passing
6. **Racing Score Events** - Fixed filtering for events with distanceInMeters: 0
7. **UX Enhancements** - Event type counts, smart suggestions, working examples

### Immediate Priorities
1. **User Functionality Concerns** - Investigate why user feels tool isn't working as desired
2. **Security** - Implement secure token storage for OAuth credentials
3. **Route Discovery** - Continue mapping high-frequency unknown routes
4. **Multi-Lap Automation** - Parse lap counts from event descriptions

### Known Issues Requiring Attention
1. **Category E** - Currently treated as Category D
2. **Rotating Race Series** - EVO CC runs different routes weekly
3. **Placeholder Route IDs** - Routes 9001-9003 need real Zwift route_ids
4. **Time Zone Display** - All times shown in local timezone

## Revision History

- 2025-05-27: Initial requirements document created
- 2025-05-27: Updated with user concerns and recent session improvements
- 2025-05-27: Comprehensive update after reviewing all 41 project *.md files
  - Added security requirements from SECURITY_AUDIT.md
  - Enhanced configuration requirements based on Bitwarden integration
  - Added physics modeling details from PHYSICAL_STATS.md
  - Incorporated pack dynamics and event type discoveries
  - Added AI development insights from AI_DEVELOPMENT.md
- Based on: Production deployment with 16.1% accuracy achieved
- Status: Requirements now comprehensive, reflecting all documented needs and discoveries
- 2025-06-01: Added discovery insights from zwiftmap and zwift-data projects
  - Found comprehensive route database in zwift-data npm package (MIT licensed)
  - Identified need for mapping between route IDs, slugs, and names
  - Added consideration for surface types (cobbles, dirt, wood, etc.)
  - Note: Future enhancements should be added to existing sections without renumbering
- 2025-06-01: Comprehensive review against all reference sources
  - Added lead-in distance handling requirements (critical for accuracy)
  - Enhanced route data requirements with slugs, external URLs, flags
  - Added hidden event tags and URL-based filtering from ZwiftHacks
  - Included OAuth token refresh from zwift-client analysis
  - Added route completion tracking and shareable configurations
- Based on: Production deployment with 16.1% accuracy achieved
- Status: Requirements now complete with insights from all reference sources
