# Zwift-Offline Route Data Investigation

## Summary
The zwift-offline export only produces 55 routes because it's intentionally limited to "event-only" routes. The full route data exists but requires a different extraction approach.

## Key Findings

### 1. Events vs Routes
- **events.txt**: Contains 55 entries - only routes marked with `eventOnly='1'` attribute
- **start_lines.txt**: Contains 278 entries - ALL routes including regular free-ride routes

### 2. Data Extraction Scripts

#### get_events.py (Limited to event routes)
```python
if route.get('eventOnly') == '1':  # This filter limits to 55 routes
    event = {
        'name': name,
        'route': int(route.get('nameHash')),
        'distance': round(float(route.get('distanceInMeters')) + float(route.get('leadinDistanceInMeters')), 1),
        'course': world_to_course[world][0],
        'sport': 1 if route.get('sportType') == '2' else 0
    }
```

#### get_start_lines.py (Extracts ALL routes)
```python
# No filter - extracts all routes
nameHash = int.from_bytes(int(route.get('nameHash')).to_bytes(4, 'little'), 'little', signed=True)
data[nameHash] = {
    'name': '%s - %s' % (world_names[world], route.get('name').strip()),
    'road': int(checkpoints[0].get('road')),
    'time': int(float(checkpoints[0].get('time')) * 1000000 + 5000)
}
```

### 3. Route Data Sources

1. **Primary source**: XML route files in `Worlds/world*/routes/` directories
2. **Extracted data**:
   - events.txt: Event-only routes with distance, course, sport type
   - start_lines.txt: All routes with start position (road, time)

### 4. Missing Route Information

The start_lines.txt has route names and hashes but lacks:
- Distance information
- Course/world ID mapping
- Sport type (cycling vs running)
- Lead-in distance

## Solutions

### Option 1: Modify route_export.py to use start_lines.txt
- Pros: Quick implementation, gets all 278 routes
- Cons: Missing distance and other metadata

### Option 2: Create new extraction script
- Extract ALL routes from XML files without the eventOnly filter
- Include all metadata (distance, lead-in, sport type, etc.)
- Would provide complete route database

### Option 3: Hybrid approach
- Use events.txt for the 55 event routes (complete data)
- Supplement with start_lines.txt for additional routes (partial data)
- Note which routes have incomplete information

## Recommendation

Create a new extraction script based on get_events.py but without the `eventOnly='1'` filter. This would provide:
- All 278+ routes
- Complete metadata for each route
- Consistent data structure with current events.txt format

## Implementation Notes

The route data extraction requires:
1. Access to Zwift game files (Windows installation)
2. wad_unpack.exe tool to extract route XML files
3. Processing of XML route definitions

Without direct access to Zwift game files, we're limited to the pre-extracted data in events.txt and start_lines.txt.