# Rust Code Coverage and Reachability Analysis Plan
**Date:** 2025-01-06 18:05:00  
**Project:** zwift-race-finder

## Executive Summary

This plan outlines a comprehensive approach to instrument Rust code for testing all reachable paths from entry points and identifying unused/dead code. The goal is to achieve high confidence in code coverage while eliminating maintenance burden from unused code paths.

## Problem Statement

Current challenges:
- No visibility into which code paths are exercised by tests
- Potential dead code that increases maintenance burden
- No systematic way to ensure all reachable paths are tested
- Difficulty identifying unused dependencies or features

## Solution Components

### 1. Code Coverage Instrumentation

#### 1.1 Source-Based Coverage (Recommended)
**Tool:** `cargo-llvm-cov` - Uses LLVM's native instrumentation
```bash
# Installation
cargo install cargo-llvm-cov

# Basic usage
cargo llvm-cov --html                    # Generate HTML report
cargo llvm-cov --lcov --output-path lcov.info  # For CI integration
cargo llvm-cov --json --summary-only    # Machine-readable summary
```

**Advantages:**
- Native LLVM integration (same as rustc)
- Accurate branch coverage
- Low overhead
- Works with all test types (unit, integration, doctests)

#### 1.2 Alternative: cargo-tarpaulin
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

**Trade-offs:**
- Linux-only
- Sometimes less accurate than llvm-cov
- Better IDE integration

### 2. Reachability Analysis

#### 2.1 Static Analysis for Dead Code
**Built-in Rust lints:**
```rust
#![warn(dead_code)]
#![warn(unused)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
```

**Enhanced with cargo-machete (unused dependencies):**
```bash
cargo install cargo-machete
cargo machete  # Find unused dependencies
```

#### 2.2 Call Graph Generation
**Tool:** `cargo-call-stack` or custom LLVM pass
```bash
# Generate call graph from entry points
cargo call-stack --entry-point main > call_graph.dot
dot -Tpng call_graph.dot -o call_graph.png
```

#### 2.3 Dynamic Reachability Tracking
Custom instrumentation using procedural macros:
```rust
#[instrument_coverage]
fn some_function() {
    // Auto-inserted: COVERAGE_TRACKER.mark_reached("some_function");
}
```

### 3. Implementation Strategy

#### Phase 1: Baseline Coverage (Week 1)
1. **Set up cargo-llvm-cov**
   ```toml
   # .cargo/config.toml
   [target.'cfg(all())']
   rustflags = ["-C", "instrument-coverage"]
   ```

2. **Create coverage script**
   ```bash
   #!/bin/bash
   # coverage.sh
   cargo llvm-cov clean
   cargo llvm-cov --all-features --workspace --html
   cargo llvm-cov report --summary-only
   ```

3. **Establish baseline metrics**
   - Current line coverage: ?%
   - Current branch coverage: ?%
   - Uncovered critical paths: ?

#### Phase 2: Reachability Analysis (Week 2)
1. **Map entry points**
   ```rust
   // src/analysis/entry_points.rs
   const ENTRY_POINTS: &[&str] = &[
       "main",
       "lib::public_api_function_1",
       // All public API surface
   ];
   ```

2. **Generate reachability report**
   - Use rustc's MIR output
   - Build call graph
   - Identify unreachable functions

3. **Create dead code report**
   ```rust
   // dead_code_analyzer.rs
   fn analyze_reachability() -> DeadCodeReport {
       let call_graph = build_call_graph();
       let reachable = traverse_from_entries(call_graph, ENTRY_POINTS);
       let all_functions = extract_all_functions();
       DeadCodeReport {
           unreachable: all_functions - reachable,
           coverage_gaps: identify_untested_reachable(reachable),
       }
   }
   ```

#### Phase 3: Continuous Monitoring (Week 3)
1. **CI Integration**
   ```yaml
   # .github/workflows/coverage.yml
   - name: Run coverage
     run: |
       cargo llvm-cov --lcov --output-path lcov.info
       cargo llvm-cov report --fail-under-lines 80
   
   - name: Upload to Codecov
     uses: codecov/codecov-action@v3
     with:
       files: lcov.info
       fail_ci_if_error: true
   ```

2. **Coverage gates**
   - Minimum line coverage: 80%
   - Minimum branch coverage: 70%
   - No new uncovered code in PRs

3. **Weekly reports**
   - Dead code candidates
   - Coverage trends
   - Untested critical paths

### 4. Specific Considerations for zwift-race-finder

#### 4.1 Entry Points to Analyze
- `main()` - CLI entry
- `estimate_duration()` - Core algorithm
- `fetch_events()` - API interaction
- `database::*` - All DB operations
- Test-only code (marked appropriately)

#### 4.2 Special Cases
1. **Error paths**: Often uncovered but critical
   ```rust
   #[cfg(test)]
   mod error_injection {
       // Force error conditions for coverage
   }
   ```

2. **Platform-specific code**
   ```rust
   #[cfg(target_os = "windows")]
   fn windows_specific() { }  // May show as uncovered on Linux CI
   ```

3. **FFI boundaries**: SQLite interactions need special attention

#### 4.3 Proposed Metrics
- **Essential coverage**: Paths from main() to core features
- **API coverage**: All public functions
- **Error coverage**: All error handling paths
- **Dead code threshold**: <5% of codebase

### 5. Tooling Setup

#### 5.1 Development Environment
```bash
# Install all tools
cargo install cargo-llvm-cov cargo-machete cargo-bloat

# Git hooks for coverage
echo '#!/bin/bash
cargo llvm-cov report --summary-only --fail-under-lines 80
' > .git/hooks/pre-push
```

#### 5.2 VSCode Integration
```json
// .vscode/tasks.json
{
  "label": "Show Coverage",
  "type": "shell",
  "command": "cargo llvm-cov --html && open target/llvm-cov/html/index.html"
}
```

#### 5.3 Reporting Dashboard
```rust
// src/bin/coverage_report.rs
fn main() {
    let coverage = collect_coverage_data();
    let reachability = analyze_reachability();
    generate_html_report(coverage, reachability);
}
```

### 6. Advanced Techniques

#### 6.1 Mutation Testing
```bash
cargo install cargo-mutants
cargo mutants  # Verify tests actually detect bugs
```

#### 6.2 Fuzzing for Path Discovery
```rust
#[cfg(fuzzing)]
use cargo_fuzz::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz inputs to discover new paths
});
```

#### 6.3 Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_all_score_ranges(score in 0u32..650) {
        // Ensures coverage of entire score range
    }
}
```

### 7. Dead Code Elimination Process

1. **Identify candidates**
   ```bash
   cargo +nightly rustc -- -Z print-type-sizes  # Find unused types
   cargo bloat --release                        # Find code bloat
   ```

2. **Verify with feature flags**
   ```toml
   [features]
   minimal = []  # Core functionality only
   full = ["feature1", "feature2"]
   ```

3. **Safe removal process**
   - Mark as deprecated first
   - Remove in next major version
   - Document in CHANGELOG

### 8. Success Metrics

- **Week 1**: Baseline coverage >60%
- **Week 2**: Reachability report identifies >90% of functions
- **Week 3**: CI enforcement active
- **Month 1**: Coverage >80%, dead code <5%
- **Month 3**: Coverage >90%, zero dead code

### 9. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| False positives in dead code | Remove live code | Manual verification + staging period |
| Coverage overhead in CI | Slow builds | Parallel jobs, caching |
| Generic/macro code | Shows as uncovered | Instantiation tests |
| Conditional compilation | Platform-specific gaps | Multi-platform CI matrix |

### 10. Next Steps

1. **Immediate (Today)**
   - Install cargo-llvm-cov
   - Run baseline coverage report
   - Document current state

2. **This Week**
   - Set up CI integration
   - Create coverage tracking
   - Identify top 10 uncovered critical paths

3. **This Month**
   - Implement reachability analysis
   - Remove identified dead code
   - Establish coverage gates

## Appendix: Example Implementation

### A. Coverage Tracking Module
```rust
// src/coverage.rs
#[cfg(coverage)]
pub mod tracking {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use std::collections::HashSet;
    
    static REACHED: Lazy<Mutex<HashSet<&'static str>>> = 
        Lazy::new(|| Mutex::new(HashSet::new()));
    
    pub fn mark(function: &'static str) {
        REACHED.lock().unwrap().insert(function);
    }
    
    pub fn report() -> Vec<&'static str> {
        REACHED.lock().unwrap().iter().copied().collect()
    }
}

// Procedural macro usage
#[track_coverage]
fn some_function() {
    // Auto-inserts: coverage::tracking::mark("some_function");
}
```

### B. Reachability Query Script
```rust
// src/bin/find_dead_code.rs
use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["rustc", "--", "--emit=mir"])
        .output()
        .expect("Failed to get MIR");
    
    let mir = String::from_utf8(output.stdout).unwrap();
    let call_graph = parse_mir_to_call_graph(&mir);
    let unreachable = find_unreachable_from_main(call_graph);
    
    println!("Potentially dead code:");
    for func in unreachable {
        println!("  - {}", func);
    }
}
```

### C. Integration Test for Coverage
```rust
// tests/coverage_integration.rs
#[test]
fn verify_minimum_coverage() {
    let report = cargo_llvm_cov::generate_report();
    assert!(report.line_coverage >= 80.0, 
            "Line coverage {}% is below 80% threshold", 
            report.line_coverage);
    assert!(report.branch_coverage >= 70.0,
            "Branch coverage {}% is below 70% threshold",
            report.branch_coverage);
}
```

## Conclusion

This plan provides a systematic approach to achieving comprehensive code coverage and identifying dead code in Rust projects. The key is to start simple with built-in tools, establish baselines, and gradually increase sophistication based on project needs. The combination of static analysis, dynamic coverage, and reachability analysis will provide high confidence in code quality while minimizing maintenance burden.