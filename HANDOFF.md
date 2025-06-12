# HANDOFF.md - Zwift Race Finder

## Project Status - 2025-01-12

### Current Branch: main

OCR strategy redesigned from cloud-based validation to community-driven configuration approach.

### Recent Work Completed

1. **OCR Strategy Overhaul**
   - Researched vision LLM APIs (Groq, HuggingFace, Together AI)
   - Pivoted to community-maintained config files
   - Focus shifted from perfect name OCR to rider order tracking
   - Created comprehensive calibration documentation

2. **Documentation Created**
   - `wip-claude/20250112_100000_ocr_comprehensive_strategy.md` - Complete strategy
   - `tools/ocr/CALIBRATION_GUIDE.md` - Step-by-step calibration guide
   - Updated OCR README with `record-monitor2.ps1` acquisition method

3. **Key Insights**
   - UI regions stable per Zwift version/resolution
   - Community configs eliminate per-user calibration
   - Fuzzy matching sufficient for rider tracking

### Current State

**Status**: OCR strategy documented, ready for implementation
**Target**: Create first community config, implement config loader
**Latest**: Community-driven approach with free API calibration tools

### Active Todo List

[ ] Create initial 1920x1080 config from recordings
[ ] Create calibration script with Groq support
[✓] Create calibration instructions for contributors
[✓] Update OCR strategy with calibration guide

### Next Step

**Implement OCR Configuration System**:
1. Create `ocr-configs/` directory structure
2. Build calibration script (`calibrate_with_vision.py`)
3. Generate first config from your 1920x1080 recordings
4. Test with existing Rust OCR implementation

### Key Principle

> "Any difference with current behavior is a bug"

The golden tests define the specification. All must pass after migration.

### Recent Commits

- `01aad09` - docs: add comprehensive UOM migration V2 documentation
- `162a261` - fix: add public database methods for test data validation
- `268bd21` - feat: add test data validation against production database
- `09da41c` - refactor: improve golden test strategy and reduce test count by 82%

### Test Commands

```bash
# Run all tests
cargo test

# Run golden tests specifically  
cargo test golden

# Validate test data quality
./tools/utils/validate_test_data.sh

# Run property tests
cargo test property

# Generate new golden baseline (if needed)
cargo test generate_golden_baseline_improved -- --ignored
```

### Files to Review

**Framework Code:**
- `src/ab_testing.rs` - A/B comparison framework
- `src/compatibility.rs` - Behavioral tracking
- `tests/golden/generate_baseline_improved.rs` - Optimized test generator
- `tests/golden/validate_test_data.rs` - Test validation
- `tests/properties/behavioral_invariants.rs` - Property tests

**Documentation:**
- `docs/development/20250108_143800_uom_migration_quick_start.md` - Start here!
- `docs/development/UOM_MIGRATION_PLAN_V2_REVISED.md` - Full plan
- `docs/development/BEHAVIORAL_PRESERVATION_TESTING.md` - Philosophy

### Key Metrics

- Test reduction: 82% (9,414 → 1,694)
- Statistical accuracy: <3% difference from production
- Framework components: 5 major systems
- Documentation: 10+ comprehensive guides

### Contact

For questions about the migration approach, see the comprehensive documentation in `docs/development/`.

---

## Previous Session Summary (2025-01-06)

Successfully fixed regression tests, cleaned database, and created comprehensive mutation testing documentation.

### What Was Done Previously

1. **Fixed Regression Tests** - Optimized queries, tests complete in ~1.2 seconds
2. **Cleaned Database** - Removed 7,460 duplicate test entries
3. **Organized Documentation** - Moved files to appropriate subdirectories
4. **Created Mutation Testing Guide** - Comprehensive guide with industry research

### Commands for Testing

```bash
# Run optimized regression tests
cargo test test_race_predictions_accuracy --lib

# Run full regression suite (slower)
cargo test test_all_race_predictions_accuracy --lib -- --ignored

# Check mutation testing on specific module
cargo mutants --file src/duration_estimation.rs

# View organized documentation
ls docs/development/
```