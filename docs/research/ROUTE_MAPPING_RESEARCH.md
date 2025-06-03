# Route Mapping Research Results

## Summary

Researched common Zwift race series to identify typical routes and durations for creating manual mappings.

## Update: 2025-05-27 - Additional High-Frequency Events

### New Discoveries from Web Search

Successfully researched three more high-frequency unmapped events by searching event websites and Zwift documentation:

#### 1. Restart Monday Mash (55x occurrences)
- **Route**: Mountain Mash (5.7km, 335m elevation)  
- **Route ID**: 1917017591
- **Location**: Watopia, starts in Jungle Pens
- **Format**: Climbing route up Epic KOM reverse
- **Duration**: 18-32 minutes (depends on w/kg: 4.0=18min, 3.0=22min, 2.0=32min)
- **Notes**: Created for Zwift Games Categories C&D climbing stage

#### 2. TEAM VTO POWERPUSH (37x occurrences)
- **Route**: Tempus Fugit (18.9km, 16m elevation)
- **Route ID**: 2128890027  
- **Location**: Watopia's Fuego Flats desert
- **Format**: 40min warm-up ride + 7.1km Team Time Trial segment
- **Duration**: 1 hour total (including warm-up)
- **Organizer**: Team Virtual Training Oceania
- **Special**: Flattest route in Zwift, designed for TT efforts

#### 3. The Bump Sprint Race (27x occurrences)
- **Route**: Tick Tock (19.2km, 59m elevation)
- **Route ID**: 3366225080
- **Location**: Watopia, Desert Pens start
- **Format**: One-lap race, "drag race from the start"
- **Organizer**: Team Endurance Nation
- **Purpose**: Help riders improve threshold power through high-intensity effort

### Key Learning
Many high-frequency events have their own websites mentioned in event descriptions. Checking these sites (or searching for event names + "zwift route") often reveals the actual routes used.

## Key Findings

### 1. EVO CC Race Series
- **Format**: Sprint races with "Start fast, hang on in the middle, then sprint for the win"
- **Duration**: 60-90 minutes (varies by route)
- **Routes**: Rotates weekly, no fixed schedule found
- **Categories**: Uses category enforcement (A/B/C/D)
- **Mapping Strategy**: Use common sprint routes like Volcano Flat as default

### 2. Sydkysten Race
- **Distance**: 29.6 km confirmed via ZwiftPower
- **Duration**: Estimated 40-60 minutes depending on category
- **Host**: Sydkysten Cycling / Carl Ras
- **Mapping Strategy**: Created placeholder route_id 9001

### 3. Tofu Tornado Race (Team Vegan)
- **Distance**: Varies 32.7-70km (regular vs XL versions)
- **Schedule**: Tuesdays and Saturdays
- **Format**: "Sometimes hilly, sometimes flat but always plant-based and competitive"
- **Mapping Strategy**: Use Volcano Flat for typical ~35km races

### 4. CAT & MOUSE KZR CHASE RACE
- **Format**: Chase race with staggered starts by category (2-4 min gaps)
- **Distance**: Approximately 40km
- **Duration**: Within 60 minutes
- **Special**: Korean Zwift Racing (KZR) organized
- **Victory**: Group with most riders when leader crosses finish
- **Mapping Strategy**: Created placeholder route_id 9002

### 5. DBR Races (Danish Bike Riders)
- **Schedule**: Multiple days (Wednesday, Thursday, Saturday, Sunday, Afternoon races)
- **Categories**: Uses Zwift Racing Score (ZRS)
- **Format**: 1 minute gaps between categories
- **Mapping Strategy**: Use Watopia Flat Route as default

### 6. ZHR Morning Tea Race
- **Distance**: 31 miles (49 km)
- **Location**: London
- **Format**: Crit race with two groups based on w/kg
- **Duration**: Estimated 60-90 minutes
- **Mapping Strategy**: Created placeholder route_id 9003

### 7. Zwift TT Club Racing - Watopia's Waistband
- **Route**: Watopia's Waistband (confirmed)
- **Distance**: 25.4 km
- **Elevation**: 96m
- **Route ID**: 3733109212 (from previous data)
- **Mapping Strategy**: Direct mapping to known route

## Implementation

Created `manual_route_mappings.sql` with:
1. Route definitions for known routes (Watopia's Waistband)
2. Placeholder routes (9001-9003) for series without confirmed route_ids
3. UPDATE statements mapping event names to appropriate routes
4. Summary queries to verify mappings

## Recommendations

1. **Get Real Route IDs**: Placeholder IDs (9001-9003) should be replaced with actual Zwift route_ids
2. **Date-Based Mapping**: EVO CC and similar rotating series would benefit from date-aware mapping
3. **Day-Specific Routes**: DBR races might use different routes on different days
4. **Distance Variants**: Handle XL/Sprint variants of races (e.g., Tofu Tornado XL)
5. **API Discovery**: Consider enhancing route discovery to extract route_id from event data

## Usage

To apply these mappings:
```bash
sqlite3 ~/.local/share/zwift-race-finder/races.db < manual_route_mappings.sql
```

This will map many of the high-frequency unmapped events to appropriate routes, improving duration estimation accuracy.