# OCR Telemetry Extraction - Compact Summary

## What We Built
OCR system to extract live telemetry from Zwift screenshots/video, similar to SpaceX telemetry extraction.

## Key Components
1. **zwift_ocr_improved.py** - Main extraction engine
2. **zwift_video_processor.py** - Video stream processing  
3. **rider_pose_detector.py** - Avatar position detection

## Extracted Data
- Speed, power, HR, cadence, distance, altitude
- Gradient %, distance to finish
- Power-ups with circular timer
- Rider positions (visual only - except supertuck)
- Leaderboard with names, w/kg, distance

## Key Discovery
Most Zwift positions are visual only! Only supertuck affects aerodynamics (-25% drag).

## Usage
```bash
cd tools/ocr/
uv sync  # Install dependencies
uv run python zwift_video_processor.py video.mp4
```

## Git Status
- Branch: `ocr` (ready to push)
- Files: 15 total (9 code, 6 docs/images)
- Commits: 2 (implementation + documentation)