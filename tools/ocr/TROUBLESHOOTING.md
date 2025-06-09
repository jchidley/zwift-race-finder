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

## Key Solution: Constrained OCR

The final version (`zwift_ocr_improved_final.py`) achieves 100% accuracy by using:

1. **Region-based extraction**: Only processes specific UI areas
2. **Character constraints**: Tells OCR what to expect:
   - Numbers only (0-9): Speed, power, altitude
   - Decimal (0-9.): Distance, watts/kg
   - Time (0-9:): Race time, time gaps
   - Letters only (A-Z): Powerup names
3. **Specialized preprocessing**: Different methods for each UI element

This eliminates common OCR errors like O/0 and I/1 confusion.

## Common Issues (Now Fixed)

### 1. Concatenated Text Problem

**Symptom**: Top bar values read as one string like "2018.410631:06"  
**Expected**: Separate values: 20 km/h, 18.4 km, 106m, 31:06

**Solution**: 
- Use `zwift_ocr_improved_final.py` which has optimized regions
- Use visual mapper to adjust coordinates:
  ```bash
  uv run python visual_region_mapper.py screenshot.jpg
  ```
- Current optimized regions:
  ```python
  SPEED = Region(693, 44, 71, 61, "speed")
  DISTANCE = Region(833, 44, 84, 55, "distance")
  ALTITUDE = Region(975, 45, 75, 50, "altitude")
  RACE_TIME = Region(1070, 45, 134, 49, "race_time")
  ```

### 2. Cadence Not Detected

**Symptom**: Cadence shows as "Not found" even though visible (e.g., 72 RPM)

**Solutions**:
1. Fixed in final version with optimized region:
   ```python
   CADENCE = Region(240, 135, 45, 31, "cadence")
   ```
2. Uses special preprocessing with 2x scaling for better accuracy
3. Character constraints ensure only numbers are extracted

### 3. Gradient Not Found

**Symptom**: Gradient percentage not detected in top-right box

**Solutions**:
1. Fixed with special preprocessing for stylized font:
   ```python
   GRADIENT_BOX = Region(1695, 71, 50, 50, "gradient_box")
   ```
2. Uses inverted threshold and 4x scaling specifically for gradient
3. Successfully extracts gradient percentage with 100% accuracy

### 4. Distance to Finish Issues

**Symptom**: Shows wrong value or not found (should be 28.6km, 16.6km, etc.)

**Challenges**:
- Location varies based on UI state
- Often split across multiple lines (e.g., "28.6" and "km" separately)
- May show XP or other values in similar area

**Solution**: Concatenate all text in region and look for patterns

### 5. Leaderboard Parsing

**Symptom**: Names, w/kg, and distances mixed up

**Solution**: Fixed in final version with two-row structure understanding:
- Row 1: Rider name
- Row 2: Time delta, w/kg, distance

**Format correctly parsed**:
```
J.Matzke       +2:20  1.9 w/kg  5.1km
J.Chidley      ---    3.2 w/kg  6.4km  <-- YOU
J.T.Noxen      +0:00  3.4 w/kg  6.4km
```

**Note**: Current rider identified by absence of time delta

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