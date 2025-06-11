# OCR Performance Truth Document

This document contains the definitive performance measurements for all OCR implementations based on our latest testing.

## Test Configuration
- **Image**: normal_1_01_16_02_21.jpg (1920x1080)
- **Fields Extracted**: 11 total
  - 7 core telemetry: speed, distance, altitude, race_time, power, cadence, heart_rate
  - 2 extra fields: gradient, distance_to_finish  
  - 1 leaderboard (with multiple entries)
  - 1 rider pose detection
- **Hardware**: 12-thread CPU

## Definitive Performance Numbers

### Python (PaddleOCR)
- **Full Extraction**: 4.77s
- **Accuracy**: 100% on all fields
- **Consistency**: Stable across runs

### Rust Sequential (v1.1)
- **Cold Start**: 0.96s (single run)
- **Warm Average**: 0.81s (benchmark)
- **Typical**: 0.88s (comparison script)
- **Accuracy**: 100% core fields, ~80% leaderboard names

### Rust Parallel (v1.3)
- **Cold Start**: 1.22s (27% slower than sequential)
- **Warm Average**: 0.52s (1.55x faster than sequential)
- **Break-even**: Need ~5 images for parallel to be worthwhile
- **Accuracy**: Same as sequential

## Speed Comparisons

### Single Image (Cold Start)
- Python: 4.77s (baseline)
- Rust Sequential: 0.88s → **5.4x faster**
- Rust Parallel: 1.14s → **4.2x faster**

### Batch/Video (Warm Start)
- Python: 4.77s (no warm-up benefit)
- Rust Sequential: 0.81s → **5.9x faster**
- Rust Parallel: 0.52s → **9.2x faster**

## Key Findings

1. **The 23x claim is wrong** - Maximum speedup is 9.2x (warm parallel)
2. **Parallel overhead is significant** - 250ms initialization penalty
3. **Sequential is best for CLI** - Faster for single images
4. **Parallel excels at batch** - 1.55x speedup after warm-up
5. **Python has no warm-up benefit** - Always ~4.77s

## When to Use Each

### Python (PaddleOCR)
- Need 100% leaderboard accuracy
- Prototyping new features
- Integration with Python ecosystem

### Rust Sequential (default)
- CLI tools
- Single image extraction
- Memory-constrained environments
- Simple integration

### Rust Parallel (--parallel flag)
- Video processing
- Batch image processing (>5 images)
- Real-time streaming
- Long-running services

## Implementation Notes

- All implementations extract the same 11 fields
- Rust uses hybrid approach: Tesseract for numbers, ocrs for leaderboard
- Parallel version uses rayon for field extraction, crossbeam for concurrent tasks
- OCRS neural network remains single-threaded (main bottleneck)