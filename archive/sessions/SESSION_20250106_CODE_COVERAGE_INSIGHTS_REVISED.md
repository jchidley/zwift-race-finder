# Session: Code Coverage as Discovery Tool (Revised)
**Date**: January 6, 2025
**Focus**: Using test coverage to drive code quality discovery through test quality evaluation

## Refined Insight from Jack

"Low coverage is just an indication of a lack of test cases. The key idea is to drive to 100% coverage and then inspect the quality of the tests. Poor testing could just be poor testing, or it could be an indication of poor code under test (or unused code). This will probably need human intervention to decide."

## Core Concept

Coverage tools don't directly identify code quality - they simply show what lacks tests. The discovery process works like this:

1. **Coverage Report**: Shows untested code (objective fact)
2. **Test Writing**: Attempt to write meaningful tests for that code
3. **Quality Evaluation**: Assess if the tests are natural or contrived
4. **Human Judgment**: Decide if poor test quality indicates:
   - Need for better test design
   - Code that's poorly designed
   - Code that's unused/unnecessary

## Discovery Through Test Writing

### When Writing Tests Reveals Good Code
- Tests feel natural and straightforward
- Clear scenarios exist for the functionality
- Tests document real use cases
- Code purpose is evident from usage

Example:
```rust
// Natural test - code is clearly useful
#[test]
fn test_duration_estimation() {
    let route = Route::new(10.0, 100.0);
    assert_eq!(route.estimate_duration(200), Duration::minutes(25));
}
```

### When Writing Tests Reveals Questionable Code
- Tests require complex setup for simple functionality
- Hard to imagine real-world scenarios
- Tests feel like they're testing implementation details
- No clear production usage found

Example:
```rust
// Contrived test - why does this function exist?
#[test]
fn test_internal_state_formatter() {
    let mut state = HashMap::new();
    state.insert("key", "value");
    // Function only called by this test
    assert_eq!(format_internal_state(&state), "key=value");
}
```

## Key Learnings

1. **Coverage is Neutral**: 40% coverage doesn't mean 60% bad code - it means 60% untested code
2. **Test Quality Matters**: The ease/difficulty of writing good tests reveals code quality
3. **Human Judgment Essential**: Only humans can distinguish between:
   - "This test is hard to write because I'm not skilled enough"
   - "This test is hard to write because the code is poorly designed"
   - "This test is hard to write because the code shouldn't exist"

## Practical Application

### Phase 1: Identify Untested Code
```bash
cargo llvm-cov --summary-only
```
Current state: 40.74% coverage = 2,733 untested lines

### Phase 2: Systematic Test Writing
For each uncovered function/module:
1. Attempt to write a meaningful test
2. Document the experience:
   - Was it easy or hard to test?
   - Does the test feel natural or forced?
   - Can you find where this code is actually used?

### Phase 3: Quality Assessment
Categorize based on test writing experience:
- **Easy to test, clear purpose**: Keep and maintain
- **Hard to test, but important**: Refactor for testability
- **Hard to test, unclear purpose**: Candidate for removal
- **Impossible to test naturally**: Strong candidate for removal

## Example from Current Analysis

### Clearly Unused (Easy Decision)
- `bin/analyze_descriptions.rs` - 0% coverage, no tests exist
- `bin/debug_tags.rs` - 0% coverage, no tests exist
- These are development tools, not core functionality
- Decision: Remove (no test writing needed - obviously unused)

### Requires Test Writing to Decide
- `main.rs` - 36.71% coverage
- `route_discovery.rs` - 41.29% coverage
- Need to write tests for uncovered parts and evaluate their quality
- Only then can we determine if code is necessary

## Next Steps

1. Start with the easy wins (remove obviously unused binaries)
2. Pick a module with partial coverage
3. Write tests for uncovered functions
4. Document test quality and ease of writing
5. Make human judgment calls based on test quality
6. Iterate until 100% coverage or documented exceptions

## Important: This is About Discovery, Not Metrics

The goal isn't to achieve 100% coverage as a metric. The goal is to use the process of driving toward 100% coverage as a tool to discover:
- What code is actually used
- What code is well-designed
- What code needs refactoring
- What code should be removed

The coverage number itself is just a progress indicator for this discovery process.