# OCR Calibration Guide

This guide explains how to create OCR region configuration files for different screen resolutions and Zwift versions.

## Quick Start

If you just want to use OCR with an existing configuration:
1. Check `ocr-configs/` for your resolution
2. If missing, request it via GitHub issue
3. Or follow this guide to create one

## Why Calibration is Needed

- Zwift UI element positions are fixed for each resolution/version combination
- Once calibrated, the config works for everyone with the same setup
- Community contributions benefit all users

## Prerequisites

### For Contributors
- Sample PNG frames from your Zwift setup
- Python environment with basic packages
- One of:
  - Groq API key (free tier) - Recommended
  - 16GB+ RAM for local Ollama
  - Patience for manual calibration

### Getting a Groq API Key (Recommended)
1. Visit https://console.groq.com
2. Sign up for free account
3. Generate API key
4. Set environment variable: `export GROQ_API_KEY="your-key-here"`

## Calibration Methods

### Method 1: Automatic with Groq (Easiest)

```bash
# Install dependencies
cd tools/ocr
pip install groq pillow click

# Run calibration on a sample frame
python calibrate_with_vision.py /path/to/your/recording/frame_000100.png

# This creates ocr_regions_draft.json
# Review and test it:
python visual_region_mapper.py /path/to/frame.png --config ocr_regions_draft.json
```

### Method 2: Local with Ollama

```bash
# Install Ollama
# Mac/Linux: curl -fsSL https://ollama.com/install.sh | sh
# Windows: Download from https://ollama.com

# Pull vision model (needs ~12GB disk space)
ollama pull llama3.2-vision:11b

# Run calibration
python calibrate_with_vision.py /path/to/frame.png --provider ollama
```

### Method 3: Manual Calibration

```bash
# Use the visual mapping tool
python tools/ocr/visual_region_mapper.py /path/to/frame.png

# Instructions will appear on screen:
# - Click and drag to create regions
# - Press 's' to save current region
# - Press 't' to test OCR on region
# - Press 'q' to quit and export
```

## What to Calibrate

### Essential Regions
1. **speed** - Top left, shows km/h
2. **distance** - Top center, shows km
3. **altitude** - Top center/right, shows m
4. **race_time** - Top center, shows MM:SS
5. **power** - Left side, shows W
6. **cadence** - Left side, shows rpm
7. **heart_rate** - Left side, shows bpm
8. **gradient** - Left side (may move during climbs)
9. **distance_to_finish** - Top right (race mode only)
10. **leaderboard** - Right side, list of riders
11. **rider_pose_avatar** - Center, for pose detection

### Region Guidelines
- Add 5-10 pixel padding around text
- Leaderboard should capture full width of names
- Test different game states (climbing, flat, sprint)

## Testing Your Configuration

### Basic Test
```bash
# Test single frame extraction
cargo run --features ocr --bin zwift_ocr_compact -- \
  --config ocr-configs/1920x1080_v1.67.0.json \
  /path/to/test/frame.png
```

### Validation Test
```bash
# Run on multiple frames
for frame in /path/to/recording/frame_*.png; do
  cargo run --features ocr --bin zwift_ocr_compact -- \
    --config your_config.json "$frame" \
    >> test_results.txt
done

# Check for consistent extraction
grep "speed:" test_results.txt | sort | uniq -c
```

## Submitting Your Configuration

### File Naming Convention
```
ocr-configs/{width}x{height}_v{zwift_version}.json
```

Examples:
- `1920x1080_v1.67.0.json`
- `2560x1440_v1.67.0.json`
- `3840x2160_v1.66.0.json`

### Pull Request Checklist
- [ ] Config file in correct location with proper naming
- [ ] Screenshot showing the regions overlaid
- [ ] Test results from at least 10 frames
- [ ] Note any special considerations
- [ ] Include your Zwift version

### Example PR Description
```markdown
## New OCR Config: 2560x1440 @ v1.67.0

### Testing
- Tested on 50 frames from Tick Tock race
- All telemetry fields extracted successfully
- Leaderboard accuracy ~85%

### Screenshots
![Regions Overview](regions_debug.png)

### Notes
- Gradient box moves up 20px during steep climbs
- Distance to finish only appears in races
```

## Troubleshooting

### Common Issues

**"Can't find text in region"**
- Region too small - add padding
- Wrong color threshold - test with different images
- UI element not visible - check game state

**"OCR errors on numbers"**
- Ensure region captures full digits
- Check for UI scaling issues
- Verify resolution matches exactly

**"Leaderboard names garbled"**
- Normal - focus on positions, not perfect names
- Ensure full width captured
- Check different rider counts

### Debug Mode
```python
# Enable visual debugging
python visual_region_mapper.py frame.png --debug

# Shows:
# - Current regions with labels
# - OCR results in real-time
# - Confidence scores
```

## Advanced Tips

### Handling Multiple Versions
- UI typically changes with major Zwift updates
- Minor updates rarely affect positions
- Keep old configs for compatibility

### Resolution Scaling
- Positions often scale linearly
- Can approximate from similar aspect ratios
- Always test before submitting

### Batch Calibration
```python
# Future tool for bulk calibration
python batch_calibrate.py --resolution 1920x1080 --samples-dir ./recordings/
```

## Community Guidelines

- Share configs even if imperfect - others can improve
- Test on various routes/conditions
- Document any quirks or limitations
- Help review others' contributions

## Need Help?

- Open an issue with your resolution/version
- Join discussions in PR comments  
- Check existing configs for examples
- Ask in the community forum

Remember: Your contribution helps every Zwift racer with your setup!