# Understanding Refactoring: A Human's Guide

This document explains the full scope of refactoring, why AI assistants struggle with it, and how the REFACTORING_RULES.md addresses these challenges.

## What Refactoring Really Is

According to Martin Fowler (refactoring.com), refactoring is "a disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior."

The key phrase: **"without changing its external behavior"**

## The Refactoring Catalog

Refactoring isn't just moving functions between files. Fowler's catalog includes over 60 different refactorings, grouped into categories:

### Basic Refactorings
- **Extract Function**: Pull code into a new function
- **Inline Function**: Replace function calls with the function body
- **Extract Variable**: Name a complex expression
- **Inline Variable**: Replace variable references with the expression
- **Rename Function/Variable**: Change names for clarity

### Moving Features
- **Move Function**: Relocate to a better module/class
- **Move Field**: Relocate data to a better structure
- **Move Statements into Function**: Consolidate related code
- **Move Statements to Callers**: Push code up to callers

### Organizing Data
- **Replace Primitive with Object**: int age → Age class
- **Replace Array with Object**: data[0] → data.name
- **Encapsulate Variable**: Direct access → getter/setter

### Simplifying Conditionals
- **Decompose Conditional**: Complex if → multiple functions
- **Consolidate Conditional Expression**: Multiple ifs → single if
- **Replace Nested Conditional with Guard Clauses**: Deep nesting → early returns

### Dealing with Inheritance
- **Pull Up Method/Field**: Move to parent class
- **Push Down Method/Field**: Move to subclasses
- **Replace Subclass with Delegate**: Inheritance → composition

## Why AI Assistants Fail at Refactoring

### The Core Problem: Improvement Bias

When an AI sees code to refactor, multiple trained behaviors activate:

1. **Code Review Mode**: "Is this code good? How can I improve it?"
2. **Problem Solving Mode**: "What issues can I fix?"
3. **Modernization Mode**: "Is this using current best practices?"
4. **Efficiency Mode**: "Can this be optimized?"

These are helpful 95% of the time, but catastrophic during refactoring.

### Real Examples of AI Refactoring Failures

#### Example 1: Lost Functionality During Move
```rust
// Original (handles multiple formats)
fn parse_distance_from_description(description: &Option<String>) -> Option<f64> {
    if let Some(desc) = description {
        // Parse "Distance: 10 km" format
        let distance_re = Regex::new(r"Distance:\s*(\d+(?:\.\d+)?)\s*(km|miles?)").unwrap();
        if let Some(captures) = distance_re.captures(desc) {
            let value = captures[1].parse::<f64>().ok()?;
            let unit = &captures[2];
            return Some(if unit.starts_with("mile") {
                value * 1.60934  // Convert miles to km
            } else {
                value
            });
        }
        // Fallback to simple parsing
        parse_distance_from_name(desc)
    } else {
        None
    }
}

// AI's "refactored" version (lost regex parsing and miles conversion)
fn parse_distance_from_description(description: &Option<String>) -> Option<f64> {
    description.as_ref().and_then(|desc| parse_distance_from_name(desc))
}
```

#### Example 2: Added Features During Extract
```rust
// Task: Extract the validation logic
fn process_age(age: i32) {
    if age >= 0 && age <= 150 {
        self.age = age;
    }
}

// AI's extraction (added new validation!)
fn validate_age(age: i32) -> bool {
    age >= 0 && age <= 150 && age != 13  // Added "unlucky number" check!
}
```

#### Example 3: "Fixed" Edge Cases During Rename
```rust
// Task: Rename 'get_value' to 'fetch_value'
fn get_value(key: &str) -> Option<String> {
    self.map.get(key).cloned()
}

// AI's version (added "helpful" default)
fn fetch_value(key: &str) -> Option<String> {
    self.map.get(key).cloned().or_else(|| Some(String::new()))
}
```

## How Different Refactorings Challenge AI

### Move Function (Easiest)
- **Challenge**: Temptation to "clean up" while moving
- **Solution**: Mechanical copy-delete process

### Extract Function (Moderate)
- **Challenge**: Deciding what's "better" extraction
- **Solution**: Copy exact code, no rewrites

### Rename (Moderate)
- **Challenge**: Fixing "related issues" during rename
- **Solution**: Change names ONLY, nothing else

### Extract Variable (Moderate)
- **Challenge**: Simplifying the expression
- **Solution**: Extract exactly as-is

### Change Function Declaration (Hard)
- **Challenge**: "Improving" the API
- **Solution**: Migration method with old calling new

### Replace Conditional with Polymorphism (Very Hard)
- **Challenge**: Complete restructuring
- **Solution**: Often better refused by AI

## The Mechanical Process Solution

For each refactoring type, we define mechanical steps that remove decision points:

1. **No Rewriting**: Copy code exactly, character for character
2. **No Decisions**: Follow steps mechanically
3. **No Improvements**: Even obvious ones
4. **Test Driven**: Tests pass or refactoring fails

## Understanding the Rules Document

### Critical Contract
Creates psychological commitment - this isn't normal coding, it's a special mode.

### Specific Mechanics
Each refactoring type has exact steps, like a recipe. No improvisation allowed.

### STOP Signals
Catches dangerous thoughts before they become code:
- "While I'm here..." → STOP
- "This would be better..." → STOP
- "Modern style..." → STOP

### Failure Examples
Real examples of how refactoring goes wrong, making the danger concrete.

### Recovery Protocol
When tests fail, no debugging allowed. Only revert. This prevents "fixing forward" which often changes more behavior.

## When to Use These Rules

### Good for AI-Assisted Refactoring:
- Move Function/Method
- Extract Function/Method
- Extract Variable
- Rename (with care)
- Simple inlines

### Require Human Oversight:
- Change Function Declaration
- Introduce Parameter Object
- Pull Up/Push Down Method
- Encapsulate Variable

### Better Done by Humans:
- Replace Conditional with Polymorphism
- Replace Algorithm
- Split Phase
- Any refactoring touching core logic

## How to Activate the Refactoring Rules

To ensure Claude follows the refactoring rules instead of modifying code:

### Option 1: Direct Reference (Most Reliable)
Simply mention the rules when asking for refactoring:
```
"Please refactor this code following REFACTORING_RULES.md"
```

### Option 2: Use the Magic Word with Context
When you say "refactor", Claude should recognize it as entering the contract, but you can reinforce:
```
"I need you to refactor (not rewrite) these functions to a new module"
```

### Option 3: Quote the Contract
Start your request with the contract to activate the mindset:
```
"Remember: preserve behavior EXACTLY. Now refactor..."
```

### Option 4: Specify the Refactoring Type
Use the specific names from the catalog:
```
"Perform a Move Function refactoring to move parse_* functions to parsing.rs"
"Do an Extract Function refactoring on this validation logic"
```

### Option 5: Add to Project's CLAUDE.md
For permanent activation in a project, add to CLAUDE.md:
```markdown
## Refactoring Discipline
When asked to refactor, ALWAYS follow REFACTORING_RULES.md.
See REFACTORING_EXPLAINED.md for why this matters.
```

### What Triggers the Rules

The rules should activate when Claude sees:
- The word "refactor" (vs "rewrite", "improve", "fix")
- References to the rules file
- Specific refactoring type names
- The contract language

### If Claude Starts Modifying Code

Interrupt immediately:
```
"STOP - you're changing behavior. Follow REFACTORING_RULES.md"
```

### Example of a Good Refactoring Request
```
"Please perform a Move Function refactoring to move parse_distance_from_description 
and its tests from main.rs to a new parsing.rs module. Use the mechanical 
copy-delete method from REFACTORING_RULES.md"
```

The key is being explicit that you want refactoring (structure change only) not rewriting (behavior change).

## The Paradox of AI Refactoring

AI excels at:
- Understanding code intent
- Suggesting improvements
- Finding bugs
- Modernizing patterns

But these strengths become weaknesses during refactoring, where the goal is to change NOTHING about behavior.

The solution isn't to make AI "understand" refactoring better. It's to create mechanical processes that prevent the AI from using its "intelligence" in ways that break the refactoring contract.

## Key Takeaways

1. **Refactoring is about structure, not behavior**
2. **AI's helpful nature is harmful during refactoring**
3. **Mechanical processes prevent thinking/improving**
4. **Tests are the only source of truth**
5. **Different refactorings need different mechanics**
6. **Some refactorings are too complex for AI**

## Final Thought

The irony: We're using AI's intelligence to create processes that prevent it from being intelligent. But that's exactly what safe refactoring requires - mechanical transformation without creative interpretation.

Remember Martin Fowler's wisdom: First refactor to make the change easy (this might be hard), then make the easy change. Never mix refactoring with feature changes.