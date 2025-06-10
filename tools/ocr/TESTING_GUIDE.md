# OCR Testing Guide

This guide provides comprehensive information about testing the OCR module in the Zwift Race Finder project.

## Testing Philosophy

The OCR module follows the project's 70% testing philosophy:
- **70% Rule**: Focus on high-impact tests that catch most issues with reasonable effort
- **Natural > Contrived**: Test real-world scenarios over artificial edge cases
- **Behavioral > Structural**: Test outcomes, not implementation details
- **Pragmatic Approach**: Good enough is better than perfect

## Test Types and Coverage

### 1. Property-Based Tests
**File**: `tests/ocr_property_tests.rs`
**Purpose**: Validate parsing functions with wide input ranges

Key properties tested:
- `parse_time`: Various time formats (MM:SS, M:SS, invalid)
- `is_likely_name`: Length boundaries, special characters, Unicode
- `parse_leaderboard_data`: Numeric ranges, unit combinations

**Running**:
```bash
cargo test ocr_property_tests
```

### 2. Snapshot Tests
**File**: `tests/ocr_snapshot_tests.rs`
**Purpose**: Detect behavioral changes via exact output comparison

Coverage:
- Parsing function variations
- Edge cases (empty strings, max lengths)
- Full telemetry structures

**Running**:
```bash
cargo test ocr_snapshot_tests

# Review snapshot changes
cargo insta review
```

### 3. Integration Tests
**File**: `tests/ocr_integration_tests.rs`
**Purpose**: Test full OCR pipeline with real images

Features:
- Golden test framework with configurable tolerances
- Support for generating baseline data
- Comparison of actual vs expected telemetry

**Running**:
```bash
cargo test ocr_integration_tests

# Generate new golden data
cargo test --features generate-golden
```

### 4. Performance Benchmarks
**File**: `benches/ocr_benchmarks.rs`
**Purpose**: Establish and monitor performance baselines

**Running**:
```bash
cargo bench ocr

# Compare with baseline
cargo bench ocr -- --baseline main
```

### 5. Fuzz Tests
**File**: `tests/ocr_fuzz_tests.rs`
**Purpose**: Find edge cases and ensure no panics

Coverage:
- Arbitrary Unicode input
- Long strings (up to 10,000 chars)
- Special characters and control codes
- Numeric edge cases

**Running**:
```bash
cargo test ocr_fuzz_tests

# Run with more iterations
PROPTEST_CASES=10000 cargo test ocr_fuzz_tests
```

## Performance Baselines

Current performance benchmarks (as of 2025-01-09):

| Operation | Performance | Description |
|-----------|-------------|-------------|
| `parse_time` | 138-186 ns | Time string parsing |
| `is_likely_name` | 55-150 ns | Name validation |
| `parse_leaderboard_data` | 365-748 ns | Complex field parsing |
| Image preprocessing | 1.44-25.69 ms | Scales with image size |
| Full extraction | 866 ms | HD image (1920x1080) |
| Regex operations | 48-238 ns | Pre-compiled patterns |

Key insight: ~0.32 μs per pixel for preprocessing

## Known Issues and Edge Cases

### 1. Distance Parsing Ambiguity
- **Input**: "+00:01 38.1 km"
- **Parsed**: 138.1 km (instead of 38.1 km)
- **Cause**: Time delta "1" concatenates with distance
- **Impact**: Low - unlikely in real OCR output

### 2. Zero Distance Edge Case
- **Input**: "0.0 km"
- **Parsed**: Sometimes 10.0 km or unparsed
- **Impact**: Low - edge case

These are documented but not fixed as they're unlikely to occur in practice (70% rule).

## Key Discoveries During Testing

1. **Name Validation**: Names can contain any printable characters, not just letters
2. **W/kg Range**: 0.0 is valid (rider stopped/coasting), not just 0.5-7.0
3. **Float Precision**: OCR values are only precise to 0.1
4. **Mutation Effectiveness**: Snapshot tests successfully catch logic inversions

## Running All Tests

```bash
# Quick test suite (unit + integration)
cargo test

# Full test suite including slow tests
cargo test --all-features

# Benchmarks
cargo bench ocr

# Mutation testing (if time permits)
cargo mutants --file src/ocr_compact.rs --timeout 180

# Coverage report
cargo tarpaulin --out Html
```

## Maintaining Tests

### When to Add Tests
- New parsing logic or field types
- Bug reports from production
- Performance optimizations (benchmark before/after)

### When NOT to Add Tests
- Contrived edge cases that won't occur in practice
- Implementation details (internal helper functions)
- Chasing 100% coverage metrics

### Updating Golden Data
When OCR models improve or behavior changes:
```bash
# Generate new baseline
cargo test generate_golden_baseline -- --test-threads=1

# Review and commit new golden files
git diff tests/golden/
git add tests/golden/*.json
```

### CI Integration
Recommended CI pipeline:
```yaml
test:
  - cargo test
  - cargo bench -- --quick
  
nightly:
  - cargo test --all-features
  - PROPTEST_CASES=10000 cargo test ocr_fuzz_tests
  - cargo mutants --file src/ocr_compact.rs
```

## Troubleshooting

### Snapshot Test Failures
```bash
# Review snapshot changes interactively
cargo insta review

# Accept all changes
cargo insta accept
```

### Flaky Integration Tests
- Check tolerance configurations
- Verify test image quality
- Consider OCR engine variations

### Performance Regressions
```bash
# Compare with saved baseline
cargo bench ocr -- --baseline main

# Profile specific benchmark
cargo bench --bench ocr_benchmarks -- --profile-time=10
```

## Test Quality Metrics

Current state (as of 2025-01-09):
- **Mutation Score**: High for parsing functions (manual verification)
- **Fuzz Test Results**: 13/14 passing, 1 known edge case
- **Benchmark Stability**: < 5% variance between runs
- **Snapshot Coverage**: All major parsing functions covered

## Further Reading

- Project testing philosophy: `docs/development/MODERN_TESTING_STRATEGY.md`
- Mutation testing guide: `docs/development/MUTATION_TESTING_GUIDE.md`
- OCR implementation details: `tools/ocr/README.md`