# Zwift OCR Troubleshooting Guide

This guide covers common issues encountered during OCR extraction and their solutions.

## Quick Diagnostic

```bash
# Run debug visualization first!
mask debug /path/to/screenshot.jpg

# This shows:
# - Where OCR is looking (colored boxes)
# - What was extracted (or "?" if failed)
# - Overall success rate
```

## Common Issues

### 1. Concatenated Text Problem

**Symptom**: Top bar values read as one string like "2018.410631:06"  
**Expected**: Separate values: 20 km/h, 18.4 km, 106m, 31:06

**Solution**: 
- Use `zwift_ocr_improved_v2.py` which splits the top bar into 4 regions
- Adjust region coordinates if needed:
  ```python
  SPEED = Region(700, 35, 100, 50, "speed")
  DISTANCE = Region(820, 35, 100, 50, "distance")
  ALTITUDE = Region(940, 35, 80, 50, "altitude")
  RACE_TIME = Region(1040, 35, 120, 50, "race_time")
  ```

### 2. Cadence Not Detected

**Symptom**: Cadence shows as "Not found" even though visible (e.g., 72 RPM)

**Solutions**:
1. Region too small - fixed in v2:
   ```python
   CADENCE = Region(240, 135, 60, 40, "cadence")  # Wider and taller
   ```
2. Add preprocessing with scaling:
   ```python
   scaled = cv2.resize(binary, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)
   ```

### 3. Gradient Not Found

**Symptom**: Gradient percentage not detected in top-right box

**Solutions**:
1. Box too large - reduced size:
   ```python
   GRADIENT_BOX = Region(1700, 75, 50, 50, "gradient_box")
   ```
2. Only appears during climbs/descents
3. Look for simple pattern: `(\d+)%`

### 4. Distance to Finish Issues

**Symptom**: Shows wrong value or not found (should be 28.6km, 16.6km, etc.)

**Challenges**:
- Location varies based on UI state
- Often split across multiple lines (e.g., "28.6" and "km" separately)
- May show XP or other values in similar area

**Solution**: Concatenate all text in region and look for patterns

### 5. Leaderboard Parsing

**Symptom**: Names, w/kg, and distances mixed up

**Format to expect**:
```
J.Chidley    3.6 w/kg  18.4km  
+0:00  C.J.Y.S    3.0 w/kg  18.4km
```

**Note**: Current rider has no time gap initially

## UI Variations

### Different Resolutions
- Current coordinates optimized for 1920x1080
- Scale coordinates proportionally for other resolutions
- Use debug mode to verify regions

### UI Elements That Move
- **Distance to finish**: Below top bar, but exact position varies
- **Gradient box**: Only visible during climbs/descents
- **Power-ups**: Center-right when active
- **Segment info**: Replaces other elements when in segment

## OCR Engine Issues

### PaddleOCR
```bash
# If PaddleOCR fails to install
uv add paddlepaddle==2.5.2 paddleocr==2.7.0.3

# Warning about ccache is normal - ignore it
```

### EasyOCR
```bash
# Alternative if PaddleOCR has issues
uv add easyocr

# Slower but sometimes more reliable
```

## Debugging Steps

1. **Always start with debug visualization**
   ```bash
   mask debug screenshot.jpg
   ```

2. **Check individual regions**
   ```python
   # Add debug=True to see raw OCR output
   extractor = ZwiftOCRExtractorV2(debug=True)
   ```

3. **Save intermediate images**
   ```python
   cv2.imwrite(f"debug_{region.name}.jpg", roi)
   cv2.imwrite(f"debug_{region.name}_processed.jpg", processed)
   ```

4. **Test with different preprocessing**
   - Adjust threshold (default 180)
   - Try without CLAHE enhancement
   - Experiment with different scaling factors

## Known Limitations

### Cannot Extract
- Rider names in events (too small)
- Detailed power graph data
- Map/route information
- Chat messages

### Accuracy Varies With
- Video compression quality
- UI transparency settings
- Background contrast
- Motion blur during fast movement

## Success Metrics

### Current Performance (v2)
- **Power/Fitness**: ~100% accuracy
- **Basic Navigation**: ~60% accuracy  
- **Overall**: ~57% of fields extracted

### Target Performance
- All critical fields >90% accuracy
- Non-critical fields >70% accuracy

## Getting Help

1. Run debug mode and share the output image
2. Note which fields are failing
3. Include Zwift UI settings (resolution, UI scale)
4. Specify OCR engine being used

## Version History

### v1 (Initial)
- Basic OCR with single top bar region
- ~40% success rate

### v2 (Current)
- Split regions for better accuracy
- Fixed cadence, gradient detection
- ~57% success rate

### Future Improvements
- Auto-calibration for different resolutions
- Machine learning for better text recognition
- Handle UI animations and transitions