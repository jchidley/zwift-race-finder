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

## Session 2025-05-26: SQL Fixes and Security Improvements

### Key Achievements
- **Fixed SQL Errors**: Discovered SQLite doesn't support table aliases in UPDATE statements
- **Implemented Workaround**: Rewrote queries using temporary table approach
- **Improved Match Rate**: Successfully matched 131/163 races (80% match rate)
- **Final Accuracy**: 23.6% mean error (exceeded <30% target)
- **Security**: Added strava_config.json to .gitignore to protect OAuth tokens

### Technical Discoveries

**SQLite UPDATE Limitations**:
- Cannot use table aliases in UPDATE statements (e.g., `UPDATE table AS t`)
- Correlated subqueries referencing outer table are problematic
- Solution: Use temporary tables to pre-calculate matches

**Data Source Limitations**:
- ZwiftPower only exports dates, not timestamps
- Race times stored as "2025-05-25" (midnight)
- Strava has full timestamps "2025-05-25T16:30:17Z"
- This creates ~16-17 hour time differences but doesn't affect date matching

**Match Rate Analysis**:
- 80% match rate (131/163) is excellent
- Unmatched races are normal:
  - Races before Strava usage
  - Technical recording failures
  - Different platforms used
  - DNFs not saved

### Security Improvements
```bash
# Added to .gitignore
strava_config.json

# Created template
strava_config.json.example

# Updated documentation
README.md - Added security note
todo.md - Added token security tasks
```

### Final Project Status
- ✅ Production ready with 23.6% accuracy
- ✅ All regression tests passing
- ✅ OAuth tokens secured
- ✅ Published to GitHub
- ✅ Ready for public use

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

## Session 2025-05-25: ZwiftPower Data Import and Regression Testing (continued)

### Key Discoveries

- **ZwiftPower Data Reliability Issues**:
  - The "Result/Score" field in ZwiftPower is NOT the Zwift Racing Score
  - Category fields are inconsistent and don't match Zwift Racing Score pen system
  - Position data is not meaningful (depends on field size)
  - Reliable fields: power metrics (watts, w/kg), heart rate (bpm), distance (km)

- **Zwift Racing Score vs ZwiftPower Data**:
  - Jack's actual Zwift Racing Score: 195 (Cat D)
  - ZwiftPower "scores" were 410-600 (completely different metric)
  - Must use hardcoded racing score, not imported values

- **Route Mapping Challenges**:
  - Event names don't reliably indicate actual routes
  - Same event series can use different routes/lap counts
  - Need route_id from Zwift API for accurate mapping
  - Created route_mappings.sql for batch updates

- **Regression Testing Results**:
  - Current algorithm shows 78% mean error - needs calibration
  - Base speed assumptions too high (25 km/h for Cat D vs actual ~20 km/h)
  - Route distances in database don't match actual race distances (lap count issues)

- **Physical Attributes Impact**:
  - Weight crucial for w/kg calculations
  - Height (1.82m for Jack) affects aerodynamics and draft benefit
  - These matter more than position/category for performance modeling

### Solutions Implemented

- **Bitwarden Integration**: 
  - Secure credential storage for ZwiftPower
  - Fixed wrapper script color code issues
  - No more hardcoded credentials

- **Database Schema Fixes**:
  - Handle mixed integer/real types for zwift_score
  - Use String for race_date (stored as text)
  - Exclude unreliable fields from import

- **Import Script Updates**:
  - Use Jack's actual Zwift Racing Score (195)
  - Remove category/position from imports
  - Focus on objective metrics (power, HR, distance)

### Technical Challenges Solved

- **SQLite Type Mismatches**: 
  ```rust
  // Handle zwift_score as either integer or real
  let zwift_score_raw: Result<u32, _> = row.get(4);
  let zwift_score = match zwift_score_raw {
      Ok(val) => val,
      Err(_) => {
          let val: f64 = row.get(4)?;
          val.round() as u32
      }
  };
  ```

- **Subshell BW_SESSION Issue**:
  ```bash
  # Ensure BW_SESSION is available to the script
  export BW_SESSION="${BW_SESSION}"
  ```

---
## Key Commands

```bash
# Bitwarden setup and usage
export BW_SESSION=$(bw unlock --raw)
./bw_config.sh setup
./zwift-race-finder-bw  # Wrapper with auto-credential loading

# Data import workflow
# 1. Extract from ZwiftPower (paste in browser console)
cat extract_zwiftpower_v3.js | xclip -selection clipboard
# 2. Import results
./dev_import_results.sh
# 3. Apply route mappings
sqlite3 ~/.local/share/zwift-race-finder/races.db < route_mappings.sql

# Regression testing
cargo test test_race_predictions_accuracy -- --nocapture

# Database queries
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT COUNT(DISTINCT event_name) FROM race_results;"
sqlite3 ~/.local/share/zwift-race-finder/races.db ".schema race_results"

# Run with specific parameters
./zwift-race-finder-bw --duration 60 --tolerance 60 --days 7

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

# ZwiftPower Data Extraction
cat ~/tools/rust/zwift-race-finder/zwiftpower_profile_extractor.js | xclip -selection clipboard
~/tools/rust/zwift-race-finder/import_zwiftpower.sh

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

## Session 2025-05-25: Zwift Race Prediction Accuracy & Strava API Integration

### Key Discoveries

- **Regression Test Results**: Current predictions show 92.8% mean error - predictions are way off
  - Root cause: We're using estimated race times from distance/category, not actual times
  - ZwiftPower doesn't display race duration on the profile summary page
  - The "actual_minutes" in our database are ESTIMATED (30 km/h for Cat D)

- **Route Distance Issues**:
  - KISS Racing showing 100km but races finish in ~1 hour (impossible for Cat D)
  - Multi-lap races not accounting for total distance
  - Route distances in database need verification

- **ZwiftPower Limitations**:
  - Summary page lacks race duration data
  - Individual event pages DO have actual times (e.g., "1:51:42" for Jack Chidley)
  - Would require visiting each event page individually

- **Zwift API Research**:
  - No official public API documentation
  - Developer API exists but not available to hobby developers
  - Contact: developers@zwift.com (limited responses)
  - Only public endpoints:
    - Events: `https://us-or-rly101.zwift.com/api/public/events/upcoming`
    - Status: `https://status.zwift.com/api/v2/status.json`

### Solutions Implemented

- **Strava API Integration**: Created complete workflow for getting actual race times
  - `strava_auth.sh` - OAuth authentication setup
  - `strava_fetch_activities.sh` - Fetch Zwift virtual rides
  - `strava_import_to_db.sh` - Match and import race times
  - `strava_analyze.py` - Analyze performance patterns

- **Popular Zwift API Projects Found**:
  - **zwift-offline** (912+ stars) - Run Zwift offline, Python
  - **Sauce4Zwift** (400+ stars) - Real-time API on localhost:1080
  - **zwift-mobile-api** (109+ stars) - Limited by Developer API restrictions

### Technical Insights

- **Speed Calibration Needed**:
  - Base Cat D speed: 25 km/h (solo) vs ~30+ km/h (race with draft)
  - Draft benefit provides ~30% speed boost
  - Must account for this in predictions

- **Data Architecture**:
  - Zwift API provides: route_id, laps, distance
  - Strava API provides: actual moving_time, elapsed_time
  - Combined approach gives complete picture

---
## Key Commands

```bash
# Strava API Integration
./strava_auth.sh                    # Set up OAuth authentication
./strava_fetch_activities.sh        # Fetch Zwift activities from Strava
./strava_import_to_db.sh           # Import race times to database
./strava_analyze.py                # Analyze race performance

# Test Zwift public API
curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" | jq '.[0]'

# Check event structure with route data
curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" | \
  jq '[.[] | select(.eventType == "RACE")][0] | {name, routeId, laps, distanceInMeters, eventSubgroups}'

# Run regression tests
cd ~/tools/rust/zwift-race-finder && cargo test regression_test -- --nocapture

# Database queries
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT * FROM routes ORDER BY route_id;"
sqlite3 ~/.local/share/zwift-race-finder/races.db ".schema race_results"

# Popular Zwift API projects
# https://github.com/zoffline/zwift-offline (912+ stars)
# https://github.com/SauceLLC/sauce4zwift (400+ stars)
# Sauce4Zwift API: http://localhost:1080/api (when running)

# Device Emulation Setup
git clone https://github.com/paixaop/zwack.git && cd zwack && npm install
git clone https://github.com/ptx2/gymnasticon.git && cd gymnasticon && npm install
git clone https://github.com/WouterJD/FortiusANT.git && cd FortiusANT

# Run Zwack simulator
node server.js  # Then use w/s for power, a/d for cadence

# Gymnasticon for Peloton bridge
npm start -- --bike peloton
```

## Session 2025-05-25: Device Emulation Projects for Zwift Testing

### Key Discoveries

- **Device Simulation Need**: For testing race predictions without actually riding
  - Can simulate different power profiles (Cat A/B/C/D)
  - Test edge cases (power spikes, dropouts)
  - Automated regression testing possibilities

- **Top GitHub Projects Found**:
  
  1. **Gymnasticon** (500+ stars) - github.com/ptx2/gymnasticon
     - JavaScript/Node.js
     - Bridges proprietary bikes (Peloton, IC4) to Zwift
     - Simulates ANT+ and BLE power/cadence sensors
     - Runs on Raspberry Pi Zero W
  
  2. **FortiusANT** (100+ stars) - github.com/WouterJD/FortiusANT
     - Python
     - Full ANT+ FE-C trainer control implementation
     - Grade simulation and power curves
     - Can run headless for testing
  
  3. **Zwack** (50+ stars) - github.com/paixaop/zwack
     - JavaScript/Node.js
     - Pure BLE sensor simulator
     - FTMS, Cycling Power, Heart Rate services
     - Keyboard controls: w/s (power), a/d (cadence)
  
  4. **openant** (300+ stars) - github.com/Tigge/openant
     - Python ANT+ library
     - Includes device simulator examples
     - Full protocol implementation
  
  5. **zwift-offline** (912+ stars) - github.com/zoffline/zwift-offline
     - Python
     - Complete offline Zwift server
     - Uses power simulators for RoboPacers

- **ANT+ vs Bluetooth Considerations**:
  - Most simulators need separate hardware (Raspberry Pi)
  - Cannot run on same computer as Zwift
  - ANT+ uses dongles, BLE uses built-in Bluetooth

### Technical Implementation Notes

- **Zwack Quick Start**:
  ```bash
  git clone https://github.com/paixaop/zwack.git
  cd zwack
  npm install
  node server.js
  # Use w/s/a/d keys to control power/cadence
  ```

- **Testing Applications for Race Finder**:
  - Simulate consistent power output to verify duration calculations
  - Test draft benefit by running multiple simulators
  - Validate category-based speed estimates
  - Edge case testing (0W to 1000W spikes)

### Integration Opportunities

- Could create automated test suite using these simulators
- Generate power profiles matching historical race data
- Test prediction accuracy without manual riding
- Simulate entire race fields for draft calculations

## Session 2025-01-25: Project Cleanup and Cruft Removal

### Key Discovery
The project has accumulated significant cruft from multiple abandoned approaches:
- Multiple versions of ZwiftPower extraction scripts (v1, v2, v3, final, safe, all_pages)
- Abandoned Bitwarden/security configuration attempts (6+ different setup scripts)
- Debug and test scripts that are no longer needed
- The project pivoted from ZwiftPower scraping to Strava API but kept all the old files

### Files Identified for Deletion (26 files)
**ZwiftPower Scraping (replaced by Strava API):**
- extract_zwiftpower.js, extract_zwiftpower_all_pages.js, extract_zwiftpower_safe.js
- extract_zwiftpower_final.js, extract_zwiftpower_v3.js
- debug_pagination.js, debug_zwiftpower_page.js, save_page_structure.js
- scrape_zwiftpower.py, scrape_zwiftpower.sh
- export_zwiftpower_logged_in.sh, import_zwiftpower_results_v2.sh
- check_time_field.js, extract_event_time.js, extract_full_race_data.js

**Configuration Cruft:**
- setup_bitwarden_config.sh, setup_bitwarden_proper.sh
- setup_encrypted_config.sh, setup_secure_config.sh, setup_personal_config.sh
- bitwarden_item_template.json, config.example.json

**Other Dead Files:**
- copy_results.sh, import_and_merge_results.sh, collect_routes.sh
- route_research.sh, sanitize_personal_data.sh
- create_extended_schema.sql, analyze_speeds.sh

### Current Active Approach
- Using Strava API for real race times (strava_*.sh scripts)
- Keeping only extract_zwiftpower_v2.js as referenced in CLAUDE.md
- Core functionality in Rust src/ files remains unchanged

### Cleanup Commands
```bash
# Create list of files to delete
cat > files_to_delete.txt << 'EOF'
extract_zwiftpower.js
extract_zwiftpower_all_pages.js
extract_zwiftpower_safe.js
extract_zwiftpower_final.js
extract_zwiftpower_v3.js
debug_pagination.js
debug_zwiftpower_page.js
save_page_structure.js
scrape_zwiftpower.py
scrape_zwiftpower.sh
export_zwiftpower_logged_in.sh
import_zwiftpower_results_v2.sh
check_time_field.js
extract_event_time.js
extract_full_race_data.js
setup_bitwarden_config.sh
setup_bitwarden_proper.sh
setup_encrypted_config.sh
setup_secure_config.sh
setup_personal_config.sh
bitwarden_item_template.json
config.example.json
copy_results.sh
import_and_merge_results.sh
collect_routes.sh
route_research.sh
sanitize_personal_data.sh
create_extended_schema.sql
analyze_speeds.sh
EOF

# Review what will be deleted
cat files_to_delete.txt | xargs -I {} ls -la {}

# Delete the files
cat files_to_delete.txt | xargs rm -v

# Clean up
rm files_to_delete.txt
```

---
## Key Commands

```bash
# Build and run with defaults
cargo run

# Install to ~/.local/bin
./install.sh

# Run with specific parameters
cargo run -- --zwift-score 195 --duration 120 --tolerance 30

# Show unknown routes
cargo run -- --show-unknown-routes

# Record race result
cargo run -- --record-result "route_id,minutes,event_name"

# Run regression tests
cargo test regression

# Extract data from ZwiftPower (browser)
cat zwiftpower_profile_extractor.js | xclip -selection clipboard

# Import ZwiftPower results
./import_zwiftpower_dev.sh           # Development
./import_zwiftpower.sh              # Production

# Apply route mappings
sqlite3 ~/.local/share/zwift-race-finder/races.db < route_mappings.sql

# Strava integration
./strava_auth.sh                     # Authenticate with Strava
./strava_fetch_activities.sh         # Fetch Zwift activities
./strava_import_to_db.sh            # Import real race times
python strava_analyze.py            # Analyze performance

# Fix KISS Racing distance
sqlite3 ~/.local/share/zwift-race-finder/races.db "UPDATE routes SET distance_km = 35.0 WHERE route_id = 2474227587"

# Check for secrets before committing
./check_secrets.sh

# List files for cleanup
find . -name "*.js" -o -name "*.py" -o -name "*_v[0-9]*" | grep -E "(v[0-9]|_old|debug_|test_|temp_|backup_)" | sort
```

## Session 2025-01-25: Major Cleanup and Strava Integration Success

### The Big Discovery
**We were comparing estimates to estimates!** The "actual_minutes" in the database weren't real race times - they were calculated as `distance ÷ 30 km/h`. This explained the 92.8% prediction error.

### Problems Solved
1. **Project Cruft** - Removed 28 dead files from abandoned approaches
2. **Confusing Filenames** - Renamed files for clarity (e.g., `extract_zwiftpower_v2.js` → `zwiftpower_profile_extractor.js`)
3. **No Real Race Times** - Successfully integrated Strava API to get actual race durations
4. **Wrong Base Speed** - Updated from 25 km/h to 30.9 km/h (based on 151 real races)
5. **Incorrect Route Distances** - Fixed major routes using Strava data

### Technical Implementation
- Created Strava OAuth flow with proper token management
- Built activity fetcher filtering for Zwift virtual rides
- Implemented smart matching between Strava activities and database races
- Used PEP 723 script dependencies for Python tools (Pillow for icon creation)

### Results
- **Prediction Error**: 92.8% → 31.2% (66% improvement!)
- **Real Data**: Now have 151 races with actual times from Strava
- **Fixed Routes**:
  - KISS Racing: 100km → 35km
  - Ottawa TopSpeed: 19.8km → 50km  
  - EVO CC (Bell Lap): 14.1km → 45km

### Key Insights
1. ZwiftPower profile exports only show estimated times
2. Individual ZwiftPower event pages (e.g., zwiftpower.com/events.php?zid=4943630) show real times
3. Strava API is the most reliable source for personal race times
4. Average Cat D race speed with draft: 30.9 km/h (not 25 km/h)
5. Many "races" are multi-lap events not reflected in base route distance

### Cleanup Commands Used
```bash
# Find versioned/debug files
find . -name "*.js" -o -name "*.py" -o -name "*_v[0-9]*" | grep -E "(v[0-9]|_old|debug_|test_|temp_|backup_)" | sort

# Batch delete with confirmation
cat files_to_delete.txt | xargs -I {} ls -la {}  # Review first
cat files_to_delete.txt | xargs rm -v             # Then delete

# Git cleanup commit
git add -A
git commit -m "refactor: major cleanup - remove dead code and rename files"
```

### Strava Integration Process
```bash
# 1. Create Strava app icon
uv run create_icon.py  # Uses PEP 723 inline dependencies

# 2. Authenticate (requires manual OAuth flow)
./strava_auth.sh

# 3. Import process
./strava_fetch_activities.sh     # Gets all Zwift virtual rides
./strava_import_to_db.sh        # Matches to races by name/date
uv run strava_analyze.py        # Shows speed statistics
```

### Database Fixes Applied
```sql
-- Fix route distances based on Strava data
UPDATE routes SET distance_km = 35.0 WHERE route_id = 2474227587;  -- KISS Racing
UPDATE routes SET distance_km = 50.0 WHERE route_id = 1656629976;  -- Ottawa TopSpeed
UPDATE routes SET distance_km = 45.0 WHERE route_id = 1258415487;  -- EVO CC Bell Lap
```

---
## Key Commands

```bash
# Build and run with defaults
cargo run

# Install to ~/.local/bin
./install.sh

# Run with specific parameters
cargo run -- --zwift-score 195 --duration 120 --tolerance 30

# Show unknown routes
cargo run -- --show-unknown-routes

# Record race result
cargo run -- --record-result "route_id,minutes,event_name"

# Run regression tests
cargo test regression_test -- --nocapture

# Extract data from ZwiftPower (browser)
cat zwiftpower_profile_extractor.js | xclip -selection clipboard

# Import ZwiftPower results
./import_zwiftpower_dev.sh           # Development
./import_zwiftpower.sh               # Production

# Apply route mappings
./apply_route_mappings.sh

# Strava integration
./strava_auth.sh                     # Authenticate with Strava
./strava_fetch_activities.sh         # Fetch Zwift activities
./strava_import_to_db.sh            # Import real race times
uv run strava_analyze.py            # Analyze performance

# Fix route distances
sqlite3 ~/.local/share/zwift-race-finder/races.db "UPDATE routes SET distance_km = 35.0 WHERE route_id = 2474227587"

# Check for secrets before committing
./check_secrets.sh

# Create Strava app icon
uv run create_icon.py               # Uses inline PEP 723 dependencies

# Find and clean up cruft
find . -name "*.js" -o -name "*.py" -o -name "*_v[0-9]*" | grep -E "(v[0-9]|_old|debug_|test_|temp_|backup_)" | sort
```

## Session 2025-05-25: Multi-Lap Race Handling & Event Subgroups

### Key Discovery
Different categories race different distances! The Zwift API provides per-category data in `event_sub_groups`, not the main event structure. For example, "3R Volcano Flat Race - 3 Laps" might be:
- Cat A/B: 3 laps (36.6km)
- Cat C/D: 2 laps (24.4km)
- Cat E: 1 lap (12.2km)

### Problems Solved
1. **Multi-lap Race Predictions** - Fixed incorrect predictions for multi-lap races (was showing 21 min for 67-74 min races)
2. **Per-Category Distances** - Now using event_sub_groups to get category-specific distances
3. **Lap Detection** - Added parsing for "X Laps" in event names and distance extraction (36.6km/22.7mi format)
4. **Regression Test Accuracy** - Reduced mean error from 31.2% to 25.1% (below 30% target!)

### Technical Implementation
- Created `find_user_subgroup()` to match user's category from event_sub_groups
- Added `estimate_duration_with_distance()` for explicit distance calculations
- Updated filtering logic to check subgroup distances when main event lacks data
- Modified regression tests to parse distances from event names for multi-lap races

### Code Changes
```rust
// Find the subgroup that matches the user's category
fn find_user_subgroup<'a>(event: &'a ZwiftEvent, zwift_score: u32) -> Option<&'a EventSubGroup> {
    let user_category = match zwift_score {
        0..=199 => "D",
        200..=299 => "C",
        300..=399 => "B",
        _ => "A",
    };
    
    event.event_sub_groups.iter().find(|sg| {
        sg.name.contains(user_category) || 
        (user_category == "D" && sg.name.contains("E"))
    })
}

// Duration estimation with explicit distance (for multi-lap races)
fn estimate_duration_with_distance(route_id: u32, distance_km: f64, zwift_score: u32) -> Option<u32>
```

### Key Insights
1. Event names often contain lap count and total distance but aren't reliable
2. The event_sub_groups field is the proper source for per-category race data
3. Base route distance × lap count = total race distance
4. Multi-lap races were the main source of prediction errors

### Results
- 3R Volcano Flat Race (3 Laps): Now correctly predicts ~71 min (was 21 min)
- Mean prediction error: 25.1% (down from 31.2%)
- All regression tests now passing

## Meta: Lessons on AI-Assisted Development

### What Makes This Approach Work
1. **Domain Knowledge Matters**: Understanding Zwift racing, power/weight ratios, and draft dynamics guided better solutions
2. **Technical Experience Helps**: 40 years of IT experience meant recognizing when to use SQLite vs JSON, understanding API patterns, and knowing what questions to ask
3. **Management Mindset**: Treating Claude as an enthusiastic employee who needs clear direction and occasional correction
4. **Transparency is Key**: Claude showing reasoning helps spot misunderstandings before they become bugs

### Key Success Patterns
- **Problem First**: Started with "I know when and how long I want to race, but not which races fit"
- **Iterate on Real Data**: Discovered fake "actual times" by testing predictions
- **Pivot When Needed**: Moved from ZwiftPower to Strava when we hit limitations
- **Test Assumptions**: Event names said one thing, but data showed different (multi-lap races)

### What This Proves
A non-coder with domain expertise and good management skills can build real, working software using AI. The 25.1% accuracy (improving from 92.8%) shows this isn't a toy - it's a practical tool solving a real problem.

## Session 2025-05-25: Documentation Philosophy & AI Development Approach

### Key Insights on AI-Assisted Development
- **Reframed the narrative**: Not "non-coder builds software" but "IT professional leverages AI without coding"
- **Management model**: Treating Claude as "a very willing and enthusiastic employee" who needs clear direction
- **Transparency value**: The real benefit is seeing WHY Claude makes decisions, not learning programming concepts
- **Domain + Technical**: 40 years IT experience + Zwift racing knowledge = effective AI direction

### Documentation Updates Made
1. **README.md**: Added "Why This Tool Exists" section with dual motivations
2. **plan.md**: Added "Development Approach" highlighting human/AI partnership
3. **CLAUDE.md**: Added "Development Philosophy" with transparency principles
4. **todo.md**: Expanded learnings to include AI development insights
5. **AI_DEVELOPMENT.md**: Created comprehensive guide on the approach

### Key Realizations
- Technical knowledge remains important - helps spot issues and guide architecture
- Success requires: clear problem definition + domain knowledge + management skills
- Data often contradicts descriptions (event names vs actual distances)
- Iterative refinement with real data reveals wrong assumptions

### Philosophy Captured
"This is like managing a very willing and enthusiastic employee. Success requires:
- Clear Direction: Think deeply about what problem you're actually solving
- Big Picture Thinking: Keep the overall goal in mind, not just the current task
- Understanding Limitations: Both yours (domain knowledge) and the LLM's (assumptions)"

## Session 2025-05-25: Pack-Based Physics Model & Drop Dynamics Discovery

### Major Discovery: Why Variance Exists
Found the root cause of prediction variance! Getting dropped from the pack on hills explains most of the variation in race times:
- **Bell Lap**: Same rider, same route varies from 32-86 minutes (82.6% variance)
- **Binary State**: Either riding with pack (30.9 km/h) OR solo without draft (23.8 km/h)
- **Weight Penalty**: At 86kg vs typical 70-75kg, Jack gets dropped on climbs
- **Cascade Effect**: Drop early → lose draft → race mostly solo → much longer time

### Technical Implementation
Created dual-speed model with drop probability:
```rust
// Calculate probability of getting dropped based on elevation and weight
fn calculate_drop_probability(elevation_per_km: f64, weight_kg: f64, category: &str) -> f64 {
    let weight_penalty = ((weight_kg - 75.0) / 10.0).max(0.0) * 0.15;
    let climb_factor = (elevation_per_km / 10.0).min(1.0);
    let category_factor = match category {
        "D" => 1.2,  // Cat D more likely to fragment
        "C" => 1.0,
        "B" => 0.8,
        "A" => 0.6,  // Cat A packs stay together
        _ => 1.0,
    };
    (weight_penalty + climb_factor * category_factor).min(1.0)
}
```

### Results
- **Accuracy**: 36.9% mean error (improved from 92.8% but still above 30% target)
- **Bell Lap**: Now predicts 32 min (matches fastest actual time)
- **Key Insight**: High variance is inherent to racing, not a prediction failure

### Pack Dynamics Research
- **Draft Benefit**: 33% power savings in Zwift vs 25% real world
- **Pack Speed**: Determined by strongest riders, not average
- **Zwift vs Real**: Martin et al. model accurate for real cycling but overestimates by 127% in Zwift
- **Context Matters**: Bigger races = more consistent draft = better predictions

## Session 2025-05-25: Route Mapping Fix & Test Suite Completion

### Key Discovery
Found incorrect route mapping causing major accuracy issues:
- **EVO CC Race Series** was mapped to Bell Lap (14.1km) 
- Actually runs on different routes like Coast Crusher (45km) and Loch Loop (35km)
- This single mapping error caused 11.2% accuracy degradation

### Solutions Implemented
1. **Fixed Route Mapping**: Unmapped EVO CC races from Bell Lap
2. **Added Test Suite**: 
   - Route mapping consistency test (would have caught this issue)
   - Multi-lap race detection test
   - Edge case tests (sprint, gran fondo, extreme elevation)
   - Database route validation test
3. **Updated Test Expectations**: Fixed 7 failing tests by updating from 27→30.9 km/h

### Results
- **Accuracy**: Improved from 36.9% to 25.7% (below 30% target!)
- **All Tests Passing**: Comprehensive test coverage prevents future regressions
- **Key Learning**: Route mapping accuracy is critical - single error can tank predictions

### Test Coverage Added
- `test_route_mapping_consistency`: Validates mapped routes have reasonable race times
- `test_multi_lap_race_detection`: Tests lap parsing and distance calculations
- `test_edge_case_estimations`: Tests extreme scenarios (5km sprints, 100km fondos)
- `test_database_route_validation`: Ensures all routes have valid data

---
## Key Commands

```bash
# Build and run with defaults
cargo run

# Install to ~/.local/bin
./install.sh

# Run with specific parameters
cargo run -- --zwift-score 195 --duration 120 --tolerance 30

# Show unknown routes
cargo run -- --show-unknown-routes

# Record race result
cargo run -- --record-result "route_id,minutes,event_name"

# Run all tests
cargo test

# Run specific test module
cargo test regression

# Update rider stats
./update_rider_stats.sh 86.0        # Weight only
./update_rider_stats.sh 86.0 250    # Weight and FTP

# Extract data from ZwiftPower (browser)
cat zwiftpower_profile_extractor.js | xclip -selection clipboard

# Import ZwiftPower results
./import_zwiftpower_dev.sh           # Development
./import_zwiftpower.sh               # Production

# Apply route mappings
./apply_route_mappings.sh

# Strava integration
./strava_auth.sh                     # Authenticate with Strava
./strava_fetch_activities.sh         # Fetch Zwift activities
./strava_import_to_db.sh            # Import real race times
uv run strava_analyze.py            # Analyze performance

# Database maintenance
sqlite3 ~/.local/share/zwift-race-finder/races.db "UPDATE routes SET distance_km = 35.0 WHERE route_id = 2474227587"
sqlite3 ~/.local/share/zwift-race-finder/races.db "UPDATE race_results SET route_id = 9999 WHERE event_name = 'EVO CC Race Series'"

# Check for secrets before committing
./check_secrets.sh
```