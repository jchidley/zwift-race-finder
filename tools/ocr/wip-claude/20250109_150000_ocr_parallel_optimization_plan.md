# OCR Parallel Optimization Plan

## Executive Summary

Current Rust OCR implementation takes 3.53s for full extraction (v1.1 hybrid). Through parallelization and optimization, we can achieve significant speedup:
- **Target**: <1s for full extraction (3.5x improvement)
- **Strategy**: Parallel field extraction, engine caching, and pipeline optimization

## Current Performance Bottlenecks

### 1. Sequential Field Extraction (0.19s)
```rust
// Current: Fields extracted one by one
for (field, x, y, width, height) in regions {
    match *field {
        "gradient" => data.gradient = extract_gradient(...),
        _ => {
            let value = extract_field(...);
            match *field { ... }
        }
    }
}
```

### 2. ocrs Engine Initialization (3.34s overhead)
```rust
// Current: Engine created for each leaderboard extraction
pub fn extract_text_from_region(...) -> Result<String> {
    let engine = create_engine()?; // ~200ms model loading
    // ...
}
```

### 3. Tesseract Instance Management
- Single Tesseract instance used sequentially
- Character whitelist set repeatedly
- No parallelism across regions

### 4. Image Preprocessing
- Each region preprocessed sequentially
- No pipeline between preprocessing and OCR
- Redundant operations for similar regions

## Optimization Strategies

### 1. Parallel Field Extraction using Rayon

**Approach**: Process all telemetry fields concurrently
```rust
use rayon::prelude::*;

fn extract_standard_fields_parallel(
    data: &mut TelemetryData,
    img: &DynamicImage,
    regions: &[(&str, u32, u32, u32, u32)],
) -> Result<()> {
    // Extract all fields in parallel
    let results: Vec<_> = regions
        .par_iter()
        .map(|(field, x, y, width, height)| {
            match *field {
                "gradient" => {
                    let result = extract_gradient(img, *x, *y, *width, *height)?;
                    Ok((*field, FieldValue::Gradient(result)))
                }
                _ => {
                    let roi = img.crop_imm(*x, *y, *width, *height);
                    let value = extract_field_with_own_ocr(&roi, field)?;
                    Ok((*field, FieldValue::Text(value)))
                }
            }
        })
        .collect();
    
    // Assign results to data struct
    for result in results {
        match result? {
            (field, value) => assign_field(data, field, value),
        }
    }
    Ok(())
}
```

**Expected Improvement**: 9 fields processed in ~0.05s (4x speedup)

### 2. OCR Engine Caching with once_cell

**Approach**: Initialize engines once and reuse
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Cached ocrs engine
static OCRS_ENGINE: Lazy<Mutex<OcrEngine>> = Lazy::new(|| {
    Mutex::new(create_engine().expect("Failed to create OCRS engine"))
});

// Pool of Tesseract instances
static TESSERACT_POOL: Lazy<Mutex<Vec<LepTess>>> = Lazy::new(|| {
    let mut pool = Vec::with_capacity(8);
    for _ in 0..8 {
        let mut ocr = LepTess::new(None, "eng").expect("Failed to init Tesseract");
        ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")
            .expect("Failed to set whitelist");
        pool.push(ocr);
    }
    Mutex::new(pool)
});
```

**Expected Improvement**: Eliminate 200ms model loading per extraction

### 3. Concurrent Leaderboard and Telemetry

**Approach**: Extract leaderboard while processing telemetry
```rust
use std::thread;

pub fn extract_telemetry_parallel(image_path: &Path) -> Result<TelemetryData> {
    let img = image::open(image_path)?;
    let img_clone = img.clone();
    
    // Start leaderboard extraction in background
    let leaderboard_handle = thread::spawn(move || {
        extract_leaderboard(&img_clone)
    });
    
    // Extract telemetry fields in parallel
    let mut data = TelemetryData::default();
    extract_standard_fields_parallel(&mut data, &img, &REGIONS)?;
    
    // Extract pose (lightweight, do after telemetry)
    data.rider_pose = extract_rider_pose(&img)?;
    
    // Wait for leaderboard
    data.leaderboard = leaderboard_handle.join()
        .map_err(|_| anyhow::anyhow!("Leaderboard thread panicked"))??;
    
    Ok(data)
}
```

**Expected Improvement**: Hide leaderboard latency behind telemetry processing

### 4. Pipeline Architecture for Batch Processing

**Approach**: Process multiple images with pipeline stages
```rust
use crossbeam::channel::{bounded, Sender, Receiver};

struct OcrPipeline {
    // Stage 1: Image loading
    loader: Sender<PathBuf>,
    
    // Stage 2: Preprocessing
    preprocessor: Receiver<DynamicImage>,
    
    // Stage 3: OCR extraction
    extractor: Receiver<PreprocessedData>,
    
    // Stage 4: Results
    results: Receiver<TelemetryData>,
}

impl OcrPipeline {
    fn process_batch(paths: Vec<PathBuf>) -> Vec<TelemetryData> {
        // Create bounded channels for backpressure
        let (load_tx, load_rx) = bounded(4);
        let (prep_tx, prep_rx) = bounded(4);
        let (extract_tx, extract_rx) = bounded(4);
        
        // Spawn pipeline stages
        thread::spawn(move || loader_stage(load_rx, prep_tx));
        thread::spawn(move || preprocessor_stage(prep_rx, extract_tx));
        thread::spawn(move || extractor_stage(extract_rx));
        
        // Feed images
        for path in paths {
            load_tx.send(path).unwrap();
        }
        
        // Collect results
        extract_rx.iter().collect()
    }
}
```

**Expected Improvement**: 10x throughput for batch processing

### 5. SIMD Optimizations for Preprocessing

**Approach**: Use SIMD for threshold operations
```rust
use std::simd::*;

fn threshold_simd(gray: &GrayImage, threshold: u8) -> GrayImage {
    let pixels = gray.as_raw();
    let mut output = vec![0u8; pixels.len()];
    
    // Process 16 pixels at a time with SIMD
    let chunks = pixels.chunks_exact(16);
    let remainder = chunks.remainder();
    let threshold_vec = u8x16::splat(threshold);
    let white = u8x16::splat(255);
    let black = u8x16::splat(0);
    
    for (i, chunk) in chunks.enumerate() {
        let pixels = u8x16::from_slice(chunk);
        let mask = pixels.simd_gt(threshold_vec);
        let result = mask.select(white, black);
        result.copy_to_slice(&mut output[i*16..(i+1)*16]);
    }
    
    // Handle remainder
    for (i, &pixel) in remainder.iter().enumerate() {
        output[pixels.len() - remainder.len() + i] = 
            if pixel > threshold { 255 } else { 0 };
    }
    
    GrayImage::from_raw(gray.width(), gray.height(), output).unwrap()
}
```

**Expected Improvement**: 2-4x faster preprocessing

## Implementation Plan

### Phase 1: Core Parallelization (Priority)
1. Implement Tesseract pool for parallel field extraction
2. Cache ocrs engine instance
3. Use rayon for parallel field processing
4. Expected: 3.53s → ~1.5s

### Phase 2: Advanced Optimizations
1. Implement concurrent leaderboard/telemetry extraction
2. Add SIMD preprocessing
3. Create pipeline for batch processing
4. Expected: 1.5s → <1s

### Phase 3: Future Enhancements
1. GPU acceleration for preprocessing (optional)
2. Custom OCR model trained on Zwift UI
3. Adaptive region detection
4. Real-time streaming support

## Dependencies to Add

```toml
[dependencies]
rayon = "1.7"          # Data parallelism
once_cell = "1.19"     # Lazy static initialization
crossbeam = "0.8"      # Advanced concurrency primitives

[dev-dependencies]
criterion = "0.5"      # Benchmarking
```

## Benchmarking Plan

1. Create micro-benchmarks for each component
2. Test with various image resolutions
3. Measure memory usage and thread contention
4. Profile with `perf` and `flamegraph`

## Risk Mitigation

1. **Thread Safety**: Use Arc<Mutex<>> for shared OCR engines
2. **Memory Usage**: Limit thread pool size and channel buffers
3. **Error Handling**: Ensure parallel errors are properly propagated
4. **Compatibility**: Maintain single-threaded fallback option

## Expected Final Performance

| Component | Current | Optimized | Improvement |
|-----------|---------|-----------|-------------|
| Telemetry (9 fields) | 0.19s | 0.05s | 3.8x |
| Leaderboard (ocrs) | 3.34s | 0.8s | 4.2x |
| Total | 3.53s | 0.85s | 4.2x |

## Conclusion

Through systematic parallelization and caching, we can achieve <1s full extraction time, making the Rust implementation 12x faster than Python while maintaining accuracy. The implementation is straightforward using Rust's excellent concurrency primitives.