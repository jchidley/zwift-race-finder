# Zwift OCR Technical Reference

## Architecture Overview

```
┌─────────────────────┐
│   Video/Image Input │
└──────────┬──────────┘
           │
           v
┌─────────────────────┐
│  Region Extraction  │ ← Optimized UI coordinates
└──────────┬──────────┘
           │
           v
┌─────────────────────┐
│   Preprocessing     │ ← Threshold, Scale, Invert
└──────────┬──────────┘
           │
           v
┌─────────────────────┐
│    OCR Engine       │ ← PaddleOCR (Python) / Tesseract (Rust)
└──────────┬──────────┘
           │
           v
┌─────────────────────┐
│  Pattern Matching   │ ← Character constraints
└──────────┬──────────┘
           │
           v
┌─────────────────────┐
│   Data Output       │ → JSON, CSV, SQLite
└─────────────────────┘
```

## UI Region Mapping (1920x1080)

### Optimized Coordinates (100% Accuracy)
```python
# Top bar elements
SPEED = Region(693, 44, 71, 61)          # km/h
DISTANCE = Region(833, 44, 84, 55)       # km
ALTITUDE = Region(975, 45, 75, 50)       # m
RACE_TIME = Region(1070, 45, 134, 49)    # MM:SS

# Power panel
POWER = Region(268, 49, 117, 61)         # W
CADENCE = Region(240, 135, 45, 31)       # RPM
HEART_RATE = Region(341, 129, 69, 38)    # BPM

# Additional telemetry (v1.0+)
GRADIENT = Region(1695, 71, 50, 50)      # % (special font)
DISTANCE_TO_FINISH = Region(1143, 138, 50, 27)  # km

# Advanced features
LEADERBOARD = Region(1500, 200, 420, 600)  # Multi-rider data (Python only)
POWERUP_NAME = Region(444, 211, 225, 48) # When active
LEADERBOARD_AREA = Region(1500, 200, 420, 600)
```

## Key Implementation Details

### 1. Character Constraints (Critical for Accuracy)
The breakthrough that achieved 100% accuracy was constraining OCR by character type:

```python
# Python implementation
constraints = {
    'number': r'[^0-9]',           # Speed, power, altitude, etc.
    'decimal': r'[^0-9.]',         # Distance, gradient
    'time': r'(\d{1,2}:\d{2})',   # Race time
    'letters': r'[^A-Za-z]'        # Powerup names
}

# Rust implementation
ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")
```

This eliminates OCR errors like O/0 and I/1 confusion.

### 2. Preprocessing by Element Type

Different UI elements require different preprocessing:

```python
# Top bar (white on dark) - Python
def preprocess_top_bar(roi):
    gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
    _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
    return cv2.resize(binary, None, fx=3, fy=3)

# Gradient (stylized font) - Python  
def preprocess_gradient(roi):
    gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
    inverted = cv2.bitwise_not(gray)  # Invert for contrast
    _, binary = cv2.threshold(inverted, 100, 255, cv2.THRESH_BINARY)
    return cv2.resize(binary, None, fx=4, fy=4)

# Gradient (Rust - optimized approach)
let binary = threshold(&gray, 150);  # No inversion needed, lower threshold
let scaled = image::imageops::resize(&binary, width * 4, height * 4, 
                                     image::imageops::FilterType::CatmullRom);
ocr.set_variable(Variable::TesseditPagesegMode, "7");  # Single text line

# Standard fields (Rust)
let binary = threshold(&gray, if field == "distance_to_finish" { 150 } else { 200 });
let scaled = image::imageops::resize(&binary, width * 3, height * 3, 
                                     image::imageops::FilterType::CatmullRom);
```

### 3. Dynamic UI Elements

Some elements move or appear conditionally:

#### Distance to Finish
- Normal: Region(1143, 138, 50, 27)
- May shift ±20 pixels during climbs
- Can be replaced by segment data
- Solution: Check multiple regions or use pattern matching

#### Powerups
- Only visible when active
- Timer shown as circular progress
- Different colors per powerup type

#### Gradient Box
- Only appears on hills (>3% or <-3%)
- Stylized font requires special processing

## Performance Comparison

### Python (PaddleOCR)
- **Accuracy**: 100% on all fields
- **Speed**: ~5.15 seconds per image
- **Strengths**: Handles all UI elements including leaderboard
- **Setup**: Requires PaddleOCR installation

### Rust (Tesseract) - v1.0
- **Accuracy**: 100% on all core fields (9/10 fields)
- **Speed**: ~1.08 seconds per image (4.8x faster than Python)
- **Features**: Speed, distance, altitude, time, power, cadence, HR, gradient, distance-to-finish
- **Limitations**: Leaderboard extraction not implemented (Python-only feature)
- **Setup**: Requires Tesseract library

## Common Issues & Solutions

### 1. Resolution Differences
**Problem**: Coordinates optimized for 1920x1080
**Current Solution**: Scale coordinates proportionally:
```python
scale_x = actual_width / 1920
scale_y = actual_height / 1080
```

**Future Enhancement**: AI-powered region auto-detection during race join:
```python
# Perfect timing: pre-race join window before start
print("Joined race - calibrating UI regions...")
regions = detect_zwift_ui_elements(pre_race_frame)
print("Calibration complete - ready for race start!")
# Cache for future races: "zwift_v1.60_1920x1080_regions.json"
save_region_cache(regions, resolution, zwift_version)
```

**Implementation Approaches**:
- **Template matching**: Use reference images of UI elements (speed box, power meter)
- **Feature detection**: SIFT/ORB to find distinctive UI features
- **Contour analysis**: Detect rectangular UI panels by shape
- **Text detection**: Use OCR engines to locate text regions, then calibrate boundaries

**Race Join Workflow**:
1. **Join race** (before start deadline)
2. **Auto-calibrate** UI regions during pre-race period
3. **Cache regions** for this resolution/version combo
4. **Race starts** - immediate high-accuracy extraction
5. **Future races** - skip calibration, use cached regions

### 2. OCR Engine Installation

**PaddleOCR (Python)**:
```bash
uv add paddlepaddle paddleocr
# Ignore ccache warnings
```

**Tesseract (Rust)**:
```bash
# Debian/Ubuntu
sudo apt-get install tesseract-ocr libtesseract-dev

# macOS
brew install tesseract
```

### 3. Debug Visualization
Always start debugging with visual output:
```bash
mask debug screenshot.jpg  # Python
# Creates annotated image showing extraction regions
```

## Zwift Physics Notes

### Rider Positions and Aerodynamic Effects

#### Detected Pose Types (`rider_pose_detector.py`)
- **normal_normal**: Standard upright position (NORMAL DRAG)
- **normal_tuck**: Tucked position (HIGH DRAG - counterintuitive!)
- **climbing_seated**: Seated climbing (NORMAL DRAG) 
- **climbing_standing**: Out of saddle climbing (HIGH DRAG)

#### Automatic Position Changes (Visual Only)
- **Hoods/Drops**: Automatic based on speed (≥33km/h) and drafting
- **Standing**: Based on cadence (31-72 RPM) on climbs
- **NO aerodynamic effect** - purely visual changes

#### Supertuck (Actually Affects Speed)
- Only position that changes aerodynamics
- Activates on steep descents when not pedaling
- ~25% drag reduction
- Cannot be used on TT/MTB bikes

#### Pose Detection Features
- **Avatar region**: (860, 400, 200, 300) for 1920x1080
- **Feature extraction**: Aspect ratio, torso angle, head position, center of mass, symmetry
- **Classification**: Rule-based system with confidence thresholds
- **Drag implications**: Important for performance analysis - tuck position increases drag in Zwift

## Data Storage Schema

```sql
CREATE TABLE telemetry (
    id INTEGER PRIMARY KEY,
    timestamp REAL NOT NULL,
    frame_number INTEGER NOT NULL,
    speed REAL,
    distance REAL,
    altitude INTEGER,
    race_time INTEGER,
    power INTEGER,
    cadence INTEGER,
    heart_rate INTEGER,
    gradient REAL,
    distance_to_finish REAL,
    powerup_name TEXT,
    rider_pose TEXT,  -- normal_normal, normal_tuck (high drag), climbing_seated, climbing_standing (high drag)
);
```

## Integration with Zwift Race Finder

The OCR tools can validate race duration estimates:
1. Extract actual race times from recordings
2. Compare with predicted durations
3. Adjust estimation algorithms based on real data

## Data Source Considerations

### Sensor vs OCR Data (Verified from Real Racing Data)
- **Direct from sensors** (ANT+/Bluetooth): Power, cadence, heart rate at **1Hz (1 second intervals)**
  - ✅ Strava analysis: 97-minute race = 5,831 data points
  - ✅ 100% data completeness for power, cadence, HR
  - ✅ Perfect for high-frequency performance metrics
- **OCR-only data**: Position, leaderboard, gradient, distance-to-finish, rider pose, powerup status
  - ⚡ Rust OCR: 1.08 seconds per extraction (nearly matches 1Hz sensor rate)
  - 🎥 Optimal for 1fps video analysis or real-time streaming
- **Optimal approach**: Real-time sensor feeds + periodic OCR for complete telemetry
- **Performance match**: OCR speed (1.08s) ≈ sensor interval (1s) - perfect sync potential

### OCR Use Cases
- **Post-race analysis**: Extract position data from race recordings
- **Broadcast overlays**: Real-time leaderboard for streaming
- **Data validation**: Cross-check sensor accuracy against UI display
- **Historical analysis**: Process old recordings where sensor data unavailable

## Future Improvements

See [README.md](README.md) for current enhancement roadmap.