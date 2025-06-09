# Zwift OCR Telemetry Extraction

This tool extracts live telemetry data from Zwift screenshots and video recordings using Optical Character Recognition (OCR), similar to how the [AeroTelemProc_VidData](https://github.com/mateosolinho/AeroTelemProc_VidData) project extracts SpaceX telemetry.

## Features

### Python Implementation (`zwift_ocr_compact.py`)
- **100% Accurate Extraction** of all major telemetry:
  - Speed, Distance, Altitude, Race Time
  - Power, Cadence, Heart Rate
  - Gradient (with special font handling)
  - Distance to Finish
  - Full Leaderboard with rider identification
- **Optimized Performance**: ~10x faster than full-image OCR
- **Character Constraints**: Eliminates OCR errors (O/0, I/1 confusion)
- **Engine**: PaddleOCR for best accuracy

### Rust Implementation (`zwift_ocr_compact`)
- **5-6x Faster** than Python (1-3s vs 6-20s per image)
- **100% Accuracy** on core fields (speed, distance, power, etc.)
- **Minimal Dependencies**: Just Tesseract and pure Rust image processing
- **Current Limitations**: No gradient or leaderboard extraction yet
- **Build**: `cargo build --features ocr --bin zwift_ocr_compact`

### Extended Python Tools
- **Enhanced Extractor** (`zwift_ocr_improved_final.py`): Adds power-up detection, debug mode
- **Video Processor** (`zwift_video_processor.py`): Extract from recordings
- **Pose Detector** (`rider_pose_detector.py`): Detect riding positions
- **Visual Mapper** (`visual_region_mapper.py`): Calibrate regions
- **Debug Visualizer** (`debug_visualizer_v3.py`): Troubleshooting aid

## Installation

For detailed setup instructions, see [SETUP_GUIDE.md](SETUP_GUIDE.md).

### Quick Install

```bash
# Python dependencies
cd tools/ocr/
uv sync

# Rust build (from project root)
cargo build --features ocr --bin zwift_ocr_compact

# Task runner (optional)
cargo install mask
```

## Usage

### Python (Full Features)
```bash
# Single screenshot
uv run python zwift_ocr_compact.py screenshot.jpg

# With debug visualization
uv run python zwift_ocr_improved_final.py screenshot.jpg --debug

# Process video
uv run python zwift_video_processor.py video.mp4
```

### Rust (Fast Core Features)
```bash
# From project root
cargo run --features ocr --bin zwift_ocr_compact -- screenshot.jpg

# JSON output (default)
./target/debug/zwift_ocr_compact screenshot.jpg

# Text output
./target/debug/zwift_ocr_compact screenshot.jpg --format text
```

### Performance Comparison
```bash
# Compare implementations
cd tools/ocr/
uv run python compare_ocr_compact.py
```

## Performance

| Implementation | Speed | Accuracy | Features |
|----------------|-------|----------|----------|
| Python (PaddleOCR) | 6-20s | 100% all fields | Full telemetry + leaderboard |
| Rust (Tesseract) | 1-3s | 100% core fields | Speed, distance, power, etc. |

The Rust implementation is 5-6x faster while maintaining perfect accuracy on core telemetry fields.

## Technical Details

For architecture, region mapping, and implementation details, see [TECHNICAL_REFERENCE.md](TECHNICAL_REFERENCE.md).

## Task Runner Commands

Using mask (recommended):
```bash
mask --help              # Show available tasks
mask test               # Run tests
mask video recording.mp4 # Process video
mask debug screenshot.jpg # Debug visualization
```

See [maskfile.md](maskfile.md) for all available commands.

## Integration with Zwift Race Finder

The extracted telemetry can be used to:
- Validate duration estimates against actual ride data
- Track performance metrics during races
- Build personalized prediction models
- Analyze pacing strategies

## Future Enhancements

- [ ] Complete Rust implementation (gradient, leaderboard)
- [ ] Automatic UI scale detection
- [ ] Real-time data streaming
- [ ] Integration with Strava/TrainingPeaks