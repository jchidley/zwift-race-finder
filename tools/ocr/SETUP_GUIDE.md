# Zwift OCR Tools - Complete Setup Guide

This guide covers everything you need to get the Zwift OCR telemetry extraction tools running on your system, including both Python and Rust implementations.

## Prerequisites

### 1. Install mask (Task Runner)
```bash
# Install mask globally using cargo
cargo install mask

# Verify installation
mask --version
```

### 2. Install uv (Python Package Manager)
```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or on Debian/Ubuntu
sudo apt update && sudo apt install -y uv

# Verify installation
uv --version
```

### 3. System Dependencies

#### Python OCR (PaddleOCR)
```bash
# Quick install script (recommended)
./install_ocr_deps.sh

# Or manually install on Debian/Ubuntu
sudo apt update
sudo apt install -y \
    python3-dev \
    libgl1-mesa-glx \
    libglib2.0-0 \
    libsm6 \
    libxext6 \
    libxrender-dev \
    libgomp1 \
    libgstreamer1.0-0 \
    libgstreamer-plugins-base1.0-0

# For video processing
sudo apt install -y ffmpeg
```

#### Rust OCR (Tesseract)
```bash
# Debian/Ubuntu
sudo apt install -y \
    libleptonica-dev \
    libtesseract-dev \
    tesseract-ocr \
    tesseract-ocr-eng

# macOS
brew install tesseract leptonica

# Note: OpenCV is NOT required for the compact implementation
```

## Installation Steps

### 1. Navigate to OCR Tools Directory
```bash
cd /home/jack/tools/rust/zwift-race-finder/tools/ocr/
```

### 2. Install Python Dependencies
```bash
# Using mask (recommended)
mask setup

# Or manually with uv
uv sync
```

### 3. Verify Installation
```bash
# Run tests to ensure everything works
mask test

# Or check OCR engines
mask compare-engines
```

### 4. Build Rust OCR (Optional)
```bash
# From project root
cd ../..
cargo build --features ocr --bin zwift_ocr_compact

# Or if mask has rust-build task
mask rust-build

# Test the Rust implementation
./target/debug/zwift_ocr_compact tools/ocr/../../docs/screenshots/normal_1_01_16_02_21.jpg

# Compare Python vs Rust
cd tools/ocr/
uv run python compare_ocr_compact.py
```

## Quick Start Examples

### Basic Screenshot Analysis
```bash
# Analyze a single screenshot
mask screenshot ../../docs/screenshots/normal_1_01_16_02_21.jpg

# Or use the wrapper script
./zwift_ocr.sh screenshot ../../docs/screenshots/with_climbing_1_01_36_01_42.jpg
```

### Video Processing
```bash
# Process a video file (1 frame per second)
mask video ~/Videos/zwift_race.mp4

# Process every 2 seconds for longer videos
mask video ~/Videos/zwift_race.mp4 --skip-frames 60

# Process without preview window (faster)
mask video ~/Videos/zwift_race.mp4 --no-preview
```

### Available Commands

View all available tasks:
```bash
mask --help
```

Key tasks:
- `mask setup` - Install dependencies
- `mask test` - Run all tests
- `mask screenshot <path>` - Extract from image
- `mask video <path>` - Process video file
- `mask compare-engines` - Compare OCR accuracy
- `mask calibrate-poses` - Calibrate rider detection
- `mask lint` - Check code quality
- `mask format` - Auto-format code
- `mask clean` - Remove generated files

## Output Files

After processing, you'll find:
- `telemetry_YYYYMMDD_HHMMSS.csv` - Spreadsheet format
- `telemetry_YYYYMMDD_HHMMSS.json` - JSON format
- `telemetry.db` - SQLite database

## Viewing Results

### CSV Output
```bash
# View first few lines
head telemetry_*.csv

# Open in spreadsheet
libreoffice --calc telemetry_*.csv
```

### JSON Output
```bash
# Pretty print JSON
cat telemetry_*.json | jq '.'

# Extract specific fields
cat telemetry_*.json | jq '.frames[0:5] | .[] | {time: .timestamp, speed: .speed, power: .power}'
```

### SQLite Database
```bash
# Interactive query
sqlite3 telemetry.db

# Sample queries
sqlite3 telemetry.db "SELECT COUNT(*) FROM telemetry;"
sqlite3 telemetry.db "SELECT timestamp, speed, power, heart_rate FROM telemetry LIMIT 10;"
sqlite3 telemetry.db "SELECT AVG(power) as avg_power, MAX(speed) as max_speed FROM telemetry;"
```

## Debugging OCR Extraction

### Visual Debug Mode
The most effective way to troubleshoot OCR issues is using the debug visualization:

```bash
# Create debug visualization
mask debug /path/to/screenshot.jpg

# This creates an image showing:
# - Colored boxes around each extraction region
# - Extracted values (or "?" if failed)
# - Success/failure status for each field
# - Overall extraction success rate
```

### Common Issues and Solutions

#### OCR Not Detecting Values
1. **Use debug mode first** to see if regions are correctly positioned
2. Check image quality (1920x1080 recommended)
3. Ensure Zwift UI scale is at default (100%)
4. Try different OCR engines (PaddleOCR vs EasyOCR)

#### Concatenated Text Problem
Some UI elements (speed, distance, altitude, time) may be read as one string:
- Example: "2018.410631:06" instead of "20", "18.4", "106", "31:06"
- Solution: Split top bar into separate regions (implemented in v2)

#### Cadence Not Reading
- Issue: Region too small for 2-3 digit numbers
- Solution: Increased region size and added scaling in preprocessing

#### Gradient Not Found
- Issue: Gradient box too large, capturing extra elements
- Solution: Reduced box size to focus on percentage only

### ImportError: No module named 'paddleocr'
```bash
# Ensure you're in the OCR directory
cd /home/jack/tools/rust/zwift-race-finder/tools/ocr/

# Reinstall dependencies
uv sync

# Or add specific engine
uv add paddlepaddle paddleocr
```

### Video Processing Errors
```bash
# Check ffmpeg is installed
ffmpeg -version

# If not, install it
sudo apt install -y ffmpeg
```

### Permission Denied on Scripts
```bash
# Make scripts executable
chmod +x zwift_ocr.sh
```

## Advanced Usage

### Custom Video Processing
```python
# Create a custom script
cat > process_my_video.py << 'EOF'
#!/usr/bin/env python3
from zwift_video_processor import ZwiftVideoProcessor

# Custom settings
processor = ZwiftVideoProcessor(
    skip_frames=15,      # Process every 0.5 seconds
    show_preview=True,   # Show live preview
    save_frames=True     # Save extracted frames
)

# Process with callbacks
def on_frame(data):
    if data.power and data.power > 300:
        print(f"High power detected: {data.power}W at {data.timestamp}s")

processor.process_video("my_race.mp4", callback=on_frame)
EOF

uv run python process_my_video.py
```

### Integration with Race Finder
```bash
# Extract telemetry from your race
mask video my_zwift_race.mp4

# Import key metrics into race finder
sqlite3 ~/.local/share/zwift-race-finder/races.db << 'EOF'
ATTACH DATABASE 'telemetry.db' AS tel;

-- Create telemetry summary
CREATE TABLE IF NOT EXISTS telemetry_summaries (
    id INTEGER PRIMARY KEY,
    filename TEXT,
    date TEXT,
    duration_seconds INTEGER,
    distance_km REAL,
    avg_power INTEGER,
    max_power INTEGER,
    avg_hr INTEGER,
    energy_kj INTEGER
);

-- Import summary
INSERT INTO telemetry_summaries 
SELECT 
    NULL,
    'my_zwift_race.mp4',
    datetime('now'),
    MAX(race_time),
    MAX(distance),
    ROUND(AVG(power)),
    MAX(power),
    ROUND(AVG(heart_rate)),
    MAX(energy)
FROM tel.telemetry;
EOF
```

## Next Steps

1. Process your own Zwift recordings
2. Build a library of telemetry data
3. Use data to validate race duration predictions
4. Create custom analysis scripts
5. Share interesting findings!

## Getting Help

- Check the README.md for feature details
- Review TECHNICAL_REFERENCE.md for implementation info
- Look at example scripts in test files
- Report issues in the GitHub repository