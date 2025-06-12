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
- **5.4x Faster** than Python for single images (0.88s vs 4.77s)
- **9.2x Faster** with parallel mode for batch/video (0.52s vs 4.77s)
- **100% Accuracy** on all core telemetry fields
- **Hybrid OCR**: Tesseract for numbers, ocrs neural network for leaderboard text
- **Feature Complete**: All 11 fields including leaderboard and rider pose detection
- **Good Leaderboard Accuracy**: ~80% name recognition with ocrs
- **Two Modes**: Sequential (default, best for CLI) and Parallel (--parallel, best for batch)
- **Build**: `cargo build --features ocr --bin zwift_ocr_compact --release`

### Extended Python Tools
- **Enhanced Extractor** (`zwift_ocr_improved_final.py`): Adds power-up detection, debug mode
- **Video Processor** (`zwift_video_processor.py`): Extract from recordings with pose analysis
- **Rider Pose Detector** (`rider_pose_detector.py`): Detect riding positions and aerodynamic implications
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

## Video Acquisition

### Windows PowerShell Recording (record-monitor2.ps1)

For capturing Zwift gameplay on Windows, use the optimized PowerShell script:

```powershell
# Basic recording (4 hours at 8fps video + 1fps PNG extraction)
.\record-monitor2.ps1

# Custom duration and frame rates
.\record-monitor2.ps1 -duration 7200 -fps 10 -pngFps 2

# PNG frames only (no video file)
.\record-monitor2.ps1 -pngOnly

# With smart frame filtering to reduce duplicate frames
.\record-monitor2.ps1 -smartFilter

# Custom resolution (e.g., for lower file sizes)
.\record-monitor2.ps1 -resolution "1280x720"

# Named recording session
.\record-monitor2.ps1 -name "alpe_du_zwift_race"
```

**Features**:
- Records from secondary monitor (where Zwift typically runs)
- Simultaneous video (MP4) and PNG frame extraction
- Smart filtering option removes duplicate/similar frames
- Configurable frame rates for video and PNG extraction
- Auto-creates timestamped output folders
- Optimized FFmpeg settings for game capture

**Parameters**:
- `-fps`: Video recording frame rate (default: 8)
- `-pngFps`: PNG extraction frame rate (default: 1)
- `-duration`: Recording duration in seconds (default: 14400 = 4 hours)
- `-outputDir`: Output directory (default: `%USERPROFILE%\Videos\Recordings`)
- `-name`: Optional prefix for output files
- `-pngOnly`: Skip video, extract PNG frames only
- `-smartFilter`: Use mpdecimate filter to remove similar frames
- `-resolution`: Override capture resolution

**Output Structure**:
```
Videos/Recordings/
├── 2025-01-12_14-30-00.mp4          # Video file
└── 2025-01-12_14-30-00/             # PNG frames folder
    ├── frame_000001.png
    ├── frame_000002.png
    └── ...
```

## Usage

### Rust Implementation (Recommended for Speed)
```bash
# Build release version (first time only)
cargo build --features ocr --bin zwift_ocr_compact --release

# Extract telemetry (JSON output)
./target/release/zwift_ocr_compact docs/screenshots/normal_1_01_16_02_21.jpg

# Human-readable output  
./target/release/zwift_ocr_compact docs/screenshots/normal_1_01_16_02_21.jpg --format text

# Using cargo run (slower)
cargo run --features ocr --bin zwift_ocr_compact --release -- screenshot.jpg

# Benchmark sequential vs parallel implementations
./target/release/zwift_ocr_benchmark screenshot.jpg --iterations 10
```

**Example Output (JSON)**:
```json
{
  "speed": 34,
  "distance": 6.4,
  "altitude": 28,
  "race_time": "11:07",
  "power": 268,
  "cadence": 72,
  "heart_rate": 160,
  "gradient": 3.0,
  "distance_to_finish": 28.6,
  "leaderboard": null
}
```

### Python Implementation (Full Features)
```bash
# Navigate to OCR directory
cd tools/ocr/

# Extract all telemetry including leaderboard
uv run python zwift_ocr_compact.py ../../docs/screenshots/normal_1_01_16_02_21.jpg

# Enhanced version with debug visualization
uv run python zwift_ocr_improved_final.py ../../docs/screenshots/normal_1_01_16_02_21.jpg --debug

# Process entire video with pose detection
uv run python zwift_video_processor.py /path/to/zwift_recording.mp4

# Calibrate pose detection with sample images
mask calibrate-poses
```

### Performance Comparison
```bash
# Compare both implementations side-by-side
cd tools/ocr/
uv run python compare_ocr_compact.py

# Time both (from project root)
time ./target/release/zwift_ocr_compact docs/screenshots/normal_1_01_16_02_21.jpg > /dev/null
time (cd tools/ocr && uv run python zwift_ocr_compact.py ../../docs/screenshots/normal_1_01_16_02_21.jpg > /dev/null)
```

## Performance Comparison

| Implementation | Speed | Accuracy | Use Case |
|----------------|-------|----------|----------|
| **Python (PaddleOCR)** | **4.77s** | 100% all fields | Perfect accuracy needed |
| **Rust Sequential** | **0.88s** | 100% telemetry, 80% leaderboard | CLI tools, single images |
| **Rust Parallel (warm)** | **0.52s** | 100% telemetry, 80% leaderboard | Batch processing, video |
| **Rust Parallel (cold)** | **1.14s** | 100% telemetry, 80% leaderboard | First run penalty |

**Speed Comparisons**: 
- Single image: Rust Sequential is **5.4x faster** than Python
- Batch/video: Rust Parallel is **9.2x faster** than Python
- Parallel speedup: **1.55x faster** than sequential (after warm-up)

**Key Notes**:
- All implementations extract the same 11 fields
- Rust has 100% accuracy on telemetry, ~80% on leaderboard names
- Python has 100% accuracy on all fields including leaderboard
- Parallel mode requires warm-up to achieve best performance

See [OCR_COMPARISON_FINDINGS.md](OCR_COMPARISON_FINDINGS.md) for detailed performance analysis of different OCR approaches.

## Technical Details

For architecture, region mapping, and implementation details, see [TECHNICAL_REFERENCE.md](TECHNICAL_REFERENCE.md).

For comprehensive testing information, benchmarks, and quality metrics, see [TESTING_GUIDE.md](TESTING_GUIDE.md).

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

## Current Status & Future Enhancements

### ✅ Completed Features
- [x] **Complete core telemetry extraction** (Rust v1.0): Speed, distance, altitude, time, power, cadence, HR
- [x] **Gradient extraction** (Rust v1.0): Current slope percentage with specialized font handling
- [x] **Distance-to-finish extraction** (Rust v1.0): Remaining race distance
- [x] **Production-ready performance** (Rust v1.0): Sub-200ms extraction speed
- [x] **Leaderboard extraction** (Rust v1.1): Multi-rider names, positions, deltas, w/kg values with ocrs
- [x] **Rider pose detection** (Rust v1.1): Detect riding positions and aerodynamic drag implications
- [x] **Parallel processing** (Rust v1.3): 1.55x speedup for batch/video processing with --parallel flag

### 🚧 Future Enhancements

#### ✅ Priority 1: Feature Completeness (COMPLETED in v1.1)
- [x] **Leaderboard extraction** (Rust): Feature parity with Python implementation
  - ✅ Multi-rider parsing with name detection, deltas, w/kg values
  - ✅ Hybrid approach: ocrs neural network for better text recognition
  - ✅ Current rider detection based on missing time delta
  - ✅ ~80% name accuracy (vs Tesseract's ~10%) with ocrs integration
  - ✅ Completed: Hybrid Tesseract + ocrs implementation

- [x] **Rider pose detection** (Rust): Ported pose classification from Python
  - ✅ Detects: normal_tuck (high drag), normal_normal, climbing_seated, climbing_standing (high drag)
  - ✅ Extracts pose features: aspect ratio, center of mass, density analysis, symmetry
  - ✅ Avatar region: (860, 400, 200, 300) for 1920x1080
  - ✅ Edge detection with Canny algorithm
  - ✅ Completed: Full feature parity achieved

#### Priority 2: Usability Improvements
- [ ] **Automatic UI scale detection**: Handle different screen resolutions automatically
  - Currently optimized for 1920x1080 displays
  - Auto-detect resolution and scale coordinates proportionally
  - Eliminate manual calibration for different screen sizes

- [ ] **AI-powered region optimization**: Auto-calibrate OCR regions during race join period
  - **Perfect timing**: Races require joining before start - provides calibration window
  - Use computer vision to detect UI elements during pre-race period
  - Template matching or feature detection to locate speed, power, distance etc.
  - Account for minor shifts during climbing (some elements move ±10-20 pixels)
  - Cache optimized regions per screen resolution for future sessions
  - Complete calibration before race starts, ready for immediate extraction

#### Priority 3: Data Integration & Advanced Features
- [ ] **Sensor data integration**: Combine OCR with direct ANT+/Bluetooth telemetry
  - ✅ **Verified**: Strava .fit files record power, cadence, HR at **1Hz (1 second intervals)**
  - ✅ **97-minute race** = 5,831 data points with 100% sensor data completeness
  - ✅ **FIT file analysis** (2025-06-06-10-23-16.fit): Confirmed consistent 1.0 second update intervals
  - OCR focus on UI-only data: position, leaderboard, gradient, distance-to-finish, rider pose
  - Cross-validate OCR accuracy against sensor ground truth
  - Optimal strategy: Real-time sensor feeds + periodic OCR extraction

- [ ] **Real-time streaming integration**: Live overlay for broadcasts
  - Process streaming video for position/leaderboard data (not available from sensors)
  - Combine with direct sensor feeds for complete telemetry
  - Enable real-time race analysis with both UI and sensor data

- [ ] **UI change adaptation**: Handle future Zwift interface updates
  - Monitor for UI layout changes between game versions
  - Automatically re-calibrate regions when changes detected
  - Version-aware region storage (e.g., "zwift_v1.60_1920x1080_regions.json")