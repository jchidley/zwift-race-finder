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

# Other elements
GRADIENT_BOX = Region(1695, 71, 50, 50)  # %
DISTANCE_TO_FINISH = Region(1143, 138, 50, 27)  # km
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

# Rust equivalent
let binary = threshold(&gray, 200);
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
- **Speed**: ~6-20 seconds per image
- **Strengths**: Handles all UI elements including leaderboard
- **Setup**: Requires PaddleOCR installation

### Rust (Tesseract)
- **Accuracy**: 100% on core fields (7/7)
- **Speed**: ~1-3 seconds per image (5-6x faster)
- **Limitations**: No gradient/leaderboard extraction yet
- **Setup**: Requires Tesseract library

## Common Issues & Solutions

### 1. Resolution Differences
**Problem**: Coordinates optimized for 1920x1080
**Solution**: Scale coordinates proportionally:
```python
scale_x = actual_width / 1920
scale_y = actual_height / 1080
```

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

### Rider Positions (Visual Only)
- **Hoods/Drops**: Automatic based on speed (≥33km/h) and drafting
- **Standing**: Based on cadence (31-72 RPM) on climbs
- **NO aerodynamic effect** - purely visual

### Supertuck (Actually Affects Speed)
- Only position that changes aerodynamics
- Activates on steep descents when not pedaling
- ~25% drag reduction
- Cannot be used on TT/MTB bikes

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
    rider_pose TEXT
);
```

## Integration with Zwift Race Finder

The OCR tools can validate race duration estimates:
1. Extract actual race times from recordings
2. Compare with predicted durations
3. Adjust estimation algorithms based on real data

## Future Improvements

1. **Auto-calibration** for different resolutions
2. **Machine learning** for Zwift-specific fonts
3. **Real-time streaming** support
4. **Complete Rust implementation** including all fields