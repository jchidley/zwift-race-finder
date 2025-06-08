# Test Data Validation Guide

**Purpose**: Ensure our reduced test dataset remains representative of real-world data

## Overview

When we reduced our golden tests from 9,414 to 1,694 cases (82% reduction), we needed a way to ensure our focused test set still accurately represents the full range of routes and conditions. This validation process compares our test data against:

1. The full production database of 264+ routes
2. Jack's actual race history (151+ races)

## Running Validation

### Quick Method (Recommended)

```bash
# Run the validation script
./tools/utils/validate_test_data.sh
```

This script will:
- Check for production database
- Run route coverage analysis
- Compare duration estimates
- Validate against race history
- Optionally regenerate golden baseline

### Manual Method

```bash
# Validate route coverage and diversity
cargo test validate_test_routes -- --ignored --nocapture

# Validate against race history
cargo test validate_against_race_history -- --ignored --nocapture
```

## What Gets Validated

### 1. Route Coverage Analysis
- **Distance Range**: Ensures test routes cover min/max/average distances
- **Elevation Range**: Verifies variety of climbing from flat to mountain
- **Difficulty Distribution**: Checks elevation/distance ratios
- **Missing Routes**: Identifies test routes not in production database

### 2. Statistical Comparison
- **Duration Estimates**: Compares mean and standard deviation
- **Distribution Analysis**: P10/P50/P90 percentiles
- **Bias Detection**: Identifies if test set skews results

### 3. Race History Validation
- **Real Race Data**: Compares against Jack's 151 actual race results
- **Prediction Accuracy**: Calculates mean error for test routes
- **Coverage Check**: How many test routes have real race data

## Interpreting Results

### Good Results ✅
```
Statistical Comparison:
  All routes:  mean=45.2 min, std=18.3
  Test routes: mean=44.8 min, std=17.9
  Difference:  0.9% mean, 2.2% std dev
```
- Less than 10% difference in mean/std dev
- Test routes with race history: 8/11 (>70%)
- Mean prediction error < 20%

### Concerning Results ⚠️
```
Statistical Comparison:
  All routes:  mean=45.2 min, std=18.3
  Test routes: mean=38.1 min, std=12.4
  Difference:  15.7% mean, 32.2% std dev
```
- Over 10% difference suggests bias
- Missing key route types (very flat or very hilly)
- Low race history coverage

## When to Run Validation

1. **After Route Database Updates**
   - New routes added
   - Route data corrections
   - Major database changes

2. **Before Major Releases**
   - Ensures test suite remains valid
   - Catches drift over time

3. **When Accuracy Issues Arise**
   - If users report bad predictions
   - After algorithm changes

## Updating Test Data

If validation shows significant drift:

1. **Identify Gaps**
   ```
   Missing routes: ["Alpe du Zwift", "The Mega Pretzel"]
   Under-represented: Mountain routes (>1000m elevation)
   ```

2. **Update Test Routes**
   ```rust
   // In generate_baseline_improved.rs
   fn get_test_routes() -> Vec<&'static str> {
       vec![
           // ... existing routes ...
           "Alpe du Zwift",    // Add missing mountain route
           "The Mega Pretzel", // Add missing long route
       ]
   }
   ```

3. **Regenerate Baseline**
   ```bash
   cargo test generate_golden_baseline_improved -- --ignored
   git add tests/golden/baseline_improved_*.json
   git commit -m "test: update golden baseline with better route coverage"
   ```

## Automation Possibilities

While currently manual, this could be automated:

1. **CI Integration**: Run validation weekly
2. **Drift Alerts**: Notify if >10% difference detected
3. **Auto-Update**: Generate PR with new baseline if needed

## Summary

This validation ensures our efficiency gains (82% fewer tests) don't compromise accuracy. By periodically checking against real data, we maintain confidence that our focused test set truly represents the full application behavior.