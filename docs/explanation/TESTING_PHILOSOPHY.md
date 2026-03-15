# Testing Philosophy

Why we test the way we do, what the research says, and the lessons that shaped our approach.

## The OCR Lesson: 234 Mutations, 0 Caught

On 2025-01-10, our OCR module had property tests, snapshot tests, integration tests, fuzz tests, and benchmarks. Mutation testing revealed **0% effectiveness** — not a single mutation was caught.

This proved that test quantity ≠ test quality, and it changed our entire approach.

## What the Research Says

### Code Coverage Is a Weak Predictor

- **Microsoft Research (2020)**: "Insignificant correlation" between coverage and post-release bugs across 100 Java projects
- **Google Research (2014)**: "Coverage is not strongly correlated with test suite effectiveness" (31,000 test suites)
- **Industry consensus**: 60–80% is the optimal coverage range. Beyond that, bug detection rate slows dramatically.

Google's guidelines: "60% acceptable, 75% commendable, 90% exemplary."

### Mutation Testing Is Better (But Imperfect)

- Mutation score correlation with fault detection: 0.6–0.8 (vs 0.3–0.5 for line coverage)
- **Google**: Uses mutation testing on 1,000+ projects — but only on changed code during review
- **Facebook**: Found >50% of mutants survived their rigorous test suite
- **Key finding**: Test suite size is a major confounding factor

### The Coupling Effect (Empirically Validated)

Tests that kill simple mutants (`+` → `-`, `&&` → `||`) also kill **99% of complex mutants**. Simple mutations are sufficient.

## Our Principles

### 1. Natural Tests Over Contrived Tests

**Natural**: Given realistic inputs, does this function produce expected outputs?
```rust
assert_eq!(format_duration(90), "01:30");  // Real use case
```

**Contrived**: Can I make this code execute?
```rust
let mock_args = MockArgs::new();
let mock_config = MockConfig::new();
// 50 lines of mock setup to test that mocks work
```

If a test takes >5 minutes to write, the code design is fighting you. Refactor first.

### 2. Coverage Grows Through Usage

```
New Feature (60–70%) → User Reports → Regression Tests → High Coverage (90%+)
```

Ship at 60–70% with quality tests. Real users find edge cases you didn't imagine. Add regression tests for actual failures. Mature features naturally reach 90%+. 100% coverage on day one = contrived tests that catch nothing.

### 3. Mutation Testing During Development, Not After

**Old**: Write all tests → Run mutations → Despair at 0%.
**New**: Write test → Mutate immediately → Fix → Repeat.

```bash
# Per function (5 min)
cargo mutants --file src/parser.rs --function parse_time --timeout 30

# Per module (30 min)
cargo mutants --file src/estimation.rs --timeout 180

# Full codebase (2–4 hours, run in background)
nohup cargo mutants --jobs 8 --timeout 180 > mutation.log 2>&1 &
```

### 4. Test What Users Experience

Track **behavioural coverage** — what the user sees — not code lines:

| Behaviour | Tested | Mutation Score |
|-----------|--------|----------------|
| Filter races by duration | ✅ | 85% |
| Handle Racing Score events | ✅ | 80% |
| Estimate unknown routes | ❌ | — |

### 5. What NOT to Unit Test

- **Simple delegators**: `fn get_name(&self) -> &str { &self.name }`
- **Type conversions**: `impl From<ConfigError> for AppError`
- **Framework boilerplate**: `#[derive(Debug, Clone, Serialize)]`
- **Logging/metrics**: `info!("Processing item: {}", id);`
- **Orchestration** (main/run): Test at integration level instead

## Red Flags: Your Tests Are Bad If…

1. **Structure-only assertions**: `assert!(result.is_some())` — survives all mutations
2. **Property tests without properties**: `let _ = parse(&s);` — just checks it doesn't panic
3. **Integration tests that test nothing**: `assert!(telemetry.speed.is_some())` — no value check
4. **The mock-everything anti-pattern**: 50 lines of mock setup, then `assert mock_db.called`
5. **The snapshot-everything anti-pattern**: Locks in current bugs as "correct"

**Fix**: Replace with concrete assertions: `assert_eq!(result, Some(42))`.

## The Test Hierarchy

| Level | Purpose | Rust Tool | When |
|-------|---------|-----------|------|
| Unit tests | Verify pure functions | `#[test]` + concrete assertions | During development |
| Property tests | Verify invariants across input space | `proptest` | After unit tests reveal complexity |
| Snapshot tests | Catch output regressions | `insta` | For complex outputs, API responses |
| Integration tests | Verify components together | `tests/` directory | After units prove individual correctness |
| E2E tests | Verify user workflows | `assert_cmd` | Define early, test throughout |

**Coverage targets**: Unit 60% → Integration 80% → E2E 95%.

## Current Project Metrics

- **MAE**: 16.6% on 125 matched races (target: <20%)
- **169 tests** passing across lib + integration + property + snapshot
- **100% natural test rate** for tested functions
- Regression test with real race data runs on every change

## References

1. Microsoft Research (2020). Code Coverage and Post-release Defects.
2. Google Research (2014). Coverage is not strongly correlated with test suite effectiveness.
3. Papadakis et al. (2018). Are Mutation Scores Correlated with Real Fault Detection?
4. Google (2018). State of Mutation Testing at Google.
5. Facebook (2021). What It Would Take to Use Mutation Testing in Industry.
