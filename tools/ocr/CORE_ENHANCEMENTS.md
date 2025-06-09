# Zwift OCR Core Enhancements

## Overview

This document details important enhancements and behaviors in the Zwift OCR extraction system that require special handling.

## 1. Powerup Detection

### Current Implementation
```python
POWERUP_NAME = Region(444, 211, 225, 48, "powerup_name")
```

### Key Points
- **Location**: Center-right of screen when active
- **Extraction**: Letters-only constraint to avoid number confusion
- **Known Powerups**: 
  - Featherweight (climbing)
  - Aero Boost (speed)
  - Draft Boost (draft enhancement)
  - Lightweight (reduced weight)
  - Steamroller (momentum)
  - Anvil (descending)
  - Burrito (no draft effect on you)

### Challenges
- Only visible when powerup is active
- Color varies by powerup type
- May overlap with other UI elements during events

## 2. Rider Pose Detection

### Current Implementation
The `rider_pose_detector.py` analyzes the avatar to detect:
- **Normal riding** (seated)
- **Standing/climbing** (out of saddle)
- **Supertuck** (extreme descending position)

### Key Discovery
- Zwift poses are **visual only** - no gameplay effect
- Supertuck provides no actual speed benefit (contrary to popular belief)
- Detection based on avatar geometry analysis

### Technical Approach
```python
# Analyze rider silhouette in avatar region
RIDER_AVATAR = Region(860, 400, 200, 300, "rider_avatar")
# Look for characteristic shapes:
# - Upright torso = standing
# - Horizontal torso = supertuck
# - Normal angle = seated
```

## 3. Distance to Finish Dynamic Positioning

### Challenge
The "distance to finish" element moves based on UI state:

**Normal Position**:
```python
DISTANCE_TO_FINISH = Region(1143, 138, 50, 27, "distance_to_finish")
```

**During Climbs**:
- May shift up/down by 10-20 pixels
- Can be obscured by climb-specific UI (gradient graph)
- Sometimes shows segment distance instead

### Solution Approaches

1. **Search Multiple Regions**:
```python
DISTANCE_REGIONS = [
    Region(1143, 138, 50, 27, "dtf_normal"),
    Region(1143, 118, 50, 27, "dtf_climb_up"),
    Region(1143, 158, 50, 27, "dtf_climb_down")
]
```

2. **Pattern Matching**:
- Look for pattern: `\d+\.?\d* ?km`
- Distinguish from XP or other numbers by "km" suffix

3. **Context Awareness**:
- If gradient > 3%, check alternate positions
- During segments, may need to skip extraction

## 4. Additional UI Variations

### Segment Active
- Progress bar appears
- Some elements shift or hide
- Segment-specific data replaces normal data

### Event-Specific UI
- Group events may show different layouts
- Race events have expanded leaderboards
- Time trials show split times

### Resolution Scaling
- Coordinates assume 1920x1080
- Need proportional scaling for other resolutions
- Visual mapper tool helps recalibrate

## Implementation Recommendations

### For Production Use
1. Use `zwift_ocr_compact.py` for core functionality
2. Add specific enhancements only as needed
3. Handle missing data gracefully (return None)

### For Development/Analysis
1. Use `zwift_ocr_improved_final.py` for full features
2. Enable debug mode for troubleshooting
3. Log edge cases for future improvements

### Best Practices
1. **Fail gracefully**: Return None rather than crash
2. **Validate data**: Check ranges (e.g., gradient -20% to +20%)
3. **Cache regions**: Don't recalculate coordinates every frame
4. **Profile performance**: Region extraction should be <20ms

## Future Enhancements

### Machine Learning
- Train model on Zwift-specific fonts
- Detect UI state automatically
- Predict element positions

### Multi-Resolution Support
- Auto-detect resolution
- Scale regions proportionally
- Template matching for calibration

### Real-Time Adaptation
- Track UI elements between frames
- Adjust regions based on success rate
- Learn user's specific UI configuration