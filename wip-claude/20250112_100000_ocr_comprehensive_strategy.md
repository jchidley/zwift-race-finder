# Comprehensive OCR Strategy for Zwift Race Finder (Final)

## Executive Summary

After extensive testing, we've discovered that the original approach was correct: **targeted region extraction with field-specific preprocessing**. The key insights:

1. **No magic bullet**: Full-image OCR detection produces too many false positives
2. **Field-specific preprocessing is essential**: Each UI element needs different threshold, scale, and processing
3. **UI stability enables community configs**: Positions are fixed per resolution/version
4. **Manual calibration produces best results**: But can be assisted by automated tools
5. **Focus on UI-exclusive data**: Skip sensor data (HR, cadence) available from FIT files/API

## Key Technical Discoveries

### 1. Why Full-Image OCR Fails
- PaddleOCR detects 70+ text regions in a typical Zwift screenshot
- Many are irrelevant (UI labels, route names, chat messages)
- Classification is error-prone without context
- Performance overhead of processing entire image

### 2. Field-Specific Requirements
Through debug testing, we found each field needs specific preprocessing:

| Field | Threshold | Scale | Special Processing | Why |
|-------|-----------|-------|-------------------|-----|
| Speed | 200 | 3x | Standard | Bright white text |
| Power | 200 | 3x | Standard | Bright white text |
| Distance | 200 | 3x | Standard | Bright white text |
| Altitude | 230 | 3x | Higher threshold | Faint text |
| Race Time | 200 | 3x | Standard | Bright white text |
| Gradient | 100 | 4x | Image inversion | Stylized font in mini-map |
| Distance to Finish | 150 | 3x | Lower threshold | Dimmer text |
| Route/Segment Name | 200 | 2x | Standard | Variable width |
| Lap Counter | 200 | 3x | Standard | Small numbers |
| Leaderboard | - | 2x | CLAHE enhancement | Complex layout |

**Note**: We skip Heart Rate, Cadence, Average Power, and Total Energy as these are available from FIT files and Zwift API.

### 3. UI-Exclusive Data Focus
We only extract data NOT available from other sources:

**Extract These (UI-Only)**:
- Speed, Power, Distance, Altitude, Race Time - Core telemetry display
- Distance to Finish - Race-specific, not in FIT files
- Gradient - Located in mini-map area (top-right), not in sensors
- Route/Segment Name - Context not in telemetry
- Lap Counter & Distance - Race progress tracking
- Lead-in Status - Race state indicator
- Leaderboard - Live positions

**Skip These (Available Elsewhere)**:
- Heart Rate - In FIT files from HRM
- Cadence - In FIT files from sensors
- Average Power - Calculated from power data
- Total Energy (kJ) - Calculated value

### 4. The Working Pipeline
```
1. Extract targeted region (e.g., power at x:118, y:34)
2. Apply field-specific preprocessing
3. Run OCR with appropriate character whitelist
4. Clean result based on field type
5. Validate against expected ranges
```

## Current Status

### What Works
- **Rust + Tesseract**: 0.88s per frame, <100MB memory
- **Manual configs**: 100% accuracy when properly calibrated
- **Debug tool**: Shows exactly which preprocessing works for each field
- **Community approach**: Configs can be shared
- **API calibration path**: UDP packet monitoring provides real-time ground truth

### What We Learned
- Original PaddleOCR also uses targeted regions, not full-image scan
- Each field's preprocessing is empirically determined
- "One size fits all" OCR doesn't work for Zwift's varied UI
- Manual calibration with tool assistance is most reliable

## Revised Architecture with API Calibration

```
┌─────────────────────────────────────────────────────────┐
│         Calibration Process (Per Resolution)             │
├─────────────────────────────────────────────────────────┤
│ 1. Manual Region Definition                             │
│    - Use visual_region_mapper.py                        │
│    - Define bounding boxes for each UI element          │
│                                                         │
│ 2. Debug Tool Optimization                              │
│    - Run debug_ocr to test each region                 │
│    - Find optimal threshold/scale per field             │
│    - Document in config notes                           │
│                                                         │
│ 3. API-Based Validation (NEW)                          │
│    - UDP packet monitoring (port 3022) for ground truth│
│    - Real-time telemetry: power, speed, distance, etc. │
│    - Auto-calibration based on telemetry comparison     │
│    - Confidence scoring for OCR accuracy                │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│         Community Config Repository                      │
│  ocr-configs/1920x1080_v1.67.0.json                    │
│  - Manually tuned regions                               │
│  - Field-specific notes                                 │
│  - Tested by contributor                                │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│            Runtime (Rust + Tesseract)                    │
│  - Load config for resolution                           │
│  - Apply hardcoded preprocessing per field type         │
│  - Extract each region with its specific settings       │
│  - Compare with UDP telemetry for confidence scoring     │
│  - Fast, accurate, low memory, self-validating          │
└─────────────────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│           API Calibration System (Optional)              │
│  UDP Packet Monitor (Port 3022):                       │
│  - Real-time power, speed, distance, gradient          │
│  - Ground truth for OCR validation                     │
│  Post-race Validation:                                 │
│  - Strava/FIT file comparison for accuracy analysis    │
│  - Historical calibration improvement                   │
└─────────────────────────────────────────────────────────┘
```

## Tools for Contributors

### 1. Visual Region Mapper (Primary Tool)
```bash
cd tools/ocr
uv run python visual_region_mapper.py /path/to/screenshot.png

# Click and drag to define regions
# Test extraction in real-time
# Save configuration when done
```

### 2. Debug OCR (Optimization Tool)
```bash
cargo run --release --features ocr --bin debug_ocr -- /path/to/screenshot.png

# Shows different preprocessing options
# Helps find optimal settings
# Identifies problem regions
```

### 3. Calibration Assistants (Helpers)
```bash
# Get initial regions from PaddleOCR
uv run python calibrate_with_paddleocr.py screenshot.png

# Use AI to identify regions
uv run python calibrate_with_vision.py screenshot.png --provider groq

# Multi-pass detection
uv run python calibrate_multipass.py screenshot.png
```

These tools provide starting points, but manual refinement is usually needed.

## Configuration Format (v1.0.0)

```json
{
  "version": "1.0.0",
  "resolution": "1920x1080",
  "zwift_version": "1.67.0",
  "created": "2025-01-12T17:30:00Z",
  "regions": {
    "speed": {
      "x": 640,
      "y": 30,
      "width": 190,
      "height": 85,
      "note": "Captures full '34KMHA' text"
    },
    "gradient": {
      "x": 1680,
      "y": 220,
      "width": 120,
      "height": 60,
      "note": "In mini-map area (top-right corner)"
    }
    // ... other regions
  },
  "notes": "Gradient requires special preprocessing (invert + threshold 100)"
}
```

## Calibration Best Practices

### 1. Start with Known Values
- Take screenshot with visible values
- Note exact numbers for validation
- Include all UI elements

### 2. Use Debug Tool
- Test each region individually
- Find optimal preprocessing
- Document special cases

### 3. Add Padding
- 10-20px padding catches full text
- Prevents edge cutoff
- Improves reliability

### 4. Test Edge Cases
- Different routes (gradient visibility)
- Race vs free ride (distance to finish)
- Various lighting conditions

## Why This Approach Works

### 1. Empirically Validated
- Each preprocessing setting tested on real data
- No theoretical assumptions
- Based on what actually works

### 2. Community Scalable
- One expert calibration helps everyone
- Configs improve over time
- Version control tracks changes

### 3. Performance Optimized
- Minimal processing per frame
- No wasted computation
- Predictable resource usage

### 4. Maintainable
- Simple JSON configs
- Clear preprocessing rules
- Easy to debug issues

## Common Pitfalls to Avoid

### 1. Over-Engineering
❌ "Let's use AI to detect everything automatically"
✅ Manual calibration with tool assistance

### 2. One-Size-Fits-All
❌ "Same preprocessing for all fields"
✅ Field-specific preprocessing based on testing

### 3. Perfect Automation
❌ "Zero manual effort calibration"
✅ Tools assist but human validates

### 4. Complex Classification
❌ "Smart algorithm to identify any UI layout"
✅ Fixed regions per resolution/version

## Implementation Status

### Completed
- ✅ Rust OCR with config loading
- ✅ Debug tool for testing
- ✅ Visual region mapper
- ✅ Multiple calibration assistants
- ✅ Working 1920x1080 config

### Next Steps
1. Polish calibration tools
2. Document contribution process
3. Create more resolution configs
4. Set up PR templates
5. **Implement UDP packet monitoring for real-time calibration**
6. **Add Strava integration for post-race validation**

## The Truth About OCR

After trying every approach:
- **Automated detection**: Gets you 70% there
- **Manual refinement**: Required for reliability
- **Field-specific tuning**: Essential for accuracy
- **Community configs**: Scalable solution

The original implementation had it right: targeted extraction with empirical preprocessing.

## For Contributors

Your calibrated config helps everyone with your setup. The process:

1. **Use tools** to get started (PaddleOCR, Vision AI)
2. **Refine manually** with visual mapper
3. **Optimize** with debug tool
4. **Test thoroughly** on multiple frames
5. **Submit PR** with config and test results

Remember: Perfect automation is a myth. Good tools + human expertise = reliable OCR.