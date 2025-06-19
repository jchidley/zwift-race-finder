# Rust Code Coverage as Discovery Tool Plan
**Date**: January 6, 2025
**Revision**: Based on insight that test coverage reveals unused code

## Executive Summary

This plan reframes code coverage from a quality metric to a **discovery tool**. By pursuing 100% test coverage, we force examination of every code path, naturally revealing unused, overcomplicated, or unnecessary code. The process of writing tests becomes an investigation into code necessity.

## Core Insight

"It seems difficult to get LLMs to analyse the code and identify parts of it that are unused. One way is to use tools get 100% test coverage and then understand what these tools are testing and the relevance of the thing being tested." - Jack

## Goals (Revised)

1. **Use 100% coverage target as a forcing function** to examine all code
2. **Discover unused/dead code** through the difficulty of testing it
3. **Document why untested code remains** (if any)
4. **Eliminate code that only exists to be tested**
5. **Create sustainable testing practices** that prevent dead code accumulation

## Philosophy Shift

### Old Approach
- Coverage is a quality metric
- 80% is "good enough"
- Dead code detection is a separate activity
- Tests prove code works

### New Approach
- Coverage is a discovery mechanism
- 100% forces examination of everything
- Dead code reveals itself through contrived tests
- Tests question code's existence

## Implementation Strategy

### Phase 1: Discovery Through Testing (Weeks 1-2)

#### Setup
```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

#### Discovery Protocol
For each uncovered function/module/path:

1. **Generate coverage report**
   ```bash
   cargo llvm-cov --html
   # Open target/llvm-cov/html/index.html
   ```

2. **Pick an uncovered item and ask:**
   - What is this code's purpose?
   - What production code calls this?
   - Is this only used in tests?
   - Is this defensive code that never triggers?

3. **Write a test and evaluate:**
   ```rust
   #[test]
   fn test_suspicious_function() {
       // DISCOVERY: This test feels contrived because...
       // TODO: Investigate if this function is actually needed
   }
   ```

4. **Document discoveries inline:**
   ```rust
   // DISCOVERY NOTES:
   // - Only called by other unused functions
   // - Test required complex mocking to reach
   // - Appears to be legacy code from removed feature
   // RECOMMENDATION: Remove in next refactor
   ```

### Phase 2: Categorize Findings (Week 2)

Create a discovery report categorizing all code:

#### 1. **Essential Code**
- Clear purpose, natural tests
- Used by multiple call sites
- Core business logic

#### 2. **Defensive Code**
- Error handlers for "impossible" states
- Backward compatibility code
- Document why it stays despite no coverage

#### 3. **Questionable Code**
- Only one caller
- Tests feel contrived
- Complex to test for minimal value

#### 4. **Dead Code**
- Never called except in tests
- Legacy from removed features
- Overengineered abstractions

### Phase 3: Elimination Sprint (Week 3)

Based on discoveries:

1. **Remove confirmed dead code**
   ```bash
   git checkout -b remove-dead-code
   # Remove one category at a time
   # Run tests after each removal
   ```

2. **Simplify questionable code**
   - Inline single-use functions
   - Remove unnecessary abstractions
   - Merge overlapping functionality

3. **Document defensive code**
   ```rust
   #[allow(dead_code)]
   // KEEP: This handles database corruption scenarios
   // While rare, the consequence of not handling is data loss
   fn recover_corrupted_data() { ... }
   ```

### Phase 4: Sustainable Practices (Week 4)

Establish ongoing practices:

1. **Coverage-Driven Development (CDD)**
   ```markdown
   1. Write feature code
   2. Run coverage report
   3. For each uncovered line, ask: "Is this needed?"
   4. Either test it or remove it
   5. Document why any code remains uncovered
   ```

2. **PR Checklist Update**
   - [ ] All new code has tests
   - [ ] No coverage decrease
   - [ ] Contrived tests are documented
   - [ ] Dead code discovered is removed

3. **Monthly Discovery Sessions**
   - Review coverage reports
   - Question untested code
   - Remove accumulated cruft

## Test Quality Indicators

### Signs of Natural Tests
```rust
#[test]
fn calculates_route_duration() {
    // Testing actual business requirement
    let route = Route::new(10.0, 100.0);
    assert_eq!(route.estimate_duration(200), Duration::minutes(25));
}
```

### Signs of Contrived Tests (Potential Dead Code)
```rust
#[test]
fn test_internal_helper() {
    // Have to construct artificial scenario
    let mut internal_state = HashMap::new();
    internal_state.insert("key", "value");
    
    // Function only used by this test
    assert_eq!(format_internal_state(&internal_state), "key=value");
    // DISCOVERY: This helper has no production callers
}
```

## Metrics and Reporting

### Traditional Metrics (De-emphasized)
- Line coverage: Target 100% minus documented exceptions
- Branch coverage: Target 100% minus documented exceptions

### Discovery Metrics (New)
- **Dead Code Found**: Lines removed per sprint
- **Test Contrivance Score**: Ratio of natural to contrived tests
- **Documentation Debt**: Uncovered code without justification
- **Simplification Rate**: Complex functions simplified

### Sample Discovery Report
```markdown
## Coverage Discovery Report - Sprint 2025-01

### Statistics
- Total functions: 127
- Covered: 98 (77%)
- Uncovered: 29 (23%)

### Discoveries
- Dead code removed: 456 lines (12 functions)
- Contrived tests written: 8 (flagged for refactor)
- Defensive code documented: 5 functions
- Simplifications: 3 abstractions removed

### Uncovered Code Justifications
1. `panic_handler()` - Only triggers on memory corruption
2. `migration_v1_to_v2()` - Kept for users with old databases
3. `debug_dump()` - Developer tool, not for production
```

## Special Considerations for zwift-race-finder

### Common Dead Code Patterns to Check

1. **Over-abstracted API Clients**
   ```rust
   // Often found: Generic API client that only makes one type of call
   impl ApiClient {
       fn post(&self) -> Result<Response> { } // Never used
       fn put(&self) -> Result<Response> { }  // Never used
       fn delete(&self) -> Result<Response> { } // Never used
       // Only get() is actually used
   }
   ```

2. **Premature Configuration**
   ```rust
   // Config options that were added "just in case"
   pub struct Config {
       pub real_option: String,
       pub unused_option: bool, // DISCOVERY: Never read
       pub future_feature: Option<String>, // DISCOVERY: Always None
   }
   ```

3. **Legacy Route Calculations**
   ```rust
   // Old estimation methods kept "for comparison"
   fn estimate_v1() { } // Superseded by estimate_v2
   fn estimate_physics_based() { } // Abandoned approach
   ```

## Discovery Tools Configuration

### cargo-llvm-cov with Inline Annotations
```bash
# Show exactly which lines lack coverage
cargo llvm-cov --show-missing-lines --fail-under-lines 100

# Generate report with branch coverage
cargo llvm-cov --branch --html

# Focus on specific module
cargo llvm-cov --html -- --test-filter "database::"
```

### Helper Script: discover-dead-code.sh
```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Code Coverage Discovery Session ==="
echo "Date: $(date)"
echo

# Generate fresh coverage
cargo llvm-cov --html --open

# Find functions with no coverage
echo "=== Uncovered Functions ==="
cargo llvm-cov --summary-only | grep "0.00%"

# Check for #[cfg(test)] only code
echo "=== Test-Only Code ==="
rg "#\[cfg\(test\)\]" --type rust -A 5

# Find single-use functions
echo "=== Potential Single-Use Functions ==="
for func in $(rg "fn \w+\(" --type rust -o | cut -d' ' -f2 | cut -d'(' -f1 | sort -u); do
    count=$(rg "\b$func\(" --type rust | wc -l)
    if [ "$count" -le "2" ]; then
        echo "$func: $count uses"
    fi
done
```

## Success Criteria (Revised)

1. **100% coverage achieved** with documented exceptions
2. **Discovery document** listing all findings
3. **X% of codebase removed** as dead code
4. **All remaining uncovered code justified** in comments
5. **Test suite runs faster** after dead code removal

## Timeline

| Week | Focus | Deliverables |
|------|-------|--------------|
| 1 | Setup & Initial Discovery | First 50% coverage + findings |
| 2 | Complete Discovery | 100% coverage attempt + categorization |
| 3 | Elimination Sprint | Dead code removed, PR submitted |
| 4 | Documentation & Process | Sustainable practices guide |

## Example Discovery Session

```rust
// Before discovery:
pub fn format_duration(minutes: u32) -> String {
    if minutes < 60 {
        format!("{} min", minutes)
    } else {
        format!("{} hr {} min", minutes / 60, minutes % 60)
    }
}

pub fn format_duration_long(minutes: u32) -> String {
    // DISCOVERY: Never called in production
    if minutes < 60 {
        format!("{} minutes", minutes)
    } else if minutes == 60 {
        "1 hour".to_string()
    } else {
        format!("{} hours {} minutes", minutes / 60, minutes % 60)
    }
}

// After discovery:
pub fn format_duration(minutes: u32) -> String {
    if minutes < 60 {
        format!("{} min", minutes)
    } else {
        format!("{} hr {} min", minutes / 60, minutes % 60)
    }
}
// Removed format_duration_long - never used
```

## Key Takeaways

1. **100% coverage is a tool, not a goal** - Use it to discover, not to game metrics
2. **Contrived tests are a code smell** - They indicate unnecessary code
3. **Document why code stays uncovered** - Future you will thank you
4. **Dead code is technical debt** - It confuses, slows builds, and adds maintenance burden
5. **Discovery is ongoing** - Not a one-time activity

## Resources

- [cargo-llvm-cov Documentation](https://github.com/taiki-e/cargo-llvm-cov)
- [Original insight: TEST_FIXES_20250603_211433.md](../../TEST_FIXES_20250603_211433.md)
- [Project Wisdom: Test Coverage Discovery](../PROJECT_WISDOM.md#2025-01-06-using-test-coverage-to-identify-unused-code)