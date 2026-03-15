# Documentation Migration Analysis
Created: 2025-06-19 21:55:00

## Current State

New directory structure exists with navigation READMEs:
- `docs/for-racers/` - User guides for racing optimization (5 guides + README)
- `docs/for-developers/` - Technical documentation (README only)
- `docs/reference/` - Core algorithms, schemas (README only)
- `docs/project-history/` - Historical context (README only)
- `docs/research/` - Existing directory with research notes
- `docs/guides/` - Existing directory with operational guides

## Critical External References Found

### In README.md (Most Critical - Public Facing)
1. `docs/ZWIFT_OFFLINE_INTEGRATION.md` - Line 233
2. `docs/ROUTE_DATA_EXTRACTION.md` - Line 475
3. `docs/guides/DATA_IMPORT.md` - Line 320 (already in correct location)

### In CLAUDE.md (Project Instructions)
1. `docs/guides/DATA_IMPORT.md#zwiftpower` - Line 59
2. `docs/ZWIFT_DOMAIN.md` - Line 70
3. Generic "docs/ subdirectories" - Line 94

### In docs/README.md (Documentation Navigation Hub)
1. `ALGORITHMS.md` - Line 25 (relative link)
2. `ARCHITECTURE.md` - Line 26 (relative link)
3. `ZWIFT_DOMAIN.md` - Line 27 (relative link)
4. Multiple navigation links use `/docs/` prefix

### In Code Files
1. `benches/ocr_benchmarks.rs` - references `docs/screenshots/` (physical path)
2. `src/ocr_parallel.rs` - references `docs/screenshots/` (physical path)
3. `tests/*.rs` - multiple files reference `docs/screenshots/` (physical paths)

## Migration Risk Assessment

### HIGH RISK (Break External Links)
- `docs/ZWIFT_OFFLINE_INTEGRATION.md` → Referenced in README
- `docs/ROUTE_DATA_EXTRACTION.md` → Referenced in README
- `docs/ZWIFT_DOMAIN.md` → Referenced in CLAUDE.md

### MEDIUM RISK (Break Development Workflows)
- `docs/ARCHITECTURE.md` → May be referenced in sessions/wip-claude
- `docs/ALGORITHMS.md` → May be referenced in sessions/wip-claude
- `docs/PROJECT_WISDOM.md` → Active development reference

### LOW RISK (Internal Only)
- All files under `docs/development/`
- All files under `docs/archive/`
- Files in sessions that reference docs

### NO RISK (Already Correct)
- `docs/guides/` - Already in final location
- `docs/research/` - Already in final location  
- `docs/screenshots/` - Must stay for code references

## Files That MUST Stay Put

1. **docs/screenshots/** - Referenced by actual code paths in:
   - `benches/ocr_benchmarks.rs`
   - `src/ocr_parallel.rs`
   - `tests/ocr_snapshot_tests.rs`
   - `tests/ocr_tests.rs`
   - `tests/ocr_integration_tests.rs`

## Recommended Migration Order

### Phase 1: Safe Internal Moves (No External Impact)
Move files that have no external references:

1. **To `docs/for-developers/`**:
   - `docs/development/AI_DEVELOPMENT.md`
   - `docs/development/MODERN_TESTING_STRATEGY.md`
   - `docs/development/MUTATION_TESTING_GUIDE.md`
   - `docs/development/REFACTORING_*.md` (all refactoring docs)
   - `docs/development/TEST_*.md` (test-related docs)
   - `docs/development/RUST_*.md` (Rust-specific docs)
   - `docs/development/OCR_*.md` (OCR development docs)
   - `docs/development/UOM_*.md` and `20250108_*_uom_*.md` (UoM migration docs)
   - `docs/development/GOLDEN_TEST_STRATEGY.md`
   - `docs/development/INTEGRATION_TEST_COVERAGE.md`

2. **To `docs/project-history/`**:
   - `docs/development/ACCURACY_TIMELINE.md`
   - `docs/development/HISTORICAL_DISCOVERIES.md`
   - `docs/development/ZWIFT_API_LOG*.md` (all API logs)
   - `docs/development/FEEDBACK.md`
   - `docs/archive/*` (all archived files)

3. **To `docs/reference/`**:
   - `docs/development/DATABASE.md`
   - `docs/development/PHYSICAL_STATS.md`

### Phase 2: Update Internal References
Before moving high-risk files, update all references in:
- `README.md`
- `CLAUDE.md`
- `HANDOFF.md`

### Phase 3: Move High-Risk Files (After References Updated)

1. **To `docs/reference/`**:
   - `docs/ARCHITECTURE.md`
   - `docs/ALGORITHMS.md`
   - `docs/ZWIFT_DOMAIN.md`

2. **To `docs/guides/`** (or stay if already there):
   - `docs/ZWIFT_OFFLINE_INTEGRATION.md`
   - `docs/ROUTE_DATA_EXTRACTION.md`

### Phase 4: Final Cleanup

1. **Consider Moving**:
   - `docs/PROJECT_WISDOM.md` → Maybe to root level with HANDOFF.md?
   - `docs/TEST_SUITE_SUMMARY.md` → To `docs/for-developers/`
   - `docs/development/plan.md` → To `docs/for-developers/CURRENT_DEVELOPMENT_PLAN.md`
   - `docs/development/log_with_context.md` → To `docs/project-history/`
   - `docs/development/FILES_REVIEW_LIST.md` → To `docs/for-developers/`
   - `docs/development/SIMULATION_TOOLS.md` → To `docs/for-developers/`

2. **Delete Empty Directories**:
   - `docs/development/` (after all files moved)
   - `docs/archive/` (after all files moved)

3. **Update docs/README.md Links**:
   - `ALGORITHMS.md` → `reference/ALGORITHMS.md`
   - `ARCHITECTURE.md` → `reference/ARCHITECTURE.md`
   - `ZWIFT_DOMAIN.md` → `reference/ZWIFT_DOMAIN.md`
   - Update navigation links from `/docs/` to relative paths

## Implementation Commands

### Phase 1 Example Commands:
```bash
# Move development docs to for-developers
mv docs/development/AI_DEVELOPMENT.md docs/for-developers/
mv docs/development/MODERN_TESTING_STRATEGY.md docs/for-developers/
mv docs/development/MUTATION_TESTING_GUIDE.md docs/for-developers/
mv docs/development/REFACTORING_*.md docs/for-developers/
mv docs/development/TEST_*.md docs/for-developers/
mv docs/development/RUST_*.md docs/for-developers/
mv docs/development/OCR_*.md docs/for-developers/
mv docs/development/UOM_*.md docs/for-developers/
mv docs/development/20250108_*_uom_*.md docs/for-developers/
mv docs/development/GOLDEN_TEST_STRATEGY.md docs/for-developers/
mv docs/development/INTEGRATION_TEST_COVERAGE.md docs/for-developers/

# Move history docs to project-history
mv docs/development/ACCURACY_TIMELINE.md docs/project-history/
mv docs/development/HISTORICAL_DISCOVERIES.md docs/project-history/
mv docs/development/ZWIFT_API_LOG*.md docs/project-history/
mv docs/development/FEEDBACK.md docs/project-history/
mv docs/archive/* docs/project-history/

# Move reference docs
mv docs/development/DATABASE.md docs/reference/
mv docs/development/PHYSICAL_STATS.md docs/reference/
```

### Phase 2 Reference Updates:
```bash
# Update README.md
# Line 233: docs/ZWIFT_OFFLINE_INTEGRATION.md → docs/guides/ZWIFT_OFFLINE_INTEGRATION.md
# Line 475: docs/ROUTE_DATA_EXTRACTION.md → docs/guides/ROUTE_DATA_EXTRACTION.md

# Update CLAUDE.md  
# Line 70: docs/ZWIFT_DOMAIN.md → docs/reference/ZWIFT_DOMAIN.md
```

## Summary

**Safe to move immediately**: 30+ files in `docs/development/` and `docs/archive/`

**Requires reference updates first**: 5 files referenced in README/CLAUDE

**Must stay in place**: `docs/screenshots/` directory (code dependencies)

**Already in correct location**: `docs/guides/`, `docs/research/`

The migration can be done incrementally, starting with Phase 1 which has zero risk of breaking anything.

## Risk Mitigation Strategy

1. **Before Starting**:
   - Create a git branch for the migration
   - Run all tests to ensure baseline
   - Document current working directory structure

2. **During Migration**:
   - Move files in small batches
   - Update references immediately after each move
   - Test documentation links after each phase
   - Keep a log of moves for potential rollback

3. **After Migration**:
   - Run full test suite
   - Manually verify all README links work
   - Check that development workflows still function
   - Update any CI/CD documentation references

## Files Needing Special Attention

1. **docs/PROJECT_WISDOM.md** - Duplicate of root PROJECT_WISDOM.md, can be deleted
2. **COMPREHENSIVE_TESTING_GUIDE.md** - Already at root level (correct location)
3. **todo.md** - Already at root level (correct location)
4. **HANDOFF.md** references - Update after Phase 3
5. **docs/README.md** - Navigation hub, needs link updates
6. **docs/TEST_SUITE_SUMMARY.md** - Should move to docs/for-developers/
7. **Session files** - Many reference old paths but are historical records
8. **wip-claude files** - Many reference old docs/ structure but are historical

## Quick Reference - Final Structure

```
docs/
├── for-racers/          # User guides for racing
├── for-developers/      # Technical documentation
├── reference/           # Core algorithms, schemas
├── project-history/     # Historical context
├── research/            # Deep technical research (stays)
├── guides/              # Operational guides (stays)
├── screenshots/         # Test data (stays - code deps)
└── README.md           # Navigation hub
```

## Final Recommendations

1. **Start with Phase 1** - Move all internal-only files first (zero risk)
2. **Test After Each Phase** - Run tests and check documentation links
3. **Keep Historical References** - Don't update old session/wip-claude files
4. **Delete Duplicates** - Remove docs/PROJECT_WISDOM.md (duplicate of root)
5. **Update Navigation** - Fix docs/README.md links after each phase
6. **Preserve Code Paths** - Never move docs/screenshots/ (code dependencies)

## Expected Outcome

After migration:
- 0 files in `docs/development/` (directory removed)
- 0 files in `docs/archive/` (directory removed)
- ~20 files moved to `docs/for-developers/`
- ~10 files moved to `docs/project-history/`
- ~5 files moved to `docs/reference/`
- All external links updated and working
- Clear, audience-focused documentation structure