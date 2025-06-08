# Zwift OCR Tools - Complete Setup Guide

This guide covers everything you need to get the Zwift OCR telemetry extraction tools running on your system.

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

For OCR to work properly, you may need:

```bash
# On Debian/Ubuntu
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

## Troubleshooting

### OCR Not Detecting Values
1. Check video/image quality (minimum 720p recommended)
2. Ensure Zwift UI scale is at default (100%)
3. Try adjusting preprocessing threshold in code

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
- Review TECHNICAL_DETAILS.md for implementation info
- Look at example scripts in test files
- Report issues in the GitHub repository