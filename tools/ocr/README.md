# Zwift OCR Telemetry Extraction

This tool extracts live telemetry data from Zwift screenshots and video recordings using Optical Character Recognition (OCR), similar to how the [AeroTelemProc_VidData](https://github.com/mateosolinho/AeroTelemProc_VidData) project extracts SpaceX telemetry.

## Features

- Extract telemetry data from Zwift UI elements:
  - **Top Middle HUD**: Speed (km/h), distance traveled (km), current altitude (m), race time (mm:ss)
  - **Top Left Panel**: Current power (W), cadence (RPM), heart rate (BPM), average power (W), energy expended (kJ)
  - **Gradient percentage** (visible during climbs in top right)
  - **Distance to finish** (km remaining in race/route)
  - **Power-ups**: Active power-up name and remaining duration
    - Detects: Featherweight, Aero Boost, Draft Boost, Lightweight, etc.
    - Analyzes circular timer (starts/ends at 3 o'clock, decreases anti-clockwise)
  - **Rider Pose Detection**: Analyzes avatar position (mostly visual only!)
    - Seated Hoods: Normal position when drafting or <33 km/h
    - Seated Drops: Hands on drops when ≥33 km/h and not drafting
    - Standing: Out of saddle (31-72 RPM on climbs ≥3%)
    - Supertuck: Descending position (**-25% drag** - only position affecting speed!)
    - Note: Regular positions are visual only in Zwift - no speed impact!
  - XP progress, route progress bars
  - **Enhanced rider leaderboard**: name, watts/kg, distance traveled (km)
    - Automatically detects current rider (no time gap)
    - Parses other riders with position gaps
- Support for both PaddleOCR and EasyOCR engines
- Process video files or live streams
- Multiple output formats: SQLite, CSV, JSON
- Real-time preview with extraction overlay
- Benchmarking tools to compare OCR engines

## Installation

```bash
# Navigate to OCR tools directory
cd tools/ocr/

# Install dependencies with uv
uv sync

# Or install specific OCR engine:
uv add paddlepaddle paddleocr  # For PaddleOCR (recommended)
uv add easyocr                  # For EasyOCR
```

## Quick Start

### Extract from Screenshots

```bash
# Test OCR on sample screenshots
uv run python zwift_ocr_improved.py

# Test enhanced extraction with gradient and leaderboard
uv run python test_enhanced_extraction.py

# Compare OCR engines
uv run python zwift_ocr_prototype.py
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

## OCR Engine Comparison

Based on testing with Zwift screenshots:

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

### OCR Not Detecting Values
- Check if UI scale in Zwift matches expected coordinates
- Try adjusting preprocessing threshold values
- Ensure video quality is sufficient (720p minimum)

### Performance Issues
- Increase `skip_frames` to process fewer frames
- Use PaddleOCR for better performance
- Disable preview window with `--no-preview`

## Future Enhancements

- [ ] Automatic UI scale detection
- [ ] Support for different resolutions
- [ ] Real-time data streaming to external apps
- [ ] Power curve analysis
- [ ] Segment effort tracking
- [ ] Integration with Strava/TrainingPeaks