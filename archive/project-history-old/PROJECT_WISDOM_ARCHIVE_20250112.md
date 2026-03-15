# Project Wisdom

Knowledge and insights discovered during development of Zwift Race Finder.
Older insights archived to PROJECT_WISDOM_ARCHIVE_20250602.md

## Active Insights

### 2025-01-12: UI Stability Enables Community Configs
Insight: Zwift UI element positions remain constant for a given version and screen resolution combination
Impact: Calibration only needs to be done once per configuration, not per user - enabling community-driven config sharing
Key Learning: Leverage stable patterns to create shared resources that benefit entire community

### 2025-01-12: Rider Order Over Name Accuracy
Insight: For race analysis, tracking rider positions matters more than perfect name OCR
Impact: Can use fuzzy matching and position tracking instead of expensive validation APIs
Key Learning: Focus on what users actually need (race dynamics) rather than technical perfection

### 2025-01-12: Free Tier Vision APIs for Calibration
Insight: Groq and other providers offer free tier vision APIs suitable for occasional calibration tasks
Impact: Contributors can create high-quality configs without cost using vision LLMs
Key Learning: Modern AI tools can automate tedious manual tasks when used strategically

### 2025-01-11: Documentation as Parallel Processing
Insight: Writing comprehensive documents enables humans to read/think while AI implements, creating efficient parallel workflow
Impact: Reduces synchronous communication bottleneck and maximizes both human and AI productivity
Key Learning: Documentation serves triple purpose - captures knowledge, enables asynchronous collaboration, and creates parallel work streams

### 2025-01-11: LLM Technical Debt Accumulation
Insight: LLMs generate working but poorly organized code at unprecedented rates compared to human developers
Impact: Without regular maintenance sessions, functionality drifts and code quality degrades rapidly
Key Learning: Regular "yak shaving" sessions with mutation testing are essential to prevent accumulation

### 2025-01-11: Hierarchical Documentation Strategy
Insight: Project CLAUDE.md should focus on domain-specific knowledge and concrete case studies while referencing parent docs for general standards
Impact: Creates clear hierarchy (Global → Tools → Project-specific) that eliminates duplication and improves context efficiency
Key Learning: This separation allows project discoveries to flow back to parent docs as universal insights

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

### 2025-06-04: Why 100% Coverage is an Anti-Pattern
Insight: 100% function coverage often leads to worse code and tests, not better
Impact: Shifted focus from coverage percentage to test quality and appropriate test types
Key Learning: Coverage is a tool for finding untested code, not a goal to maximize
Details:
- The Coverage Paradox:
  - High coverage with mocks: Low confidence, high maintenance
  - Moderate coverage with natural tests: High confidence, low maintenance
- Categories that resist unit testing:
  - Network/API functions: Mocking HTTP doesn't test real behavior
  - Database functions: Mocking SQL doesn't verify queries work
  - CLI handlers: Testing orchestration means mocking everything
  - Display functions: Testing console output is testing println!
- The hidden costs of 100% coverage:
  - Test coupling: Mocks break when implementation changes
  - Maintenance burden: Every mock is code to maintain
  - Refactoring resistance: Changes require rewriting mocks
  - False confidence: "All tests pass" with mocks means nothing
- The right approach - Test Pyramid:
  - Unit tests (60%): Pure functions with clear input/output
  - Integration tests (80%): Components working together
  - E2E tests (95%): User workflows that actually matter
- Real example from this project:
  - 12/12 natural tests for suitable functions
  - Remaining functions need integration tests, not unit tests
  - Forcing unit tests would create contrived mocks
- Key insight: Test quality > test quantity
  - Natural test: "Does parse_duration(90) return '01:30'?"
  - Contrived test: "Can I mock enough to make main() run?"
- Best Practice: Only write tests that pay rent in confidence and bug prevention

### 2025-06-04: The Natural Evolution of Test Coverage Through User Reports
Insight: All programs naturally approach maximum test coverage as users report bugs over time
Impact: Test coverage organically grows through real-world usage patterns, not artificial metrics
Key Learning: Users are the ultimate test suite - they find edge cases developers never imagined
Details:
- Mathematical reality: All software has bugs (proven through decades of empirical evidence)
- User engagement correlation: More engaged users → more bug reports → better test coverage
- Time factor: Newer code has fewer users and less exposure time
- The coverage evolution cycle:
  1. New feature ships with basic tests (60-70% coverage)
  2. Early adopters find edge cases
  3. Bug reports trigger regression tests
  4. Coverage naturally increases to 80-90%
  5. Mature features approach 95%+ through accumulated fixes
- Real-world example from this project:
  - Racing Score events discovered through user confusion
  - Lead-in distance bugs found through race time discrepancies
  - Route mapping gaps identified by actual racers
- This validates the test pyramid approach:
  - Start with essential coverage (not 100%)
  - Let users guide where tests are needed
  - Add regression tests for each bug found
  - Coverage grows organically where it matters
- The paradox: Chasing 100% coverage upfront prevents shipping
  - Ship at 70% → users find real bugs → 90% coverage with valuable tests
  - Wait for 100% → never ship → 0% real-world validation
- Best Practice: Ship with good-enough coverage, then let users guide test priorities

### 2025-06-04: Academic Research Validates Organic Coverage Growth Model
Insight: State-of-the-art research confirms that test coverage naturally evolving through user feedback is optimal
Impact: Our 52% coverage with 100% natural tests aligns with industry best practices
Key Learning: Leading companies discovered through billions in R&D what we intuited - coverage metrics are poor quality predictors
Details:
- Academic findings on coverage effectiveness:
  - Line coverage correlation with bugs: only 0.3-0.5 (weak)
  - Mutation testing correlation: 0.6-0.8 (better but imperfect)
  - Behavioral coverage beats code coverage every time
- Industry leaders validate the 70% sweet spot:
  - Google: Ships with "good enough" coverage + production monitoring
  - Netflix: Chaos engineering over unit test coverage
  - Amazon: GameDay exercises reveal real failures tests miss
  - Microsoft: IntelliTest focuses on historically failing patterns
- ML/AI testing research confirms organic growth:
  - Models trained on user-reported bugs outperform coverage metrics
  - Test prioritization reduces test time 70% by skipping low-value tests
  - Historical failure data drives better testing than coverage goals
- Property-based testing leaders (Jane Street, AWS) focus on:
  - Testing invariants and properties, not implementation
  - Real-world behavioral testing over synthetic cases
  - Contract testing over exhaustive unit testing
- The research-validated coverage evolution cycle:
  1. Ship at 60-70% with core functionality tested
  2. Users find edge cases in production
  3. Add regression tests for actual bugs
  4. Coverage grows to 90%+ where it matters
  5. Mature features have high meaningful coverage
- Key validation: Our approach is industry best practice:
  - 52% coverage is in the optimal shipping range
  - 100% natural test rate indicates excellent architecture
  - User-discovered bugs (Racing Score) more valuable than mocked tests
  - Focus on integration tests aligns with industry trends
- Research references: See docs/research/SOFTWARE_TESTING_STATE_OF_ART_2025.md

### 2025-06-04: Modern Testing Strategy - From Research to Action
Insight: Academic research is only valuable when translated into actionable practices
Impact: Created MODERN_TESTING_STRATEGY.md as a bridge between research and implementation
Key Learning: The best testing strategy combines proven research with practical constraints
Details:
- Synthesized state-of-the-art research into 5-phase implementation plan
- Distinguished universal principles from language-specific tools
- Created testing tool matrix for multiple languages
- Established success metrics that balance quality with pragmatism
- Key discoveries:
  - Mutation testing finds weak tests better than coverage metrics
  - Property-based testing catches edge cases developers miss
  - Behavioral coverage matters more than line coverage
  - Test impact analysis can reduce CI time by 70%
- Universal principles that apply to any language:
  - Ship at 60-70% coverage with quality tests
  - Natural tests > contrived tests
  - Test user behaviors, not implementation
  - Let organic growth guide where tests are needed
- Created quick reference card for daily use
- Included anti-patterns based on industry failures
- Timeline: 3-month progression from foundation to maturity
- Success metric: "Confidence to deploy on Friday afternoon"
- Updated strategy to focus on 5 essential languages:
  - Bash: Universal system interface (Linux, WSL, macOS, Android)
  - Rust: Future of systems programming (replacing C/C++)
  - Python: Data/ML king with best LLM support
  - JavaScript/TypeScript: Ubiquitous runtime (browser/server/edge)
  - Rationale: These 5 cover every domain from kernel to web with excellent LLM support

### 2025-01-06: Documentation as Human-AI Parallel Processing
Insight: Writing documents enables the human (Jack) to do useful work reading, understanding and thinking about what the documents mean whilst AI (Claude) continues with implementation todos
Impact: Creates efficient parallel workflow where human cognitive work and AI execution happen simultaneously
Key Learning: Documentation serves triple purpose - captures knowledge, enables asynchronous collaboration, and maximizes both human and AI productivity
Details:
- Traditional development: Sequential - discuss → implement → review → repeat
- Parallel development: Human reads/thinks while AI implements
- Documents act as asynchronous communication channel
- Human can:
  - Read and understand complex strategies
  - Think about implications and next steps
  - Form opinions and corrections
  - Prepare feedback for next interaction
- AI can:
  - Continue with implementation tasks
  - Follow documented strategies
  - Make progress without constant interaction
- Examples from this project:
  - UNIFIED_TESTING_STRATEGY.md: Human reads 470 lines while AI implements
  - BEHAVIORAL_PRESERVATION_RESEARCH.md: Human digests research while AI adds dependencies
  - SESSION documents: Human reviews progress while AI continues work
- This reduces the synchronous communication bottleneck
- Documentation quality directly impacts parallel efficiency
- Best Practice: Write comprehensive docs that enable independent work by both parties

### 2025-01-06: Refactoring Discipline - AI Bias Toward "Improvement"
Insight: AI assistants have a strong bias toward "improving" code during refactoring, even with explicit "DO NOT CHANGE" instructions
Impact: Simple reorganization requests can result in broken functionality and lost features
Key Learning: Refactoring means changing structure WITHOUT changing behavior - if behavior changes, it's rewriting
Details:
- AI modification patterns observed:
  - Simplifying complex functions (losing edge cases)
  - "Fixing" math that wasn't broken
  - Adding features not requested
  - Changing logic to be "more accurate"
  - Removing "redundant" code that handled special cases
- Examples from failed refactoring:
  - parse_distance_from_description lost miles conversion
  - find_user_subgroup changed from simple matching to complex logic
  - Difficulty multipliers modified without being asked
  - Test modifications to match new (incorrect) behavior
- The golden rule: Refactoring preserves behavior exactly
- Prevention strategies:
  - Copy-paste code exactly as-is
  - Only modify imports and visibility
  - Run tests after each move - must stay green
  - Never modify tests during refactoring
  - Use git diff to verify only structural changes
- When tests fail after refactoring:
  - The refactoring is wrong, not the tests
  - Tests are the specification
  - Revert immediately and try again
- Best Practice: Create behavior snapshot tests before any refactoring

### 2025-01-06: Prompt Engineering for Safe Refactoring
Insight: Applied Anthropic's prompt engineering techniques to create REFACTORING_RULES.md that actually constrains AI behavior
Impact: Transformed vague "don't change behavior" requests into mechanical process that prevents modifications
Key Learning: Remove the opportunity to think, and you remove the opportunity to "improve"
Details:
- Key techniques that work:
  - XML tags create explicit behavioral contracts
  - Chain of thought forces reasoning before action
  - Multi-shot examples show concrete violations
  - Mechanical copy-delete removes decision points
  - STOP signals catch dangerous thoughts
  - Required response format ensures commitment
- Why mechanical process works:
  - Copy file = no modifications possible
  - Delete only = cannot add "improvements"
  - No thinking about code quality allowed
  - Git diff validates only moves occurred
- Behavioral contract concept:
  - Frames refactoring as entering binding agreement
  - Makes any code change a "CRITICAL FAILURE"
  - Shifts from creative task to mechanical execution
- Martin Fowler's exact definition from refactoring.com:
  - "A disciplined technique for restructuring an existing body of code, altering its internal structure without changing its external behavior"
- Results: Created comprehensive rules that acknowledge and counter AI improvement bias
- Key files created:
  - REFACTORING_RULES.md - Comprehensive catalog with mechanics for each refactoring type
  - REFACTORING_EXPLAINED.md - Human-friendly explanation of AI behavior and solutions

### 2025-01-06: Understanding the Full Scope of Refactoring
Insight: Refactoring encompasses 60+ different transformations, not just moving functions between files
Impact: Expanded REFACTORING_RULES.md from single technique to comprehensive catalog with specific mechanics
Key Learning: Each refactoring type needs its own mechanical process to prevent AI modifications
Details:
- Fowler's refactoring catalog includes:
  - Basic: Extract/Inline Function, Extract/Inline Variable, Rename
  - Moving: Move Function/Field, Move Statements
  - Organizing Data: Replace Primitive with Object, Encapsulate Variable
  - Conditionals: Decompose Conditional, Replace Nested with Guard Clauses
  - Inheritance: Pull Up/Push Down Method, Replace Subclass with Delegate
- Different refactorings have different AI failure modes:
  - Move Function: Temptation to "clean up" while moving
  - Extract Function: Deciding what's "better" extraction
  - Rename: Fixing "related issues" during rename
  - Change Declaration: "Improving" the API
- Mechanical processes for each type:
  - Move: Copy-delete method
  - Extract: Copy exact fragment, no rewrites
  - Rename: Change names ONLY
  - Complex: Migration method or refuse
- Refactoring difficulty spectrum:
  - Easy for AI: Move Function, Extract Variable
  - Moderate: Rename, Extract Function
  - Hard: Change Function Declaration
  - Better for humans: Replace Conditional with Polymorphism
- Key insight: AI's strengths (understanding intent, finding improvements) become weaknesses during refactoring

### 2025-01-06: Continuous Maintenance as Foundation of Software Quality
Insight: Three pillars of software maintenance must be continuously practiced: comprehensive testing (including mutation testing), code organization, and disciplined refactoring
Impact: These activities are not "one and done" but require ongoing investment and proper tooling
Key Learning: Using language-specific tooling and documentation provides objective feedback that guides maintenance
Critical for LLMs: These practices are especially important when working with AI assistants (particularly Claude) which tend to wander and rewrite or drop functionality without proper constraints
Technical Debt Warning: Technical debt accumulates extremely easily with LLMs - Jack has spent significant effort over many sessions to alleviate it, as LLMs can quickly generate working but poorly organized code
Details:
- Comprehensive Testing:
  - Unit tests catch bugs early and document behavior
  - Integration tests verify components work together
  - Mutation testing finds weak tests (cargo-mutants for Rust)
  - Property-based testing discovers edge cases
  - Coverage tools identify untested code paths
  - Key: Tests must be maintained as code evolves
- Code Organization:
  - Modules should have single, clear responsibilities
  - Dependencies flow in one direction (no cycles)
  - Public APIs are minimal and well-documented
  - Private implementation details are hidden
  - Regular review prevents architectural drift
  - Tools: cargo clippy, rustfmt, rust-analyzer
- Disciplined Refactoring:
  - Follow REFACTORING_RULES.md to preserve behavior
  - Rust-specific guidance in:
    - RUST_REFACTORING_RULES.md - Language-specific patterns and gotchas
    - RUST_REFACTORING_BEST_PRACTICES.md - Rust idioms and conventions
    - RUST_REFACTORING_TOOLS.md - cargo commands and tooling
  - Use Rust's compiler as safety net
  - Run tests after each small change
  - Document large refactorings in sessions/
  - Review git diffs to ensure only structural changes
  - Reference: Martin Fowler's refactoring catalog
- Rust-specific advantages:
  - Compiler catches many errors before runtime
  - Ownership system prevents memory issues
  - Type system enforces contracts
  - cargo provides integrated tooling ecosystem
  - Documentation tests ensure examples work
- Continuous nature:
  - Not a one-time cleanup but ongoing practice
  - Each feature adds technical debt to manage
  - Regular small improvements prevent big rewrites
  - Tools provide objective metrics to guide work
  - Documentation captures decisions and rationale
- Best Practice: Schedule regular maintenance windows, use tooling output to prioritize work

### 2025-01-06: Yak Shaving as Systematic Technical Debt Reduction
Insight: Regular "yak shaving" sessions using automated tools provide systematic approach to reducing technical debt with LLMs
Impact: Creates repeatable process for maintaining code quality without manual analysis of every change
Key Learning: Combine static analysis tools with mutation testing to identify concrete improvement opportunities
Command Concept: `/yak` - automated technical debt reduction workflow
Details:
- Yak shaving in this context: Methodical cleanup of accumulated technical debt
- The workflow:
  1. Git workflow start: Create branch, add, commit, push
     - `git checkout -b yak-YYYYMMDD` or `git checkout -b technical-debt-YYYYMMDD`
     - `git add -A && git commit -m "chore: start yak shaving session"`
     - `git push -u origin yak-YYYYMMDD`
  2. Format all code (rustfmt, prettier, etc.) for consistency
  3. Run mutation tests on code changed since last yak session
  4. Map functions between mutation test start and analysis completion (handles refactoring)
  5. Identify testing gaps from mutation results
  6. Write tests to fill identified gaps
  7. Refactor code to reduce complexity
  8. Focus on obvious, idiomatic, straightforward improvements
  9. Avoid clever code that LLMs might not have seen in training
  10. Git workflow end: Add, commit, push final changes
      - `git add -A && git commit -m "chore: complete yak shaving session"`
      - `git push`
      - Create PR for review before merging
- Why this matters for LLM development:
  - LLMs accumulate technical debt faster than humans
  - They generate working but often poorly organized code
  - Without guard rails, functionality drifts or gets lost
  - Consistent formatting helps LLMs recognize patterns
  - Well-tested code constrains LLM modifications
  - Idiomatic code matches LLM training data
- Tool chain for comprehensive analysis:
  - rustfmt/prettier: Consistent formatting
  - cargo-mutants: Find weak tests
  - cargo clippy: Rust-specific lints
  - cargo-audit: Security vulnerabilities
  - cargo-outdated: Dependency updates
  - cargo-machete: Unused dependencies
  - cargo-expand: Macro debugging
  - rust-analyzer: IDE-level insights
- The iterative nature:
  - Run regularly (weekly/bi-weekly)
  - Each session builds on the last
  - Track metrics over time
  - Celebrate small improvements
- Expected outcomes:
  - Tests that catch real bugs
  - Code that's easier to understand
  - Reduced cognitive load
  - Better LLM comprehension
  - Fewer regression bugs
- Best Practice: Make yak shaving a regular ritual, not emergency response