# Rust OCR Implementation TODO

## ⚠️ This file is outdated!

**Please see [RUST_IMPLEMENTATION_STATUS.md](RUST_IMPLEMENTATION_STATUS.md) for current implementation status.**

This file documents the original v1.0 implementation and is kept for historical reference.

---

## Original v1.0 Status

The initial Rust implementation (`src/ocr_compact.rs`) successfully extracted basic telemetry:
- ✅ **speed** (u32) - Current speed in km/h
- ✅ **distance** (f64) - Distance covered in km
- ✅ **altitude** (u32) - Current altitude in meters
- ✅ **race_time** (String) - Elapsed time in MM:SS format
- ✅ **power** (u32) - Current power output in watts
- ✅ **cadence** (u32) - Pedaling cadence in RPM
- ✅ **heart_rate** (u32) - Heart rate in BPM
- ✅ **gradient** (f64) - Current slope percentage (e.g., 3.0%)
- ✅ **distance_to_finish** (f64) - Remaining distance in km

**Performance**: 1.08s extraction (4.8x faster than Python's 5.15s)

## ❌ Remaining Feature

### 1. Leaderboard Extraction (Complex - Low Priority)
**Python implementation details:**
- Region: `(1500, 200, 420, 600)`
- Complex multi-step process:
  1. Apply CLAHE enhancement for contrast
  2. OCR entire region
  3. Sort detections by Y coordinate
  4. Identify rider names (contain dots or start with uppercase)
  5. Find associated data below each name (delta, km, w/kg)
  6. Mark current rider (has green box indicator)

**Data structure needed:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub current: bool,      // Is this the current rider?
    pub delta: Option<String>,  // Time delta (e.g., "+2:20")
    pub km: Option<f64>,        // Distance covered
    pub wkg: Option<f64>,       // Watts per kilogram
}
```

**Rust implementation challenges:**
- CLAHE algorithm for contrast enhancement
- Complex text detection and grouping logic
- Multiple OCR passes for different regions
- Heuristic-based name detection

## Implementation Notes

### ✅ Completed Implementations

#### Gradient Extraction (Completed)
- **Region**: `(1695, 71, 50, 50)`
- **Special processing**: Yellow/orange text on dark background
- **Technique**: Threshold at 150 (no color inversion), 4x scaling, page seg mode 7
- **Performance**: Works reliably with stylized gradient font

#### Distance to Finish (Completed) 
- **Region**: `(1143, 138, 50, 27)`
- **Processing**: Lower threshold (150) for dimmer text, standard decimal extraction
- **Performance**: Accurate extraction of remaining race distance

### 🚧 Future Implementation: Leaderboard (v2.0)

See [README.md](README.md) for current enhancement roadmap and implementation details.

## Current Status Summary

**Production Ready**: ✅ Rust implementation covers 90% of telemetry needs  
**Missing Only**: Leaderboard extraction (advanced feature)  
**Performance**: 4.8x faster than Python for core telemetry (1.08s vs 5.15s)  
**Recommendation**: Use Rust for faster batch processing, Python when leaderboard analysis needed