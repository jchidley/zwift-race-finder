# Refactoring Rules for Claude

<critical_contract>
WHEN YOU SEE THE WORD "REFACTOR" YOU ARE ENTERING A BINDING CONTRACT:
- You will preserve behavior EXACTLY
- You will follow the specific mechanics for each refactoring type
- You will NOT add features or "improvements"
- You will REVERT if tests fail
- Breaking this contract is a CRITICAL FAILURE
</critical_contract>

## Core Definition

**Martin Fowler (refactoring.com):**
"A disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior"

**Key Principles:**
- Small behavior-preserving transformations
- System fully working after each change
- Tests are the specification

## Catalog of Safe Refactoring Mechanics

### 1. Move Function (Between Files)

<mechanics>
MECHANICAL COPY-DELETE METHOD:

Step 1: Copy ENTIRE source file
```bash
cp src/main.rs src/parsing.rs
```

Step 2: DELETE everything except functions to move
- Use ONLY delete key
- Keep function EXACTLY as is

Step 3: DELETE moved functions from original
- Add module declaration
- Add imports

Step 4: Run tests - must pass unchanged
</mechanics>

### 2. Extract Function

<mechanics>
Step 1: Identify code fragment to extract
Step 2: Create new function with descriptive name
Step 3: COPY (not rewrite) the code fragment
Step 4: Replace original with function call
Step 5: Pass needed parameters
Step 6: Return needed values
Step 7: Run tests - must pass unchanged
</mechanics>

### 3. Rename Function/Variable

<mechanics>
Step 1: Change declaration
Step 2: Find ALL references (use IDE/grep)
Step 3: Update each reference MECHANICALLY
Step 4: Run tests - must pass unchanged

NEVER:
- Change logic while renaming
- "Fix" parameter order
- Add/remove parameters
</mechanics>

### 4. Extract Variable

<mechanics>
Step 1: Identify expression to extract
Step 2: Create variable with descriptive name
Step 3: Assign expression to variable
Step 4: Replace ALL occurrences with variable
Step 5: Run tests - must pass unchanged

Example:
```rust
// Before
if (order.quantity * order.item_price > 1000) { ... }

// After
let base_price = order.quantity * order.item_price;
if (base_price > 1000) { ... }
```
</mechanics>

### 5. Inline Function/Variable

<mechanics>
Step 1: Find all callers/uses
Step 2: Replace each call with body/value
Step 3: Remove the function/variable
Step 4: Run tests - must pass unchanged

CAUTION: Easy to change behavior accidentally
</mechanics>

### 6. Change Function Declaration

<mechanics>
MIGRATION METHOD (safer):

Step 1: Create new function with desired signature
Step 2: COPY old function body
Step 3: Update old function to call new
Step 4: Find and update each caller
Step 5: Remove old function
Step 6: Run tests after EACH step
</mechanics>

## Universal STOP Signals

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

## Validation for ALL Refactorings

<validation>
□ All tests pass WITHOUT modification
□ No test files changed
□ Behavior identical (not just "equivalent")
□ No new features
□ No bug fixes
□ No optimizations
□ No style updates
</validation>

## Complex Refactorings to AVOID

<danger_zone>
These require extreme care - consider refusing:

- Replace Conditional with Polymorphism
- Replace Type Code with Subclasses
- Replace Algorithm
- Split Phase
- Replace Loop with Pipeline

Response: "This complex refactoring requires careful human review at each step. I can provide the mechanics but should not execute automatically."
</danger_zone>

## Examples of Behavior Change (FAILURES)

<failures>
1. ADDING validation:
```rust
// Before
fn set_age(age: i32) { self.age = age; }

// WRONG refactor
fn set_age(age: i32) { 
    if age >= 0 { self.age = age; }  // Added validation!
}
```

2. FIXING edge cases:
```rust
// Before  
fn parse(s: &str) -> i32 { s.parse().unwrap() }

// WRONG refactor
fn parse(s: &str) -> i32 { 
    s.parse().unwrap_or(0)  // "Fixed" panic!
}
```

3. IMPROVING efficiency:
```rust
// Before
items.iter().filter(|x| x.active).collect::<Vec<_>>().len()

// WRONG refactor  
items.iter().filter(|x| x.active).count()  // "More efficient!"
```
</failures>

## Recovery Protocol

<recovery>
WHEN TESTS FAIL:
1. DO NOT debug the code
2. DO NOT modify tests
3. DO NOT try to fix
4. IMMEDIATELY revert all changes
5. Report: "Refactoring failed - behavior changed"
</recovery>

## The Refactoring Decision Tree

<decision_tree>
1. Is it a simple mechanical refactoring? → Use specific mechanics
2. Does it require creating new abstractions? → Proceed with extreme care
3. Does it touch core algorithms? → Consider refusing
4. Are there no tests? → REFUSE: "Cannot refactor without tests"
5. Do tests use mocks? → WARN: "Mock-based tests may hide behavior changes"
</decision_tree>

## Required Response Format

<response_template>
I will perform a [refactoring type] refactoring.

Refactoring Plan:
- Type: [Extract Function/Rename/Move Function/etc.]
- Scope: [What code is affected]
- Mechanics: [Specific steps from catalog]
- Validation: Tests must pass without modification

I understand that ANY behavior change means failure.
</response_template>

## The Golden Rule

**Tests failing = You failed, not the tests**

Tests are the specification. If they fail after refactoring, you changed behavior. The ONLY correct response is to revert and try again.

## Final Wisdom

Martin Fowler: "When you find you have to add a feature to a program, and the program's code is not structured in a convenient way to add the feature, first refactor the program to make it easy to add the feature, then add the feature."

The key: Refactoring is SEPARATE from feature addition. Never mix them.