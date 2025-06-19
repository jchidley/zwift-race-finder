# PROJECT_WISDOM_ARCHIVE_20250614.md - Archived Insights

Archived from PROJECT_WISDOM.md on 2025-06-14 to keep main file under 5KB.

## 2025-01-12: OCR Detection - Full-Image vs Targeted Regions
**Insight**: Full-image OCR detection with PaddleOCR produces 70+ regions with many false positives. The original targeted region extraction approach is more reliable.
**Impact**: Don't try to "improve" by detecting all text at once - stick with targeted extraction for specific UI elements.

## 2025-01-12: OCR Preprocessing - Field-Specific Requirements
**Insight**: Each Zwift UI element requires different OCR preprocessing settings:
- Gradient (in mini-map area, top-right): Image inversion + threshold 100 + 4x scale (stylized font)
- Altitude: Threshold 230 (faint text)
- Distance to Finish: Threshold 150 (dimmer text)
- Standard fields (speed, power): Threshold 200 + 3x scale
- Leaderboard: CLAHE enhancement instead of binary threshold
**Impact**: One-size-fits-all preprocessing doesn't work. Each field must be tuned individually.

## 2025-01-12: OCR Debugging - Essential Tool Usage
**Insight**: The `cargo run --release --features ocr --bin debug_ocr` tool is essential for finding optimal preprocessing settings per field - it shows all variations and what works.
**Impact**: Always use debug_ocr when calibrating new regions. The --release flag is critical for performance.

## 2025-01-12: Python Tools - Use uv Instead of pip/venv
**Insight**: When running Python scripts in tools/ocr, use `uv run python script.py` instead of `python script.py` or activating venv - it's the standard for this project.
**Impact**: Avoids "module not found" errors and ensures consistent environment across all Python tools.

## 2025-06-12: OCR Data Strategy - Focus Only on UI-Exclusive Elements
**Insight**: Don't extract data available from other sources (HR, cadence, avg power) - focus only on UI-exclusive data that can't be obtained from sensors, Zwift API, or FIT files.
**Impact**: More efficient OCR, avoid redundant data collection, focus effort on truly unique UI elements.

## 2025-06-12: API Calibration - UDP Packet Monitoring for Ground Truth
**Insight**: Zwift broadcasts real-time telemetry via UDP packets (port 3022) that can serve as ground truth for OCR validation. Overlaps significantly with OCR targets: power, speed, distance, gradient.
**Impact**: Enables automatic OCR accuracy validation, confidence scoring, and potential auto-calibration of regions.

## 2025-01-12: OCR Focus - UI-Exclusive Data Only
**Insight**: Only extract data NOT available from sensors/downloads:
- UI-exclusive: Speed, Power, Distance, Altitude, Race Time, Distance to Finish, Gradient (mini-map area), Route/Segment name, Lap counter, Lead-in status, Leaderboard
- Skip sensor data: Heart rate, Cadence, Average power, Total energy (all available from FIT files/Zwift API)
**Impact**: Reduced complexity, faster processing, no redundant data extraction. Focus OCR efforts only on data unique to the UI.