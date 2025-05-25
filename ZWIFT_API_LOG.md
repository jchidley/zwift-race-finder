# Zwift API Integration Log

## Session 2025-05-24: Zwift Race Finder Tool Development

Built a comprehensive CLI tool to find Zwift races suitable for specific rider categories and durations.

### Key Discoveries

- **Zwift Public API**: Found working endpoint `https://us-or-rly101.zwift.com/api/public/events/upcoming`
  - Returns JSON array of upcoming events with distance, duration, route info
  - No authentication required for public events
  - Event IDs are integers (u64), not strings

- **ZwiftPower Integration**: Successfully parsed ZwiftPower profile pages
  - URL format: `https://zwiftpower.com/profile.php?z=USER_ID&sid=SESSION_ID`
  - Can extract Zwift Racing Score and category via regex parsing
  - Implemented 24-hour caching to avoid repeated requests

- **Speed Estimation Algorithm**: Created category-based speed estimates
  - Cat A: 40 km/h, Cat B: 35 km/h, Cat C: 30 km/h, Cat D: 25 km/h
  - Strong Cat D (190-199 score): 27 km/h for better accuracy
  - Route difficulty multipliers for hills/flats (0.7x to 1.1x)

### Technical Implementation

- **Rust + Tokio**: Async HTTP client with reqwest
- **Auto-detection**: Fetches user stats from ZwiftPower with fallback to hardcoded values
- **Caching**: Stores user stats in `~/.cache/zwift-race-finder/user_stats.json`
- **Duration Filtering**: Estimates race duration based on distance, route, and rider category

### Solutions That Worked

- **JSON Deserialization**: Used serde with proper field mapping (`eventStart` → `event_start`)
- **Category Speed Mapping**: Fine-grained speed estimates by score ranges (not just A/B/C/D)
- **Graceful Fallback**: Tool works offline with cached/hardcoded stats if ZwiftPower unavailable
- **User-Centric Defaults**: Auto-detects Jack's actual stats (195 score, Cat D) for personalized results

### Configuration Insights

- Default tolerance of 30 minutes provides 1.5-2.5 hour event range
- 24-hour event window (`-n 1`) focuses on immediate opportunities
- Route name pattern matching helps adjust speed estimates for terrain

### Installation Pattern

Created reusable installation script pattern:
- Build release binary with `cargo build --release`
- Copy to `~/.local/bin/` for user access
- Provide usage examples in install output

## Session 2025-05-25: Zwift Race Finder Codebase Analysis & Chung Method Research

### Codebase Architecture Deep Dive

- **Rust Architecture**: Clean separation of concerns with data models, API clients, and business logic
  - Uses `tokio`/`reqwest` for async HTTP operations
  - `serde` for JSON deserialization with field renaming (`#[serde(rename_all = "camelCase")]`)
  - `clap` for CLI with derive macros for clean argument parsing
  - `colored` crate for rich terminal output

- **Smart Caching Strategy**: 
  - Cache location: `~/.cache/zwift-race-finder/user_stats.json`
  - 24-hour TTL balances API politeness with data freshness
  - Graceful degradation chain: CLI args → cached stats → ZwiftPower fetch → hardcoded defaults

- **Duration Estimation Algorithm**:
  - Base speeds: A=40, B=35, C=30, D=25, Strong D (190-199)=27 km/h
  - Route multipliers: Alpe/Ventoux=0.7x, Epic/Mountain=0.8x, Flat/Tempus=1.1x
  - Function at `rust/zwift-race-finder/src/main.rs:106`

### Chung Method Paper Discovery

- **Original Paper Found**: "Estimating CdA with a Power Meter" by R. Chung (March 2012)
  - Located at: `/mnt/c/Users/YOUR_USERNAME/OneDrive/Library/indirect-cda.pdf`
  - Email: your.email@example.com
  - Method also known as "Virtual Elevation Method"

- **Key Concept**: Compare calculated elevation (from power/speed) with actual GPS elevation
  - Adjust CdA and Crr until virtual elevation matches real elevation
  - Works even with "lousy" data (variable power/speed, hills, wind)
  - Validated against Martin et al. (1998, 2006) studies

### PDF Extraction Challenges

- Standard `Read` tool cannot handle binary PDFs
- `pdftotext` not available in WSL environment
- Solution: Used `uvx pdfplumber --format text` for PDF text extraction
- Note: pdfplumber generates CropBox warnings but successfully extracts text

### Potential Enhancement Ideas

- Could implement Chung method in Zwift context for personalized CdA measurement
- Zwift's controlled environment (no wind, exact elevation) ideal for virtual elevation testing
- Could replace category-based speed estimates with rider-specific CdA values

---

## Key Commands

```bash
# Build and install Zwift race finder
cd rust/zwift-race-finder && cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/

# Basic usage with auto-detected stats
zwift-race-finder

# Show only races in next 3 days
zwift-race-finder -r -n 3

# Override Zwift score and adjust tolerance
zwift-race-finder -s 220 -t 20

# Target shorter races (1.5h ±15min)
zwift-race-finder -d 90 -t 15

# Test Zwift API directly
curl "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '.[0]'

# Clear cached user stats to force refresh
rm ~/.cache/zwift-race-finder/user_stats.json

# Extract text from PDF files
uvx pdfplumber "/path/to/file.pdf" --format text --pages 1-5

# Search for academic papers in local library
find /mnt/c/Users/YOUR_USERNAME/OneDrive/Library -name "*chung*.pdf" -o -name "*CdA*.pdf"
```

## Session 2025-05-25: Zwift Physics Engine Deep Dive & Cycling Power Research

### Martin et al. (1998) Paper Analysis

- **Paper Located & Downloaded**: "Validation of a Mathematical Model for Road Cycling Power"
  - Source: University of Utah digital repository
  - Successfully extracted text using `pdftotext` (now available as standard Unix tool)
  - Key finding: Model predictions highly correlated with measured power (R² = 0.97)

- **Power Equation Components**:
  - Aerodynamic drag: 56-96% of total power (largest component)
  - Rolling resistance: 10-20% of total power
  - Potential energy changes: 10-20% even on 0.3% grade
  - Chain efficiency: 97.64% (2.36% loss)

### Zwift Physics Engine Research

- **Core Physics Model**: Modified Martin equation
  ```
  P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)ρ·CD·A·v³
  ```

- **CdA Calculations**:
  - Rider frontal area: A = 0.0276·h^0.725·m^0.425
  - Equipment CdA: 0.1647 base value
  - Height/weight dependent (taller = less aero)

- **Surface-Specific Rolling Resistance**:
  - Pavement: Crr = 0.004 (road), 0.008 (gravel), 0.009 (MTB)
  - Dirt surfaces: Rolling resistance doubles+
  - Nov 2023 update: 80W reduction for road bikes on dirt

- **Pack Dynamics 4.1 Drafting**:
  - 24.7-33% power savings by position
  - "Sticky draft" with wattage windows
  - Dynamic CdA: 3% bonus for attacks
  - Visual cue: avatar sits up when drafting >33kph

### Technical Discoveries

- **PDF Text Extraction**: `pdftotext` now preferred over Python solutions (bash-first philosophy)
- **Physics Simplifications**: Fixed air density (1.225 kg/m³), no wind variations, no collision detection
- **Speed Relationships**:
  - Flats: Speed ∝ ∛(Power/CdA)
  - Hills: Speed ∝ Power/Weight
  - Equipment matters: surface-specific optimal choices

### Potential Enhancements

- Implement Chung virtual elevation method in Zwift context
- Replace category-based speed estimates with rider-specific CdA
- Zwift's controlled environment ideal for CdA testing

---

## Key Commands

```bash
# Build and install Zwift race finder
cd rust/zwift-race-finder && cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/

# Basic usage with auto-detected stats
zwift-race-finder

# Show only races in next 3 days
zwift-race-finder -r -n 3

# Override Zwift score and adjust tolerance
zwift-race-finder -s 220 -t 20

# Target shorter races (1.5h ±15min)
zwift-race-finder -d 90 -t 15

# Test Zwift API directly
curl "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '.[0]'

# Clear cached user stats to force refresh
rm ~/.cache/zwift-race-finder/user_stats.json

# Extract text from PDF files
pdftotext "/path/to/file.pdf" -                    # to stdout
pdftotext -f 1 -l 5 file.pdf output.txt           # pages 1-5

# Search for academic papers in local library
find /mnt/c/Users/YOUR_USERNAME/OneDrive/Library -name "*chung*.pdf" -o -name "*CdA*.pdf"

# Download research papers
curl -L -o filename.pdf "URL"
```

## Session 2025-05-25: Cycling Physics Research & Enhanced Event Type Filtering

### Cycling Power Modeling Research Papers

#### Martin et al. (1998) - Foundation Paper
- **Full Title**: "Validation of a Mathematical Model for Road Cycling Power"
- **Journal**: Journal of Applied Biomechanics, Volume 14, Issue 3, pages 276-291
- **Authors**: James C. Martin, Douglas L. Milliken, John E. Cobb, Kevin L. McFadden, Andrew R. Coggan
- **Key Finding**: Model predictions highly correlated with measured power (R² = 0.97), standard error only 2.7W

**Complete Power Equation**:
```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)ρ·CD·A·v³
```

**Power Components Breakdown**:
- **Aerodynamic drag**: 56-96% of total power (largest component)
- **Rolling resistance**: 10-20% of total power  
- **Potential energy changes**: 10-20% even on 0.3% grade
- **Chain efficiency**: 97.64% (2.36% loss)
- **Bearing friction**: ~1% of total power
- **Kinetic energy changes**: ~1% of total power

#### Chung Method - Virtual Elevation Analysis
- **Paper Found**: "Estimating CdA with a Power Meter" by R. Chung (March 2012)
- **Location**: `/mnt/c/Users/YOUR_USERNAME/OneDrive/Library/indirect-cda.pdf`
- **Email**: your.email@example.com
- **Also Known As**: "Virtual Elevation Method"

**Key Concept**: Compare calculated elevation (from power/speed) with actual GPS elevation
- Adjust CdA and Crr until virtual elevation matches real elevation
- Works even with "lousy" data (variable power/speed, hills, wind)
- Validated against Martin et al. (1998, 2006) studies
- Method disclosed around April 2003, widely adopted in cycling community

#### Zwift Physics Engine Analysis

**Core Physics Implementation**:
- **Base Model**: Modified Martin equation with Zwift-specific adaptations
- **CdA Calculations**: 
  - Rider frontal area: `A = 0.0276·h^0.725·m^0.425` (height/weight dependent)
  - Equipment CdA: 0.1647 base value for bike + wheels
  - Taller/heavier riders have higher CdA (less aerodynamic)

**Surface-Specific Rolling Resistance**:
- **Pavement**: Crr = 0.004 (road), 0.008 (gravel), 0.009 (MTB)
- **Dirt/Gravel**: Rolling resistance doubles or more
- **Nov 2023 Update**: 80W reduction for road bikes on dirt surfaces

**Pack Dynamics 4.1 Drafting**:
- 24.7-33% power savings depending on position in pack
- "Sticky draft" with wattage windows for gameplay
- Dynamic CdA: 3% bonus for attacks (>20% power increase)
- Visual cue: avatar sits up when drafting at >33kph

**Physics Simplifications for Performance**:
- Fixed air density: 1.225 kg/m³ (no weather variations)
- No wind variations (unlike real world)
- No collision detection (riders can overlap)
- Simplified bearing friction calculations

**Speed Relationships in Zwift**:
- **Flats**: Speed ∝ ∛(Power/CdA) - raw watts matter most
- **Hills**: Speed ∝ Power/Weight - W/kg becomes critical  
- **Sprints**: Power/CdA ratio determines top speed

### Research Paper Discovery Process

**PDF Extraction Tools**:
- **Primary**: `pdftotext` (standard Unix tool, bash-first philosophy)
- **Fallback**: `uvx pdfplumber --format text` for complex PDFs
- **Issue**: Standard `Read` tool cannot handle binary PDFs

**Academic Paper Sources**:
- University of Utah digital repository (Martin et al. 1998)
- ResearchGate for cycling aerodynamics papers
- ZwiftHacks for comprehensive route data
- Zwift Insider for physics analysis and route information

## Session 2025-01-26: SQLite Integration & ZwiftPower Data Extraction

### Key Achievements

- **SQLite Database Integration**: Migrated from hardcoded route data to persistent SQLite storage
  - Routes table with distance, elevation, world, and surface type
  - Race results table for storing Jack's actual completion times
  - Unknown routes tracking for automatic data collection
  
- **Route ID as Primary Key**: Refactored all duration calculations to use route_id
  - More reliable than route names which can vary
  - Automatic logging of unknown route IDs for future mapping
  - Database stores 10+ mapped routes with elevation data

- **ZwiftPower Profile Extraction**: Created browser-based extraction workflow
  - JavaScript console script to extract race history after login
  - Handles authentication requirement (ZwiftPower now requires login)
  - Exports to JSON for database import

### Technical Solutions

- **Database Location**: `~/.local/share/zwift-race-finder/races.db`
- **New CLI Commands**:
  - `--show-unknown-routes`: Lists routes needing mapping
  - `--record-result "route_id,minutes,event_name"`: Records race results
  
- **Elevation-Based Speed Multipliers**: 
  - Calculate difficulty based on meters of elevation per km
  - <5m/km = 1.1x speed (very flat)
  - >40m/km = 0.7x speed (very hilly like Mt. Fuji)

### Browser Data Extraction Pattern

1. Save extraction JavaScript: `~/tools/rust/zwift-race-finder/extract_zwiftpower.js`
2. Copy to clipboard: `cat ~/tools/rust/zwift-race-finder/extract_zwiftpower.js | xclip -selection clipboard`
3. Run in browser console after login
4. File downloads to: `/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json`
5. Import: `~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import`

### Lessons Learned

- **Bash-First Philosophy**: Started with bash/curl approach, only moved to browser extraction when API required auth
- **Page Structure Analysis**: Created `save_page_structure.js` to capture table layouts for debugging
- **WSL Path Integration**: Downloads go to Windows Downloads folder, need explicit copy step
- **Rusqlite Bundled SQLite**: No need for system sqlite3 in Rust code, but bash scripts need it

### Documentation Created

- `/home/jack/tools/docs/WEBSITE_SCRAPING_GUIDE.md` - General guide for website data extraction
- `/home/jack/tools/rust/zwift-race-finder/ZWIFTPOWER_EXPORT_STEPS.md` - Specific workflow steps

### Cycling Performance Insights

**Real-World vs Virtual**:
- **Zwift Advantage**: Controlled environment ideal for CdA testing using Chung method
- **Missing Variables**: No crosswinds, consistent air density, exact elevation data
- **Equipment Effects**: Surface-specific optimal equipment (MTB for jungle, road for tarmac)

**Category Speed Estimates** (from tool analysis):
- Cat A: 40 km/h, Cat B: 35 km/h, Cat C: 30 km/h, Cat D: 25 km/h
- Strong Cat D (190-199 score): 27 km/h for better accuracy
- Route difficulty multipliers: 0.7x (climbs) to 1.1x (flats)

### Research Paper References

#### Downloaded PDFs (Local Library)
- **Martin et al. (1998)**: `martin_1998_cycling_power.pdf` (working directory)
  - Source: University of Utah digital repository
  - URL: https://collections.lib.utah.edu/dl_files/b4/8e/b48ef26086091662c561e673d7bd990d77868437.pdf
  - Size: 960k, successfully downloaded and text extracted

- **Chung (2012)**: `/mnt/c/Users/YOUR_USERNAME/OneDrive/Library/indirect-cda.pdf`
  - Original filename: `indirect-cda.pdf`
  - Author: R. Chung (your.email@example.com)
  - Version: March 2012
  - Historical URL: http://anonymous.coward.free.fr/wattage/cda/indirect-cda.pdf (server down)

#### Online Research Sources
- **Debraux et al. (2011)**: "Aerodynamic drag in cycling: Methods of assessment"
  - ResearchGate: https://www.researchgate.net/publication/51660070_Aerodynamic_drag_in_cycling_Methods_of_assessment
  - Review of 30 years of CdA evaluation methods

- **Martin et al. (2006a)**: "Modeling sprint cycling using field-derived parameters and forward integration"
  - MSSE 38(3):592-597

- **Martin et al. (2006b)**: "Aerodynamic drag area of cyclists determined with field-based measures"
  - Sportscience 10: 68-9

- **Zwift Physics Analysis Sources**:
  - Zwift Insider: https://zwiftinsider.com/zwift-speeds/
  - ZwiftHacks Routes: https://zwifthacks.com/app/routes/
  - Pack Dynamics Research: https://zwiftinsider.com/road-bike-drafting-pd41/

#### PDF Extraction Commands Used
```bash
# Primary method (successful)
pdftotext "/mnt/c/Users/YOUR_USERNAME/OneDrive/Library/indirect-cda.pdf" -

# Alternative method (tried)
uvx pdfplumber "/path/to/file.pdf" --format text --pages 1-5

# Download command
curl -L -o martin_1998_cycling_power.pdf "https://collections.lib.utah.edu/dl_files/b4/8e/b48ef26086091662c561e673d7bd990d77868437.pdf"
```

## Session 2025-05-25: Enhanced Event Type Filtering & Route Distance Mapping

### Key Problems Solved

- **Race Discovery Issue**: Fixed critical bug where actual races weren't being found despite being available in API
- **Event Type Granularity**: Replaced simple `-r` races-only flag with comprehensive `-e` event type filtering
- **Missing Route Data**: Implemented route distance estimation when API doesn't provide explicit distance/duration

### Technical Challenges & Solutions

- **API Data Structure Complexity**:
  - **Problem**: Many Zwift races have `distanceInMeters: 0` but include `routeId` for course identification
  - **Solution**: Added route ID → distance mapping based on common race routes
  - **Pattern**: `get_route_distance_by_id()` function with hardcoded popular routes

- **Duration vs. Distance Fields**:
  - **Problem**: API uses both `durationInMinutes` and `durationInSeconds` inconsistently
  - **Solution**: Added fallback chain: `duration_in_minutes.or_else(|| duration_in_seconds.map(|s| s / 60))`

- **Event Type Classification**:
  - **Discovery**: Fondos are `GROUP_RIDE` events but identifiable by name patterns
  - **Implementation**: Smart filtering by both `eventType` and name keywords
  - **Categories**: race, tt, fondo, group, workout, all

### Route Distance Mapping Strategy

- **Direct Route IDs**: Mapped specific `routeId` values to known distances
  ```rust
  2143464829 => Some(33.4),  // DBR races - Watopia Flat Route variants
  2927651296 => Some(67.5),  // Makuri Pretzel
  4107749591 => Some(25.7),  // London routes
  ```

- **Name Pattern Matching**: Fallback estimation from event names
  ```rust
  "3r" + "flat" → 33.4km
  "epic" + "pretzel" → 67.5km
  "crit" → 20.0km
  ```

### Debugging Infrastructure Added

- **Debug Mode**: `--debug` flag shows filtering pipeline stats
- **Multi-stage Filtering**: Separate retain() calls for each filter stage
- **Filter Chain Stats**: Shows how many events survive each filtering step

### CLI Interface Improvements

**Before**: `-r` (races only) boolean flag
**After**: `-e <type>` with options:
- `all` - All cycling events (default)
- `race` - Official Zwift races only
- `fondo` - Gran fondos, sportives, centuries
- `group` - Group rides (excluding fondos)
- `workout` - Structured group workouts
- `tt` - Time trials

### Testing Enhancements

- **Event Type Filter Tests**: Verify correct classification of race vs fondo vs group rides
- **Route Distance Estimation**: Test that common routes return expected durations
- **Multiple Test Events**: Create events with different `eventType` values for comprehensive testing

### Root Cause Analysis

**Original Issue**: Tool showed "No matching events found" even with wide criteria
**Root Cause**: Zwift races often lack explicit distance data at main event level
**Key Insight**: Course information available via `routeId` but requires mapping to distances
**Solution Hierarchy**:
1. Explicit distance/duration from API
2. Route ID → distance lookup
3. Name pattern → distance estimation
4. Subgroup data checking

### Performance Considerations

- **Route Lookup**: O(1) hashmap-style matching for route IDs
- **Pattern Matching**: Simple string contains() operations
- **Filter Pipeline**: Early filtering reduces processing load

---

## Key Commands

```bash
# Build and install Zwift race finder
cd rust/zwift-race-finder && cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/

# Event type filtering - NEW in this session
zwift-race-finder -e race -n 3              # Find races in next 3 days
zwift-race-finder -e fondo -t 60            # Find fondos with ±60 min tolerance
zwift-race-finder -e group                  # Find regular group rides
zwift-race-finder -e workout                # Find structured workouts
zwift-race-finder -e tt                     # Find time trials

# Debug mode to troubleshoot filtering
zwift-race-finder -e race --debug -n 2      # Show filtering pipeline stats

# Basic usage with auto-detected stats
zwift-race-finder

# Show only races in next 3 days
zwift-race-finder -r -n 3

# Override Zwift score and adjust tolerance
zwift-race-finder -s 220 -t 20

# Target shorter races (1.5h ±15min)
zwift-race-finder -d 90 -t 15

# Test Zwift API directly
curl "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '.[0]'

# Clear cached user stats to force refresh
rm ~/.cache/zwift-race-finder/user_stats.json

# Extract text from PDF files
pdftotext "/path/to/file.pdf" -                    # to stdout
pdftotext -f 1 -l 5 file.pdf output.txt           # pages 1-5

# Search for academic papers in local library
find /mnt/c/Users/YOUR_USERNAME/OneDrive/Library -name "*chung*.pdf" -o -name "*CdA*.pdf"

# Download research papers
curl -L -o filename.pdf "URL"

# Check event types in Zwift API
curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '[.[] | select(.sport == "CYCLING") | .eventType] | unique | sort'

# Debug specific event structure
curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '[.[] | select(.sport == "CYCLING" and .eventType == "RACE")] | .[0]'

# Zwift Race Finder with SQLite
cd ~/tools/rust/zwift-race-finder && cargo run -- --duration 90 --tolerance 30

# Show unknown routes that need mapping
zwift-race-finder --show-unknown-routes

# Record actual race result
zwift-race-finder --record-result "route_id,minutes,event_name"

# Extract ZwiftPower data (after login)
cat ~/tools/rust/zwift-race-finder/extract_zwiftpower.js | xclip -selection clipboard
~/tools/rust/zwift-race-finder/copy_results.sh
~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import

# Query race results database
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT * FROM zwiftpower_results;"

## Session 2025-05-25: ZwiftPower Data Import and Regression Testing

### Major Accomplishments
- Successfully imported 163 historical races from ZwiftPower using browser-based extraction
- Built regression testing framework comparing predicted vs actual race times
- Discovered critical issue: multi-lap races need total distance calculation (e.g., "3 Laps" = 3x route distance)
- Updated scoring system to use Zwift Racing Score (Jack's score: 189 in Category D)
- Created route mapping system using route_id as primary key

### Technical Discoveries

**ZwiftPower Data Extraction**:
- ZwiftPower requires login, so built browser console JavaScript extractors
- Page structure: `#profile_results` table contains race history
- Data includes: date, event name, category, position, distance, score
- Successfully extracted all 163 races with pagination handling

**Database Schema Alignment**:
- Rust app uses `race_results` table with route_id foreign key
- Created import workflow using placeholder route_id 9999 for unmapped routes
- Tracks unknown routes for progressive mapping
- Recent results (last 3 months) are most reliable for performance

**Route Mapping Insights**:
- Event names often contain route info: "3R Volcano Flat Race - 3 Laps (36.6km/22.7mi 138m)"
- Key routes identified: Volcano Flat (1015), Alpe du Zwift (6), Innsbruckring (236)
- Multi-lap races multiply base route distance
- Draft benefit in races yields ~30% higher speeds than solo riding

**Regression Testing Results**:
- Built test framework to validate predictions
- Discovered major discrepancy: predicted 26 min vs actual 69 min for Volcano Flat
- Root cause: 3 laps (36.6km) not accounted for in base route distance (12.2km)
- Actual race speed with draft: ~31.8 km/h (reasonable for Cat D)

# ZwiftPower Data Extraction
cat ~/tools/rust/zwift-race-finder/extract_zwiftpower_final.js | xclip -selection clipboard
~/tools/rust/zwift-race-finder/import_zwiftpower_results.sh

# Route Research and Mapping
~/tools/rust/zwift-race-finder/route_research.sh                  # Analyze event patterns
~/tools/rust/zwift-race-finder/route_research.sh "3R Racing"      # Search specific series
~/tools/rust/zwift-race-finder/fix_unknown_routes.sh              # Update unknown routes tracking
~/tools/rust/zwift-race-finder/apply_mappings.sh                  # Apply route mappings

# Database Queries
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT COUNT(*) FROM race_results;"
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT * FROM routes;"
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT DISTINCT event_name FROM race_results WHERE route_id = 9999;"

# Regression Testing
cd ~/tools/rust/zwift-race-finder && cargo test regression_test -- --nocapture
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT * FROM unknown_routes ORDER BY times_seen DESC;"
```