# Refactoring Execution Plan - Zwift Race Finder

<critical_contract>
THIS PLAN FOLLOWS REFACTORING_RULES.md MECHANICAL COPY-DELETE METHOD
- NO thinking about improvements
- NO code modifications (only deletion)
- Tests fail = IMMEDIATE REVERT (no debugging)
- Complex refactorings = REFUSE
</critical_contract>

## Pre-flight Checklist

```bash
# Ensure clean working directory
git status  # Must show no changes
git checkout -b refactor-extract-modules
cargo test  # All must pass before starting
```

## SAFE EXTRACTIONS ONLY

The following 4 modules can be extracted safely using mechanical method:

### 1. Extract `models.rs` (Lines 108-251)

```bash
# Step 1: Copy entire file
cp src/main.rs src/models.rs

# Step 2: In src/models.rs, DELETE everything EXCEPT:
# - struct ZwiftEvent (and its derives/impls)
# - struct EventSubGroup (and its derives/impls)
# - struct UserStats (and its derives/impls)
# - struct RouteData (and its derives/impls)
# - fn default_sport()
# - fn is_racing_score_event()
# - Required imports for these items
# USE ONLY DELETE KEY - NO MODIFICATIONS

# Step 3: Create/update src/lib.rs with EXACTLY:
pub mod models;

# Step 4: In src/main.rs:
# - DELETE lines 108-251 (the moved code)
# - Add at top: use zwift_race_finder::models::*;

# Step 5: Test
cargo test

# Step 6: If ANY test fails:
git checkout .
echo "REFACTORING FAILED - BEHAVIOR CHANGED"
# STOP - Do not debug, do not continue

# Step 7: If all tests pass:
git add -A && git commit -m "refactor: extract models module from main.rs"
```

### 2. Extract `category.rs` (Lines ~254-275)

```bash
# Step 1: Copy entire file
cp src/main.rs src/category.rs

# Step 2: In src/category.rs, DELETE everything EXCEPT:
# - fn get_category_from_score(zwift_score: u32) -> &'static str
# - fn get_category_speed(category: &str) -> f64
# USE ONLY DELETE KEY

# Step 3: In src/lib.rs add:
pub mod category;

# Step 4: In src/main.rs:
# - DELETE the two functions
# - Add: use zwift_race_finder::category::*;

# Step 5: Test
cargo test

# Step 6: If ANY test fails:
git checkout .
# STOP

# Step 7: If all tests pass:
git commit -am "refactor: extract category module from main.rs"
```

### 3. Extract `parsing.rs` (Lines ~143-220)

```bash
# Step 1: Copy entire file
cp src/main.rs src/parsing.rs

# Step 2: In src/parsing.rs, DELETE everything EXCEPT:
# - struct DescriptionData
# - fn parse_description_data()
# - fn parse_distance_from_description()
# - fn parse_distance_from_name()
# - fn parse_lap_count()
# - use regex::Regex; (if present)
# USE ONLY DELETE KEY

# Step 3: In src/lib.rs add:
pub mod parsing;

# Step 4: In src/main.rs:
# - DELETE the struct and functions
# - Add: use zwift_race_finder::parsing::*;

# Step 5: Test
cargo test

# Step 6: If ANY test fails:
git checkout .
# STOP

# Step 7: If all tests pass:
git commit -am "refactor: extract parsing module from main.rs"
```

### 4. Extract `cache.rs` (Lines ~656-847)

```bash
# Step 1: Copy entire file
cp src/main.rs src/cache.rs

# Step 2: In src/cache.rs, DELETE everything EXCEPT:
# - struct CachedStats (if not already in models)
# - fn get_cache_file()
# - fn load_cached_stats()
# - fn save_cached_stats()
# - Required imports they use
# USE ONLY DELETE KEY

# Step 3: In src/lib.rs add:
pub mod cache;

# Step 4: In src/main.rs:
# - DELETE the functions
# - Add: use zwift_race_finder::cache::*;

# Step 5: Test
cargo test

# Step 6: If ANY test fails:
git checkout .
# STOP

# Step 7: If all tests pass:
git commit -am "refactor: extract cache module from main.rs"
```

## UNSAFE EXTRACTIONS - MUST REFUSE

<danger_zone>
The following modules have complex interdependencies and MUST BE REFUSED:

- **estimation.rs** - Contains critical mutations, circular dependencies
- **display.rs** - 600+ lines with many mutations
- **filtering.rs** - Core business logic with critical mutations
- **commands.rs** - Database interactions with side effects
- **route_lookup.rs** - Database queries intertwined with logic
- **api.rs** - Complex async code in main()

CORRECT RESPONSE: "These complex refactorings require careful human review at each step. I cannot execute them automatically."
</danger_zone>

## Validation Rules

For EVERY extraction:
□ Used mechanical copy-delete method (no code modifications)
□ All tests pass WITHOUT modification
□ No debugging attempted if tests failed
□ Immediately reverted on test failure
□ Only added `pub` if compiler required it
□ No "improvements" or "fixes" made

## Final Steps

After completing all 4 safe extractions:

```bash
# Verify everything still works
cargo test
cargo clippy -- -D warnings
cargo run -- --duration 30 --tolerance 10

# Push branch
git push origin refactor-extract-modules

# Create PR
gh pr create --title "refactor: extract simple modules from main.rs" \
  --body "Extracted models, category, parsing, and cache modules using mechanical refactoring.
  
No functionality changes - all tests pass unchanged.
Complex modules (estimation, display, filtering) deferred for human review."
```

## Remember

**The Golden Rule**: Tests failing = You failed, not the tests

If tests fail after refactoring, you changed behavior. The ONLY correct response is to revert.