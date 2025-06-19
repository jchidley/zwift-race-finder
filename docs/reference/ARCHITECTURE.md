# Zwift Race Finder Architecture

## Overview

Zwift Race Finder helps riders find races that match their schedule and fitness level by estimating race completion times based on route characteristics and rider capabilities.

## System Architecture

```
Zwift API → Filter Events → Estimate Duration → Display Results
     ↓                            ↑
ZwiftPower → Import → SQLite → Route Mappings
```

## Core Components

### 1. Event Fetching (`main.rs`)
- Fetches events from Zwift's public API
- Handles both Traditional (A/B/C/D) and Racing Score (0-650) events
- Filters by type (race, time trial, etc.)

### 2. Duration Estimation (`duration_estimation.rs`)
- Primary: Uses route_id to lookup known route data
- Secondary: Parses distance from event descriptions
- Applies rider-specific adjustments based on weight and category

### 3. Event Filtering (`event_filtering.rs`)
- Filters by estimated duration within tolerance
- Matches user's category or racing score
- Handles multi-category events

### 4. Database (`database.rs`)
- SQLite storage in `~/.local/share/zwift-race-finder/races.db`
- Tables: routes, race_results, unknown_routes
- Caches route information and historical results

## Data Flow

1. **API Request**: Fetch events from Zwift API
2. **Event Processing**: Parse event data, identify event type
3. **Route Resolution**: Map event to route_id or extract from description
4. **Duration Calculation**: Apply algorithm based on route and rider data
5. **Filtering**: Select events matching criteria
6. **Display**: Format results with estimated times

## Event Types

### Traditional Categories
- Categories: A/B/C/D/E
- Has `distanceInMeters` field populated
- Simple category matching

### Racing Score Events
- Score ranges: 0-650
- `distanceInMeters: 0` (must parse from description)
- Identified by `rangeAccessLabel` field presence
- More granular matching

## Route ID System

- Zwift uses internal route IDs that are stable across event name changes
- Route 9999 is a placeholder for unmapped routes
- Routes discovered through:
  - ZwiftHacks.com database
  - WhatsOnZwift route information
  - Empirical testing with known events

## API Integration Points

### Zwift Public API
- Endpoint: `https://us-or-rly101.zwift.com/api/public/events`
- No authentication required
- Returns upcoming events

### ZwiftPower Integration
- Manual export via browser console script
- Imports historical race results
- Provides regression testing data

### Strava Integration
- OAuth authentication
- Fetches completed Zwift activities
- Validates route predictions

## Error Handling

- Network failures: Retry with exponential backoff
- Unknown routes: Log to database for investigation
- Missing data: Fallback estimation methods
- API changes: Defensive parsing with defaults

## Performance Considerations

- SQLite for fast local queries
- Route data cached indefinitely
- API responses cached for session
- Minimal external dependencies

## Security

- No credentials stored in code
- OAuth tokens in secure storage
- Read-only API access
- Local database with user data only