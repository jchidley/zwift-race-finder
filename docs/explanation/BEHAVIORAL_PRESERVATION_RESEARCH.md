# Behavioral Preservation Research: Preventing Unintended Code Changes

## Executive Summary

This research explores state-of-the-art approaches for preventing unintended behavioral changes in code, particularly relevant when working with AI assistants or during refactoring. The findings prioritize practical, implementable solutions for Rust projects.

**Key Finding**: The most effective approach combines multiple layers: comprehensive testing (unit, property, snapshot), behavioral documentation, and automated change detection tools.

## The Challenge

When modifying code, especially with AI assistance, we face several risks:
1. **Subtle behavioral changes** that pass existing tests
2. **Regression in edge cases** not covered by tests
3. **Performance degradation** that goes unnoticed
4. **API contract violations** that break downstream code
5. **Loss of implicit knowledge** encoded in the original implementation

## Research Findings

### 1. Immediate High-Impact Solutions for Rust

#### Snapshot Testing with `insta`
```toml
[dev-dependencies]
insta = { version = "1.34", features = ["yaml", "redactions"] }
```

**Benefits**:
- Captures exact output/behavior before changes
- Visual diffs show behavioral changes clearly
- Supports complex data structures and redactions
- CI-friendly with review tools

**Example**:
```rust
#[test]
fn test_duration_estimation() {
    let result = estimate_duration(&test_event);
    insta::assert_yaml_snapshot!(result);
}
```

#### Property-Based Testing with `proptest`
```toml
[dev-dependencies]
proptest = "1.4"
```

**Benefits**:
- Defines behavioral invariants that must hold
- Automatically generates test cases
- Finds edge cases humans miss
- Acts as executable specifications

**Example**:
```rust
proptest! {
    #[test]
    fn duration_always_positive(
        distance in 1.0..200.0,
        elevation in 0.0..2000.0
    ) {
        let duration = calculate_duration(distance, elevation);
        assert!(duration > 0.0);
    }
}
```

#### Mutation Testing with `cargo-mutants`
```bash
cargo install cargo-mutants
cargo mutants
```

**Benefits**:
- Verifies test quality by mutating code
- Identifies tests that don't actually test behavior
- Finds gaps in test coverage
- Ensures tests will catch future regressions

### 2. Industry Best Practices

#### Google's Approach
- **Test Impact Analysis**: Only run tests affected by changes
- **Semantic Code Review**: Tools that understand behavioral changes
- **Large-Scale Testing**: Continuous testing across entire codebase
- **Beyonce Rule**: "If you liked it, you should have put a test on it"

#### Facebook/Meta's Approach
- **Infer**: Static analysis for behavioral bugs
- **Sapienz**: Automated test generation
- **Differential Testing**: Compare behaviors across versions
- **Time-Travel Debugging**: Record and replay production issues

#### Microsoft's Approach
- **IntelliTest**: Automated white-box testing
- **Code Contracts**: Runtime and static checking
- **CHESS**: Systematic concurrency testing
- **PREfix/PREfast**: Path-sensitive analysis

#### Amazon's Approach
- **TLA+**: Formal specification for critical systems
- **Canary Deployments**: Gradual rollout with monitoring
- **Chaos Engineering**: Proactive failure testing
- **GameDays**: Simulated failure scenarios

### 3. Advanced Techniques

#### Formal Methods (When Needed)
- **Design by Contract**: Explicit pre/post conditions
- **Model Checking**: Verify all possible states
- **Theorem Proving**: Mathematical correctness proofs
- **Refinement Types**: Types that encode invariants

**Practical in Rust**:
```rust
// Using contracts crate
#[requires(x > 0)]
#[ensures(ret > x)]
fn increment(x: i32) -> i32 {
    x + 1
}
```

#### Semantic Diff Tools
- **SemanticDiff**: Understands code structure, not just text
- **Difftastic**: AST-based diff for better change understanding
- **ReviewDog**: Automated code review with semantic understanding

#### Behavioral Documentation
```rust
/// Estimates race duration based on route and rider characteristics.
/// 
/// # Behavioral Guarantees
/// - Duration is always positive
/// - Longer routes take more time (monotonic)
/// - Higher elevation increases duration
/// - Result within 20% of historical data
/// 
/// # Edge Cases
/// - Returns minimum 5 minutes for any route
/// - Caps at 180 minutes for safety
/// - Handles missing route data gracefully
```

### 4. AI-Assisted Development Safety

#### Specific Risks with AI
1. **Overfitting to examples**: AI might change behavior to match test cases
2. **Loss of domain knowledge**: Implicit assumptions may be removed
3. **Style over substance**: Code might look better but work differently
4. **Test gaming**: AI might make tests pass without preserving behavior

#### Mitigation Strategies
1. **Comprehensive test suite** before AI modifications
2. **Snapshot testing** for all AI-touched code
3. **Human review** of behavioral changes
4. **Incremental changes** with validation
5. **Clear specifications** in comments/docs

### 5. Practical Implementation Plan

#### Phase 1: Foundation (Week 1)
```bash
# Add dependencies
cargo add --dev insta proptest criterion rstest

# Install tools
cargo install cargo-mutants
cargo install cargo-nextest
cargo install cargo-llvm-cov
```

#### Phase 2: Baseline (Week 2)
1. Add snapshot tests for core functions
2. Create golden test suite with known good outputs
3. Define property tests for invariants
4. Document behavioral guarantees

#### Phase 3: Automation (Week 3)
1. Set up mutation testing in CI
2. Create behavioral diff reports
3. Add performance benchmarks
4. Implement contract checking

#### Phase 4: Monitoring (Ongoing)
1. Track behavioral changes in PRs
2. Monitor production accuracy
3. Collect edge cases as tests
4. Refine property definitions

### 6. Specific Recommendations for Zwift Race Finder

#### Core Behavioral Invariants
```rust
// Duration estimation properties
proptest! {
    #[test]
    fn duration_monotonic(d1 in 10.0..100.0, d2 in 10.0..100.0) {
        if d1 < d2 {
            assert!(estimate_duration(d1, 0.0) < estimate_duration(d2, 0.0));
        }
    }
    
    #[test]
    fn duration_bounded(distance in 1.0..200.0) {
        let duration = estimate_duration(distance, 0.0);
        assert!(duration >= 5.0 && duration <= 180.0);
    }
}
```

#### Snapshot Tests for Regression Detection
```rust
#[test]
fn snapshot_known_routes() {
    let routes = vec![
        ("watopia_flat", 34.0, 100.0),
        ("alpe_du_zwift", 12.2, 1036.0),
        // ... more routes
    ];
    
    for (name, distance, elevation) in routes {
        let result = estimate_duration(distance, elevation);
        insta::assert_snapshot!(
            format!("duration_{}_{}", name, ZWIFT_SCORE),
            result
        );
    }
}
```

#### Golden Test Suite
```rust
// tests/golden_races.rs
const GOLDEN_RACES: &[(&str, f64, f64, f64)] = &[
    // (route_name, distance, elevation, expected_duration)
    ("Beach Island Loop", 12.5, 85.0, 24.5),
    ("Volcano Circuit", 4.1, 21.0, 8.2),
    // Based on actual race history
];
```

#### Behavioral Documentation
```yaml
# behaviors.yaml
duration_estimation:
  invariants:
    - "Always returns positive duration"
    - "Monotonic with distance"
    - "Increases with elevation"
    - "Within 20% of historical data"
  edge_cases:
    - "Minimum 5 minutes"
    - "Maximum 180 minutes"
    - "Handles missing data"
  
event_filtering:
  invariants:
    - "Never returns expired events"
    - "Respects all filter criteria"
    - "Maintains sort order"
```

### 7. Tools Comparison Matrix

| Tool | Purpose | Effort | Impact | Rust Support |
|------|---------|--------|--------|--------------|
| insta | Snapshot testing | Low | High | Excellent |
| proptest | Property testing | Medium | High | Excellent |
| cargo-mutants | Mutation testing | Low | High | Native |
| contracts | Design by contract | Medium | Medium | Good |
| criterion | Performance regression | Low | Medium | Excellent |
| cargo-nextest | Test optimization | Low | Medium | Native |
| tarpaulin/llvm-cov | Coverage analysis | Low | Medium | Native |

### 8. Decision Framework

When modifying code, follow this checklist:

1. **Before changes**:
   - [ ] Run full test suite
   - [ ] Create snapshot tests for modified functions
   - [ ] Document current behavior
   - [ ] Note performance baseline

2. **During changes**:
   - [ ] Keep changes minimal and focused
   - [ ] Run tests frequently
   - [ ] Check snapshot diffs
   - [ ] Verify property tests still pass

3. **After changes**:
   - [ ] Run mutation testing
   - [ ] Compare performance
   - [ ] Review behavioral diffs
   - [ ] Update documentation

4. **For AI-assisted changes**:
   - [ ] Extra snapshot coverage
   - [ ] Manual behavior verification
   - [ ] Check for implicit knowledge loss
   - [ ] Verify edge case handling

## Conclusion

Preventing behavioral regression requires a multi-layered approach:

1. **Testing**: Comprehensive unit, property, and snapshot tests
2. **Documentation**: Clear behavioral contracts and invariants
3. **Tooling**: Automated detection of behavioral changes
4. **Process**: Structured review and validation workflow

For the Zwift Race Finder project, the immediate priorities are:
1. Add snapshot tests for duration estimation
2. Define property tests for core invariants
3. Run mutation testing to validate test quality
4. Document behavioral guarantees

This approach provides strong protection against unintended changes while remaining practical and maintainable.

## References

1. "Testing Terraform" by Yevgeniy Brikman
2. "Property-Based Testing with PropTest" by Jason Kölker
3. "Mutation Testing in Rust" by cargo-mutants documentation
4. "Snapshot Testing Best Practices" by Insta documentation
5. "Formal Methods in Practice" by Amazon's TLA+ team
6. "Google's Testing Blog" on test impact analysis
7. "Facebook's Infer" static analysis papers
8. "Microsoft Research" on automated testing
9. "Chaos Engineering" by Netflix and Amazon
10. "The Art of Software Testing" by Glenford Myers et al.