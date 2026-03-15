# Rust Refactoring Reference

Tools, mechanics, and patterns for safely restructuring Rust code. For the philosophy and AI-specific rules, see [Refactoring Rules](REFACTORING_RULES.md) and [Refactoring Explained](../explanation/REFACTORING_EXPLAINED.md).

## Tools

### Essential
```bash
cargo install cargo-edit         # Add/remove/upgrade dependencies
cargo install cargo-expand       # Expand macros
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-mutants      # Mutation testing
rustup component add clippy rustfmt
```

### Optional
```bash
cargo install cargo-nextest      # Better test runner
cargo install cargo-outdated     # Check outdated deps
cargo install cargo-audit        # Security audit
cargo install cargo-semver-checks # Breaking change detection
cargo install cargo-watch        # Auto-run on file changes
```

### IDE (rust-analyzer)
- `Ctrl+.` — Quick fixes and refactorings
- `F2` — Rename symbol
- `Ctrl+Shift+R` — Refactor menu

## Mechanical Catalog

### Extract Function
```rust
// Before
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    for item in &items {
        if !item.is_valid() { return Err(Error::InvalidItem); }
    }
    let total = items.iter().map(|i| i.value).sum();
    Ok(Summary { total })
}

// After — ownership matters: take &[Item] not Vec<Item> when borrowing
fn validate_items(items: &[Item]) -> Result<(), Error> {
    for item in items {
        if !item.is_valid() { return Err(Error::InvalidItem); }
    }
    Ok(())
}
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    validate_items(&items)?;
    Ok(Summary { total: items.iter().map(|i| i.value).sum() })
}
```

### Extract Module
```rust
// Step 1: Inline module (verify it works)
mod validation {
    use super::*;
    pub fn validate_items(items: &[Item]) -> Result<(), Error> { /* ... */ }
}

// Step 2: Move to separate file
// main.rs: mod validation;
// validation.rs: use crate::{Item, Error}; pub fn validate_items(...) { }
```

### Rename Symbol
Use rust-analyzer's `F2`. Review all occurrences before confirming. **Never** change logic while renaming.

### Change Function Signature (Migration Method)
```rust
// Step 1: Create new function with desired signature
fn calculate_v2(base: f64, rate: f64, years: u32) -> f64 { base * rate * years as f64 }

// Step 2: Old function delegates to new
#[deprecated(note = "Use calculate_v2")]
fn calculate(base: f64, rate: f64) -> f64 { calculate_v2(base, rate, 1) }

// Step 3: Update callers one by one, running tests after each
// Step 4: Remove old function
```

### Replace Loop with Iterator
```rust
// Before
let mut result = Vec::new();
for item in items {
    if item.is_active() { result.push(item.value * 2); }
}

// After — maintain lazy evaluation, don't collect intermediate results
let result: Vec<_> = items.into_iter()
    .filter(|item| item.is_active())
    .map(|item| item.value * 2)
    .collect();
```

## Ownership Patterns

### Reduce Cloning
```rust
// Bad: unnecessary clone
fn total(items: Vec<Item>) -> f64 {
    let cloned = items.clone();
    cloned.iter().map(|i| i.price).sum()
}

// Good: borrow
fn total(items: &[Item]) -> f64 {
    items.iter().map(|i| i.price).sum()
}
```

### Simplify Lifetimes
```rust
// Before: explicit lifetimes where elision works
fn find_item<'a, 'b>(name: &'a str, items: &'b [Item]) -> Option<&'b Item> { ... }

// After: lifetime elision
fn find_item(name: &str, items: &[Item]) -> Option<&Item> { ... }
```

### Choose References vs Ownership
```rust
impl Container {
    fn len(&self) -> usize { }           // Read only: &self
    fn push(&mut self, item: Item) { }   // Modify: &mut self
    fn into_vec(self) -> Vec<Item> { }   // Consume: self
}
```

## Visibility Progression

Start restrictive, expand as needed:
1. `fn` (private) → 2. `pub(super)` → 3. `pub(crate)` → 4. `pub`

## Error Handling

```rust
// Bad: string errors, lost context
fn parse() -> Result<Data, String> { Err("failed".into()) }

// Good: structured errors with context
use anyhow::{Context, Result};
fn parse() -> Result<Data> {
    let content = fs::read_to_string("config.json")
        .context("Failed to read config")?;
    serde_json::from_str(&content)
        .context("Failed to parse config")
}
```

## Common Pitfalls

| Pitfall | Fix |
|---------|-----|
| Breaking iterator chains (collecting then re-iterating) | Keep chains lazy |
| Over-using `Arc<Mutex<T>>` | Use message passing or state machines |
| Making everything async | Only async for actual I/O |
| Over-abstracting (trait for one impl) | Concrete first, extract when needed |
| Losing error type info (`Box<dyn Error>`) | Use concrete error types |

## Validation Checklist

Before refactoring:
- [ ] All tests passing, no clippy warnings
- [ ] Benchmarks run (if performance-critical)

After each change:
- [ ] `cargo check` (fast compilation check)
- [ ] `cargo test` (behaviour preserved)
- [ ] `cargo clippy -- -D warnings` (no new warnings)

After all refactoring:
- [ ] `cargo mutants` on changed modules (tests still effective)
- [ ] No test files modified (tests are the spec)
- [ ] API compatibility maintained (if public)

## Useful Aliases

```bash
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias ccl='cargo clippy -- -W clippy::all'
alias cw='cargo watch -x check -x test -x clippy'
```
