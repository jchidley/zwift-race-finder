# Documentation Migration Plan

## Overview

This plan reorganizes documentation to serve multiple audiences while preserving all existing content. No documents will be deleted - only reorganized and consolidated where appropriate.

## New Directory Structure

```
/docs/
├── README.md                    # Main navigation guide
├── /for-racers/                # NEW - User-focused racing optimization
├── /for-developers/            # REORGANIZED from current /development/
├── /reference/                 # Core technical documentation
├── /project-history/           # NEW - Development journey and decisions
├── /guides/                    # EXISTING - Operational guides
├── /research/                  # EXISTING - Research and investigations
├── /archive/                   # EXISTING - Old versions
└── /screenshots/               # EXISTING - Visual documentation
```

## Detailed Migration Plan

### 1. Root Level Files (`/docs/`)

**KEEP in root:**
- `README.md` - Transform into main navigation guide
- `COMPREHENSIVE_TESTING_GUIDE.md` - High-level testing overview
- `TEST_SUITE_SUMMARY.md` - Current test status

**MOVE to `/reference/`:**
- `ALGORITHMS.md` → `/reference/ALGORITHMS.md`
- `ARCHITECTURE.md` → `/reference/ARCHITECTURE.md`
- `ZWIFT_DOMAIN.md` → `/reference/ZWIFT_DOMAIN.md`

**KEEP in root:**
- `PROJECT_WISDOM.md` - Key learnings and insights (stays in root)

**MOVE to `/for-developers/`:**
- `ROUTE_DATA_EXTRACTION.md` → `/for-developers/data-extraction/ROUTE_DATA_EXTRACTION.md`
- `ZWIFT_OFFLINE_INTEGRATION.md` → `/for-developers/integrations/ZWIFT_OFFLINE_INTEGRATION.md`

### 2. Development Directory (`/development/`)

**Testing Documentation** → `/for-developers/testing/`
- `MODERN_TESTING_STRATEGY.md` → `/for-developers/testing/MODERN_TESTING_STRATEGY.md`
- `TEST_REORGANIZATION_SUMMARY.md` → `/for-developers/testing/current/TEST_REORGANIZATION_SUMMARY.md`
- `WHY_NOT_100_PERCENT_COVERAGE.md` → `/for-developers/testing/philosophy/WHY_NOT_100_PERCENT_COVERAGE.md`
- `INLINE_TEST_GUIDELINES.md` → `/for-developers/testing/guidelines/INLINE_TEST_GUIDELINES.md`
- `MUTATION_TESTING_EVALUATION_SUMMARY.md` → `/for-developers/testing/techniques/MUTATION_TESTING.md`
- `TEST_FIXES_*.md` → `/project-history/test-evolution/`
- `PHASE*_COVERAGE_PLAN_*.md` → `/project-history/coverage-plans/`

**Refactoring Documentation** → `/for-developers/refactoring/`
- `RUST_REFACTORING_BEST_PRACTICES.md` → `/for-developers/refactoring/BEST_PRACTICES.md`
- `MODERN_RUST_ENHANCEMENTS.md` → `/for-developers/refactoring/MODERN_RUST.md`
- `INLINE_TESTS_REFACTORING_PLAN.md` → `/project-history/refactoring-plans/INLINE_TESTS.md`
- `*_REFACTORING_PLAN_*.md` → `/project-history/refactoring-plans/`

**API Documentation** → `/for-developers/api-research/`
- `ZWIFT_API_LOG.md` + `ZWIFT_API_LOG_RECENT.md` + `ZWIFT_API_LOG_SUMMARY.md` → 
  - CONSOLIDATE into `/for-developers/api-research/API_KNOWLEDGE_BASE.md`
  - Move historical logs to `/project-history/api-discoveries/`

**Migration Plans** → `/for-developers/active-plans/`
- `MIGRATION_TO_UOM_PLAN.md` → `/for-developers/active-plans/UOM_MIGRATION_PLAN.md` (ACTIVE)
- `UOM_CRATE_EVALUATION.md` → `/for-developers/active-plans/UOM_EVALUATION.md` (ACTIVE)
- All other UOM files → `/for-developers/active-plans/uom/` (ACTIVE WORK)

**Other Development Files:**
- `plan.md` → `/for-developers/CURRENT_DEVELOPMENT_PLAN.md`
- `HISTORICAL_DISCOVERIES.md` → `/project-history/DISCOVERIES_TIMELINE.md`
- `PHYSICAL_STATS.md` → `/reference/PHYSICAL_STATS.md`
- `DATABASE.md` → `/reference/DATABASE_SCHEMA.md`
- `SIMULATION_TOOLS.md` → `/research/SIMULATION_TOOLS.md`
- `FEEDBACK.md` → `/for-developers/COMMUNITY_FEEDBACK.md`
- `ACCURACY_TIMELINE.md` → `/project-history/ACCURACY_TIMELINE.md`
- `ERROR_INVESTIGATION_*.md` → `/project-history/debugging/`

### 3. New User-Focused Content (`/for-racers/`)

**CREATE NEW (Initial drafts - to be expanded):**
- `README.md` - "Optimize Your Zwift Racing Performance" (DRAFT)
- `POWER_OPTIMIZATION.md` - Managing the only variable you control (INITIAL CONCEPTS)
- `DRAFT_STRATEGIES.md` - Maximizing draft benefit (EARLY DRAFT)
- `ROUTE_TACTICS.md` - Route-specific power distribution (PLACEHOLDER WITH BASIC IDEAS)
- `ZWIFT_AS_GAME.md` - Understanding and exploiting game mechanics (PHILOSOPHY DRAFT)
- `CATEGORY_RACING.md` - Optimizing for your category (OUTLINE ONLY)

**Note:** These documents will start as basic outlines and grow as we gather more racing insights and user feedback.

### 4. Research Directory (`/research/`)

**KEEP AS IS:**
- All existing research documents remain in place

**UPDATE:**
- `ZWIFT_PHYSICS_EQUATIONS.md` - Remove "controversy" language, add game mechanics perspective

**ADD NEW:**
- `HEIGHT_WEIGHT_PHILOSOPHY.md` - Move from wip-claude research

### 5. Guides Directory (`/guides/`)

**KEEP AS IS:**
- All operational guides remain in current location
- These are well-organized and serve their purpose

### 6. Archive Directory (`/archive/`)

**KEEP AS IS:**
- Existing archived documents remain
- Will receive more content as timestamped plans are moved to project-history

## Consolidation Strategy

### Documents to Consolidate:

1. **API Logs** (3 files → 1 comprehensive + historical archive)
   - Create comprehensive API_KNOWLEDGE_BASE.md
   - Archive individual session logs

2. **Test Documentation** (15+ files → organized subdirectories)
   - Current strategy and guidelines in main directory
   - Historical plans and fixes in project-history

3. **Coverage Plans** (5+ timestamped files → project-history)
   - Keep latest as current plan
   - Archive iterations showing evolution

### Documents to Keep Separate:

1. **Research documents** - Each addresses distinct topics
2. **Operational guides** - Each serves specific setup/operation need
3. **Reference documents** - Core technical specs need individual files

## Implementation Order

1. Create new directory structure
2. Create navigation README files in each directory
3. Move files according to plan (no content changes yet)
4. Consolidate API logs and test documentation
5. Create new user-focused content
6. Update existing docs to reflect new philosophy (game mechanics vs physics complaints)

## Success Criteria

- Every audience can find relevant documentation quickly
- Development history is preserved and organized
- No information is lost
- Clear separation between active development and historical record
- User-focused content is prominent and accessible

## Notes

- All moves preserve git history
- Consolidation involves creating new summary docs, not deleting originals
- Historical documents provide context for future development
- New structure supports both ongoing development and user needs