# Zwift Integrations Research

Research findings on Zwift's connections to external apps and potential data sources.

## External App Integrations

### 1. Strava
- **Auto-sync**: Zwift automatically exports completed activities to Strava
- **Data included**: Route names, distances, elevation profiles, power data
- **API access**: Can retrieve via Strava API with proper authentication
- **Our usage**: `strava_auth.sh` → `strava_fetch_activities.sh` → import to DB

### 2. Garmin Connect
- **Direct sync**: Zwift can sync to Garmin Connect
- **Limitation**: Garmin won't "pass on" Zwift workouts to other platforms (to avoid duplicates)
- **Workaround**: Some users run Garmin Edge device alongside Zwift to capture data
- **Best practice**: Set up individual connections to each platform

### 3. TrainingPeaks
- **Structured workouts**: TrainingPeaks workouts sync TO Zwift automatically
- **Activity sync**: Zwift activities sync TO TrainingPeaks
- **Historical data**: No bulk historical sync - must export/import manually
- **File formats**: Supports .ERG (power) and .FIT (HR/pace) files

### 4. Other Integrations
- **JOIN Cycling**: Auto-exports workouts to connected accounts
- **Xert**: Via Strava intermediary
- **Terra API/Vital**: Third-party services offering Zwift integration

## Zwift Companion App Capabilities

### Creating Meetups
- **Access**: Events → People icon → Create Meetup
- **Features**:
  - Schedule up to 7 days in advance
  - Invite up to 100 followers
  - Choose any free-ride route (no event-only routes)
  - Set world, route, and distance
- **Limitations**: 
  - Only followers can be invited
  - No custom workouts in meetups
  - No rubber-band effect for mixed abilities

### Creating Club Events
- **Who can create**: Club owners and designated moderators
- **Customization options**:
  - Description with hyperlinks
  - Category groups (A/B/C/D/E with fixed pacing)
  - Leader (yellow beacon) and Sweeper (red beacon)
  - Any free-ride route on any world
  - Length by distance, duration, or laps
- **Visibility**: 
  - Club members only
  - Anyone with event link
- **Management**: All via Companion app (no web interface)

## API Access

### Official APIs
1. **Training Connections API (2024)**
   - For training platforms (TrainerRoad, JOIN, TriDot)
   - Scalable developer integration model
   - Not available to individual developers

2. **Status API**
   - Public at status.zwift.com/api
   - System status information only

3. **Developer API**
   - Requires special developer account
   - Not available to hobby developers
   - Contact: developers@zwift.com

### Unofficial Solutions
- **zwift-mobile-api** (JavaScript)
- **zwift-client** (Python)
- Risk: May violate ToS

## Potential Data Extraction Strategy

### Using Club Events + External Apps
1. **Create controlled club events** with known routes
2. **Participate and complete** the events
3. **Export data via Strava/Garmin** with accurate route info
4. **Analyze exported files** for route details:
   - Actual distance (including lead-in)
   - Elevation profile
   - Segment information
   - GPS coordinates (simulated)

### Advantages
- Ground truth data from actual rides
- Captures all route variations (lead-in differences)
- Can test multiple categories/configurations
- Legal and ToS-compliant

### Implementation Ideas
1. Create weekly "Route Discovery" club events
2. Rotate through all Zwift routes systematically
3. Use different event types (race/group ride/meetup)
4. Export and analyze results automatically
5. Build comprehensive route database

## Key Insights

1. **Strava as data pipeline**: Most reliable way to extract detailed ride data
2. **No public event creation API**: Must use Companion app manually
3. **Club events as research tool**: Can create controlled experiments
4. **Multiple integrations = redundancy**: Use several services for validation
5. **Historical data challenge**: Most integrations are forward-only

## Next Steps

1. Test creating a "Chidley" club event with known route
2. Complete the event and analyze Strava export
3. Compare actual vs published route data
4. Document any discrepancies (lead-in, etc.)
5. Consider automating analysis pipeline