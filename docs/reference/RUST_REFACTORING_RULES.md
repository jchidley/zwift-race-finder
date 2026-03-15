# Rust Refactoring Rules

This document provides Rust-specific refactoring guidelines to ensure safe, mechanical transformations that preserve behavior while improving code structure.

## Core Principles

1. **Compiler-Driven Refactoring**: Let Rust's type system guide you
2. **Zero Behavioral Changes**: Refactoring never changes what code does
3. **Incremental Steps**: Make one small change at a time
4. **Test Coverage**: Never refactor without tests
5. **Atomic Commits**: One refactoring type per commit

## Tool Setup

### Essential Tools
```bash
# Install core refactoring tools
cargo install cargo-edit         # Dependency management
cargo install cargo-expand       # Macro expansion viewer
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-mutants      # Mutation testing
rustup component add clippy      # Linting and suggestions
```

### IDE Setup
- **Primary**: rust-analyzer (VS Code, Neovim, Emacs)
- **Alternative**: IntelliJ Rust plugin
- Enable format-on-save with rustfmt

## Mechanical Refactoring Catalog

### 1. Extract Function
**When**: Code block is doing one clear task
**How**: 
```rust
// Before
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    // validation logic
    for item in &items {
        if !item.is_valid() {
            return Err(Error::InvalidItem);
        }
    }
    
    // processing logic
    let total = items.iter().map(|i| i.value).sum();
    Ok(Summary { total })
}

// After
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    validate_items(&items)?;
    let total = calculate_total(&items);
    Ok(Summary { total })
}

fn validate_items(items: &[Item]) -> Result<(), Error> {
    for item in items {
        if !item.is_valid() {
            return Err(Error::InvalidItem);
        }
    }
    Ok(())
}

fn calculate_total(items: &[Item]) -> u32 {
    items.iter().map(|i| i.value).sum()
}
```

**rust-analyzer**: Select code → "Extract into function" (Ctrl+.)

### 2. Extract Module
**When**: Related functions/types form a cohesive unit
**Process**:
1. Create inline module first
2. Move to separate file only after it's working
3. Use `pub(crate)` for internal visibility

```rust
// Step 1: Inline module
mod validation {
    use super::*;
    
    pub fn validate_items(items: &[Item]) -> Result<(), Error> {
        // ...
    }
}

// Step 2: Move to validation.rs
// main.rs
mod validation;
use validation::validate_items;

// validation.rs
use crate::{Item, Error};

pub fn validate_items(items: &[Item]) -> Result<(), Error> {
    // ...
}
```

### 3. Extract Type Alias
**When**: Complex type signatures appear multiple times
```rust
// Before
fn process(data: HashMap<String, Vec<(u32, String)>>) -> HashMap<String, Vec<(u32, String)>> {
    // ...
}

// After
type UserRecords = HashMap<String, Vec<(u32, String)>>;

fn process(data: UserRecords) -> UserRecords {
    // ...
}
```

### 4. Extract Constant
**When**: Magic numbers or repeated literals
```rust
// Before
if retries > 3 {
    return Err("Max retries exceeded");
}

// After
const MAX_RETRIES: u32 = 3;
const ERROR_MAX_RETRIES: &str = "Max retries exceeded";

if retries > MAX_RETRIES {
    return Err(ERROR_MAX_RETRIES);
}
```

### 5. Inline Variable/Function
**When**: Variable/function adds no clarity
```rust
// Before
let is_valid = item.validate();
if is_valid {
    process(item);
}

// After
if item.validate() {
    process(item);
}
```

**rust-analyzer**: Place cursor on variable → "Inline variable"

### 6. Rename Symbol
**When**: Name doesn't clearly express intent
**Process**:
1. Use rust-analyzer's rename (F2)
2. Review all occurrences before confirming
3. Update documentation comments

### 7. Change Function Signature
**When**: Parameters need reordering/adding/removing
```rust
// Before
fn calculate(base: f64, rate: f64) -> f64 {
    base * rate
}

// After (adding parameter with default)
fn calculate(base: f64, rate: f64, years: Option<u32>) -> f64 {
    let years = years.unwrap_or(1);
    base * rate * years as f64
}
```

### 8. Convert Between Patterns
**rust-analyzer** assists with these conversions:

```rust
// if/else → match
// Before
if let Some(x) = opt {
    x + 1
} else {
    0
}

// After
match opt {
    Some(x) => x + 1,
    None => 0,
}

// Or better: opt.map_or(0, |x| x + 1)
```

### 9. Extract Trait
**When**: Multiple types share common behavior
```rust
// Before
impl Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

impl Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}

// After
trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}
```

### 10. Replace Loop with Iterator
**When**: Loop is transforming/filtering collection
```rust
// Before
let mut result = Vec::new();
for item in items {
    if item.is_active() {
        result.push(item.value * 2);
    }
}

// After
let result: Vec<_> = items
    .into_iter()
    .filter(|item| item.is_active())
    .map(|item| item.value * 2)
    .collect();
```

## Ownership & Borrowing Considerations

### 1. Reduce Cloning
```rust
// Before: Unnecessary clone
fn process(data: Vec<Item>) -> Vec<Item> {
    let filtered = data.clone();
    filtered.into_iter().filter(|i| i.valid).collect()
}

// After: Take ownership
fn process(data: Vec<Item>) -> Vec<Item> {
    data.into_iter().filter(|i| i.valid).collect()
}
```

### 2. Simplify Lifetimes
```rust
// Before: Explicit lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {
    &s[..s.find(' ').unwrap_or(s.len())]
}

// After: Lifetime elision
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

### 3. Choose References vs Ownership
```rust
// When to take &self, &mut self, or self
impl Container {
    fn len(&self) -> usize { }           // Read only
    fn push(&mut self, item: Item) { }   // Modify
    fn into_vec(self) -> Vec<Item> { }   // Consume
}
```

## Module System Refactoring

### 1. Progressive Extraction
```rust
// Step 1: Private functions in same file
fn helper() { }

// Step 2: Module in same file
mod helpers {
    pub(super) fn helper() { }
}

// Step 3: Separate file
// helpers.rs
pub(crate) fn helper() { }
```

### 2. Visibility Progression
Start restrictive, expand as needed:
1. `fn` (private)
2. `pub(super)` (parent module)
3. `pub(crate)` (crate only)
4. `pub` (public API)

### 3. Re-export Strategy
```rust
// lib.rs
mod parsing;
mod formatting;

// Re-export for clean API
pub use parsing::{parse_event, ParseError};
pub use formatting::format_duration;
```

## Error Handling Refactoring

### 1. Consolidate Error Types
```rust
// Before: String errors
fn parse() -> Result<Data, String> {
    Err("Parse failed".to_string())
}

// After: Structured errors
#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Parse failed: {0}")]
    Failed(String),
}

fn parse() -> Result<Data, ParseError> {
    Err(ParseError::Failed("reason".to_string()))
}
```

### 2. Preserve Error Context
```rust
// Before: Context lost
let data = parse().map_err(|_| MyError::ParseFailed)?;

// After: Context preserved
let data = parse()
    .map_err(|e| MyError::ParseFailed(e.to_string()))?;
```

## Performance-Preserving Refactoring

### 1. Maintain Iterator Chains
```rust
// DON'T break lazy evaluation
// Bad
let vec: Vec<_> = items.iter().map(|i| i.value).collect();
let sum: u32 = vec.iter().sum();

// Good: Keep it lazy
let sum: u32 = items.iter().map(|i| i.value).sum();
```

### 2. Const Generics for Compile-Time
```rust
// Runtime dimension
fn matrix_multiply(a: &[f64], b: &[f64], size: usize) { }

// Compile-time dimension
fn matrix_multiply<const N: usize>(a: &[f64; N*N], b: &[f64; N*N]) { }
```

### 3. Strategic Inlining
```rust
// For hot path functions
#[inline]
fn is_valid(&self) -> bool {
    self.flags & VALID_FLAG != 0
}

// For cross-crate optimization
#[inline(always)]
pub fn performance_critical() { }
```

## Testing During Refactoring

### 1. Test Organization
```rust
// Keep tests with the code they test
mod parsing {
    pub fn parse() -> Result<Data, Error> { }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_parse_valid() { }
    }
}
```

### 2. Regression Test Suite
Create before major refactoring:
```rust
#[cfg(test)]
mod regression_tests {
    // Snapshot current behavior
    #[test]
    fn behavior_snapshot_1() {
        let input = include_str!("../testdata/case1.txt");
        let output = process(input);
        assert_eq!(output, include_str!("../testdata/case1_expected.txt"));
    }
}
```

### 3. Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn refactored_matches_original(input: Vec<u32>) {
        let old_result = old_implementation(&input);
        let new_result = new_implementation(&input);
        assert_eq!(old_result, new_result);
    }
}
```

## Async/Await Refactoring

### 1. Sync to Async Migration
```rust
// Step 1: Identify I/O boundaries
fn fetch_data() -> Result<Data, Error> {
    let response = blocking_http_get(url)?;
    Ok(parse(response))
}

// Step 2: Make async at boundaries
async fn fetch_data() -> Result<Data, Error> {
    let response = async_http_get(url).await?;
    Ok(parse(response))
}

// Step 3: Propagate async upward
async fn process() -> Result<Summary, Error> {
    let data = fetch_data().await?;
    Ok(summarize(data))
}
```

### 2. Avoid Async Contamination
```rust
// Keep computation sync
fn calculate(data: &Data) -> u32 {
    // Pure computation, no async needed
}

// Only I/O is async
async fn fetch_and_calculate() -> Result<u32, Error> {
    let data = fetch_data().await?;
    Ok(calculate(&data))  // Sync function called normally
}
```

## Macro Refactoring

### 1. Macro to Function
```rust
// Before: Unnecessary macro
macro_rules! add_one {
    ($x:expr) => { $x + 1 }
}

// After: Simple function
fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 2. Simplify Macro Rules
```rust
// Before: Complex matching
macro_rules! vec_init {
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { vec![$elem; $n] };
    ($($x:expr),*) => { vec![$($x),*] };
}

// After: Defer to std
macro_rules! vec_init {
    ($($tokens:tt)*) => { vec![$($tokens)*] };
}
```

## Safety Checklist

Before any refactoring:
- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Code compiles with no warnings
- [ ] Benchmarks run (if performance critical)

After refactoring:
- [ ] All tests still passing
- [ ] No new clippy warnings
- [ ] Performance unchanged (if measured)
- [ ] API compatibility maintained (if public)

## Common Pitfalls

### 1. Losing Type Information
```rust
// Bad: Erases error type
fn process() -> Result<Data, Box<dyn Error>> {
    parse().map_err(|e| e.into())
}

// Good: Preserves error type
fn process() -> Result<Data, ParseError> {
    parse()
}
```

### 2. Over-Abstracting
```rust
// Bad: Premature abstraction
trait Handler<T, E> {
    fn handle(&self, input: T) -> Result<T, E>;
}

// Good: Concrete first
fn handle_request(input: Request) -> Result<Response, Error> {
    // ...
}
```

### 3. Breaking Iterator Chains
```rust
// Bad: Materialized unnecessarily
let items: Vec<_> = data.iter().filter(|x| x.valid).collect();
let count = items.len();

// Good: Stay lazy
let count = data.iter().filter(|x| x.valid).count();
```

## Incremental Refactoring Strategy

### 1. Parallel Implementation
```rust
// Keep old version during transition
mod old_parser {
    pub fn parse(input: &str) -> OldResult { }
}

mod new_parser {
    pub fn parse(input: &str) -> NewResult { }
}

// Temporarily run both
fn parse_with_verification(input: &str) -> NewResult {
    let old = old_parser::parse(input);
    let new = new_parser::parse(input);
    assert_eq!(old.normalize(), new.normalize());
    new
}
```

### 2. Feature Flag Migration
```rust
#[cfg(feature = "new-parser")]
pub use new_parser::parse;

#[cfg(not(feature = "new-parser"))]
pub use old_parser::parse;
```

### 3. Deprecation Path
```rust
#[deprecated(since = "2.0.0", note = "Use `new_function` instead")]
pub fn old_function() {
    new_function()
}
```

## Command Reference

```bash
# Analyze before refactoring
cargo clippy -- -W clippy::all
cargo test
cargo bench --no-run  # Ensure benchmarks compile

# During refactoring
cargo check           # Fast compilation check
cargo test specific_test -- --nocapture  # Debug specific test

# Validate refactoring
cargo test
cargo clippy -- -D warnings
cargo +nightly fmt -- --check
cargo mutants  # Ensure tests still effective

# Find improvement opportunities
cargo outdated
cargo machete  # Unused dependencies
cargo +nightly udeps  # More thorough unused deps
```

## Git Workflow for Refactoring

```bash
# Create refactoring branch
git checkout -b refactor/extract-parsing-module

# Make atomic commits
git add -p  # Stage selectively
git commit -m "refactor: extract parse_event to parsing module"

# One refactoring type per commit
git commit -m "refactor: rename EventType to EventKind"
git commit -m "refactor: extract validation functions"

# Squash only related fixes
git rebase -i main  # Clean up history before merging
```

## Refactoring Prioritization

1. **High Value**: Extract modules from large files (>500 lines)
2. **High Value**: Consolidate error handling
3. **Medium Value**: Extract common patterns into functions
4. **Medium Value**: Improve naming consistency
5. **Low Value**: Restructure for aesthetic reasons
6. **Avoid**: Refactoring without tests

Remember: The best refactoring is often no refactoring. Only refactor when it provides clear value in terms of maintainability, performance, or correctness.