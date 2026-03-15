# CLAUDE.md Final Refactoring Plan

## Executive Summary

After analyzing parent CLAUDE.md files, the project CLAUDE.md should focus on **Zwift-specific knowledge** and **project case studies** while referencing parent docs for general standards.

## Current Overlap Analysis

### Already Covered in Parent Files:
- ✅ General mutation testing requirement (Tools CLAUDE.md)
- ✅ Refactoring discipline and rules (Global CLAUDE.md)
- ✅ Testing standards and TDD (Tools CLAUDE.md)
- ✅ Language selection hierarchy (Tools CLAUDE.md)
- ✅ Build tools and mask usage (Tools CLAUDE.md)
- ✅ Session management (Global CLAUDE.md)
- ✅ wip-claude/ documentation (Global CLAUDE.md)

### Unique to This Project:
- 🏁 Zwift domain knowledge (routes, pack dynamics, event types)
- 📊 Duration estimation algorithms with empirical data
- 🔬 OCR 0% case study (specific implementation story)
- 🏃 Jack's racing data for regression testing
- 🛠️ Project-specific commands and workflows
- 📈 Historical discoveries and API insights

## Final CLAUDE.md Structure (~200 lines)

```markdown
# CLAUDE.md - Zwift Race Finder

Project-specific guidance for Claude Code. 

**Parent Documentation**:
- General behaviors: ~/.claude/CLAUDE.md
- Development standards: /home/jack/tools/CLAUDE.md
- Testing requirements: /home/jack/tools/CLAUDE.md#testing-standards

## Project Philosophy

This project demonstrates AI-assisted development where:
- **The Human** (Jack): Zwift racing expertise + 40 years IT experience
- **The AI** (Claude): Implementation, debugging, transparent reasoning
- **Success**: Empirical validation - when data contradicts docs, investigate

## ⚠️ Project Case Study: OCR Module 0% Effectiveness

**Context**: While general mutation testing is covered in Tools CLAUDE.md, this project provides a concrete case study.

**What happened** (2025-01-10):
- OCR module had ALL test types: property, snapshot, integration, fuzz
- Mutation testing: 234 mutations, 0 caught (0% effectiveness)
- All tests were smoke tests - verified structure, not correctness

**Project-specific lesson**:
```bash
# For THIS project, always:
cargo test ocr_compact::tests --features ocr
cargo mutants --file src/ocr_compact.rs --timeout 30
```

**Documentation**:
- Full analysis: COMPREHENSIVE_TESTING_GUIDE.md
- Mutation report: wip-claude/20250110_083000_ocr_mutation_testing_final_report.md

## Essential Commands

```bash
# Run with your Zwift Racing Score
cargo run -- --zwift-score 195 --duration 120

# Import race history from ZwiftPower
./tools/zwiftpower/import_zwiftpower_dev.sh

# Check unknown routes
cargo run -- --show-unknown-routes

# Run regression tests against Jack's races
cargo test regression
```

## Zwift Domain Knowledge

### Quick Reference
- **Racing Score**: 0-199 (D), 200-299 (C), 300-399 (B), 400+ (A)
- **Route ID**: Internal Zwift identifier, stable across name changes
- **Pack Speed**: Cat D ~30.9 km/h with draft benefit
- **Drop Dynamics**: 86kg vs 70kg = disadvantage on climbs

### Detailed Documentation
- Architecture → docs/ARCHITECTURE.md
- Algorithms → docs/ALGORITHMS.md
- Database → docs/development/DATABASE.md
- Import Guide → docs/guides/DATA_IMPORT.md
- Domain Insights → docs/ZWIFT_DOMAIN.md

## Project-Specific Workflows

### Adding New Routes
1. Find route_id on ZwiftHacks.com
2. Get metrics from Zwift Insider
3. See: docs/development/DATABASE.md#adding-routes

### Improving Duration Estimates
1. Run: `cargo test regression`
2. Analyze errors by route
3. See: docs/ALGORITHMS.md#tuning-parameters

### Debugging Event Filtering
- Racing Score events: `distanceInMeters: 0`
- Check `rangeAccessLabel` field
- See: docs/ARCHITECTURE.md#event-types

## Regression Testing Strategy

Uses Jack's 151 actual races for calibration:
- Current accuracy: 23.6% mean absolute error
- Target: <20% error
- Database: ~/.local/share/zwift-race-finder/races.db

## Key Discoveries Summary

1. **Racing Score vs Traditional** (2025-05-26): Two mutually exclusive systems
2. **Drop Dynamics** (2025-05-25): Binary state explains 82.6% variance
3. **Zero as API Signal**: `distanceInMeters: 0` means check description

Full history: docs/development/HISTORICAL_DISCOVERIES.md

## Log Management

- Summary: docs/development/ZWIFT_API_LOG_SUMMARY.md (<3KB)
- Recent: docs/development/ZWIFT_API_LOG_RECENT.md
- Archives: sessions/ZWIFT_API_LOG_SESSION_*.md
```

## What Gets Moved Where

### To `docs/ARCHITECTURE.md`:
- Detailed architecture overview
- Data flow diagrams
- Database schema
- Route ID system details
- Event type specifications

### To `docs/ALGORITHMS.md`:
- Full duration estimation algorithm
- Pack dynamics research
- Drop dynamics model
- Mathematical formulas
- Calibration approach

### To `docs/ZWIFT_DOMAIN.md` (New):
- Zwift-specific concepts
- Racing categories explained
- Draft benefits
- Route types and surfaces
- Event terminology

### To `docs/development/DATABASE.md`:
- Schema details
- Backup procedures
- Migration commands
- Route management SQL

### To `docs/guides/DATA_IMPORT.md`:
- ZwiftPower extraction
- Import scripts usage
- Strava integration
- Route mapping workflow

### To `docs/development/HISTORICAL_DISCOVERIES.md`:
- Full session discoveries
- API workarounds
- UX enhancement history
- Evolution of understanding

## Implementation Steps

1. **Create new documentation files** (6 files)
2. **Move content from current CLAUDE.md** to appropriate docs
3. **Update CLAUDE.md** to ~200 line focused version
4. **Add cross-references** between documents
5. **Update README.md** with user-facing commands
6. **Verify no information lost** via diff

## Success Criteria

- ✅ CLAUDE.md under 250 lines
- ✅ No duplication with parent CLAUDE.md files  
- ✅ OCR case study prominently featured
- ✅ Zwift domain knowledge preserved
- ✅ Clear references to detailed docs
- ✅ Project-specific focus maintained

## Key Insight

The project CLAUDE.md becomes a **Zwift-specific guide** with **concrete case studies** that complement the general standards in parent files. This creates a clear hierarchy:

1. **Global**: Core behaviors and session management
2. **Tools**: Development standards and general testing
3. **Project**: Domain knowledge and specific implementations