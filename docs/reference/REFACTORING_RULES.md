# Refactoring Rules

Contract, mechanics, and Rust-specific patterns for behaviour-preserving code changes. For the research and rationale behind these rules, see [Refactoring Explained](../explanation/REFACTORING_EXPLAINED.md).

These rules apply to an explicitly requested behaviour-preserving change; they do not automatically invoke a workflow. Read the applicable [architecture constraints](ARCHITECTURE.md) and [selected examples](../code-truth/PATTERNS.md#selected-first-party-examples) before choosing a transformation.

## Scope contract

- Preserve the agreed externally observable behaviour and compatibility. Do not add features, bug fixes or optimizations under a behaviour-preserving scope.
- Inspect current requirements, decisions, code, tests and the working-tree diff first. Record existing failures; current code and tests are evidence, not automatically the requirement.
- Choose small transformations and validate the affected boundary after each step. Tests may move with code or gain requirement-derived coverage without changing the accepted behaviour.
- On a failure, stop and diagnose before continuing. Preserve unrelated work; rollback is limited to your own edits and must not discard pre-existing changes.

## Core Definition

**Martin Fowler:** "A disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior."

**Key principles:**
- Small behaviour-preserving transformations
- System fully working after each change
- Requirements and explicit owner decisions define the contract; tests provide incomplete, sometimes stale evidence of it

## Research context (not acceptance criteria)

The figures below are retained from earlier guidance. Their sources and applicability were not reverified during standards adoption; they do not establish current model capability or require a particular prompting method. See [Refactoring Explained](../explanation/REFACTORING_EXPLAINED.md) for the earlier narrative, not operational authority.

### Prompt strategy

Research (2024–2026) demonstrates that *how* you prompt matters enormously:

| Strategy | Effect |
|----------|--------|
| Specify the refactoring type explicitly | Success jumps from 15.6% → 86.7% (Liu et al., 2024) |
| One-shot/few-shot with before/after examples | Significant correctness gains, fewer hallucinations |
| Chain-of-thought with refactoring definitions | Higher test pass rates, more diverse transformations |
| Restrict context to relevant code only | Reduces overrefactoring and hallucinations |
| Sample multiple generations (pass@5) | Unit test pass rates up 28.8% (Cordeiro et al., 2024) |

**Practical default:** Name the intended transformation and preserved contract rather than requesting unbounded cleanup.

### Reported strengths and weaknesses

Earlier guidance summarized the following reported strengths and weaknesses (Cordeiro et al., 2024; Emergent Mind survey, Jan 2026). Treat them as source-specific historical claims, not a precise competence boundary for the agent doing this task:

#### Reported strengths:
- Magic Number elimination
- Long Statement splitting
- Extract Method (localized)
- Rename Variable/Function (mechanical)
- Systematic, repetitive transformations
- Code smell reduction: 44.4% (LLM) vs 24.3% (developer)

#### Reported weaknesses:
- Cross-module/architectural refactoring
- Context-dependent transformations requiring domain knowledge
- Refactorings touching core algorithms
- Multi-file package reorganisations
- Hallucination rate: 6–8% of unfiltered outputs break behaviour

#### Reported overreach:
- "Improving" already-clean code (overrefactoring)
- Adding validation or error handling during moves
- "Modernising" patterns that work correctly
- Dropping comments and metadata

## Catalog of Safe Refactoring Mechanics

### 1. Move Function (Between Files)

```
MECHANICAL COPY-DELETE METHOD:
Step 1: Identify the complete function, supporting items and matching tests
Step 2: Move those items without rewriting their behaviour
Step 3: DELETE moved functions from original, add module + imports
Step 4: Run tests — must pass unchanged
```

**Rust-specific notes (learned from applying these rules):**

- **Import resolution is the hard part.** In a binary crate that depends on its own library crate, you must distinguish `crate::` paths (binary-private modules) from `your_crate_name::` paths (library public modules). This is where most cognitive effort goes — not the copy-delete itself.
- **Keep visibility narrow.** A move may need `pub(super)` or `pub(crate)`, but does not automatically justify a public API. Library/binary crossings can require public access; identify compatibility effects explicitly rather than treating expansion as a blanket exception.
- **Decide what moves together before starting.** Group functions by cohesion (shared data, shared callers, shared domain concept). The mechanical method covers *how* to move but not *what* belongs together — that's a judgment call you make up front.
- **Move private unit tests with their owning logic when appropriate.** An adjacent `#[cfg(test)]` module can retain `super::*` access in the destination. Do not expose implementation details solely to preserve a test's old location.

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

## Scope and validation

Separate opportunistic improvements from the requested transformation. Discovering
a stale test, bug or unsuitable abstraction is a reason to investigate and report,
not permission to change behaviour or a reason to ignore contradictory evidence.

1. Establish the relevant baseline and preserve the pre-existing diff.
2. Run focused tests after a bounded step, then the applicable default-feature,
   formatting and lint gates before completion. Use the isolation and prerequisite
   rules in the [testing guide](../howto/TESTING_GUIDE.md); do not run the suite
   against normal user state merely because a refactor is small.
3. Test the preserved contract, including prohibited cases where consequential.
   Do not weaken an assertion or regenerate a baseline simply to obtain a pass.
4. A formula or threshold change is not behaviour-preserving. If explicitly
   authorized separately, follow the populated-data accuracy requirements and
   report before/after MAE; synthetic checks do not replace that evidence.
5. Use targeted mutation testing when consequential logic needs stronger
   fault-detection evidence. It is not mandatory after every refactor.
6. Report executed checks, existing failures and unverified boundaries separately.
   Known formatting/lint failures remain visible; broad suppressions are not a fix.

Cross-module moves, ownership changes, algorithm replacement and pipeline rewrites
need a bounded design and stronger regression coverage than a rename. Split the
work where useful; seek approval for changed behaviour or material scope, not
merely because a catalog gives the transformation a complex name.

## Recovery protocol

When a check fails or produces an unexpected result:

1. Stop, inspect the diagnostic and state the revised plan.
2. Distinguish a regression from a pre-existing failure, stale expectation,
   missing prerequisite or live-service failure. Do not assume any one of them.
3. Fix a regression within the agreed behaviour-preserving scope, or roll back
   only your own bounded edits. Never reset the dirty checkout or revert another
   contributor's work.
4. If resolving the failure requires changed requirements or behaviour, preserve
   completed work and ask for that scope. Record any unperformed acceptance check.

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

### Optional tools

Use the installed toolchain first. The commands below change the local toolchain;
install only when needed and authorized, not as a routine refactoring prerequisite.

```bash
cargo install cargo-edit         # Add/remove/upgrade dependencies
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-mutants      # Mutation testing
rustup component add clippy rustfmt
```

### Convenience aliases (not acceptance gates)

These do not isolate application state or qualify accuracy. Use the testing
guide's wrappers and applicable gates for validation; watchers are opt-in
long-running jobs, not required checks.

```bash
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt -- --check'
alias ccl='cargo clippy -- -W clippy::all'
alias cw='cargo watch -x check -x test -x clippy'
```

## The Golden Rule

**Preserve the agreed contract, investigate failures, and preserve unrelated work.**

A passing suite supports behaviour preservation; it does not prove correctness or turn stale tests into requirements.
