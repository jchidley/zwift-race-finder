# Zwift OCR Quick Reference

## One-Time Setup
```bash
cargo install mask              # Install task runner
cd tools/ocr/                   # Navigate to OCR directory  
mask setup                      # Install Python dependencies
```

## Common Commands

### Process Files
```bash
mask screenshot image.jpg       # Extract from screenshot
mask video recording.mp4        # Process video (1 fps)
mask video file.mp4 --skip-frames 60  # Process every 2 seconds
```

### Testing & Development
```bash
mask test                       # Run all tests
mask compare-engines            # Compare OCR accuracy
mask calibrate-poses            # Test pose detection
mask lint                       # Check code quality
mask format                     # Auto-format code
```

### View Results
```bash
# Quick view
head telemetry_*.csv           # View CSV data
sqlite3 telemetry.db ".tables" # List database tables

# Analysis
sqlite3 telemetry.db "SELECT AVG(power), MAX(speed) FROM telemetry;"
cat telemetry_*.json | jq '.frames[0]'  # View first frame
```

### Cleanup
```bash
mask clean                      # Remove generated files
```

## Data Fields Extracted

**Performance**: Power (W), Cadence (RPM), Heart Rate (BPM), Speed (km/h)  
**Progress**: Distance (km), Altitude (m), Race Time, Energy (kJ)  
**Racing**: Gradient (%), Distance to Finish, Power-ups, Leaderboard  
**Visual**: Rider Position (cosmetic except supertuck)

## Tips
- Use `--no-preview` for faster processing
- PaddleOCR is faster, EasyOCR is easier to install
- Default processes 1 frame/second (30 fps = skip 30 frames)
- Supertuck is the only position affecting speed (-25% drag)