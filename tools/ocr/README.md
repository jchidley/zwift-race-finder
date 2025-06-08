# Zwift OCR Telemetry Extraction

This tool extracts live telemetry data from Zwift screenshots and video recordings using Optical Character Recognition (OCR), similar to how the [AeroTelemProc_VidData](https://github.com/mateosolinho/AeroTelemProc_VidData) project extracts SpaceX telemetry.

## Features

- Extract telemetry data from Zwift UI elements:
  - **Top Bar** (split into 4 regions for accuracy):
    - Speed (km/h) - Working ✓
    - Distance traveled (km) - Working ✓
    - Current altitude (m) - In development
    - Race time (mm:ss) - In development
  - **Top Left Power Panel**:
    - Current power (W) - Working ✓
    - Cadence (RPM) - Working ✓
    - Heart rate (BPM) - Working ✓
    - Average power (W) - Working ✓
    - Energy expended (kJ) - Working ✓
  - **Gradient** (top right box during climbs) - Working ✓
  - **Distance to finish** (below top bar) - In development
  - **Power-ups**: Active power-up name and remaining duration
    - Detects: Featherweight, Aero Boost, Draft Boost, etc.
    - Circular timer analysis (starts at 3 o'clock position)
  - **Leaderboard**: Rider names, watts/kg, distance (km) - Partial ✓
  - **Rider Pose Detection**: Avatar position analysis
    - Note: Only supertuck affects speed (-25% drag)
    - Other positions are visual only in Zwift
- Debug visualization mode showing extraction regions
- Support for both PaddleOCR and EasyOCR engines
- Process video files or live streams
- Multiple output formats: SQLite, CSV, JSON
- Real-time preview with extraction overlay

## Installation

For detailed setup instructions, see [SETUP_GUIDE.md](SETUP_GUIDE.md).

### Quick Install

```bash
# Install mask (if not already installed)
cargo install mask

# Navigate to OCR tools directory
cd tools/ocr/

# Install dependencies with mask
mask setup

# Or manually with uv
uv sync
```

## Quick Start

### Using mask (recommended)

```bash
# Show available tasks
mask --help

# Run tests
mask test

# Process a video
mask video recording.mp4 --skip-frames 60

# Lint and format code
mask lint
mask format
```

### Extract from Screenshots

```bash
# Test OCR on sample screenshots
uv run python zwift_ocr_improved.py

# Test enhanced extraction with gradient and leaderboard
uv run python test_enhanced_extraction.py

# Compare OCR engines
uv run python zwift_ocr_prototype.py

# Or use the wrapper script
./zwift_ocr.sh screenshot docs/screenshots/normal_1_01_16_02_21.jpg
```

### Process Video Files

```bash
# Basic video processing (1 sample per second)
uv run python zwift_video_processor.py path/to/zwift_recording.mp4

# Process every frame for maximum detail
uv run python zwift_video_processor.py recording.mp4 --skip-frames 1

# Process without preview window
uv run python zwift_video_processor.py recording.mp4 --no-preview

# Analyze extracted data after processing
uv run python zwift_video_processor.py recording.mp4 --analyze
```

### Process Live Streams

```bash
# From webcam (for screen capture)
uv run python zwift_video_processor.py 0

# From RTMP stream
uv run python zwift_video_processor.py rtmp://stream.url/live

# From HTTP stream
uv run python zwift_video_processor.py http://stream.url/live.m3u8
```

## Architecture

### OCR Extraction Pipeline

1. **Frame Capture**: Video frames or screenshots are loaded
2. **Region Extraction**: Specific UI regions are extracted based on coordinates
3. **Preprocessing**: Images are enhanced for OCR:
   - Convert to grayscale
   - Apply contrast enhancement (CLAHE)
   - Threshold to isolate white text
   - Upscale for better accuracy
4. **Text Recognition**: OCR engine extracts text
5. **Pattern Matching**: Zwift-specific patterns parse values
6. **Storage**: Data saved to multiple formats

### UI Region Mapping

The system uses precise coordinates for Zwift UI elements at 1920x1080 resolution:

```python
# Example regions
SPEED = Region(520, 35, 90, 50, "speed")
POWER = Region(200, 40, 90, 50, "power")
HEART_RATE = Region(260, 105, 60, 35, "heart_rate")
```

### Data Storage

Extracted telemetry is stored in:
- **SQLite database**: For queries and analysis
- **CSV file**: For spreadsheet import
- **JSON file**: For programmatic access

## Current Extraction Accuracy

Based on testing with Zwift screenshots (as of latest session):

### Successfully Extracting (✓)
- **Power**: 277W (100% accuracy)
- **Heart Rate**: 169 BPM (100% accuracy)
- **Cadence**: 72 RPM (fixed in latest version)
- **Average Power**: 217W (100% accuracy)
- **Energy**: 400 kJ (100% accuracy)
- **Speed**: 20 km/h (works when properly positioned)
- **Distance**: 18.4 km (works when properly positioned)
- **Gradient**: 5% (fixed with smaller detection box)
- **Leaderboard**: Names extracted (J.Chidley, C.J.Y.S)

### Still In Development (✗)
- **Altitude**: Concatenated with other values
- **Race Time**: Concatenated with other values
- **Distance to Finish**: Region identified, OCR needs refinement
- **Power-ups**: Detection logic implemented, needs testing
- **Segment Gradient**: Region defined, needs active segment

### Overall Success Rate
- ~57% of fields successfully extracted
- Power/fitness data: 100% success
- Navigation/race data: Partial success

## OCR Engine Comparison

| Engine | Pros | Cons | Accuracy |
|--------|------|------|----------|
| PaddleOCR | Fast, good with numbers | Larger install | ~85% |
| EasyOCR | Easy setup, good accuracy | Slower | ~80% |

## Output Data Format

### Telemetry Frame
```json
{
  "timestamp": 11.5,
  "frame_number": 345,
  "speed": 34.0,
  "distance": 6.4,
  "altitude": 28,
  "race_time": 667,
  "power": 268,
  "cadence": 72,
  "heart_rate": 160,
  "avg_power": 222,
  "energy": 142,
  "gradient": 5.0,
  "distance_to_finish": 28.6,
  "powerup_name": "Featherweight",
  "powerup_remaining": 15.0,
  "rider_pose": "climbing_seated"
}
```

### Rider Leaderboard Entry
```json
{
  "name": "J.Matske",
  "watts_per_kg": 3.1,
  "distance_km": 6.3,
  "is_current_rider": false
}
```

## Integration with Zwift Race Finder

The extracted telemetry can be used to:
- Validate duration estimates against actual ride data
- Track performance metrics during races
- Build personalized prediction models
- Analyze pacing strategies

```bash
# Import telemetry into race finder database
sqlite3 ~/.local/share/zwift-race-finder/races.db < import_telemetry.sql
```

## Tips for Best Results

1. **Video Quality**: Higher resolution = better OCR accuracy
2. **Stable UI**: Avoid UI animations during capture
3. **Frame Rate**: 1 sample/second is usually sufficient
4. **Preprocessing**: Adjust thresholds for your display settings

## Troubleshooting

For comprehensive troubleshooting, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

### Quick Fixes

#### Use Debug Mode First
```bash
mask debug screenshot.jpg
```
This shows exactly where OCR is looking and what's being extracted.

#### Common Issues
- **Concatenated text**: Top bar values merged → Use v2 extractor
- **Cadence not reading**: Region too small → Fixed in v2
- **Gradient missing**: Box too large → Reduced size in v2
- **Poor accuracy**: Check image quality (1920x1080 recommended)

### Installation Issues
- See [SETUP_GUIDE.md](SETUP_GUIDE.md) for setup help
- Ensure mask is installed: `cargo install mask`
- Check dependencies: `mask setup`

## Future Enhancements

- [ ] Automatic UI scale detection
- [ ] Support for different resolutions
- [ ] Real-time data streaming to external apps
- [ ] Power curve analysis
- [ ] Segment effort tracking
- [ ] Integration with Strava/TrainingPeaks