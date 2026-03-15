# Rust Code Coverage and Reachability Analysis Plan
**Date**: January 6, 2025, 14:45:00 UTC

## Executive Summary

This plan outlines a comprehensive approach to instrument Rust code for testing all reachable paths from entry points and identifying unused/dead code in the zwift-race-finder project. The strategy combines multiple tools and techniques to achieve maximum visibility into code usage and test coverage.

## Goals

1. **Achieve 100% visibility** of which code paths are executed during tests
2. **Identify unreachable code** from any entry point
3. **Detect dead code** that can be safely removed
4. **Automate coverage reporting** in CI/CD pipeline
5. **Establish baseline metrics** for continuous improvement

## Tool Selection

### Primary Tools

#### 1. **cargo-llvm-cov** (Recommended)
- **Purpose**: Source-based code coverage using LLVM
- **Advantages**: 
  - Most accurate coverage data
  - Works with all test types (unit, integration, doctests)
  - Generates multiple report formats (HTML, lcov, JSON)
  - Low overhead
- **Installation**: `cargo install cargo-llvm-cov`

#### 2. **cargo-tarpaulin**
- **Purpose**: Alternative coverage tool
- **Advantages**:
  - Simpler setup
  - Direct integration with coverage services
  - Good for Linux environments
- **Limitations**: Linux-only, some accuracy issues with async code
- **Installation**: `cargo install cargo-tarpaulin`

### Complementary Tools

#### 3. **cargo-unused-features**
- **Purpose**: Detect unused feature flags
- **Installation**: `cargo install cargo-unused-features`

#### 4. **cargo-udeps**
- **Purpose**: Find unused dependencies
- **Installation**: `cargo install cargo-udeps --locked`

#### 5. **cargo-bloat**
- **Purpose**: Analyze binary size and identify large functions
- **Installation**: `cargo install cargo-bloat`

#### 6. **cargo-expand**
- **Purpose**: See macro expansions to ensure coverage
- **Installation**: `cargo install cargo-expand`

## Implementation Strategy

### Phase 1: Baseline Measurement (Week 1)

1. **Install cargo-llvm-cov**
   ```bash
   cargo install cargo-llvm-cov
   rustup component add llvm-tools-preview
   ```

2. **Generate initial coverage report**
   ```bash
   cargo llvm-cov --html
   cargo llvm-cov --lcov --output-path lcov.info
   ```

3. **Identify coverage gaps**
   - Review HTML report in `target/llvm-cov/html/index.html`
   - Document uncovered functions and modules
   - Prioritize based on criticality

### Phase 2: Dead Code Detection (Week 1-2)

1. **Enable strict compiler warnings**
   ```toml
   # In Cargo.toml
   [lints.rust]
   dead_code = "warn"
   unused_imports = "warn"
   unused_variables = "warn"
   unused_mut = "warn"
   unreachable_code = "warn"
   unreachable_patterns = "warn"
   ```

2. **Run dependency analysis**
   ```bash
   cargo +nightly udeps
   cargo unused-features analyze
   cargo unused-features build-report
   ```

3. **Analyze reachability from entry points**
   ```bash
   # Custom script to trace call graphs
   cargo rustc -- -Z print-mono-items=eager > mono_items.txt
   ```

### Phase 3: Test Enhancement (Week 2-3)

1. **Add missing test cases**
   - Focus on uncovered branches
   - Test error paths explicitly
   - Add property-based tests for complex logic

2. **Create coverage-driven test targets**
   ```toml
   # In Cargo.toml
   [[test]]
   name = "coverage_integration"
   path = "tests/coverage_integration.rs"
   ```

3. **Implement test fixtures for edge cases**

### Phase 4: Automation (Week 3-4)

1. **Create coverage script**
   ```bash
   #!/usr/bin/env bash
   # coverage.sh
   set -euo pipefail
   
   cargo llvm-cov clean
   cargo llvm-cov --html
   cargo llvm-cov --lcov --output-path lcov.info
   
   # Generate summary
   cargo llvm-cov report
   ```

2. **GitHub Actions integration**
   ```yaml
   name: Coverage
   on: [push, pull_request]
   
   jobs:
     coverage:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: dtolnay/rust-toolchain@stable
           with:
             components: llvm-tools-preview
         - name: Install cargo-llvm-cov
           uses: taiki-e/install-action@cargo-llvm-cov
         - name: Generate coverage
           run: cargo llvm-cov --lcov --output-path lcov.info
         - name: Upload to Codecov
           uses: codecov/codecov-action@v3
           with:
             files: lcov.info
   ```

## Specific Techniques for Reachability Analysis

### 1. **Entry Point Mapping**
```rust
// Create explicit entry point tests
#[cfg(test)]
mod reachability_tests {
    use super::*;
    
    #[test]
    fn test_main_entry_paths() {
        // Test all CLI argument combinations
        // Ensure each code path from main() is exercised
    }
}
```

### 2. **Conditional Compilation Coverage**
```rust
#[cfg(all(test, coverage))]
mod coverage_helpers {
    // Test helpers that exercise rarely-used code paths
}
```

### 3. **Panic Path Testing**
```rust
#[test]
#[should_panic(expected = "specific message")]
fn test_error_path() {
    // Force error conditions
}
```

## Metrics and Goals

### Initial Targets
- **Line Coverage**: 80% minimum
- **Branch Coverage**: 75% minimum
- **Function Coverage**: 90% minimum

### Quality Gates
- No PR merged that reduces coverage by >2%
- All new code must have >80% coverage
- Critical paths must have 100% coverage

## Dead Code Elimination Process

### 1. **Identification**
```bash
# Find all dead code warnings
cargo clippy -- -W dead-code 2>&1 | grep "warning: .* is never used"

# Generate call graph
cargo call-stack --all-features > call_graph.txt
```

### 2. **Verification**
- Mark suspected dead code with `#[deprecated]`
- Run full test suite
- Check for runtime usage via logging

### 3. **Safe Removal**
- Remove in small batches
- Keep removal commits separate
- Document why code was removed

## Special Considerations for zwift-race-finder

### 1. **API Mock Coverage**
- Ensure all Zwift API response variants are tested
- Cover error cases (network failures, rate limits)

### 2. **Database Path Coverage**
- Test with empty database
- Test with corrupted data
- Test migration paths

### 3. **CLI Argument Combinations**
- Generate test matrix for all flag combinations
- Ensure help/version paths are covered

### 4. **Time-based Logic**
- Mock time for consistent testing
- Cover timezone edge cases
- Test event filtering at boundaries

## Tooling Configuration

### cargo-llvm-cov Configuration
```toml
# .cargo/config.toml
[env]
CARGO_INCREMENTAL = "0"
RUSTFLAGS = "-C instrument-coverage"
LLVM_PROFILE_FILE = "target/coverage/prof-%p-%m.profraw"
```

### Exclusions
```toml
# llvm-cov.toml
[ignore]
paths = [
    "tests/*",
    "benches/*",
    "target/*"
]
```

## Timeline

| Week | Activities | Deliverables |
|------|-----------|--------------|
| 1 | Tool setup, baseline measurement | Initial coverage report |
| 2 | Dead code analysis, test gaps | Dead code removal PR |
| 3 | Test enhancement, edge cases | Improved coverage (>80%) |
| 4 | CI/CD integration, documentation | Automated pipeline |

## Success Criteria

1. **Coverage**: Achieve 80%+ line coverage
2. **Dead Code**: Remove all identified dead code
3. **Automation**: Coverage reports on every PR
4. **Documentation**: Clear guide for maintaining coverage
5. **Performance**: <10% test runtime increase

## Risk Mitigation

### Potential Issues
1. **False positives** in dead code detection
   - Mitigation: Manual review, gradual removal
   
2. **Coverage overhead** slowing tests
   - Mitigation: Separate coverage runs from regular tests
   
3. **Platform differences** (WSL vs native)
   - Mitigation: Test on multiple platforms

## Next Steps

1. Create `coverage.sh` script
2. Run baseline coverage analysis
3. File issues for uncovered critical paths
4. Set up GitHub Actions workflow
5. Document coverage guidelines in CONTRIBUTING.md

## Resources

- [cargo-llvm-cov Documentation](https://github.com/taiki-e/cargo-llvm-cov)
- [Rust Coverage Book](https://doc.rust-lang.org/rustc/instrument-coverage.html)
- [Codecov Rust Guide](https://docs.codecov.com/docs/rust)
- [Dead Code Detection in Rust](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0.html#lint-reasons)

## Appendix: Example Commands

```bash
# Full coverage with all features
cargo llvm-cov --all-features --workspace --html

# Coverage for specific test
cargo llvm-cov --test integration_tests

# Branch coverage report
cargo llvm-cov --branch --html

# JSON output for tooling
cargo llvm-cov --json --output-path coverage.json

# Coverage with inline annotations
cargo llvm-cov --show-instantiations --show-line-counts-or-regions

# Find completely uncovered files
cargo llvm-cov --summary-only --fail-under-lines 1
```