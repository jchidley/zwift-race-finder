# OCR Calibration Guide for Zwift Race Finder

This guide explains how to calibrate the OCR system for different screen resolutions and Zwift versions.

## Why Calibration is Needed

Zwift's UI elements appear at different positions depending on:
- Screen resolution (1920x1080, 2560x1440, 3840x2160, etc.)
- Zwift app version (UI updates may shift elements)
- UI scaling settings

Once calibrated for a specific resolution and version, the configuration can be shared with the community.

## Key Discovery: Field-Specific Preprocessing

The OCR system uses **targeted region extraction** with **field-specific preprocessing**:

### Standard Fields (bright white text)
- **Threshold**: 200-230
- **Scale**: 3x
- **Examples**: Speed, Distance, Power, Race Time

### Special Cases
- **Gradient**: Located in mini-map area (top-right), stylized font requires image inversion + lower threshold (100) + 4x scale
- **Distance to Finish**: Dimmer text needs lower threshold (150)
- **Altitude**: Works best with higher threshold (230)
- **Route Name**: Variable width, may need wider region
- **Lap Counter**: Small text, ensure adequate padding

### Complex Regions
- **Leaderboard**: Uses CLAHE enhancement instead of binary threshold
- **Rider Pose**: Uses edge detection on avatar region

## Manual Calibration Process

### Step 1: Capture a Reference Screenshot

1. Start Zwift and join any ride/race
2. Take a screenshot when all UI elements are visible:
   - Power, cadence, heart rate (left panel)
   - Speed, distance, altitude, time (top bar)
   - Gradient (top right, above leaderboard position indicator)
   - Distance to finish (below top bar)
   - Leaderboard (right side)
3. Note the exact values shown for verification

### Step 2: Use Debug Tool for Fine-Tuning

The `debug_ocr` tool helps find optimal settings for each field:

```bash
cd /path/to/zwift-race-finder
cargo run --release --features ocr --bin debug_ocr -- /path/to/screenshot.png
```

This shows different preprocessing options for each region and helps identify:
- Optimal threshold values
- Required scale factors
- Best segmentation modes

### Step 3: Create Configuration

Create a JSON configuration file with your regions:

```json
{
  "version": "1.0.0",
  "resolution": "1920x1080",
  "zwift_version": "1.67.0",
  "created": "2025-01-12T10:00:00Z",
  "regions": {
    "speed": {
      "x": 640,
      "y": 30,
      "width": 190,
      "height": 85,
      "note": "34KMHA - includes units"
    },
    "power": {
      "x": 118,
      "y": 34,
      "width": 202,
      "height": 95,
      "note": "195w - current power"
    },
    "gradient": {
      "x": 1680,
      "y": 120,
      "width": 120,
      "height": 60,
      "note": "1% - in mini-map area (top-right corner)"
    },
    // ... more regions
  },
  "notes": "Add padding (10-20px) to ensure complete text capture"
}
```

## Automated Calibration Tools

### Method 1: AI Vision Analysis (Groq)

1. Sign up at https://console.groq.com
2. Get your API key
3. Use the calibration script:

```bash
cd tools/ocr
export GROQ_API_KEY="your-api-key"
uv run python calibrate_with_vision.py /path/to/screenshot.png --provider groq
```

### Method 2: PaddleOCR Detection

Use PaddleOCR to find all text regions:

```bash
cd tools/ocr
uv run python calibrate_with_paddleocr.py /path/to/screenshot.png
```

### Method 3: Multi-Pass Detection

Tries multiple preprocessing configurations to find optimal settings:

```bash
cd tools/ocr
uv run python calibrate_multipass.py /path/to/screenshot.png --validate
```

The scripts will:
- Analyze the screenshot
- Identify UI elements and their positions
- Test different preprocessing options
- Generate a configuration file
- Save to `ocr-configs/`

## Visual Region Mapper (Manual Fine-Tuning)

For precise manual adjustment:

```bash
cd tools/ocr
uv run python visual_region_mapper.py /path/to/screenshot.png

# Controls:
# - Click and drag to create/adjust regions
# - Press 't' to test current region
# - Press 's' to save configuration
# - Press 'q' to quit
```

## What to Calibrate

### UI-Exclusive Data Only
We focus on data NOT available from sensors or Zwift API:

1. **speed** - Top bar, current speed
2. **power** - Left panel, current watts  
3. **distance** - Top bar, distance traveled
4. **altitude** - Top bar, elevation
5. **race_time** - Top bar, elapsed time
6. **distance_to_finish** - Below top bar (race mode)
7. **gradient** - Mini-map area (top-right corner)
8. **route_name** - Current route/segment name
9. **lap_counter** - Lap number and distance
10. **lead_in_status** - "Lead-in" indicator
11. **leaderboard** - Right side panel positions

### Skip These (Available from Sensors/API)
- Heart Rate - Available in FIT files
- Cadence - Available in FIT files  
- Average Power - Calculated from FIT data
- Total Energy (kJ) - Calculated from power data

### Known Locations (1920x1080)
Based on testing:
- **Gradient**: In the mini-map area (top-right corner of screen)
- **Power**: Large number in top-left of left panel
- **Distance to Finish**: Below the top bar in race mode

## Common Issues and Solutions

### Region Detection Problems

1. **Gradient not detected**: 
   - Located in mini-map area (top-right corner)
   - Uses stylized font requiring image inversion
   - Threshold 100, scale 4x

2. **Partial number detection** (e.g., "9" instead of "129"):
   - Expand region width/height
   - Add more padding (15-20px)
   - Check threshold settings

3. **Wrong text extracted**:
   - Region overlaps with other UI elements
   - Reduce region size or reposition

### Testing Your Configuration

```bash
# Test with debug tool
cargo run --release --features ocr --bin debug_ocr -- /path/to/screenshot.png

# Run full extraction
cargo test --release --features ocr test_ocr_extraction
```

### Advanced: API-Based Validation

For ultimate accuracy, OCR results can be validated against real-time telemetry:

- **UDP Packet Monitoring**: Zwift broadcasts telemetry on port 3022 (power, speed, distance, gradient)
- **Ground Truth Comparison**: Compare OCR extracted values with actual telemetry
- **Confidence Scoring**: Rate OCR accuracy based on telemetry agreement
- **Auto-Calibration**: Automatically adjust regions based on consistent discrepancies

This enables self-validating OCR that improves over time.

## Contributing Your Configuration

Once you've created and tested a configuration:

1. Name it: `{width}x{height}_v{zwift_version}.json`
   - Example: `1920x1080_v1.67.0.json`

2. Verify all fields extract correctly

3. Submit a pull request with:
   - Your configuration file in `ocr-configs/`
   - Test results showing successful extraction
   - The Zwift version and your system details

### Example Test Output
```
Speed: 34 km/h ✓
Power: 195 W ✓
Distance: 0.3 km ✓
Altitude: 1 m ✓
Race Time: 00:02:15 ✓
Gradient: 1% ✓
Distance to Finish: 52.5 km ✓
Route: Watopia Flat Route ✓
Lap: 1 / 2.5km ✓
```

## Advanced: Understanding the OCR Pipeline

1. **Region Extraction**: Cut out specific UI area from screenshot
2. **Preprocessing**: Apply field-specific threshold, scaling, inversion
3. **OCR**: Tesseract extracts text with character whitelist
4. **Cleaning**: Remove non-numeric characters based on field type
5. **Validation**: Check value ranges for sanity

### Per-Field Processing Details

| Field | Threshold | Scale | Invert | Special |
|-------|-----------|-------|---------|---------|
| Speed | 200 | 3x | No | Numbers only |
| Power | 200 | 3x | No | Numbers only |
| Distance | 200 | 3x | No | Numbers only |
| Altitude | 230 | 3x | No | Higher threshold |
| Race Time | 200 | 3x | No | Time format |
| Gradient | 100 | 4x | Yes | Mini-map area, stylized font |
| Distance to Finish | 150 | 3x | No | Dimmer text |
| Route Name | 200 | 2x | No | Variable width |
| Lap Counter | 200 | 3x | No | Small numbers |
| Leaderboard | - | 2x | No | CLAHE enhancement |

## Future: Real-time Validation

The OCR system can be enhanced with real-time validation:
1. **UDP monitoring** provides ground truth telemetry
2. **Confidence scoring** rates OCR accuracy
3. **Auto-calibration** adjusts regions automatically
4. **Community improvement** through validated configs

## Need Help?

- Open an issue with your resolution/version
- Include screenshot and what values should be detected
- Share partial configs - others can help complete them

Remember: Your contribution helps every Zwift racer with your setup!