# Refactoring Rules

Contract, mechanics, and Rust-specific patterns for behaviour-preserving code changes. For the research and rationale behind these rules, see [Refactoring Explained](../explanation/REFACTORING_EXPLAINED.md).

<critical_contract>
WHEN YOU SEE THE WORD "REFACTOR" YOU ARE ENTERING A BINDING CONTRACT:
- You will preserve behavior EXACTLY
- You will follow the specific mechanics for each refactoring type
- You will NOT add features or "improvements"
- You will REVERT if tests fail
- Breaking this contract is a CRITICAL FAILURE
</critical_contract>

## Core Definition

**Martin Fowler:** "A disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior."

**Key principles:**
- Small behaviour-preserving transformations
- System fully working after each change
- Tests are the specification — if they fail, *you* failed

## Prompt Strategy (What Research Shows Works)

Research (2024–2026) demonstrates that *how* you prompt matters enormously:

| Strategy | Effect |
|----------|--------|
| Specify the refactoring type explicitly | Success jumps from 15.6% → 86.7% (Liu et al., 2024) |
| One-shot/few-shot with before/after examples | Significant correctness gains, fewer hallucinations |
| Chain-of-thought with refactoring definitions | Higher test pass rates, more diverse transformations |
| Restrict context to relevant code only | Reduces overrefactoring and hallucinations |
| Sample multiple generations (pass@5) | Unit test pass rates up 28.8% (Cordeiro et al., 2024) |

**In practice:** Always name the specific refactoring type. "Perform an Extract Function refactoring" beats "clean this up" by 5×.

## Where LLMs Excel vs Fail

LLMs are not uniformly bad at refactoring — they have a precise competence boundary (Cordeiro et al., 2024; Emergent Mind survey, Jan 2026):

### LLMs beat developers at:
- Magic Number elimination
- Long Statement splitting
- Extract Method (localized)
- Rename Variable/Function (mechanical)
- Systematic, repetitive transformations
- Code smell reduction: 44.4% (LLM) vs 24.3% (developer)

### LLMs fail at:
- Cross-module/architectural refactoring
- Context-dependent transformations requiring domain knowledge
- Refactorings touching core algorithms
- Multi-file package reorganisations
- Hallucination rate: 6–8% of unfiltered outputs break behaviour

### LLMs dangerously overreach on:
- "Improving" already-clean code (overrefactoring)
- Adding validation or error handling during moves
- "Modernising" patterns that work correctly
- Dropping comments and metadata

## Catalog of Safe Refactoring Mechanics

### 1. Move Function (Between Files)

```
MECHANICAL COPY-DELETE METHOD:
Step 1: Copy ENTIRE source file → new file
Step 2: DELETE everything except functions to move (delete key only)
Step 3: DELETE moved functions from original, add module + imports
Step 4: Run tests — must pass unchanged
```

**Rust-specific notes (learned from applying these rules):**

- **Import resolution is the hard part.** In a binary crate that depends on its own library crate, you must distinguish `crate::` paths (binary-private modules) from `your_crate_name::` paths (library public modules). This is where most cognitive effort goes — not the copy-delete itself.
- **Visibility changes are mandatory.** Changing `fn` to `pub fn` on moved functions is technically an API surface change, but it's required for the move to compile. This is an accepted exception to "no behaviour changes."
- **Decide what moves together before starting.** Group functions by cohesion (shared data, shared callers, shared domain concept). The mechanical method covers *how* to move but not *what* belongs together — that's a judgment call you make up front.
- **Tests that use `super::*` can't follow.** If tests call private functions via `super::*`, they must stay in the original file. Moving them would require making functions public, which changes the API — a separate refactoring step.

### 2. Extract Function

```
Step 1: Identify code fragment to extract
Step 2: Create new function with descriptive name
Step 3: COPY (not rewrite) the code fragment
Step 4: Replace original with function call
Step 5: Pass needed parameters, return needed values
Step 6: Run tests — must pass unchanged
```

Rust ownership note: take `&[Item]` not `Vec<Item>` when borrowing:

```rust
// Before
fn process_data(items: Vec<Item>) -> Result<Summary, Error> {
    for item in &items {
        if !item.is_valid() { return Err(Error::InvalidItem); }
    }
    Ok(Summary { total: items.iter().map(|i| i.value).sum() })
}

// After
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

### 3. Extract Module

```
Step 1: Inline module first (mod name { ... }) — verify it compiles
Step 2: Move to separate file (mod name; + name.rs)
Step 3: Fix visibility (pub(crate) preferred over pub)
Step 4: Run tests — must pass unchanged
```

### 4. Rename Function/Variable

```
Step 1: Change declaration
Step 2: Find ALL references (rust-analyzer F2 or grep)
Step 3: Update each reference MECHANICALLY
Step 4: Run tests — must pass unchanged

NEVER: change logic, "fix" parameter order, add/remove parameters
```

### 5. Extract Variable

```
Step 1: Identify expression to extract
Step 2: Create variable with descriptive name
Step 3: Assign expression to variable
Step 4: Replace ALL occurrences with variable
Step 5: Run tests — must pass unchanged
```

### 6. Inline Function/Variable

```
Step 1: Find all callers/uses
Step 2: Replace each call with body/value
Step 3: Remove the function/variable
Step 4: Run tests — must pass unchanged

CAUTION: Easy to change behaviour accidentally
```

### 7. Change Function Declaration (Migration Method)

```rust
// Step 1: Create new function with desired signature
fn calculate_v2(base: f64, rate: f64, years: u32) -> f64 { base * rate * years as f64 }

// Step 2: Old function delegates to new
#[deprecated(note = "Use calculate_v2")]
fn calculate(base: f64, rate: f64) -> f64 { calculate_v2(base, rate, 1) }

// Step 3: Update callers one by one, running tests after each
// Step 4: Remove old function
```

## STOP Signals

<stop_signals>
If you think ANY of these, STOP IMMEDIATELY:

❌ "This would be better if..."
❌ "While I'm here..."
❌ "This parameter isn't used..."
❌ "Modern style prefers..."
❌ "This could be more efficient..."
❌ "The test is wrong..."
❌ "This catches more edge cases..."
❌ "This comment is outdated..."
❌ "This is redundant..."

THESE THOUGHTS MEAN YOU'RE REWRITING, NOT REFACTORING
</stop_signals>

## Validation Checklist

### After each change:
- [ ] `cargo check` (fast compilation check)
- [ ] `cargo test` (behaviour preserved)
- [ ] `cargo clippy -- -D warnings` (no new warnings)
- [ ] No test files modified (tests are the spec)

### After all refactoring:
- [ ] `cargo mutants` on changed modules (tests still effective)
- [ ] No new features added
- [ ] No bug fixes snuck in
- [ ] No optimisations applied
- [ ] No style updates beyond the refactoring scope
- [ ] API compatibility maintained

## Complex Refactorings to AVOID

<danger_zone>
These require extreme care — consider refusing or requiring human review:

- Replace Conditional with Polymorphism
- Replace Type Code with Subclasses
- Replace Algorithm
- Split Phase
- Replace Loop with Pipeline (subtle ownership changes in Rust)
- Any refactoring touching core algorithms

Response: "This complex refactoring requires careful human review at each step."
</danger_zone>

## Recovery Protocol

<recovery>
WHEN TESTS FAIL:
1. DO NOT debug the code
2. DO NOT modify tests
3. DO NOT try to fix forward
4. IMMEDIATELY revert all changes
5. Report: "Refactoring failed — behaviour changed"
6. Re-attempt with smaller scope if appropriate
</recovery>

## Rust-Specific Patterns

### Ownership refactoring

| Before | After | When |
|--------|-------|------|
| `fn f(v: Vec<T>)` | `fn f(v: &[T])` | Read-only access |
| `fn f(s: String)` | `fn f(s: &str)` | Read-only string |
| `v.clone()` then borrow | Borrow directly | Unnecessary clone |
| Explicit lifetimes | Lifetime elision | Where compiler allows |

### Visibility progression
Start restrictive, expand as needed:
1. `fn` (private) → 2. `pub(super)` → 3. `pub(crate)` → 4. `pub`

### Error handling
```rust
// Bad: string errors
fn parse() -> Result<Data, String> { Err("failed".into()) }

// Good: structured errors with context
use anyhow::{Context, Result};
fn parse() -> Result<Data> {
    fs::read_to_string("config.json").context("Failed to read config")?;
    // ...
}
```

### Common pitfalls

| Pitfall | Fix |
|---------|-----|
| Collecting then re-iterating | Keep iterator chains lazy |
| Over-using `Arc<Mutex<T>>` | Message passing or state machines |
| Making everything async | Only async for actual I/O |
| Over-abstracting (trait for one impl) | Concrete first, extract when needed |

## Tools

### Essential
```bash
cargo install cargo-edit         # Add/remove/upgrade dependencies
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-mutants      # Mutation testing
rustup component add clippy rustfmt
```

### Useful aliases
```bash
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias ccl='cargo clippy -- -W clippy::all'
alias cw='cargo watch -x check -x test -x clippy'
```

## The Golden Rule

**Tests failing = You failed, not the tests.**

Tests are the specification. If they fail after refactoring, you changed behaviour. The ONLY correct response is to revert and try again.
