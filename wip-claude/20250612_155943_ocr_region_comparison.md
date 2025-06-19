# OCR Region Comparison: Hardcoded vs Generated Config

## Summary of Significant Differences

### Critical Differences That Could Explain OCR Failures

#### 1. **Speed Region** - MAJOR DIFFERENCE
- **Hardcoded**: (693, 44, 71, 61) 
- **Generated**: (550, 50, 100, 40)
- **Difference**: X-offset differs by 143 pixels! This is a huge discrepancy.
- **Impact**: OCR would be reading the wrong part of the screen entirely.

#### 2. **Distance Region** - MAJOR DIFFERENCE  
- **Hardcoded**: (833, 44, 84, 55)
- **Generated**: (630, 50, 100, 40)
- **Difference**: X-offset differs by 203 pixels!
- **Impact**: OCR would miss the distance value completely.

#### 3. **Altitude Region** - MAJOR DIFFERENCE
- **Hardcoded**: (975, 45, 75, 50)
- **Generated**: (710, 50, 100, 40)
- **Difference**: X-offset differs by 265 pixels!
- **Impact**: Would be reading wrong screen area.

#### 4. **Race Time Region** - MAJOR DIFFERENCE
- **Hardcoded**: (1070, 45, 134, 49)
- **Generated**: (790, 50, 100, 40)
- **Difference**: X-offset differs by 280 pixels!
- **Impact**: Would completely miss the race time display.

#### 5. **Power Region** - MAJOR DIFFERENCE
- **Hardcoded**: (268, 49, 117, 61)
- **Generated**: (50, 100, 100, 40)
- **Difference**: Both X (218 pixel diff) and Y (51 pixel diff) are way off.
- **Impact**: Would be reading wrong screen area.

#### 6. **Gradient Region** - MASSIVE DIFFERENCE
- **Hardcoded**: (1695, 71, 50, 50)
- **Generated**: (960, 50, 50, 20)
- **Difference**: X-offset differs by 735 pixels! This places it completely off-screen for 1920x1080.
- **Impact**: The hardcoded X position (1695) is beyond the screen width (1920), suggesting it's for a different resolution.

#### 7. **Leaderboard Region** - MASSIVE DIFFERENCE
- **Hardcoded**: (1500, 200, 420, 600)
- **Generated**: (830, 200, 200, 300)
- **Difference**: X-offset differs by 670 pixels, width by 220, height by 300.
- **Impact**: Would miss most of the leaderboard.

### Pattern Analysis

1. **Resolution Mismatch**: The hardcoded values appear to be for a higher resolution display (possibly 2560x1440 or 4K), while the generated config is for 1920x1080.

2. **Consistent X-Offset Pattern**: Most telemetry regions (speed, distance, altitude, race_time) in the hardcoded version are shifted significantly to the right compared to the generated version.

3. **Off-Screen Regions**: The gradient region X coordinate (1695) in hardcoded values would be partially off-screen on a 1920-wide display, confirming these are for a different resolution.

4. **Size Differences**: The generated config uses more uniform sizes (100x40 for most regions), while hardcoded values vary more, suggesting they may be more precisely tuned.

### Recommendations

1. **Immediate Fix**: The hardcoded regions need to be updated to match the actual 1920x1080 layout, or the code needs to support multiple resolution configs.

2. **Resolution Detection**: The application should detect the screen resolution and load appropriate region definitions.

3. **Validation**: Add bounds checking to ensure regions don't exceed screen dimensions.

4. **Testing**: The calibration tool should be used to generate correct regions for each supported resolution.

### Conclusion

The OCR is failing because it's looking in completely wrong screen locations. The hardcoded regions appear to be for a different (higher) resolution than 1920x1080, causing the OCR to read incorrect areas of the screen or even attempt to read beyond screen boundaries.