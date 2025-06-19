# Zwift Route Data Extraction Documentation

This document details the investigation into extracting complete route data from Zwift, including findings, methods, and current limitations.

## Overview

Zwift route data is essential for calculating race durations. Routes have three key components:
- **Base route distance** - The core loop distance
- **Lead-in distance** - Distance from start point to the loop beginning
- **Elevation gain** - Total meters of climbing

## Route Data Sources

### 1. WAD Archive Files (Authoritative Source)

All route data is stored in compressed WAD archives within the Zwift installation:
```
C:\Program Files (x86)\Zwift\assets\Worlds\world*\data_1.wad
```

These archives contain XML files with complete route definitions:
```xml
<route>
  <name>Route Name</name>
  <nameHash>12345678</nameHash>
  <distanceInMeters>15000</distanceInMeters>
  <elevationGainInMeters>250</elevationGainInMeters>
  <leadinDistanceInMeters>500</leadinDistanceInMeters>
  <eventOnly>1</eventOnly>
  <sportType>1</sportType>
</route>
```

### 2. zwift-offline Export (Limited)

The zwift-offline project provides route data through its API, but with limitations:
- Only exports routes marked with `eventOnly='1'`
- Provides 55 routes out of 300+ total routes
- Missing free-ride routes used in many events

### 3. Third-Party Sources

Several community sources provide route data:
- ZwiftHacks.com - Comprehensive route database
- WhatsOnZwift.com - Route profiles and details
- ZwiftInsider.com - Route descriptions and strategies

## Extraction Methods

### Method 1: WAD File Extraction (Requires Tools)

```python
# From zwift-offline/scripts/get_events.py
subprocess.run(['wad_unpack.exe', os.path.join(worlds, directory, 'data_1.wad')])
routes = os.path.join('Worlds', directory, 'routes')
```

**Requirements:**
- `wad_unpack.exe` - A decompression tool (no longer publicly available)
- Access to Zwift game files

**Process:**
1. Decompress WAD files using wad_unpack.exe
2. Parse XML route definitions
3. Extract distance, elevation, and metadata

### Method 2: zwift-offline Integration (Currently Used)

```bash
# Import from running zwift-offline server
./scripts/import_from_zwift_offline.sh --skip-ssl-verify
```

**Limitations:**
- Only event routes (eventOnly='1')
- No free-ride routes
- Limited to 55 routes

### Method 3: Empirical Collection (Last Resort)

For missing routes:
1. Create a Zwift event on the target route
2. Ride the event and collect FIT file
3. Analyze to determine lead-in and lap distances

## Event System Understanding

### How Zwift Events Work

Events modify base routes using three parameters:

1. **Laps** - Repeat the route loop N times
   ```protobuf
   optional uint32 laps = 25;
   ```

2. **Distance** - Fixed total distance
   ```protobuf
   optional float distanceInMeters = 24;
   ```

3. **Duration** - Time-based events
   ```protobuf
   optional uint32 durationInSeconds = 34;
   ```

### Route Structure

```
[Start Point] → [Lead-in Distance] → [Loop Start] → [Loop Distance] → [Loop End]
                                           ↑                               ↓
                                           └─────────── (if laps > 1) ─────┘
```

### Free-Ride vs Event Routes

- **Event-only routes**: Fixed courses for organized events
- **Free-ride routes**: Open world routes with spawn points
- Both can be used for races with appropriate modifiers

## Current Implementation Status

### What We Have

1. **378 routes in database** from third-party imports
2. **55 event routes** from zwift-offline integration
3. **Working import tools** for various sources
4. **License-compliant integration** via API boundary

### What We Tried

1. Created extraction scripts:
   - `get_all_routes.py` - Extract all routes from WAD files
   - `extract_all_routes_wsl.sh` - WSL wrapper for Windows

2. Investigated workarounds:
   - Merging events.txt and start_lines.txt (abandoned - no distance data)
   - Memory dump extraction (against Zwift ToS)

### Current Limitations

1. **Missing wad_unpack.exe**
   - Referenced tools no longer available:
     - github.com/h4l/zwift-routes (doesn't exist)
     - github.com/h4l/zwift-map-parser (doesn't exist)
   
2. **Incomplete route coverage**
   - Free-ride routes not accessible
   - Some event routes use free-ride base routes

## Technical Details

### World/Course ID Mapping

```python
world_to_course = {
    '1': (6, 'Watopia'),
    '2': (2, 'Richmond'),
    '3': (7, 'London'),
    '4': (8, 'New York'),
    '5': (9, 'Innsbruck'),
    '6': (10, 'Bologna'),
    '7': (11, 'Yorkshire'),
    '8': (12, 'Crit City'),
    '9': (13, 'Makuri Islands'),
    '10': (14, 'France'),
    '11': (15, 'Paris'),
    '12': (16, 'Gravel Mountain'),
    '13': (17, 'Scotland')
}
```

### Route ID Handling

- Route IDs can be negative (signed integers)
- Use route_id as identifier, never route name
- Names change, IDs are permanent

## Future Possibilities

### If wad_unpack.exe Becomes Available

1. Run `get_all_routes.py` to extract complete route database
2. Import both event and free-ride routes
3. Achieve 100% route coverage

### Alternative Tools

- **zwf** (github.com/h4l/zwf) - Works only with decompressed WADs
- **zwift-utils** (gitlab.com/r3dey3/zwift-utils) - Claims decompression support

### Community Collaboration

- Crowdsource FIT files for missing routes
- Share route data with other developers
- Contribute findings back to zwift-offline

## Recommendations

1. **Continue using current data sources** - 378 routes provide good coverage
2. **Monitor for tool availability** - Check periodically for wad_unpack.exe
3. **Document missing routes** - Track which routes need data
4. **Consider empirical collection** - For critical missing routes only

## Conclusion

The route data exists and is well-structured in Zwift's WAD files. The primary limitation is the availability of decompression tools, not the data itself. Current third-party sources and zwift-offline integration provide sufficient coverage for most use cases.