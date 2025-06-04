# Session: Code Coverage as Discovery Tool
**Date**: January 6, 2025
**Focus**: Reframing code coverage from metric to discovery mechanism

**Note**: See SESSION_20250106_CODE_COVERAGE_INSIGHTS_REVISED.md for refined understanding after further discussion with Jack about test quality evaluation and human judgment.

## Key Insight from Jack

"It seems difficult to get LLMs to analyse the code and identify parts of it that are unused. One way is to use tools get 100% test coverage and then understand what these tools are testing and the relevance of the thing being tested."

## Work Completed

### 1. Added Insight to PROJECT_WISDOM.md
- Documented how test coverage tools reveal unused code indirectly
- Added that coverage tools operate external to LLMs with different speed/cost
- Referenced TEST_FIXES_20250603_211433.md as practical example

### 2. Analyzed Existing Coverage Plan
Found that RUST_CODE_COVERAGE_PLAN_20250106_144500.md treated coverage as:
- A quality metric (80% target)
- Separate from dead code detection
- A compliance requirement

### 3. Created New Discovery-Based Plan
Created RUST_CODE_COVERAGE_DISCOVERY_PLAN.md with:
- 100% coverage as forcing function for code examination
- Discovery protocol for each uncovered function
- Test quality indicators (natural vs contrived)
- New metrics: dead code found, test contrivance score
- Practical examples from zwift-race-finder

## Key Philosophical Shifts

### Old Approach
- Coverage = Quality metric
- 80% = Good enough
- Dead code detection = Separate activity
- Tests = Prove code works

### New Approach
- Coverage = Discovery mechanism
- 100% = Forces examination of everything
- Dead code = Reveals itself through contrived tests
- Tests = Question code's existence

## Discovery Protocol Established

For each uncovered code path:
1. What is this code's purpose?
2. What production code calls this?
3. Is the test contrived or natural?
4. Should this exist at all?

## Benefits of External Tools

Coverage tools complement LLMs by:
- Running in seconds vs minutes
- Providing deterministic results
- Operating continuously in CI/CD
- No token costs
- Generating data for LLM interpretation

## Next Steps

1. Run `cargo install cargo-llvm-cov` and generate baseline
2. Begin discovery sessions on uncovered code
3. Document findings with DISCOVERY comments
4. Remove confirmed dead code
5. Establish monthly discovery practice

## Files Modified

- docs/PROJECT_WISDOM.md - Added coverage insights
- docs/development/RUST_CODE_COVERAGE_DISCOVERY_PLAN.md - New discovery-based plan
- Multiple test files already demonstrating the approach