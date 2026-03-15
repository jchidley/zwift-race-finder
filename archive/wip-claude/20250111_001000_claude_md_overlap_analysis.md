# CLAUDE.md Overlap Analysis

## Parent CLAUDE.md Files Analysis

### 1. Global CLAUDE.md (`~/.claude/CLAUDE.md`)
**Purpose**: Global behaviors for all projects

**Key Content**:
- Core behaviors (ASK, EXPLAIN, NEVER sudo, VALIDATE, SEARCH PROJECT_WISDOM.md)
- Documentation in `wip-claude/` folder
- Session management commands (/start, /checkpoint, /wrap-session)
- Project requirements tracking (/req commands)
- Refactoring discipline (references REFACTORING_RULES.md)
- Rust-specific refactoring guidance
- Environment (WSL Debian/Alpine)

### 2. Tools CLAUDE.md (`/home/jack/tools/CLAUDE.md`)
**Purpose**: Development standards hub for all tools

**Key Content**:
- 🚨 Language selection hierarchy (Bash → Rust → Python → TypeScript)
- Bash script standards and testing with bats
- **Mutation Testing Standards** (includes the general lesson)
- Testing requirements and TDD process
- Visual & multi-device app guidance
- Dependencies & tool execution
- Installation standards
- MCP development
- Python/TypeScript/Rust standards
- Source control (GitHub CLI preferred)
- Build tools (mask)
- Security/Logging standards
- UV field manual
- Slash commands reference
- Python to Rust migration guide

### 3. Project CLAUDE.md (Current - 634 lines)
**Purpose**: Zwift Race Finder specific guidance

**Unique Content**:
- Development philosophy (Human-AI collaboration specific to Jack & Zwift)
- Project-specific commands (cargo run examples)
- Architecture (Zwift-specific)
- Database schema (races.db structure)
- Duration estimation algorithm
- Pack/Drop dynamics discoveries
- Route ID system
- Event types (Racing Score vs Traditional)
- ZwiftPower import workflow
- Regression testing with Jack's race history
- Zwift-specific API insights
- Project-specific log management

## What's Already Covered Elsewhere

### In Global CLAUDE.md:
- ✅ Refactoring discipline
- ✅ Session management
- ✅ wip-claude/ documentation approach
- ✅ Core behaviors (ASK, EXPLAIN, etc.)

### In Tools CLAUDE.md:
- ✅ General mutation testing requirement and lesson
- ✅ Testing standards and TDD
- ✅ Language selection (Rust for this project)
- ✅ Build tools (mask usage)
- ✅ Installation standards (~/.local/bin)
- ✅ Bash script standards
- ✅ Source control with gh

## What MUST Stay in Project CLAUDE.md

### 1. Project-Specific Philosophy
The Human-AI collaboration model specific to Jack's domain expertise in Zwift racing is unique and essential.

### 2. The OCR 0% Lesson - PROJECT CONTEXT
While Tools CLAUDE.md has the general mutation testing lesson, this project needs:
- The specific OCR module story (property + snapshot + integration + fuzz = 0%)
- Links to project-specific mutation reports
- The exact workflow for THIS codebase

### 3. Zwift Domain Knowledge
All the Zwift-specific content is irreplaceable:
- Route systems and IDs
- Pack dynamics discoveries
- Racing Score vs Traditional categories
- Duration estimation specifics
- ZwiftPower import process

### 4. Project Commands & Workflows
- Specific cargo run examples
- Database management commands
- Import scripts usage
- Route mapping workflows

### 5. Regression Testing Strategy
Jack's actual race history and calibration approach is unique to this project.

## Revised CLAUDE.md Structure

```markdown
# CLAUDE.md - Zwift Race Finder

Project-specific guidance for Claude Code. For general standards, see:
- Global: ~/.claude/CLAUDE.md
- Tools: /home/jack/tools/CLAUDE.md

## Project Philosophy
[Keep - unique Human-AI collaboration for Zwift domain]

## ⚠️ PROJECT CRITICAL: OCR Module 0% Lesson
[Keep expanded - while general lesson is in Tools, this specific case study is vital]
- Link to COMPREHENSIVE_TESTING_GUIDE.md
- Link to mutation reports in wip-claude/
- Specific commands for THIS project

## Essential Commands
[Keep - all project-specific]

## Architecture & Domain Knowledge
Brief overview with links:
- Zwift Concepts → docs/ZWIFT_DOMAIN.md (new)
- System Design → docs/ARCHITECTURE.md
- Algorithms → docs/ALGORITHMS.md
- Database → docs/development/DATABASE.md

## Common Tasks
[Keep brief versions with links to detailed guides]

## Project-Specific Testing
- Regression tests with Jack's race data
- Route validation approach
- Integration with Zwift APIs

## Log Management
[Keep - project-specific approach]
```

## Key Differences to Preserve

### 1. Domain Expertise Integration
This project uniquely integrates Jack's 40 years IT + Zwift racing expertise. The philosophy section captures how AI assists domain experts.

### 2. Empirical Approach
"When data contradicts descriptions, investigate" - this empirical approach led to major discoveries (Racing Score events, pack dynamics).

### 3. Historical Discoveries
The evolution of understanding (Bell Lap variance, drop dynamics, event types) provides critical context for future work.

### 4. Specific Mutation Testing Integration
While parent has general guidance, this project needs:
- `cargo mutants --file src/file.rs --function my_function`
- OCR module as cautionary tale
- Links to project mutation reports

## Recommendations

1. **Remove from Project CLAUDE.md**:
   - General refactoring rules (covered in global)
   - General testing philosophy (covered in tools)
   - Installation standards (covered in tools)
   - General development principles (covered in tools)

2. **Keep but Condense**:
   - Architecture (move details to docs/)
   - Algorithms (move details to docs/)
   - Database (move details to docs/)

3. **Keep and Emphasize**:
   - OCR 0% lesson with project context
   - Zwift domain knowledge
   - Project-specific commands
   - Regression testing approach
   - Historical discoveries summary

4. **Add Clear References**:
   - "For refactoring rules, see ~/.claude/CLAUDE.md"
   - "For testing standards, see /home/jack/tools/CLAUDE.md"
   - "For language selection, see /home/jack/tools/CLAUDE.md"

This approach eliminates duplication while preserving project-specific knowledge that Claude needs for effective assistance.