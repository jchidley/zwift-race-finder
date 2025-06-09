# OCR Module Refactoring Plan

## Overview
This plan follows the mechanical refactoring rules from REFACTORING_RULES.md and Rust-specific guidelines to improve the OCR modules while preserving exact behavior.

## Code Analysis

### Identified Issues

1. **Duplication Between Modules**
   - Image preprocessing logic (grayscale, threshold, scale) in both modules
   - Similar pattern for converting images to PNG format for OCR
   - Repeated region extraction patterns

2. **Magic Numbers**
   - Hardcoded coordinates: (693, 44, 71, 61), etc.
   - Threshold values: 150, 200
   - Scale factors: 3x, 4x
   - Pose detection thresholds: 1.7, 0.45, etc.

3. **Non-Idiomatic Rust**
   - `Result<Option<T>>` where `Option<T>` would suffice
   - Multiple regex compilations on each call
   - `.unwrap()` calls on regex operations
   - String allocations that could be avoided

4. **Large Functions**
   - `extract_telemetry`: 50+ lines with multiple responsibilities
   - `extract_leaderboard`: Complex parsing logic mixed with OCR
   - `calculate_pose_features`: Dense calculation logic

## Refactoring Steps (Following REFACTORING_RULES.md)

### Phase 1: Extract Constants (Mechanical)

**Type**: Extract Variable
**Rule**: Extract expressions as-is, no modifications

1. Create constants module:
   ```rust
   mod constants {
       pub const SPEED_REGION: (u32, u32, u32, u32) = (693, 44, 71, 61);
       pub const DISTANCE_REGION: (u32, u32, u32, u32) = (833, 44, 84, 55);
       // ... etc
       
       pub const DEFAULT_THRESHOLD: u8 = 200;
       pub const DIM_TEXT_THRESHOLD: u8 = 150;
       pub const DEFAULT_SCALE_FACTOR: u32 = 3;
       pub const GRADIENT_SCALE_FACTOR: u32 = 4;
   }
   ```

2. Replace magic numbers with constants (copy-paste exact values)

### Phase 2: Extract Common Functions (Mechanical)

**Type**: Extract Function
**Rule**: Copy code exactly, no rewrites

1. Extract image preprocessing:
   ```rust
   // In a new module: image_processing.rs
   pub fn preprocess_for_ocr(
       roi: &DynamicImage,
       threshold_value: u8,
       scale_factor: u32
   ) -> Result<Vec<u8>>
   ```
   - Copy exact code from `extract_field` and `extract_gradient`
   - No modifications to logic

2. Extract PNG conversion:
   ```rust
   pub fn to_png_bytes(image: &DynamicImage) -> Result<Vec<u8>>
   ```

### Phase 3: Lazy Static Regex (Mechanical)

**Type**: Extract Variable + Performance
**Rule**: Initialize once, reuse

1. Add lazy_static dependency
2. Create regex module:
   ```rust
   lazy_static! {
       static ref TIME_REGEX: Regex = Regex::new(r"(\d{1,2}:\d{2})").unwrap();
       static ref DELTA_REGEX: Regex = Regex::new(r"([+-]\d{1,2}:\d{2})").unwrap();
       // ... etc
   }
   ```

### Phase 4: Error Handling (Mechanical)

**Type**: Change Function Declaration
**Rule**: Migration method

1. Create safe versions alongside unsafe:
   ```rust
   fn is_likely_name_safe(text: &str) -> Result<bool> {
       // Same logic but with ? instead of unwrap()
   }
   ```

2. Migrate callers one by one
3. Delete old versions

### Phase 5: Simplify Types (Mechanical)

**Type**: Change Function Declaration
**Rule**: Only where tests allow

1. Functions returning `Result<Option<T>>` where error is never used:
   - Change to `Option<T>`
   - Update all callers
   - Only if tests still pass

### Phase 6: Extract Smaller Functions (Mechanical)

**Type**: Extract Function
**Rule**: Copy exact code blocks

1. From `extract_telemetry`:
   - `extract_standard_fields()`
   - `extract_special_fields()`

2. From `extract_leaderboard`:
   - `parse_leaderboard_lines()`
   - `create_leaderboard_entry()`

3. From `calculate_pose_features`:
   - `calculate_bounding_box()`
   - `calculate_density_metrics()`
   - `calculate_symmetry_score()`

## Testing Strategy

### CRITICAL FINDING: No existing tests for OCR modules!

The OCR modules (`ocr_compact.rs` and `ocr_ocrs.rs`) have **NO TEST COVERAGE**. According to REFACTORING_RULES.md, we MUST add characterization tests before any refactoring can begin.

1. **Before ANY refactoring** (MANDATORY):
   - Create comprehensive characterization tests
   - Test with sample images covering all functionality
   - Document expected behavior for each function
   - Verify tests pass with current implementation

2. **After EACH refactoring step**:
   - Run all tests
   - Verify no behavior changes
   - Commit atomically

3. **Regression prevention**:
   - Keep original functions temporarily
   - A/B test refactored vs original
   - Only delete originals after validation

### Characterization Tests Needed

1. **ocr_compact module**:
   - `extract_telemetry()` with various screenshot types
   - `extract_field()` for each field type
   - `parse_time()` with various time formats
   - `extract_gradient()` with different gradient values
   - `extract_leaderboard()` with different leaderboard states
   - `extract_rider_pose()` for each pose type
   - `is_likely_name()` with edge cases
   - `parse_leaderboard_data()` with various formats

2. **ocr_ocrs module**:
   - `create_engine()` initialization
   - `extract_text_from_region()` with various regions

## Implementation Order

1. ~~Review test coverage~~ ✓ COMPLETE - NO TESTS FOUND
2. **Add characterization tests** (MUST be done before ANY refactoring)
3. Extract constants (lowest risk)
4. Extract common image processing
5. Implement lazy_static regex
6. Fix error handling
7. Simplify types (if safe)
8. Extract smaller functions
9. Final test run

## Success Criteria

- All existing tests pass without modification
- No changes to public API
- No changes to output format
- Performance equal or better
- Code follows Rust idioms

## Risks and Mitigations

1. **Risk**: Changing OCR accuracy
   - **Mitigation**: Keep exact same preprocessing parameters
   
2. **Risk**: Breaking API compatibility
   - **Mitigation**: Only internal refactoring, no public changes
   
3. **Risk**: Performance regression
   - **Mitigation**: Benchmark before/after each change

## Next Steps

1. Check test coverage with `cargo test ocr`
2. Begin Phase 1: Extract Constants
3. Commit after each successful phase