# Understanding LLM-Driven Refactoring

Why AI assistants struggle with refactoring, what recent research reveals about when they excel, and how the [Refactoring Rules](../reference/REFACTORING_RULES.md) address these challenges. Updated March 2026 with findings from ICSE 2025 IDE workshop, Emergent Mind survey (Jan 2026), and empirical studies on StarCoder2/GPT-4o refactoring performance.

## What Refactoring Really Is

Martin Fowler defines refactoring as "a disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior."

The key phrase: **"without changing its external behavior."**

Fowler's catalog includes over 60 refactoring types across categories: extracting/inlining functions and variables, moving features between modules, organising data, simplifying conditionals, and restructuring inheritance. The scope is far broader than "moving code between files."

## The Improvement Bias Problem

When an LLM encounters code to refactor, multiple trained behaviours activate simultaneously:

1. **Code review mode**: "Is this code good? How can I improve it?"
2. **Problem solving mode**: "What issues can I fix?"
3. **Modernisation mode**: "Is this using current best practices?"
4. **Efficiency mode**: "Can this be optimised?"

These are helpful 95% of the time, but catastrophic during refactoring. Research quantifies this: **6–8% of unfiltered LLM refactoring outputs introduce semantic changes** — modifications that compile and look correct but alter behaviour (Liu et al., 2024; Cordeiro et al., 2024).

### Real Examples of Behaviour Change

**Lost functionality during move:**
```rust
// Original (handles multiple formats)
fn parse_distance(desc: &str) -> Option<f64> {
    let re = Regex::new(r"Distance:\s*(\d+(?:\.\d+)?)\s*(km|miles?)").unwrap();
    if let Some(caps) = re.captures(desc) {
        let value = caps[1].parse::<f64>().ok()?;
        return Some(if caps[2].starts_with("mile") { value * 1.60934 } else { value });
    }
    parse_distance_from_name(desc)  // fallback
}

// AI's "refactored" version (lost regex parsing and miles conversion)
fn parse_distance(desc: &str) -> Option<f64> {
    parse_distance_from_name(desc)
}
```

**Added features during extract:**
```rust
// Task: Extract the validation logic
fn process_age(age: i32) { if age >= 0 && age <= 150 { self.age = age; } }

// AI's extraction (added new validation!)
fn validate_age(age: i32) -> bool { age >= 0 && age <= 150 && age != 13 }
```

**"Fixed" edge cases during rename:**
```rust
// Task: Rename get_value → fetch_value
fn get_value(key: &str) -> Option<String> { self.map.get(key).cloned() }

// AI's version (added "helpful" default)
fn fetch_value(key: &str) -> Option<String> {
    self.map.get(key).cloned().or_else(|| Some(String::new()))
}
```

## What the Research Actually Shows (2024–2026)

The narrative that "LLMs can't refactor" is outdated. The reality is nuanced — LLMs have a precise competence boundary.

### LLMs outperform developers on systematic refactorings

The largest empirical study (Cordeiro et al., 2024 — 30 open-source Java projects, StarCoder2) found:

- **Code smell reduction rate**: LLM 44.4% vs developers 24.3% — a 20pp advantage
- LLMs excel at: Magic Number elimination, Long Statement splitting, Empty Catch Clause fixing, mechanical Extract Method
- Developers excel at: Broken Modularisation, Deficient Encapsulation, Multifaceted Abstraction — anything requiring architectural reasoning

### Prompt engineering has massive impact

Liu et al. (2024) on ChatGPT with Java:
- **Without specifying refactoring type**: 15.6% success
- **With explicit refactoring type in prompt**: 86.7% success — a 71pp improvement

This single finding justifies the entire prompt discipline in our [Refactoring Rules](../reference/REFACTORING_RULES.md). Saying "Perform an Extract Function refactoring" instead of "clean this up" is not pedantry — it's a 5× improvement.

Additional findings:
- One-shot prompting improves test pass rate by 6.15% over zero-shot (Cordeiro et al., 2024)
- Sampling 5 generations (pass@5) raises unit test pass rate by 28.8% over single generation
- Chain-of-thought prompting increases both smell reduction and functional correctness

### Multi-agent architectures dramatically improve safety

Multi-agent refactoring systems (RefAgent, MANTRA) decompose refactoring into pipelined stages — planning, generation, compilation, testing, and self-reflection — handled by specialised agents:

- **Self-reflection loops** (iterative re-prompting on compile/test errors) raise functional correctness by **40–65 percentage points** over naive single-shot output (Oueslati et al., 2025)
- RefAgent achieves 90% unit test pass rate with 52.5% smell reduction
- Multi-agent RAG systems achieve 82.8% compile+pass rate vs 8.7% baseline (Xu et al., 2025)

This maps directly to our recovery protocol: revert and retry with smaller scope is the manual equivalent of what multi-agent systems do automatically.

### The overrefactoring problem is real

LLMs don't just fail by breaking behaviour — they also fail by modifying code that doesn't need modification (Shirafuji et al., 2023; Midolo et al., Jan 2026):

- Rewriting already-clean expressions in "more modern" style
- Dropping comments and documentation during transformations
- Renaming variables to non-idiomatic alternatives
- Reducing readability while "improving" structure

This is why our STOP signals exist. "While I'm here..." is the thought pattern that produces overrefactoring.

## The Competence Boundary

| Domain | LLM Capability | Human Oversight |
|--------|---------------|-----------------|
| Rename Variable/Function | ✅ Excellent | Low — mechanical |
| Extract Function (localised) | ✅ Very good | Low — verify scope |
| Magic Number → Constant | ✅ Excellent | Low |
| Long Statement splitting | ✅ Very good | Low |
| Move Function between files | ⚠️ Good with discipline | Medium — watch for "improvements" |
| Change Function Declaration | ⚠️ Moderate | Medium — migration method required |
| Cross-module restructuring | ❌ Poor | High — architectural reasoning needed |
| Replace Algorithm | ❌ Dangerous | Very high — behaviour change likely |
| Replace Conditional with Polymorphism | ❌ Unreliable | Very high — complete restructuring |

## How Different Refactorings Challenge LLMs

### Move Function (Moderate risk)
- **Challenge**: Temptation to "clean up" while moving
- **Solution**: Mechanical copy-delete process — no rewriting
- **Research note**: LLMs succeed when context is restricted to source and target files only

### Extract Function (Moderate risk)
- **Challenge**: Deciding what's "better" extraction
- **Solution**: Copy exact code, no rewrites
- **Research note**: LLM+IDE hybrid approaches (embedding RAG) achieve 53.4% recall vs 39.4% for static analysis alone (Pomian et al., 2024)

### Rename (Low risk)
- **Challenge**: "Fixing related issues" during rename
- **Solution**: Change names ONLY, nothing else

### Change Function Declaration (High risk)
- **Challenge**: "Improving" the API while changing it
- **Solution**: Migration method — old calls new, update callers one by one

### Replace Conditional with Polymorphism (Very high risk)
- **Challenge**: Complete structural rewrite
- **Solution**: Refuse or require human review at each step

## Practical Agentic Workflow

Armin Ronacher (June 2025) observes that agentic coding alters refactoring timing:

> "Agents handle tasks effectively until project complexity surpasses some manageable threshold... You don't want to refactor too early and you definitely do not want to do it too late."

In practice with Claude Code or similar agents:
1. **Don't refactor speculatively** — wait until complexity blocks progress
2. **Keep refactorings small and testable** — agents work best on localised changes
3. **Run the full test suite after each refactoring** — not after a batch
4. **Use `cargo clippy` and `cargo mutants`** as automated verification gates
5. **If a refactoring fails tests, revert entirely** — don't debug forward

## How to Activate the Rules

### When giving instructions to an LLM:

**Most effective** (research-validated):
```
"Perform an Extract Function refactoring to move validation logic into 
validate_items(). Follow REFACTORING_RULES.md mechanics."
```

**Why it works**: Naming the specific refactoring type (Extract Function) raises success from 15.6% → 86.7%. Referencing the rules file activates the contract.

**Also effective:**
- "I need you to refactor (not rewrite) these functions to a new module"
- "Remember: preserve behaviour EXACTLY. Now refactor..."
- Use specific catalog names: Move Function, Extract Variable, Rename

**If the LLM starts modifying behaviour:**
```
"STOP — you're changing behaviour. Follow REFACTORING_RULES.md"
```

## The Paradox of AI Refactoring

LLMs excel at understanding code intent, suggesting improvements, finding bugs, and modernising patterns. These strengths become weaknesses during refactoring, where the goal is to change **nothing** about behaviour.

The research resolution is not to make LLMs "understand" refactoring better. It's to:
1. **Constrain via prompting** — explicit refactoring types, restricted context, STOP signals
2. **Verify mechanically** — test suites, static analysis, mutation testing
3. **Use the right tool** — LLMs for systematic/localised refactorings, humans for architectural ones
4. **Deploy pipeline architecture** — generate, compile, test, reflect, retry

Our [Refactoring Rules](../reference/REFACTORING_RULES.md) implement this approach as a manual contract. The mechanical processes prevent the LLM from using its "intelligence" in ways that break the refactoring contract.

## Key Takeaways

1. **Refactoring is about structure, not behaviour** — the Fowler definition is non-negotiable
2. **LLMs are excellent at systematic refactorings** — 20pp better than developers on smell reduction
3. **LLMs are dangerous at architectural refactorings** — 6–8% hallucination rate without controls
4. **Prompt specificity is the single biggest lever** — naming the refactoring type = 5× improvement
5. **Verification is mandatory** — tests, clippy, mutation testing after every change
6. **Revert, don't fix forward** — matches multi-agent self-reflection loops
7. **Refactor at the right time** — when complexity blocks progress, not speculatively

## References

- Cordeiro, Noei & Zou (2024). "An Empirical Study on the Code Refactoring Capability of Large Language Models." ICSE 2025.
- Liu et al. (2024). "LLM-Driven Code Refactoring: Opportunities and Limitations." IDE Workshop, ICSE 2025.
- Shirafuji et al. (2023). "Refactoring programs using large language models with few-shot examples." APSEC 2023.
- Oueslati et al. (Nov 2025). "RefAgent: Multi-agent refactoring with planning, tool-calls, and self-reflection."
- Xu et al. (Mar 2025). "Multi-agent RAG for method-level refactoring."
- Pomian et al. (2024). "Extract Method via LLM+IDE plugin." 
- Midolo et al. (Jan 2026). "Class-level refactoring with GPT-4o."
- Batole et al. (Mar 2025). "IDE-native foundation model agents for Move Method."
- Robredo et al. (Sep 2025). "LLM-driven study of refactoring motivations in open-source projects."
- Fulcini et al. (2026). "Enhancing Software Maintainability Through LLM-Assisted Code Refactoring." PROFES 2025.
- Ronacher, Armin (Jun 2025). "Agentic Coding Recommendations." lucumr.pocoo.org.
- Emergent Mind (Jan 2026). "LLM-Driven Code Refactoring" topic survey. emergentmind.com.
