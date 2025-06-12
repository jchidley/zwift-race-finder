# Comprehensive OCR Strategy for Zwift Race Finder (Revised)

## Executive Summary

Based on the insight that UI layouts are stable per Zwift version and screen resolution, we've developed a community-driven approach that eliminates per-user calibration. The focus shifts from perfect name OCR to maintaining correct rider ordering through fuzzy matching.

## Key Insights

1. **UI Stability**: Region mappings remain constant for a given Zwift version + screen resolution
2. **Community Leverage**: Configuration files can be shared via GitHub PRs
3. **Priority Shift**: Rider ordering matters more than perfect name OCR
4. **Validation Simplification**: Only need to track "who's where", not exact name spelling

## Current OCR Implementation Status

### What We Have
1. **Rust Implementation** (Primary)
   - Speed: 0.88s per frame (5.4x faster than Python)
   - Accuracy: 100% on telemetry, ~80% on leaderboard names
   - Uses Tesseract + ocrs neural network
   - Memory efficient (<100MB)

2. **Python Implementation** (Reference)
   - Speed: 4.77s per frame
   - Accuracy: 100% on all fields
   - Uses PaddleOCR
   - Memory: ~2GB when active

## Revised Architecture: Community-Driven Configuration

```
┌─────────────────────────────────────────────────────────┐
│      Community Configuration Repository                  │
│  GitHub: ocr-configs/                                    │
│  - 1920x1080_v1.67.0.json                              │
│  - 2560x1440_v1.67.0.json                              │
│  - 3840x2160_v1.67.0.json                              │
│  Contributors submit PRs for new resolutions/versions   │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│         Runtime Processing                               │
│  Rust + Tesseract (Local)                               │
│  - Load config for user's resolution/version            │
│  - Process every frame with known regions               │
│  - Speed: 0.88s per frame                               │
│  - Memory: <100MB                                       │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│      Rider Order Validation (Optional)                   │
│  Fuzzy Matching + Position Tracking                     │
│  - Track rider positions across frames                  │
│  - Use fuzzy matching to handle OCR errors              │
│  - Alert on significant position changes                │
└─────────────────────────────────────────────────────────┘
```

## Configuration File Format

### Proposed Structure
```json
{
  "version": "1.67.0",
  "resolution": "1920x1080",
  "aspect_ratio": "16:9",
  "created_by": "jchidley",
  "created_date": "2025-01-12",
  "regions": {
    "speed": {"x": 34, "y": 12, "width": 60, "height": 32},
    "distance": {"x": 456, "y": 12, "width": 80, "height": 32},
    "altitude": {"x": 890, "y": 12, "width": 70, "height": 32},
    "race_time": {"x": 456, "y": 48, "width": 90, "height": 40},
    "power": {"x": 34, "y": 180, "width": 70, "height": 35},
    "cadence": {"x": 34, "y": 220, "width": 60, "height": 30},
    "heart_rate": {"x": 34, "y": 250, "width": 60, "height": 30},
    "gradient": {"x": 34, "y": 380, "width": 50, "height": 30},
    "distance_to_finish": {"x": 1200, "y": 80, "width": 100, "height": 35},
    "leaderboard": {"x": 1400, "y": 120, "width": 480, "height": 600},
    "leaderboard_entry_height": 75,
    "rider_pose_avatar": {"x": 860, "y": 400, "width": 200, "height": 300}
  },
  "notes": "Calibrated using Tick Tock route in race mode"
}
```

### Resolution Variants
- **16:9 Aspect**: 1920x1080, 2560x1440, 3840x2160
- **21:9 Ultrawide**: 2560x1080, 3440x1440, 5120x2160
- **16:10 Aspect**: 1920x1200, 2560x1600
- **4:3 Legacy**: 1024x768, 1280x1024

## Community Contribution Process

### Initial Calibration (One-Time per Config)
1. **Manual Method**:
   - Use existing `visual_region_mapper.py` tool
   - Click and drag to define regions
   - Test OCR accuracy on each region
   - Export as JSON

2. **Semi-Automated Method** (Optional):
   - Use cloud vision API (Groq/HuggingFace) to detect regions
   - Manually verify and adjust
   - Test on multiple frames
   - Submit PR with evidence screenshots

### Contribution Workflow
```bash
# Fork zwift-race-finder
# Create new config
cp ocr-configs/template.json ocr-configs/2560x1440_v1.67.0.json
# Edit regions using mapper tool
python tools/ocr/visual_region_mapper.py recording.png
# Test configuration
cargo test --features ocr -- --test-config ocr-configs/2560x1440_v1.67.0.json
# Submit PR with:
# - Config file
# - Screenshot showing regions
# - Test results
```

## Rider Order Tracking (The Real Priority)

### Why Order Matters More Than Names
1. **Race dynamics**: "Did I pass that rider?" matters more than spelling their name
2. **Fuzzy matching sufficient**: "J.Ch1dley" vs "J.Chidley" - same rider
3. **Position stability**: Riders don't teleport; positions change gradually
4. **Performance tracking**: Track gaps and time differences, not exact names

### Rider Tracking Algorithm
```rust
struct RiderTracker {
    riders: Vec<TrackedRider>,
    position_history: VecDeque<Vec<String>>, // Last N frames
}

struct TrackedRider {
    fuzzy_id: String,        // Best guess at name
    variations: HashSet<String>, // All seen OCR variations
    last_position: usize,
    stability_score: f32,    // How consistent the OCR has been
}

impl RiderTracker {
    fn update(&mut self, ocr_names: Vec<String>) {
        // 1. Fuzzy match against known riders
        // 2. Track position changes
        // 3. Flag anomalies (rider jumped 5 positions?)
        // 4. Maintain rolling history
    }
}
```

## Implementation Plan

### Phase 1: Configuration System (Week 1)
1. **Create Config Structure**:
   - Define JSON schema for region configs
   - Add version detection logic
   - Create resolution detection utility

2. **Bootstrap Initial Configs**:
   - Create 1920x1080 config from existing constants
   - Test on your recordings
   - Document calibration process

3. **GitHub Integration**:
   - Create `ocr-configs/` directory
   - Add README with contribution guide
   - Set up PR template for new configs

### Phase 2: Runtime Integration (Week 2)
```rust
// Config loader
impl OCRConfig {
    fn load_for_system() -> Result<Self> {
        let resolution = detect_resolution()?;
        let zwift_version = detect_zwift_version()?;
        
        // Try exact match first
        let config_name = format!("{}_{}.json", resolution, zwift_version);
        if let Ok(config) = Self::load_from_file(&config_name) {
            return Ok(config);
        }
        
        // Fall back to resolution-only match
        self::find_closest_config(resolution)
    }
}
```

### Phase 3: Rider Tracking (Week 3)
1. **Fuzzy Matching**:
   - Use Levenshtein distance for name similarity
   - Track all variations seen for each rider
   - Build confidence scores

2. **Position Validation**:
   - Detect impossible position changes
   - Smooth out OCR glitches
   - Maintain position history

3. **Integration**:
   - Add to existing Rust OCR pipeline
   - Optional validation mode
   - Performance metrics

## Key Decisions & Rationale

### 1. **Why Community Configs Over Per-User Calibration?**
- UI regions are stable per version/resolution
- One person's work benefits everyone
- Eliminates calibration complexity for users
- Enables crowd-sourced quality improvements

### 2. **Why Focus on Rider Order Over Perfect Names?**
- Race dynamics (positions) matter more than spelling
- Fuzzy matching handles OCR errors gracefully
- Reduces need for expensive validation
- Aligns with actual user needs

### 3. **Why Keep It Simple?**
- No cloud APIs needed for normal operation
- Community handles the hard part (calibration)
- Focus on speed and reliability
- Lower barrier to contribution

## Benefits of This Approach

### For Users
- **Zero setup**: Download config for your resolution
- **Fast processing**: 0.88s per frame, no cloud delays
- **Reliable tracking**: Rider positions remain consistent
- **Free forever**: No API costs or limits

### For Contributors
- **Clear process**: Use existing mapper tool
- **Quick validation**: Test on your own recordings
- **Immediate impact**: Help entire community
- **Version control**: Git tracks all changes

### For the Project
- **Scalable**: Community grows the config library
- **Maintainable**: Simple JSON files
- **Robust**: Failures gracefully handled
- **Future-proof**: New resolutions easily added

## Memory & Performance Profile

### Runtime (Continuous)
- Rust + Tesseract: ~100MB
- Config storage: <1MB
- Rider tracking: ~10MB
- **Total**: <150MB constant

### No Cloud Dependencies
- No API calls during processing
- No network latency
- No rate limits
- No costs

## Potential Enhancements

### 1. **Auto-Config Detection**
```rust
// Future: Detect config needs
if !config_exists_for_resolution() {
    suggest_contribution_or_fallback()
}
```

### 2. **Config Interpolation**
- Scale regions for missing resolutions
- Use aspect ratio matching
- Confidence-based adjustments

### 3. **Version Migration**
- Track Zwift UI changes
- Auto-update configs when possible
- Alert community for major changes

## Calibration Guide for Contributors

### Prerequisites
Contributors who create new configs should have:
- Sample PNG frames from their resolution
- Basic Python environment (for tools)
- One of: Groq API key (free), Ollama + 16GB RAM, or manual patience

### Method 1: Cloud Vision API (Recommended)
```bash
# 1. Get a free Groq API key from https://console.groq.com
export GROQ_API_KEY="your-key-here"

# 2. Install calibration tool
cd tools/ocr
pip install groq pillow

# 3. Run calibration script
python calibrate_with_vision.py /path/to/sample.png --provider groq

# 4. Review and adjust output
python visual_region_mapper.py /path/to/sample.png --config ocr_regions_draft.json

# 5. Test the configuration
cargo run --features ocr --bin test_ocr_config -- --config ocr_regions_draft.json --image /path/to/sample.png
```

### Method 2: Local Ollama (16GB+ RAM)
```bash
# 1. Install Ollama and pull vision model
ollama pull llama3.2-vision:11b

# 2. Run calibration
python calibrate_with_vision.py /path/to/sample.png --provider ollama

# 3. Continue with steps 4-5 above
```

### Method 3: Manual Calibration
```bash
# 1. Use visual mapper directly
python tools/ocr/visual_region_mapper.py /path/to/sample.png

# 2. Click and drag to define each region:
#    - Speed, Distance, Altitude (top bar)
#    - Power, Cadence, HR (left side)
#    - Gradient (left side, may move)
#    - Race time (top center)
#    - Distance to finish (top right)
#    - Leaderboard (right side)

# 3. Test each region as you go
# 4. Export when complete
```

### Calibration Script (calibrate_with_vision.py)
```python
#!/usr/bin/env python3
"""Auto-calibrate OCR regions using vision LLMs"""

import json
import base64
from pathlib import Path
import click
from PIL import Image
import io

@click.command()
@click.argument('image_path', type=click.Path(exists=True))
@click.option('--provider', type=click.Choice(['groq', 'ollama']), default='groq')
@click.option('--output', default='ocr_regions_draft.json')
def calibrate(image_path, provider, output):
    """Generate OCR regions config using vision AI"""
    
    # Detect resolution from image
    img = Image.open(image_path)
    width, height = img.size
    
    print(f"Detected resolution: {width}x{height}")
    
    # Encode image
    with open(image_path, 'rb') as f:
        image_base64 = base64.b64encode(f.read()).decode()
    
    # Prepare prompt
    prompt = """
    Analyze this Zwift racing game screenshot and identify the exact pixel coordinates 
    for each UI element. Return a JSON object with this exact structure:
    
    {
      "speed": {"x": ?, "y": ?, "width": ?, "height": ?},
      "distance": {"x": ?, "y": ?, "width": ?, "height": ?},
      "altitude": {"x": ?, "y": ?, "width": ?, "height": ?},
      "race_time": {"x": ?, "y": ?, "width": ?, "height": ?},
      "power": {"x": ?, "y": ?, "width": ?, "height": ?},
      "cadence": {"x": ?, "y": ?, "width": ?, "height": ?},
      "heart_rate": {"x": ?, "y": ?, "width": ?, "height": ?},
      "gradient": {"x": ?, "y": ?, "width": ?, "height": ?},
      "distance_to_finish": {"x": ?, "y": ?, "width": ?, "height": ?},
      "leaderboard": {"x": ?, "y": ?, "width": ?, "height": ?},
      "leaderboard_entry_height": ?,
      "rider_pose_avatar": {"x": ?, "y": ?, "width": ?, "height": ?}
    }
    
    Be precise with coordinates. The speed is in top left, distance in top center,
    power/cadence/HR on left side vertically, leaderboard on right side.
    """
    
    if provider == 'groq':
        regions = calibrate_with_groq(image_base64, prompt)
    else:
        regions = calibrate_with_ollama(image_path, prompt)
    
    # Create full config
    config = {
        "version": "1.67.0",  # Update as needed
        "resolution": f"{width}x{height}",
        "aspect_ratio": f"{width//gcd(width,height)}:{height//gcd(width,height)}",
        "created_by": "calibration_tool",
        "created_date": str(date.today()),
        "regions": regions,
        "notes": "Auto-calibrated, please verify"
    }
    
    # Save draft
    with open(output, 'w') as f:
        json.dump(config, f, indent=2)
    
    print(f"Draft config saved to {output}")
    print("Please review and test before submitting PR")

def calibrate_with_groq(image_base64, prompt):
    from groq import Groq
    client = Groq()
    
    response = client.chat.completions.create(
        model="llama-3.2-90b-vision-preview",
        messages=[{
            "role": "user",
            "content": [
                {"type": "text", "text": prompt},
                {"type": "image_url", "image_url": {
                    "url": f"data:image/png;base64,{image_base64}"
                }}
            ]
        }],
        response_format={"type": "json_object"}
    )
    
    return json.loads(response.choices[0].message.content)

def calibrate_with_ollama(image_path, prompt):
    import subprocess
    # Implementation for Ollama
    result = subprocess.run([
        'ollama', 'run', 'llama3.2-vision:11b',
        f'--image', image_path,
        prompt
    ], capture_output=True, text=True)
    
    # Parse JSON from output
    return json.loads(result.stdout)

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

if __name__ == '__main__':
    calibrate()
```

### Submission Process
1. Fork the repository
2. Create config file in `ocr-configs/` with naming: `{width}x{height}_v{version}.json`
3. Include in PR:
   - The config file
   - Screenshot showing detected regions (use debug mode)
   - Test results showing successful extraction
   - Your Zwift version and any special notes

### Testing Your Config
```bash
# Test with Rust OCR
cargo test --features ocr -- --test-config path/to/config.json

# Visual verification
python tools/ocr/test_config_visual.py path/to/config.json path/to/test/image.png
```

## Next Steps

1. **Immediate** (Today)
   - Create calibration scripts
   - Set up `ocr-configs/` directory
   - Calibrate your 1920x1080 config

2. **This Week**
   - Update Rust code to load configs
   - Create visual testing tools
   - Document PR process

3. **Community Launch**
   - Announce config contribution process
   - Create example PR
   - Build config library

## Conclusion

This community-driven approach with expert calibration provides:
- **Professional quality** configs from experienced users
- **Multiple methods** for different skill levels
- **Free tools** using cloud APIs or local models
- **Rigorous testing** before acceptance

By making calibration a one-time expert task, we ensure high-quality configs while keeping the runtime simple and fast for all users.