# Project Wisdom

Knowledge and insights discovered during development of Zwift Race Finder.
Older insights archived to PROJECT_WISDOM_ARCHIVE_20250602.md

## Active Insights

### 2025-06-02: Lead-in Distance Critical for Accuracy
Insight: Lead-in distance varies by event type (race vs free ride vs meetup) and can be significant (0.2-5.7 km)
Impact: Ignoring lead-in was causing systematic underestimation of race duration
Key Learning: Always check for "hidden" distance components in racing/sports applications

### 2025-06-02: Database Schema Evolution Pattern
Insight: Adding columns requires updating ALL database access points - queries, inserts, tests, and seed data
Impact: Test failures revealed incomplete migration despite main code working
Best Practice: Create migration checklist: schema → struct → queries → inserts → tests → seeds

### 2025-06-02: External Data Import Strategy
Insight: Large reference datasets (zwift-data) best imported via script rather than hardcoding
Impact: 264 routes imported with accurate data, maintainable via re-run
Pattern: Parse external format → map to internal schema → upsert with conflict handling

### 2025-06-02: URL Generation from Slugs
Insight: Route slugs enable external service integration without APIs
Impact: Users get direct links to detailed route information on WhatsOnZwift
Design: Store slugs during import, generate URLs on display

### 2025-05-27: Configuration Management Success
Insight: Multi-level config with environment overrides provides maximum flexibility
Impact: Users can configure via files, env vars, or wrapper scripts
Key Pattern: env → local → ~/.config → ~/.local/share → defaults

### 2025-05-27: Secure Storage Design Pattern
Insight: Support multiple storage backends with automatic fallback (env → keyring → file)
Impact: Users get best available security without configuration burden

### 2025-05-26: Racing Score vs Traditional Categories
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D and Racing Score (0-650)
Impact: Filtering logic excluded half the events due to `distanceInMeters: 0`

### 2025-05-26: Zero as API Signal
Insight: Some APIs use 0/null to mean "check elsewhere", not actual zero
Impact: Changed filtering to accept 0 and check alternatives

### 2025-05-26: Field Presence Type Detection
Insight: `rangeAccessLabel` presence identifies Racing Score events
Impact: Differentiate event types without explicit type field

### 2025-05-26: Browser DevTools Power
Insight: Browser tools reveal undocumented API behavior quickly
Impact: Found Racing Score pattern in minutes vs hours

### 2025-06-02: WhatsOnZwift Data Sources Discovery
Insight: WhatsOnZwift has permission from Zwift to display route/workout data but provides no public API
Impact: Third-party tools must parse web pages or use indirect sources like zwift-data npm package
Key Learning: Popular services often have special agreements unavailable to indie developers
Details:
- Zwift's developer API requires special accounts not available to hobby developers
- WhatsOnZwift likely has privileged access through their partnership
- Our approach: Use zwift-data package + public Zwift endpoints + manual curation
- Web scraping tools exist (wozzwo) but check ToS first

### 2025-06-02: Strava as Accurate Zwift Route Data Source
Insight: Zwift exports completed rides to Strava with accurate route information embedded
Impact: Strava activities provide ground truth data for route details, distances, and elevation
Key Learning: Sometimes the best API is an indirect one - use data exhaust from integrations
Details:
- Zwift automatically syncs rides to Strava (when connected)
- Strava activities contain actual route names, distances, elevation profiles
- Can download activity data via Strava API with proper authentication
- Our project uses this: strava_auth.sh → strava_fetch_activities.sh → import to DB
- This gives us real-world validation data for route predictions

### 2025-06-02: Club Events as Route Discovery Tool
Insight: Zwift Companion app allows creating club events with specific routes for controlled testing
Impact: Can systematically map all routes by creating events and analyzing the exported data
Key Learning: When APIs are locked down, create your own data through controlled experiments
Details:
- Club owners can create events on any free-ride route
- Participants' rides export to Strava/Garmin with full route data
- Captures actual distances including lead-in variations
- Different event types (race/ride/meetup) may have different lead-ins
- Strategy: Create "Route Discovery" club events to map entire Zwift ecosystem

### 2025-06-02: WhatsOnZwift Route URL Patterns
Insight: WhatsOnZwift has accurate route data but reverse routes don't have separate pages
Impact: Need to map reverse routes to their forward equivalents for data lookup
Key Learning: External data sources may have different organization than internal models
Details:
- URL pattern: https://whatsonzwift.com/world/{world}/route/{slug}
- Reverse routes (e.g., "hilly-route-rev") must map to base route ("hilly-route")
- Data includes: distance, elevation, lead-in distance, lead-in elevation
- Most comprehensive public source for Zwift route data

### 2025-01-06: Test-Driven Development Essential for Refactoring
Insight: Code reorganization without comprehensive tests leads to subtle behavioral changes
Impact: Attempted modular refactor broke functionality despite "only moving code"
Key Learning: AI assistants may inadvertently change logic when reorganizing code
Details:
- What seemed like simple code movement actually changed 5+ critical functions
- Math operations inverted (multiply → divide), logic altered, constants changed
- Without tests, these changes were only caught through manual comparison
- Solution: TDD - write tests FIRST, then refactor, ensuring tests still pass
- Best Practice: Create comparison tests between old and new versions during refactoring

### 2025-01-06: AI Code Modification Patterns
Insight: AI assistants tend to "improve" code even when asked only to reorganize
Impact: Simple reorganization request resulted in behavioral changes
Key Learning: Be extremely explicit about preserving exact behavior
Details:
- AI changed find_user_subgroup from simple matching to complex racing score logic
- Modified difficulty multipliers "to be more accurate" without being asked
- Changed math operations thinking it was "fixing" them
- Added features (like route discovery integration) not in original
- Prevention: Use TDD, explicit "preserve behavior" instructions, comparison testing
- Always verify: "The only changes must be strictly about code organization"

### 2025-01-06: Test Development Communication Requirements
Insight: Tests should be explained to and confirmed by the developer before implementation
Impact: Prevents incorrect test assumptions and ensures tests validate actual requirements
Key Learning: AI-generated tests may make incorrect assumptions about intended behavior
Details:
- Tests represent contracts about how code should behave
- Wrong test assumptions can force incorrect implementations
- Developer should confirm: "Is this test required?" and "Is this functionality correct?"
- Example: Property test assumed all races > 5 minutes, but 1km races can be 2 minutes
- Example: Config test assumed empty TOML returns None, but Default trait provides values
- Best Practice: Present test scenarios to developer for validation before coding
- Include test rationale: WHY this behavior is being tested

### 2025-01-06: Using Test Coverage as a Code Discovery Tool
Insight: Low coverage is just an indication of missing tests - the key is driving to 100% and then inspecting test quality
Impact: Pursuing 100% coverage forces examination of all code paths, revealing code quality issues
Key Learning: Test coverage is a discovery mechanism, not a quality metric
Details:
- Low coverage simply means lack of test cases, not necessarily bad code
- The real insight comes from attempting to write tests for uncovered code
- When driving to 100% coverage, you must understand what each piece of code does
- Poor quality tests can indicate either:
  - Poor testing skills (needs improvement)
  - Poor/unused code under test (needs removal)
  - This distinction requires human judgment
- Example: TEST_FIXES_20250603_211433.md demonstrates this discovery process
- Contrived or difficult-to-write tests often signal unnecessary code
- Natural, straightforward tests usually indicate essential functionality
- Best Practice: 
  1. Use coverage tools to find untested code
  2. Attempt to write meaningful tests
  3. Evaluate test quality - if tests feel forced, question the code's necessity
  4. Human judgment required to decide: improve tests or remove code
- Additional benefit: Coverage tools are external to LLMs, operating at different speed/resource cost
  - Tools like cargo-llvm-cov run in seconds vs minutes of LLM analysis
  - Can be run continuously in CI/CD without LLM token costs
  - Provides objective starting point for human analysis
  - Complements LLM capabilities by providing data LLMs can then interpret

### 2025-06-04: Meta-Process of AI-Assisted Development - Building Tools to Build Tools
Insight: Successful AI-assisted development follows a hierarchy of tool-building and validation
Impact: Achieved 10.3% coverage improvement in one session with 100% natural tests
Key Learning: The process is as important as the code - humans guide, AI executes, tools validate
Details:
- Three levels of tool-building discovered:
  1. **Planning Tools**: Documents that structure thinking (FUNCTION_COVERAGE_PLAN_20250604_084639.md)
  2. **Discovery Tools**: Coverage analysis reveals what needs attention
  3. **Validation Tools**: Tests ensure correctness and quality
- The development cycle:
  1. Think about the problem (human provides context and goals)
  2. Write detailed plans (AI helps structure, human validates)
  3. Evaluate plans before execution (feasibility check)
  4. Execute systematically (AI implements, following the plan)
  5. Use coverage to find gaps (objective external tool)
  6. Write tests to validate behavior (TDD approach)
  7. Evaluate test quality (natural vs contrived)
  8. Iterate based on findings
- Human's critical role:
  - Provides domain knowledge and quality standards
  - Validates that plans make sense
  - Judges test quality (natural vs contrived)
  - Decides when to refactor vs accept
  - Guides the AI away from over-engineering
- Today's results validate this approach:
  - 11 functions tested, all with natural tests
  - Zero contrived tests indicates well-structured code
  - Systematic execution beats ad-hoc development
- Best Practice: Don't just code - build the tools to build the code

### 2025-06-04: Test Quality as Code Quality Indicator
Insight: The ease of writing natural tests directly correlates with code quality
Impact: 11/11 functions tested with natural tests, confirming excellent code structure
Key Learning: If tests feel contrived, the code probably needs refactoring
Details:
- Natural test indicators:
  - Clear inputs and outputs
  - Obvious test scenarios from real usage
  - No complex mocking or setup required
  - Tests read like documentation
- Examples from today:
  - `format_duration(60)` → "01:00" (obviously correct)
  - `estimate_distance_from_name("3R Flat Route")` → 33.4km (pattern matching)
  - Cache functions tested with temp directories (proper isolation)
- Contrived test warning signs:
  - Excessive mocking
  - Testing implementation details
  - Unclear what "correct" behavior should be
  - Tests that just exercise code without validating behavior
- The 100% natural test rate confirms:
  - Functions have single, clear responsibilities
  - Good separation of concerns
  - Appropriate abstraction levels
  - No unnecessary coupling
- Strategy: Use test difficulty as refactoring trigger

### 2025-06-04: Deterministic Tools vs Pure LLM Execution
Insight: Building concrete tools and procedures provides deterministic control vs relying solely on LLM
Impact: Dramatically reduces LLM load and increases development efficiency
Key Learning: LLMs are best used to build tools that then operate independently
Details:
- Tools provide deterministic, repeatable operations:
  - Coverage analysis runs in seconds, costs nothing
  - Test suites validate behavior consistently
  - Build scripts automate complex workflows
  - Documentation captures decisions permanently
- Efficiency gains:
  - `cargo llvm-cov` gives instant feedback vs asking LLM to analyze coverage
  - `cargo test` validates in seconds vs LLM reasoning about correctness
  - Scripts like `import_zwiftpower.sh` encapsulate complex processes
  - Plans like FUNCTION_COVERAGE_PLAN let humans execute without LLM
- Reduces cognitive load on LLM:
  - LLM focuses on creative problem-solving
  - Tools handle repetitive validation
  - Procedures ensure consistent execution
  - Documentation preserves context between sessions
- Examples from this project:
  - Shell scripts for data import (deterministic, reusable)
  - Coverage reports guide development objectively
  - Test suite prevents regressions automatically
  - Session documents maintain context efficiently

### 2025-06-04: Pedantic Languages as LLM Force Multipliers
Insight: "Difficult" languages like Rust and TypeScript actually help LLMs write better code
Impact: Compilation errors provide immediate, precise feedback that guides LLM corrections
Key Learning: The stricter the language, the better the AI-assisted development experience
Details:
- Rust's pedantic compiler is an LLM's best friend:
  - Type errors caught immediately with exact location
  - Borrow checker prevents memory safety issues
  - Pattern matching ensures exhaustive handling
  - Lifetime annotations make ownership explicit
- TypeScript similarly helps with:
  - Type inference catches inconsistencies
  - Strict null checking prevents common errors
  - Interface contracts enforce API consistency
  - Compiler configuration enforces standards
- The feedback loop is critical:
  - LLM writes code → compiler checks → precise errors → LLM fixes
  - Each error message teaches the LLM about constraints
  - No ambiguity about what's wrong or where
  - Contrast with Python: errors only appear at runtime
- Real examples from today:
  - Rust caught wrong struct fields in UserStats immediately
  - Import errors showed exactly what was missing
  - Type mismatches prevented incorrect test assumptions
- This creates a virtuous cycle:
  - Strict languages force clear thinking
  - Clear thinking produces better initial code
  - Compiler catches remaining issues
  - LLM learns from compiler feedback
  - Result: Higher quality code faster
- Best Practice: Choose the strictest language appropriate for the task