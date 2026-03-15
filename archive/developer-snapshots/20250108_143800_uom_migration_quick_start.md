# UOM Migration V2 Quick Start Guide
**Created**: 2025-01-08 14:38:00

## 🚀 TL;DR - Where We Are

**Status**: Framework complete, migration not started  
**Problem**: UOM v1 broke everything (0 races vs 10)  
**Solution**: Built behavioral preservation framework  
**Next Step**: Run mutation testing, then migrate first function

## 📋 Quick Checklist - What's Done

✅ **Testing Framework**
- 1,694 golden tests (was 9,414)
- Property-based tests
- A/B comparison framework
- Test data validation (<3% difference)

✅ **Code Refactoring**
- Display functions extracted
- Tests reorganized
- Better modularity

✅ **Documentation**
- Comprehensive guides
- Migration plan
- This quick start!

❌ **Not Done Yet**
- No UOM code written
- No functions migrated
- No benchmarks run

## 🎯 How to Start Migration

### 1. Run Mutation Testing First
```bash
# Install cargo-mutants if needed
cargo install cargo-mutants

# Run mutation testing
cargo mutants

# Analyze results
cat mutants.out/missed.txt
```

### 2. Add UOM Dependency
```toml
# Cargo.toml
[dependencies]
uom = "0.36"
```

### 3. Start With Simplest Function
```rust
// Pick calculate_pack_speed() in duration_estimation.rs
// It's pure, simple, well-tested

// Old version:
pub fn calculate_pack_speed(zwift_score: u32) -> f64 {
    // Returns km/h
}

// New version:
use uom::si::f64::*;
use uom::si::velocity::kilometer_per_hour;

pub fn calculate_pack_speed_uom(zwift_score: u32) -> Velocity {
    Velocity::new::<kilometer_per_hour>(speed_kmh)
}
```

### 4. Use A/B Testing
```rust
// In main code:
let test = ABTest {
    name: "calculate_pack_speed",
    old_impl: Box::new(|| calculate_pack_speed(250)),
    new_impl: Box::new(|| calculate_pack_speed_uom(250).get::<kilometer_per_hour>()),
    context: "Testing UOM migration".to_string(),
};

let result = test.run()?;
assert!(result.matches);
```

### 5. Validate With Golden Tests
```bash
# Re-run golden tests
cargo test golden

# Check for any failures
# All should pass if behavior preserved
```

## 📁 Key Files to Know

### Testing Infrastructure
```
tests/golden/
├── generate_baseline_improved.rs  # Creates golden tests
├── validate_test_data.rs         # Validates representativeness
└── baseline_improved_*.json      # Golden test data

tests/properties/
└── behavioral_invariants.rs      # Property tests

src/
├── ab_testing.rs                 # A/B comparison
└── compatibility.rs              # Tracking divergences
```

### Migration Candidates
```
src/
├── duration_estimation.rs        # Start here!
├── estimation.rs                 # Next target
└── models.rs                     # Data structures
```

## 🎓 Key Concepts

### Behavioral Preservation
> "Any difference with current behavior is a bug"

- Golden tests capture exact current behavior
- Property tests ensure mathematical relationships
- A/B tests compare implementations

### Test Data Quality
- Started with 9,414 tests (too many!)
- Reduced to 1,694 representative tests
- Validated: <3% statistical difference
- No database dependency

### Migration Strategy
1. Function by function (not file by file)
2. Keep both versions temporarily
3. A/B test in production
4. Remove old version when confident

## ⚠️ Common Pitfalls

### Don't Do This
```rust
// ❌ Changing behavior during migration
pub fn calculate_speed_uom(score: u32) -> Velocity {
    // Oops, "fixed" a bug while migrating
    Velocity::new::<kilometer_per_hour>(score as f64 * 0.124 + 5.0)
}
```

### Do This Instead
```rust
// ✅ Preserve exact behavior, even bugs
pub fn calculate_speed_uom(score: u32) -> Velocity {
    let kmh = calculate_pack_speed(score); // Use existing logic
    Velocity::new::<kilometer_per_hour>(kmh)
}
```

## 🔍 Validation Commands

```bash
# Run all behavioral tests
cargo test

# Run golden tests specifically
cargo test golden

# Validate test data quality
./tools/utils/validate_test_data.sh

# Run property tests
cargo test property

# Check specific function
cargo test test_calculate_pack_speed
```

## 📊 Success Metrics

- **Golden Tests**: 100% must pass
- **Property Tests**: 100% must pass
- **Performance**: Within 5% of original
- **Binary Size**: < 10% increase acceptable

## 🚦 Go/No-Go Decision Points

### Before Each Function Migration
1. ✓ Mutation testing shows good coverage?
2. ✓ Golden tests exist for function?
3. ✓ Property tests defined?
4. ✓ A/B test framework ready?

### After Migration
1. ✓ All tests still pass?
2. ✓ No behavioral divergences?
3. ✓ Performance acceptable?
4. ✓ Code still readable?

## 💡 Pro Tips

1. **Start Small**: Pick the simplest function first
2. **Test Everything**: Every behavior change is a bug
3. **Document Surprises**: If something seems wrong, investigate
4. **Ask Questions**: The framework is here to help
5. **Trust the Tests**: They define correct behavior

## 🎉 You're Ready!

The framework is built, tested, and validated. All that's left is the actual migration. Good luck!

Remember: **Current behavior is sacred**. Any difference is a bug.